//! Virtual Scrolling.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

// ============================================================================
// Types
// ============================================================================

/// A visible virtual item.
#[derive(Debug, Clone)]
pub struct VirtualItem<T> {
    /// Index.
    pub index: usize,
    /// Start row.
    pub start: f64,
    /// End row.
    pub end: f64,
    /// Height.
    pub height: f64,
    /// Data.
    pub data: T,
}

/// Item height measure signature.
pub type MeasureHeight<T> = dyn Fn(&T, usize) -> f64 + Send + Sync;

/// Manager options.
pub struct VirtualScrollOptions<T> {
    /// Item count.
    pub item_count: usize,
    /// Item getter.
    pub get_item: Arc<dyn Fn(usize) -> T + Send + Sync>,
    /// Height measure function.
    pub measure_height: Box<MeasureHeight<T>>,
    /// Overscan items.
    pub overscan: usize,
    /// Container height in rows.
    pub container_height: f64,
    /// Smooth scrolling enabled.
    pub smooth_scroll: bool,
    /// Scroll duration in ms.
    pub scroll_duration: u64,
    /// Initial offset.
    pub initial_offset: f64,
    /// Scroll callback.
    pub on_scroll: Box<dyn FnMut(f64) + Send + Sync>,
    /// End callback.
    pub on_end_reached: Box<dyn FnMut() + Send + Sync>,
    /// End threshold in rows.
    pub end_reached_threshold: usize,
    /// Estimated item height.
    pub estimated_item_height: f64,
}

impl<T> VirtualScrollOptions<T> {
    /// Build options with defaults.
    pub fn with_defaults(
        item_count: usize,
        get_item: impl Fn(usize) -> T + Send + Sync + 'static,
        container_height: f64,
    ) -> Self {
        Self {
            item_count,
            get_item: Arc::new(get_item),
            measure_height: Box::new(|_, _| 1.0),
            overscan: 3,
            container_height,
            smooth_scroll: true,
            scroll_duration: 150,
            initial_offset: 0.0,
            on_scroll: Box::new(|_| {}),
            on_end_reached: Box::new(|| {}),
            end_reached_threshold: 5,
            estimated_item_height: 1.0,
        }
    }
}

/// Runtime state.
#[derive(Debug, Clone)]
pub struct VirtualScrollState {
    pub offset: f64,
    pub target_offset: f64,
    pub is_scrolling: bool,
    pub height_cache: HashMap<usize, f64>,
    pub position_cache: HashMap<usize, f64>,
    pub total_height: f64,
    pub dirty: bool,
}

impl Default for VirtualScrollState {
    fn default() -> Self {
        Self {
            offset: 0.0,
            target_offset: 0.0,
            is_scrolling: false,
            height_cache: HashMap::new(),
            position_cache: HashMap::new(),
            total_height: 0.0,
            dirty: true,
        }
    }
}

/// Visible result payload.
#[derive(Debug, Clone)]
pub struct VirtualScrollResult<T> {
    pub items: Vec<VirtualItem<T>>,
    pub total_height: f64,
    pub offset: f64,
    pub container_height: f64,
    pub can_scroll_up: bool,
    pub can_scroll_down: bool,
    pub start_index: usize,
    pub end_index: usize,
    pub scroll_progress: f64,
}

// ============================================================================
// Manager
// ============================================================================

/// Virtual scroll manager.
pub struct VirtualScrollManager<T> {
    options: VirtualScrollOptions<T>,
    state: VirtualScrollState,
}

impl<T> VirtualScrollManager<T> {
    /// New manager.
    pub fn new(options: VirtualScrollOptions<T>) -> Self {
        let initial_offset = options.initial_offset.max(0.0);
        let mut manager = Self {
            options,
            state: VirtualScrollState {
                offset: initial_offset,
                target_offset: initial_offset,
                ..Default::default()
            },
        };
        manager.recalculate_layout();
        manager
    }

    /// Recompute full layout.
    pub fn recalculate_layout(&mut self) {
        let mut position = 0.0;
        self.state.position_cache.clear();
        for index in 0..self.options.item_count {
            self.state.position_cache.insert(index, position);
            let height = self.get_cached_height(index);
            position += height;
        }
        self.state.total_height = position;
        self.state.offset = self
            .state
            .offset
            .min((self.state.total_height - self.options.container_height).max(0.0));
        self.state.target_offset = self.state.offset;
        self.state.dirty = false;
    }

