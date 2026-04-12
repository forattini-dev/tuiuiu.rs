//! Graphics protocol support.
//!
//! This is a conservative port of the JS API surface with terminal-safe,
//! dependency-free Rust behavior.

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, Mutex,
};

// ============================================================================
// Types
// ============================================================================

/// Supported image protocols.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsProtocol {
    /// Kitty graphics protocol.
    Kitty,
    /// iTerm2 inline graphics.
    Iterm2,
    /// Sixel protocol.
    Sixel,
    /// Half-block fallback.
    Halfblock,
    /// Braille fallback.
    Braille,
    /// No explicit preference.
    None,
}

impl Display for GraphicsProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Kitty => "kitty",
            Self::Iterm2 => "iterm2",
            Self::Sixel => "sixel",
            Self::Halfblock => "halfblock",
            Self::Braille => "braille",
            Self::None => "none",
        };
        write!(f, "{name}")
    }
}

/// Cell dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellSize {
    /// Cell width in px.
    pub width: usize,
    /// Cell height in px.
    pub height: usize,
}

/// Image render options.
#[derive(Debug, Clone)]
pub struct ImageOptions {
    /// Target width in cells.
    pub width: Option<usize>,
    /// Target height in cells.
    pub height: Option<usize>,
    /// Scale mode.
    pub fit: Option<String>,
    /// Preserve aspect ratio.
    pub preserve_aspect_ratio: bool,
    /// Braille fallback threshold.
    pub threshold: Option<u8>,
    /// Enable dithering.
    pub dither: bool,
    /// Stable image id for protocol-managed caching.
    pub image_id: Option<u64>,
}

impl Default for ImageOptions {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            fit: None,
            preserve_aspect_ratio: true,
            threshold: None,
            dither: false,
            image_id: None,
        }
    }
}

/// Raw pixel source.
#[derive(Debug, Clone)]
pub struct ImageData {
    /// RGBA pixel buffer.
    pub pixels: Vec<u8>,
    /// Width in px.
    pub width: usize,
    /// Height in px.
    pub height: usize,
}

/// Capabilities common to all protocols.
#[derive(Debug, Clone)]
pub struct ProtocolCapabilities {
    /// Protocol type.
    pub protocol: GraphicsProtocol,
    /// Supports alpha.
    pub supports_transparency: bool,
    /// Supports animation sequences.
    pub supports_animation: bool,
    /// Max supported width in cells.
    pub max_width: Option<usize>,
    /// Max supported height in cells.
    pub max_height: Option<usize>,
}

/// Terminal image capabilities.
#[derive(Debug, Clone)]
pub struct TerminalImageCapabilities {
    /// Base capabilities.
    pub protocol: GraphicsProtocol,
    /// Base capabilities.
    pub supports_transparency: bool,
    /// Supports animation.
    pub supports_animation: bool,
    /// How it was detected.
    pub detected_by: String,
    /// Query support.
    pub supports_queries: bool,
    /// Can place images.
    pub supports_placement: bool,
    /// Can clear images.
    pub supports_clear: bool,
    /// Cell size.
    pub cell_size: CellSize,
}

/// Terminal image source.
#[derive(Debug, Clone)]
pub struct TerminalImageSource {
    /// Raw RGBA bytes.
    pub pixels: Vec<u8>,
    /// Pixel width.
    pub width: usize,
    /// Pixel height.
    pub height: usize,
    /// Cell size used.
    pub cell_size: CellSize,
    /// Desired columns.
    pub desired_columns: usize,
    /// Desired rows.
    pub desired_rows: usize,
    /// Hash key.
    pub hash: String,
}

/// Rect to render.
#[derive(Debug, Clone, Copy)]
pub struct ImagePixelRect {
    /// Position x.
    pub x: usize,
    /// Position y.
    pub y: usize,
    /// Width.
    pub width: usize,
    /// Height.
    pub height: usize,
}

/// Render plan.
#[derive(Debug, Clone)]
pub struct TerminalImageRenderPlan {
    /// Fit mode.
    pub fit: String,
    /// Cell size.
    pub cell_size: CellSize,
    /// Target columns.
    pub target_columns: usize,
    /// Target rows.
    pub target_rows: usize,
    /// Render columns.
    pub render_columns: usize,
    /// Render rows.
    pub render_rows: usize,
    /// Resized pixel width.
    pub resized_pixel_width: usize,
    /// Resized pixel height.
    pub resized_pixel_height: usize,
    /// Visible pixels rectangle.
    pub visible_pixels: ImagePixelRect,
}

/// Protocol options.
#[derive(Debug, Clone)]
pub struct TerminalImageProtocolRenderOptions {
    /// Source options.
    pub image: ImageOptions,
    /// Protocol.
    pub protocol: GraphicsProtocol,
}

impl From<ImageOptions> for TerminalImageProtocolRenderOptions {
    fn from(image: ImageOptions) -> Self {
        Self {
            image,
            protocol: GraphicsProtocol::None,
        }
    }
}

/// Render result.
#[derive(Debug, Clone)]
pub struct TerminalImageProtocolRenderResult {
    /// Cache key.
    pub cache_key: String,
    /// Terminal payload.
    pub payload: String,
    /// Render plan.
    pub plan: TerminalImageRenderPlan,
    /// Source.
    pub source: TerminalImageSource,
    /// Chosen protocol.
    pub protocol: GraphicsProtocol,
    /// Rendered as cell graphics.
    pub cell_render: bool,
    /// Reused cached item.
    pub reused: bool,
}

/// Protocol state.
#[derive(Debug)]
pub struct TerminalImageProtocolState {
    instance_key: String,
    kitty_image_id: u64,
    cache: Mutex<HashMap<String, u64>>,
    next_image_id: AtomicU64,
    hits: std::sync::atomic::AtomicUsize,
    misses: std::sync::atomic::AtomicUsize,
}

impl TerminalImageProtocolState {
    /// Render image with the selected protocol.
    pub fn render(
        &self,
        source_or_image_data: &ImageData,
        options: TerminalImageProtocolRenderOptions,
    ) -> TerminalImageProtocolRenderResult {
        let source = create_terminal_image_source(
            source_or_image_data.clone(),
            options.image.width,
            options.image.height,
            options.image.preserve_aspect_ratio,
        );
        let plan = plan_image_render(
            &source,
            ImageRenderRequest {
                target_columns: options.image.width,
                target_rows: options.image.height,
                preserve_aspect_ratio: options.image.preserve_aspect_ratio,
            },
        );
        let cache_key = self.get_cache_key(source_or_image_data, &options.image);
        let protocol = resolve_renderable_protocol(options.protocol);
        let cell_render = is_cell_graphics_protocol(protocol);
        let reused = {
            let mut cache = self.cache.lock().expect("cache lock");
            if cache.contains_key(&cache_key) {
                self.hits.fetch_add(1, Ordering::Relaxed);
                true
            } else {
                cache.insert(
                    cache_key.clone(),
                    self.next_image_id.fetch_add(1, Ordering::Relaxed),
                );
                self.misses.fetch_add(1, Ordering::Relaxed);
                false
            }
        };

        let payload = match protocol {
            GraphicsProtocol::Kitty => format!("kitty:{cache_key}:{}", source.hash),
            GraphicsProtocol::Iterm2 => format!("iterm2:{cache_key}:{}", source.hash),
            GraphicsProtocol::Sixel => format!("sixel:{cache_key}:{}", source.hash),
            GraphicsProtocol::Halfblock => "halfblock".to_string(),
            GraphicsProtocol::Braille => "braille".to_string(),
            GraphicsProtocol::None => "none".to_string(),
        };

        TerminalImageProtocolRenderResult {
            cache_key,
            payload,
            plan,
            source,
            protocol,
            cell_render,
            reused,
        }
    }