    fn get_cached_height(&mut self, index: usize) -> f64 {
        if let Some(&height) = self.state.height_cache.get(&index) {
            return height;
        }
        let item = (self.options.get_item)(index);
        let raw = (self.options.measure_height)(&item, index);
        let height = if raw.is_finite() && raw > 0.0 {
            raw
        } else {
            self.options.estimated_item_height
        };
        self.state.height_cache.insert(index, height);
        height
    }

    /// Item height.
    pub fn get_item_height(&mut self, index: usize) -> f64 {
        self.get_cached_height(index)
    }

    /// Start position.
    pub fn get_item_position(&mut self, index: usize) -> f64 {
        if self.state.dirty {
            self.recalculate_layout();
        }
        self.state
            .position_cache
            .get(&index)
            .copied()
            .unwrap_or_default()
    }

    /// Update item height cache.
    pub fn update_item_height(&mut self, index: usize, height: f64) {
        let resolved = if height.is_finite() && height > 0.0 {
            height
        } else {
            self.options.estimated_item_height
        };
        self.state.height_cache.insert(index, resolved);
        self.state.dirty = true;
    }

    /// Invalidate all cached heights.
    pub fn invalidate_heights(&mut self) {
        self.state.height_cache.clear();
        self.state.position_cache.clear();
        self.state.dirty = true;
    }

    /// Scroll absolute.
    pub fn scroll_to(&mut self, offset: f64) {
        let max_offset = (self.state.total_height - self.options.container_height).max(0.0);
        self.state.target_offset = offset.max(0.0).min(max_offset);

        if self.options.smooth_scroll {
            self.animate_scroll();
        } else {
            self.state.offset = self.state.target_offset;
            (self.options.on_scroll)(self.state.offset);
        }
        self.check_end_reached();
    }

    /// Scroll by delta.
    pub fn scroll_by(&mut self, delta: f64) {
        self.scroll_to(self.state.offset + delta);
    }

    /// Scroll to index.
    pub fn scroll_to_item(&mut self, index: usize, align: &str) {
        if self.options.item_count == 0 || index >= self.options.item_count {
            return;
        }
        if self.state.dirty {
            self.recalculate_layout();
        }

        let item_start = self.get_item_position(index);
        let item_height = self.get_item_height(index);
        let item_end = item_start + item_height;
        let target = match align {
            "start" => item_start,
            "center" => item_start - (self.options.container_height - item_height) / 2.0,
            "end" => item_end - self.options.container_height,
            _ => {
                if item_start < self.state.offset {
                    item_start
                } else if item_end > self.state.offset + self.options.container_height {
                    item_end - self.options.container_height
                } else {
                    self.state.offset
                }
            }
        };
        self.scroll_to(target);
    }

    /// Scroll top.
    pub fn scroll_to_top(&mut self) {
        self.scroll_to(0.0);
    }