    /// Stable cache key.
    pub fn get_cache_key(
        &self,
        source_or_image_data: &ImageData,
        options: &ImageOptions,
    ) -> String {
        let src_key = hash_image_data(source_or_image_data);
        let size = format!(
            "{}x{}",
            options.width.unwrap_or(0),
            options.height.unwrap_or(0)
        );
        let modifiers = format!(
            "{}:{}:{}:{}:{}",
            options.preserve_aspect_ratio,
            options.threshold.unwrap_or(0),
            options.dither,
            options.fit.clone().unwrap_or_default(),
            size
        );
        format!("{src_key}:{modifiers}")
    }

    /// Drop one image from cache.
    pub fn invalidate(&self, key: Option<&str>) {
        let mut cache = self.cache.lock().expect("cache lock");
        if let Some(key) = key {
            cache.remove(key);
        } else {
            cache.clear();
        }
    }

    /// Clear all state.
    pub fn clear(&self) {
        self.invalidate(None);
        self.hits.store(0, Ordering::Relaxed);
        self.misses.store(0, Ordering::Relaxed);
    }

    /// Cache stats.
    pub fn stats(&self) -> (usize, usize, usize) {
        let len = self.cache.lock().expect("cache lock").len();
        (
            self.hits.load(Ordering::Relaxed),
            self.misses.load(Ordering::Relaxed),
            len,
        )
    }

    /// Read-only id.
    pub fn instance_key(&self) -> &str {
        &self.instance_key
    }

    /// Read-only kitty id.
    pub fn kitty_image_id(&self) -> u64 {
        self.kitty_image_id
    }
}

// ============================================================================
// Protocol detection and cache
// ============================================================================

static MANUAL_PROTOCOL_OVERRIDE: Mutex<Option<GraphicsProtocol>> = Mutex::new(None);
static DETECTED_PROTOCOL: Mutex<Option<GraphicsProtocol>> = Mutex::new(None);
static NEGOTIATED_CAPABILITIES: Mutex<Option<TerminalImageCapabilities>> = Mutex::new(None);
static CELL_SIZE_LISTENERS: Mutex<Vec<Arc<dyn Fn(CellSize) + Send + Sync>>> =
    Mutex::new(Vec::new());
static NEXT_STATE_ID: AtomicU64 = AtomicU64::new(1);
static NEXT_KITTY_IMAGE_ID: AtomicU64 = AtomicU64::new(1);

const DEFAULT_CELL_SIZE: CellSize = CellSize {
    width: 10,
    height: 20,
};

fn env_flag(name: &str) -> String {
    std::env::var(name).unwrap_or_default().to_lowercase()
}

fn supports_rich_color_fallback() -> bool {
    let term = env_flag("TERM");
    let color_term = env_flag("COLORTERM");
    let term_program = env_flag("TERM_PROGRAM");
    let force_color = env_flag("FORCE_COLOR");

    if force_color == "0" {
        return false;
    }
    if force_color == "2" || force_color == "3" {
        return true;
    }

    color_term == "truecolor"
        || color_term == "24bit"
        || term.contains("256color")
        || term.contains("direct")
        || term_program == "iterm.app"
        || term_program == "wezterm"
        || term_program == "hyper"
        || std::env::var("WT_SESSION").is_ok()
}

fn fallback_protocol() -> GraphicsProtocol {
    if supports_rich_color_fallback() {
        GraphicsProtocol::Halfblock
    } else {
        GraphicsProtocol::Braille
    }
}

fn detect_graphics_protocol_internal() -> GraphicsProtocol {
    if let Some(override_protocol) = MANUAL_PROTOCOL_OVERRIDE
        .lock()
        .expect("override lock")
        .clone()
    {
        return resolve_renderable_protocol(override_protocol);
    }

    if let Some(detected) = DETECTED_PROTOCOL.lock().expect("detected lock").clone() {
        return detected;
    }

    if std::env::var("KITTY_WINDOW_ID").is_ok() {
        return GraphicsProtocol::Kitty;
    }
    if env_flag("TERM_PROGRAM") == "iTerm.app" {
        return GraphicsProtocol::Iterm2;
    }
    if std::env::var("SIXEL").is_ok() {
        return GraphicsProtocol::Sixel;
    }

    fallback_protocol()
}

fn build_capabilities(
    protocol: GraphicsProtocol,
    cell_size: CellSize,
) -> TerminalImageCapabilities {
    match protocol {
        GraphicsProtocol::Kitty => TerminalImageCapabilities {
            protocol,
            supports_transparency: true,
            supports_animation: true,
            detected_by: "env".to_string(),
            supports_queries: true,
            supports_placement: true,
            supports_clear: true,
            cell_size,
        },
        GraphicsProtocol::Iterm2 => TerminalImageCapabilities {
            protocol,
            supports_transparency: true,
            supports_animation: true,
            detected_by: "env".to_string(),
            supports_queries: true,
            supports_placement: true,
            supports_clear: false,
            cell_size,
        },
        GraphicsProtocol::Sixel => TerminalImageCapabilities {
            protocol,
            supports_transparency: false,
            supports_animation: false,
            detected_by: "env".to_string(),
            supports_queries: false,
            supports_placement: true,
            supports_clear: true,
            cell_size,
        },
        GraphicsProtocol::Halfblock => TerminalImageCapabilities {
            protocol,
            supports_transparency: false,
            supports_animation: false,
            detected_by: "fallback".to_string(),
            supports_queries: false,
            supports_placement: false,
            supports_clear: false,
            cell_size,
        },
        GraphicsProtocol::Braille => TerminalImageCapabilities {
            protocol,
            supports_transparency: false,
            supports_animation: false,
            detected_by: "fallback".to_string(),
            supports_queries: false,
            supports_placement: false,
            supports_clear: false,
            cell_size,
        },
        GraphicsProtocol::None => build_capabilities(fallback_protocol(), cell_size),
    }
}

/// Resolve protocols to a rendered protocol.
pub fn resolve_renderable_protocol(protocol: GraphicsProtocol) -> GraphicsProtocol {
    if protocol == GraphicsProtocol::None {
        fallback_protocol()
    } else {
        protocol
    }
}

/// True when protocol is cell-based.
pub fn is_cell_graphics_protocol(protocol: GraphicsProtocol) -> bool {
    matches!(
        protocol,
        GraphicsProtocol::Halfblock | GraphicsProtocol::Braille
    )
}

/// True when protocol uses terminal escape payload.
pub fn is_protocol_graphics(protocol: GraphicsProtocol) -> bool {
    matches!(
        protocol,
        GraphicsProtocol::Kitty | GraphicsProtocol::Iterm2 | GraphicsProtocol::Sixel
    )
}

/// Return negotiated capabilities.
pub fn get_graphics_capabilities() -> TerminalImageCapabilities {
    if let Some(existing) = NEGOTIATED_CAPABILITIES.lock().expect("cap lock").clone() {
        return existing;
    }

    let protocol = detect_graphics_protocol();
    let caps = build_capabilities(protocol, DEFAULT_CELL_SIZE);
    *NEGOTIATED_CAPABILITIES.lock().expect("cap lock") = Some(caps.clone());
    caps
}

/// Detect protocol.
pub fn detect_graphics_protocol() -> GraphicsProtocol {
    let protocol = detect_graphics_protocol_internal();
    *DETECTED_PROTOCOL.lock().expect("detected lock") = Some(protocol);
    protocol
}

/// Manually force protocol.
pub fn set_graphics_protocol(protocol: GraphicsProtocol) {
    *MANUAL_PROTOCOL_OVERRIDE.lock().expect("override lock") = Some(protocol);
    let resolved = resolve_renderable_protocol(protocol);
    *NEGOTIATED_CAPABILITIES.lock().expect("cap lock") =
        Some(build_capabilities(resolved, DEFAULT_CELL_SIZE));
}