    /// Scroll bottom.
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_to(self.state.total_height - self.options.container_height);
    }

    fn animate_scroll(&mut self) {
        self.state.is_scrolling = true;
        let start_offset = self.state.offset;
        let target = self.state.target_offset;
        let distance = target - start_offset;

        if distance.abs() <= f64::EPSILON || self.options.scroll_duration == 0 {
            self.state.offset = target;
            self.state.is_scrolling = false;
            (self.options.on_scroll)(self.state.offset);
            return;
        }

        let start = Instant::now();
        let duration = Duration::from_millis(self.options.scroll_duration.max(1));
        while start.elapsed() < duration {
            let t = start.elapsed().as_secs_f64() / duration.as_secs_f64();
            let eased = 1.0 - (1.0 - t).powi(3);
            self.state.offset = start_offset + distance * eased;
            (self.options.on_scroll)(self.state.offset);
            std::thread::sleep(Duration::from_millis(16));
        }
        self.state.offset = target;
        self.state.is_scrolling = false;
        (self.options.on_scroll)(self.state.offset);
    }

    fn check_end_reached(&mut self) {
        let remaining = self.state.total_height - self.state.offset - self.options.container_height;
        let threshold =
            self.options.end_reached_threshold as f64 * self.options.estimated_item_height;
        if remaining <= threshold && self.options.item_count > 0 {
            (self.options.on_end_reached)();
        }
    }

    /// Visible items.
    pub fn get_visible_items(&mut self) -> VirtualScrollResult<T> {
        if self.state.dirty {
            self.recalculate_layout();
        }

        if self.options.item_count == 0 {
            return VirtualScrollResult {
                items: Vec::new(),
                total_height: 0.0,
                offset: 0.0,
                container_height: self.options.container_height,
                can_scroll_up: false,
                can_scroll_down: false,
                start_index: 0,
                end_index: 0,
                scroll_progress: 0.0,
            };
        }

        let start_index = self
            .find_index_at_offset(self.state.offset)
            .saturating_sub(self.options.overscan);
        let end_visible = self.state.offset + self.options.container_height;
        let end_index = self
            .find_index_at_offset(end_visible)
            .saturating_add(self.options.overscan)
            .min(self.options.item_count - 1);

        let mut items = Vec::new();
        for index in start_index..=end_index {
            let start = self.get_item_position(index);
            let height = self.get_item_height(index);
            items.push(VirtualItem {
                index,
                start,
                end: start + height,
                height,
                data: (self.options.get_item)(index),
            });
        }

        let max_offset = (self.state.total_height - self.options.container_height).max(0.0);
        let scroll_progress = if max_offset > 0.0 {
            self.state.offset / max_offset
        } else {
            0.0
        };

        VirtualScrollResult {
            items,
            total_height: self.state.total_height,
            offset: self.state.offset,
            container_height: self.options.container_height,
            can_scroll_up: self.state.offset > 0.0,
            can_scroll_down: self.state.offset < max_offset,
            start_index,
            end_index,
            scroll_progress,
        }
    }

    /// Find item index for offset.
    pub fn find_index_at_offset(&mut self, offset: f64) -> usize {
        if self.options.item_count == 0 {
            return 0;
        }
        let mut low = 0usize;
        let mut high = self.options.item_count - 1;
        while low <= high {
            let mid = (low + high) / 2;
            let start = self.get_item_position(mid);
            let end = start + self.get_item_height(mid);
            if offset < start {
                if mid == 0 {
                    return 0;
                }
                high = mid.saturating_sub(1);
            } else if offset >= end {
                low = mid + 1;
            } else {
                return mid;
            }
        }
        high.min(self.options.item_count - 1)
    }

    /// Current offset.
    pub fn offset(&self) -> f64 {
        self.state.offset
    }

    /// Total height.
    pub fn total_height(&mut self) -> f64 {
        if self.state.dirty {
            self.recalculate_layout();
        }
        self.state.total_height
    }

    /// Active scroll state.
    pub fn is_scrolling(&self) -> bool {
        self.state.is_scrolling
    }

    /// Item at position.
    pub fn item_at_position(&mut self, position: f64) -> usize {
        self.find_index_at_offset(position)
    }

    /// Update count.
    pub fn set_item_count(&mut self, count: usize) {
        if count != self.options.item_count {
            self.options.item_count = count;
            self.state.height_cache.clear();
            self.state.position_cache.clear();
            self.state.dirty = true;
        }
    }

    /// Update container height.
    pub fn set_container_height(&mut self, height: f64) {
        self.options.container_height = height.max(0.0);
        self.state.dirty = true;
    }

    /// Update options via callback.
    pub fn update_options(&mut self, updater: impl FnOnce(&mut VirtualScrollOptions<T>)) {
        updater(&mut self.options);
        self.state.dirty = true;
    }

    /// Dispose.
    pub fn destroy(&mut self) {
        self.state.height_cache.clear();
        self.state.position_cache.clear();
    }
}

/// Create with custom options.
pub fn create_virtual_scroll<T>(options: VirtualScrollOptions<T>) -> VirtualScrollManager<T> {
    VirtualScrollManager::new(options)
}

/// Builder for fixed-height lists.
pub struct VirtualScrollOptionsFixedHeight<T> {
    pub item_count: usize,
    pub get_item: Arc<dyn Fn(usize) -> T + Send + Sync>,
    pub item_height: f64,
    pub container_height: f64,
    pub overscan: usize,
    pub smooth_scroll: bool,
    pub scroll_duration: u64,
    pub initial_offset: f64,
    pub on_scroll: Box<dyn FnMut(f64) + Send + Sync>,
    pub on_end_reached: Box<dyn FnMut() + Send + Sync>,
    pub end_reached_threshold: usize,
}

impl<T> Default for VirtualScrollOptionsFixedHeight<T> {
    fn default() -> Self {
        Self {
            item_count: 0,
            get_item: Arc::new(|_| panic!("get_item not set")),
            item_height: 1.0,
            container_height: 0.0,
            overscan: 3,
            smooth_scroll: true,
            scroll_duration: 150,
            initial_offset: 0.0,
            on_scroll: Box::new(|_| {}),
            on_end_reached: Box::new(|| {}),
            end_reached_threshold: 5,
        }
    }
}