/// Current protocol (resolved).
pub fn get_graphics_protocol() -> GraphicsProtocol {
    resolve_renderable_protocol(detect_graphics_protocol_internal())
}

/// Convenience protocol capabilities.
pub fn get_protocol_capabilities() -> ProtocolCapabilities {
    let caps = get_graphics_capabilities();
    ProtocolCapabilities {
        protocol: caps.protocol,
        supports_transparency: caps.supports_transparency,
        supports_animation: caps.supports_animation,
        max_width: None,
        max_height: None,
    }
}

/// Reset detection cache.
pub fn reset_graphics_detection() {
    *DETECTED_PROTOCOL.lock().expect("detected lock") = None;
    *NEGOTIATED_CAPABILITIES.lock().expect("cap lock") = None;
    *MANUAL_PROTOCOL_OVERRIDE.lock().expect("override lock") = None;
}

/// Notify listeners when cell size changes.
pub fn on_cell_size_change(callback: impl Fn(CellSize) + Send + Sync + 'static) -> impl FnOnce() {
    let callback: Arc<dyn Fn(CellSize) + Send + Sync> = Arc::new(callback);
    CELL_SIZE_LISTENERS
        .lock()
        .expect("listener lock")
        .push(callback.clone());

    move || {
        let mut listeners = CELL_SIZE_LISTENERS.lock().expect("listener lock");
        listeners.retain(|listener| !Arc::ptr_eq(listener, &callback));
    }
}

/// Rebuild capabilities with new cell size.
pub fn invalidate_cell_size(new_size: Option<CellSize>) -> TerminalImageCapabilities {
    let size = new_size.unwrap_or(DEFAULT_CELL_SIZE);
    let protocol = get_graphics_protocol();
    let caps = build_capabilities(protocol, size);
    *NEGOTIATED_CAPABILITIES.lock().expect("cap lock") = Some(caps.clone());

    let listeners = CELL_SIZE_LISTENERS.lock().expect("listener lock").clone();
    for listener in listeners {
        listener(size);
    }

    caps
}

/// Create terminal source from raw image data.
pub fn create_terminal_image_source(
    data: ImageData,
    target_width: Option<usize>,
    target_height: Option<usize>,
    _preserve_aspect: bool,
) -> TerminalImageSource {
    let cell_size = DEFAULT_CELL_SIZE;
    let desired_columns = target_width
        .unwrap_or_else(|| ((data.width as f64) / (cell_size.width as f64)).ceil() as usize);
    let desired_rows = target_height
        .unwrap_or_else(|| ((data.height as f64) / (cell_size.height as f64)).ceil() as usize);
    let hash = hash_image_data(&data);
    TerminalImageSource {
        pixels: data.pixels,
        width: data.width,
        height: data.height,
        cell_size,
        desired_columns,
        desired_rows,
        hash,
    }
}

#[derive(Debug)]
pub struct ImageRenderRequest {
    target_columns: Option<usize>,
    target_rows: Option<usize>,
    preserve_aspect_ratio: bool,
}

/// Build render plan.
pub fn plan_image_render(
    source: &TerminalImageSource,
    request: ImageRenderRequest,
) -> TerminalImageRenderPlan {
    let target_columns = request
        .target_columns
        .unwrap_or(source.desired_columns.max(1));
    let target_rows = request.target_rows.unwrap_or(source.desired_rows.max(1));
    let img_ratio = (source.width as f64) / (source.height.max(1) as f64);
    let target_ratio = (target_columns as f64) / (target_rows.max(1) as f64);

    let (render_columns, render_rows) = if request.preserve_aspect_ratio && img_ratio > 0.0 {
        if img_ratio > target_ratio {
            (
                target_columns,
                (((target_columns as f64) / img_ratio).round() as usize).max(1),
            )
        } else {
            (
                (((target_rows as f64) * img_ratio).round() as usize).max(1),
                target_rows,
            )
        }
    } else {
        (target_columns, target_rows)
    };

    let resized_pixel_width = render_columns * source.cell_size.width;
    let resized_pixel_height = render_rows * source.cell_size.height;

    TerminalImageRenderPlan {
        fit: "contain".to_string(),
        cell_size: source.cell_size,
        target_columns,
        target_rows,
        render_columns,
        render_rows,
        resized_pixel_width,
        resized_pixel_height,
        visible_pixels: ImagePixelRect {
            x: 0,
            y: 0,
            width: source.width,
            height: source.height,
        },
    }
}