/// Fixed height helper.
pub fn create_fixed_height_virtual_scroll<T>(
    options: VirtualScrollOptionsFixedHeight<T>,
) -> VirtualScrollManager<T> {
    let item_height = options.item_height;
    let measured = move |_item: &T, _index: usize| item_height;
    VirtualScrollManager::new(VirtualScrollOptions {
        measure_height: Box::new(measured),
        estimated_item_height: item_height,
        item_count: options.item_count,
        get_item: options.get_item,
        overscan: options.overscan,
        container_height: options.container_height,
        smooth_scroll: options.smooth_scroll,
        scroll_duration: options.scroll_duration,
        initial_offset: options.initial_offset,
        on_scroll: options.on_scroll,
        on_end_reached: options.on_end_reached,
        end_reached_threshold: options.end_reached_threshold,
    })
}

/// Scroll direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Up,
    Down,
    None,
}

/// Scroll direction helper.
pub fn get_scroll_direction(delta: f64) -> ScrollDirection {
    if delta < 0.0 {
        ScrollDirection::Up
    } else if delta > 0.0 {
        ScrollDirection::Down
    } else {
        ScrollDirection::None
    }
}

/// Percentage bounds.
pub fn get_visible_range_percent(
    offset: f64,
    container_height: f64,
    total_height: f64,
) -> (f64, f64) {
    if total_height <= 0.0 {
        (0.0, 1.0)
    } else {
        (
            offset / total_height,
            ((offset + container_height) / total_height).min(1.0),
        )
    }
}

/// Create scroll indicator geometry.
pub fn create_scroll_indicator(
    container_height: f64,
    total_height: f64,
    offset: f64,
    min_thumb_size: f64,
) -> (f64, f64) {
    if total_height <= container_height || container_height <= 0.0 {
        return (0.0, container_height);
    }

    let ratio = container_height / total_height;
    let size = (container_height * ratio).max(min_thumb_size);
    let max_position = (container_height - size).max(0.0);
    let max_scroll = (total_height - container_height).max(0.0);
    let position = if max_scroll > 0.0 {
        (offset / max_scroll) * max_position
    } else {
        0.0
    };
    (position.min(max_position), size)
}

/// Infinite scroll state.
#[derive(Debug, Clone)]
pub struct InfiniteScrollState<T> {
    pub items: Vec<T>,
    pub is_loading: bool,
    pub has_more: bool,
    pub error: Option<String>,
}

/// Infinite scroll manager with synchronous loader.
pub struct InfiniteScrollManager<T, F>
where
    F: FnMut(usize, usize) -> Result<Vec<T>, String>,
{
    load_more: F,
    pub state: InfiniteScrollState<T>,
}

impl<T, F> InfiniteScrollManager<T, F>
where
    F: FnMut(usize, usize) -> Result<Vec<T>, String>,
{
    /// Load next page.
    pub fn load_next_page(&mut self, page_size: usize) -> bool {
        if self.state.is_loading || !self.state.has_more {
            return false;
        }
        self.state.is_loading = true;
        let offset = self.state.items.len();
        match (self.load_more)(offset, page_size) {
            Ok(next) => {
                let is_empty = next.is_empty();
                self.state.items.extend(next);
                if is_empty {
                    self.state.has_more = false;
                }
                self.state.error = None;
            }
            Err(error) => self.state.error = Some(error),
        }
        self.state.is_loading = false;
        true
    }

    /// Replace items.
    pub fn set_items(&mut self, items: Vec<T>) {
        self.state.items = items;
    }

    /// Reset manager.
    pub fn reset(&mut self) {
        self.state = InfiniteScrollState {
            items: Vec::new(),
            is_loading: false,
            has_more: true,
            error: None,
        };
    }
}

/// Create an infinite manager.
pub fn create_infinite_scroll<T, F>(load_more: F) -> InfiniteScrollManager<T, F>
where
    F: FnMut(usize, usize) -> Result<Vec<T>, String>,
{
    InfiniteScrollManager {
        load_more,
        state: InfiniteScrollState {
            items: Vec::new(),
            is_loading: false,
            has_more: true,
            error: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_height_manager() {
        let options = VirtualScrollOptions {
            item_count: 2,
            get_item: Arc::new(|index| index),
            measure_height: Box::new(|_, _| 2.0),
            overscan: 1,
            container_height: 4.0,
            smooth_scroll: false,
            scroll_duration: 0,
            initial_offset: 0.0,
            on_scroll: Box::new(|_| {}),
            on_end_reached: Box::new(|| {}),
            end_reached_threshold: 1,
            estimated_item_height: 2.0,
        };
        let mut manager = create_virtual_scroll(options);
        let result = manager.get_visible_items();
        assert!(!result.items.is_empty());
        assert_eq!(result.total_height, 4.0);
    }
}