/// Render image into terminal payload (string placeholder format).
pub fn render_image(image_data: ImageData, options: Option<ImageOptions>) -> String {
    let options = options.unwrap_or_default();
    let source = create_terminal_image_source(
        image_data,
        options.width,
        options.height,
        options.preserve_aspect_ratio,
    );
    let state = create_terminal_image_protocol_state();
    let protocol = resolve_renderable_protocol(
        options
            .image_id
            .map_or(GraphicsProtocol::None, |_| GraphicsProtocol::Kitty),
    );
    let result = state.render(
        &ImageData {
            pixels: source.pixels.clone(),
            width: source.width,
            height: source.height,
        },
        TerminalImageProtocolRenderOptions {
            image: options,
            protocol,
        },
    );
    result.payload
}

/// Clear protocol-specific images.
pub fn clear_images_for_protocol(protocol: GraphicsProtocol) -> String {
    match protocol {
        GraphicsProtocol::Kitty => "clear_kitty_images",
        GraphicsProtocol::Iterm2 => "clear_iterm2_images",
        GraphicsProtocol::Sixel => "clear_sixel_images",
        GraphicsProtocol::Halfblock => "clear_halfblock_images",
        GraphicsProtocol::Braille => "clear_braille_images",
        GraphicsProtocol::None => "clear_images",
    }
    .to_string()
}

/// Clear all image caches.
pub fn clear_images() -> String {
    clear_images_for_protocol(GraphicsProtocol::Kitty);
    "clear_images".to_string()
}

/// Create protocol state.
pub fn create_terminal_image_protocol_state() -> TerminalImageProtocolState {
    TerminalImageProtocolState {
        instance_key: format!(
            "protocol-state-{}",
            NEXT_STATE_ID.fetch_add(1, Ordering::SeqCst)
        ),
        kitty_image_id: NEXT_KITTY_IMAGE_ID.fetch_add(1, Ordering::SeqCst),
        cache: Mutex::new(HashMap::new()),
        next_image_id: AtomicU64::new(1),
        hits: std::sync::atomic::AtomicUsize::new(0),
        misses: std::sync::atomic::AtomicUsize::new(0),
    }
}

/// Very small hash used for cache keys.
fn hash_image_data(image: &ImageData) -> String {
    let mut hash: u64 = 14695981039346656037;
    for byte in &image.pixels {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(1099511628211);
    }
    hash = hash.wrapping_add((image.width as u64) << 32) ^ (image.height as u64);
    format!("{:016x}", hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_fallback_is_deterministic() {
        let protocol = resolve_renderable_protocol(GraphicsProtocol::None);
        assert!(matches!(
            protocol,
            GraphicsProtocol::Halfblock | GraphicsProtocol::Braille
        ));
    }

    #[test]
    fn image_render_plan_has_positive_size() {
        let source = create_terminal_image_source(
            ImageData {
                pixels: vec![0; 16],
                width: 4,
                height: 4,
            },
            Some(10),
            Some(10),
            true,
        );
        let plan = plan_image_render(
            &source,
            ImageRenderRequest {
                target_columns: Some(10),
                target_rows: Some(10),
                preserve_aspect_ratio: true,
            },
        );
        assert!(plan.render_columns >= 1);
        assert!(plan.render_rows >= 1);
    }
}
