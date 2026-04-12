//! Picture Component
//!
//! Display ASCII art, character-based images, and colored pixel art.
//!
//! Features:
//! - ASCII art rendering
//! - Colored pixel art support
//! - Alignment and cropping
//! - Border and padding
//! - Pre-made ASCII patterns

use crate::core::component::{BorderStyle, BoxNode, BoxStyle, Color, NamedColor, TextStyle, VNode};
use crate::core::layout::FlexDirection;
use std::collections::HashMap;

// =============================================================================
// Pixel Types
// =============================================================================

/// A single pixel with character and optional colors.
#[derive(Debug, Clone)]
pub struct Pixel {
    /// The character to display
    pub char: char,
    /// Foreground color
    pub fg: Option<Color>,
    /// Background color
    pub bg: Option<Color>,
}

impl Pixel {
    /// Create a new pixel.
    pub fn new(char: char) -> Self {
        Self {
            char,
            fg: None,
            bg: None,
        }
    }

    /// Set foreground color.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }
}

/// A 2D pixel grid for colored ASCII art.
pub type PixelGrid = Vec<Vec<Pixel>>;

/// Color palette for mapping characters to colors.
pub type ColorPalette = HashMap<char, (Option<Color>, Option<Color>)>;

/// Create a pixel grid from source and color palette.
pub fn create_pixel_grid(source: &str, palette: &ColorPalette) -> PixelGrid {
    let mut grid = Vec::new();

    for line in source.lines() {
        if line.is_empty() {
            continue;
        }
        let mut row = Vec::new();
        for char in line.chars() {
            let (fg, bg) = palette.get(&char).cloned().unwrap_or((None, None));
            row.push(Pixel { char, fg, bg });
        }
        grid.push(row);
    }

    grid
}

/// Create a pixel grid from color map using block characters.
pub fn create_pixel_grid_from_colors(colors: &[Vec<Option<Color>>], char: char) -> PixelGrid {
    colors
        .iter()
        .map(|row| {
            row.iter()
                .map(|color| Pixel {
                    char: if color.is_some() { char } else { ' ' },
                    fg: color.clone(),
                    bg: None,
                })
                .collect()
        })
        .collect()
}

// =============================================================================
// Types
// =============================================================================

/// How the picture should fit within its container.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PictureFit {
    /// No scaling, show at original size
    #[default]
    None,
    /// Scale to fit, maintain aspect ratio, may have empty space
    Contain,
    /// Scale to cover, maintain aspect ratio, may crop
    Cover,
    /// Stretch to fill, may distort
    Fill,
    /// Crop to fit, no scaling
    Crop,
}

/// Horizontal alignment within container.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PictureAlignX {
    #[default]
    Left,
    Center,
    Right,
}

/// Vertical alignment within container.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PictureAlignY {
    #[default]
    Top,
    Center,
    Bottom,
}

/// Picture component props.
#[derive(Debug, Clone, Default)]
pub struct PictureProps {
    /// Fixed width (characters)
    pub width: Option<usize>,
    /// Fixed height (lines)
    pub height: Option<usize>,
    /// How to fit the picture
    pub fit: PictureFit,
    /// Horizontal alignment
    pub align_x: PictureAlignX,
    /// Vertical alignment
    pub align_y: PictureAlignY,
    /// Foreground color
    pub color: Option<Color>,
    /// Background color
    pub background_color: Option<Color>,
    /// Character to treat as transparent
    pub transparent: Option<char>,
    /// Border style
    pub border_style: Option<BorderStyle>,
    /// Border color
    pub border_color: Option<Color>,
    /// Padding inside the picture frame
    pub padding: u16,
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Parse source into lines.
fn parse_source(source: &str) -> Vec<String> {
    source.lines().map(|s| s.to_string()).collect()
}

/// Get visible width of a string (approximate, doesn't handle all unicode).
fn visible_width(s: &str) -> usize {
    s.chars().count()
}

/// Pad a line to a specific width.
fn pad_line(line: &str, target_width: usize, align: PictureAlignX) -> String {
    let current_width = visible_width(line);
    if current_width >= target_width {
        return line.to_string();
    }

    let padding = target_width - current_width;
    match align {
        PictureAlignX::Left => format!("{}{}", line, " ".repeat(padding)),
        PictureAlignX::Right => format!("{}{}", " ".repeat(padding), line),
        PictureAlignX::Center => {
            let left = padding / 2;
            let right = padding - left;
            format!("{}{}{}", " ".repeat(left), line, " ".repeat(right))
        }
    }
}

/// Crop a line to a specific width.
fn crop_line(line: &str, target_width: usize, align: PictureAlignX) -> String {
    let current_width = visible_width(line);
    if current_width <= target_width {
        return line.to_string();
    }

    let chars: Vec<char> = line.chars().collect();
    let excess = current_width - target_width;

    let start = match align {
        PictureAlignX::Left => 0,
        PictureAlignX::Right => excess,
        PictureAlignX::Center => excess / 2,
    };

    chars.iter().skip(start).take(target_width).collect()
}

/// Process lines to fit target dimensions.
fn process_lines(
    lines: Vec<String>,
    target_width: usize,
    target_height: usize,
    fit: PictureFit,
    align_x: PictureAlignX,
    align_y: PictureAlignY,
) -> Vec<String> {
    let mut result = lines;

    // Adjust height
    if result.len() < target_height {
        let padding = target_height - result.len();
        let empty_line = String::new();

        match align_y {
            PictureAlignY::Top => {
                result.extend(std::iter::repeat(empty_line).take(padding));
            }
            PictureAlignY::Bottom => {
                let mut new = vec![empty_line; padding];
                new.extend(result);
                result = new;
            }
            PictureAlignY::Center => {
                let top = padding / 2;
                let bottom = padding - top;
                let mut new = vec![empty_line.clone(); top];
                new.extend(result);
                new.extend(std::iter::repeat(empty_line).take(bottom));
                result = new;
            }
        }
    } else if result.len() > target_height && (fit == PictureFit::Crop || fit == PictureFit::Cover)
    {
        let excess = result.len() - target_height;
        match align_y {
            PictureAlignY::Top => result.truncate(target_height),
            PictureAlignY::Bottom => result = result.into_iter().skip(excess).collect(),
            PictureAlignY::Center => {
                let start = excess / 2;
                result = result.into_iter().skip(start).take(target_height).collect();
            }
        }
    }

    // Adjust width for each line
    result
        .iter()
        .map(|line| {
            let line_width = visible_width(line);
            if line_width < target_width {
                pad_line(line, target_width, align_x)
            } else if line_width > target_width
                && (fit == PictureFit::Crop || fit == PictureFit::Cover)
            {
                crop_line(line, target_width, align_x)
            } else {
                pad_line(line, target_width, align_x)
            }
        })
        .collect()
}

// =============================================================================
// Picture Component
// =============================================================================

/// Picture component - Display ASCII art and character-based images.
#[derive(Debug, Clone)]
pub struct Picture {
    /// Source content
    source: String,
    /// Props
    props: PictureProps,
}

impl Picture {
    /// Create a new picture from source.
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            props: PictureProps::default(),
        }
    }

    /// Set fixed width.
    pub fn width(mut self, width: usize) -> Self {
        self.props.width = Some(width);
        self
    }

    /// Set fixed height.
    pub fn height(mut self, height: usize) -> Self {
        self.props.height = Some(height);
        self
    }

    /// Set fit mode.
    pub fn fit(mut self, fit: PictureFit) -> Self {
        self.props.fit = fit;
        self
    }

    /// Set horizontal alignment.
    pub fn align_x(mut self, align: PictureAlignX) -> Self {
        self.props.align_x = align;
        self
    }

    /// Set vertical alignment.
    pub fn align_y(mut self, align: PictureAlignY) -> Self {
        self.props.align_y = align;
        self
    }

    /// Set foreground color.
    pub fn color(mut self, color: Color) -> Self {
        self.props.color = Some(color);
        self
    }

    /// Set background color.
    pub fn background(mut self, color: Color) -> Self {
        self.props.background_color = Some(color);
        self
    }

    /// Set transparent character.
    pub fn transparent(mut self, char: char) -> Self {
        self.props.transparent = Some(char);
        self
    }

    /// Set border style.
    pub fn border(mut self, style: BorderStyle) -> Self {
        self.props.border_style = Some(style);
        self
    }

    /// Set border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.props.border_color = Some(color);
        self
    }

    /// Set padding.
    pub fn padding(mut self, padding: u16) -> Self {
        self.props.padding = padding;
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let mut lines = parse_source(&self.source);

        // Remove leading/trailing empty lines
        while !lines.is_empty() && lines[0].trim().is_empty() {
            lines.remove(0);
        }
        while !lines.is_empty() && lines.last().map(|s| s.trim().is_empty()).unwrap_or(false) {
            lines.pop();
        }

        // Calculate dimensions
        let src_height = lines.len();
        let src_width = lines.iter().map(|l| visible_width(l)).max().unwrap_or(0);

        let target_width = self.props.width.unwrap_or(src_width);
        let target_height = self.props.height.unwrap_or(src_height);

        // Process lines
        lines = process_lines(
            lines,
            target_width,
            target_height,
            self.props.fit,
            self.props.align_x,
            self.props.align_y,
        );

        // Apply transparency
        if let Some(trans) = self.props.transparent {
            lines = lines
                .iter()
                .map(|line| {
                    line.chars()
                        .map(|c| if c == trans { ' ' } else { c })
                        .collect()
                })
                .collect();
        }

        // Build content
        let content: Vec<VNode> = lines
            .iter()
            .map(|line| {
                let style = TextStyle {
                    color: self.props.color.clone(),
                    ..Default::default()
                };
                VNode::styled_text(line.clone(), style)
            })
            .collect();

        // Wrap in Box if border or padding needed
        if self.props.border_style.is_some() || self.props.padding > 0 {
            VNode::Box(BoxNode {
                children: content,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Column),
                    border_style: self.props.border_style.clone(),
                    padding_left: Some(self.props.padding),
                    padding_right: Some(self.props.padding),
                    padding_top: Some(self.props.padding),
                    padding_bottom: Some(self.props.padding),
                    ..Default::default()
                },
                ..Default::default()
            })
        } else {
            VNode::Box(BoxNode {
                children: content,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Column),
                    ..Default::default()
                },
                ..Default::default()
            })
        }
    }
}

// =============================================================================
// Colored Picture Component
// =============================================================================

/// Colored picture component - Display colored pixel art.
#[derive(Debug, Clone)]
pub struct ColoredPicture {
    /// Pixel grid
    pixels: PixelGrid,
    /// Fixed width
    width: Option<usize>,
    /// Fixed height
    height: Option<usize>,
    /// Horizontal alignment
    align_x: PictureAlignX,
    /// Vertical alignment
    align_y: PictureAlignY,
    /// Border style
    border_style: Option<BorderStyle>,
    /// Border color
    border_color: Option<Color>,
    /// Padding
    padding: u16,
}

impl ColoredPicture {
    /// Create from pixel grid.
    pub fn new(pixels: PixelGrid) -> Self {
        Self {
            pixels,
            width: None,
            height: None,
            align_x: PictureAlignX::Left,
            align_y: PictureAlignY::Top,
            border_style: None,
            border_color: None,
            padding: 0,
        }
    }

    /// Set fixed width.
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Set fixed height.
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// Set horizontal alignment.
    pub fn align_x(mut self, align: PictureAlignX) -> Self {
        self.align_x = align;
        self
    }

    /// Set vertical alignment.
    pub fn align_y(mut self, align: PictureAlignY) -> Self {
        self.align_y = align;
        self
    }

    /// Set border style.
    pub fn border(mut self, style: BorderStyle) -> Self {
        self.border_style = Some(style);
        self
    }

    /// Set border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Set padding.
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let grid_height = self.pixels.len();
        let grid_width = self.pixels.iter().map(|r| r.len()).max().unwrap_or(0);

        let target_width = self.width.unwrap_or(grid_width);
        let target_height = self.height.unwrap_or(grid_height);

        let mut rows: Vec<VNode> = Vec::new();

        for y in 0..target_height {
            // Calculate source row based on alignment
            let src_y = match self.align_y {
                PictureAlignY::Top => y,
                PictureAlignY::Center => {
                    if target_height > grid_height {
                        y.checked_sub((target_height - grid_height) / 2)
                            .unwrap_or(usize::MAX)
                    } else {
                        y
                    }
                }
                PictureAlignY::Bottom => {
                    if target_height > grid_height {
                        y.checked_sub(target_height - grid_height)
                            .unwrap_or(usize::MAX)
                    } else {
                        y
                    }
                }
            };

            let row = self.pixels.get(src_y);

            if row.is_none() || src_y >= grid_height {
                rows.push(VNode::text(" ".repeat(target_width)));
                continue;
            }

            let row = row.unwrap();

            // Build row with colors
            let mut segments: Vec<VNode> = Vec::new();
            let mut current_text = String::new();
            let mut current_style = TextStyle::default();

            for x in 0..target_width {
                // Calculate source column based on alignment
                let src_x = match self.align_x {
                    PictureAlignX::Left => x,
                    PictureAlignX::Center => {
                        if target_width > row.len() {
                            x.checked_sub((target_width - row.len()) / 2)
                                .unwrap_or(usize::MAX)
                        } else {
                            x
                        }
                    }
                    PictureAlignX::Right => {
                        if target_width > row.len() {
                            x.checked_sub(target_width - row.len())
                                .unwrap_or(usize::MAX)
                        } else {
                            x
                        }
                    }
                };

                let pixel = row.get(src_x);

                if pixel.is_none() || src_x >= row.len() {
                    // Flush current text if style changes
                    if !current_text.is_empty() {
                        segments.push(VNode::styled_text(
                            current_text.clone(),
                            current_style.clone(),
                        ));
                        current_text.clear();
                    }
                    current_text.push(' ');
                    current_style = TextStyle::default();
                    continue;
                }

                let pixel = pixel.unwrap();
                let new_style = TextStyle {
                    color: pixel.fg.clone(),
                    ..Default::default()
                };

                // Check if style changed
                if new_style.color != current_style.color && !current_text.is_empty() {
                    segments.push(VNode::styled_text(
                        current_text.clone(),
                        current_style.clone(),
                    ));
                    current_text.clear();
                }

                current_style = new_style;
                current_text.push(pixel.char);
            }

            // Flush remaining text
            if !current_text.is_empty() {
                segments.push(VNode::styled_text(current_text, current_style));
            }

            if segments.len() == 1 {
                rows.push(segments.remove(0));
            } else {
                rows.push(VNode::Box(BoxNode {
                    children: segments,
                    style: BoxStyle {
                        flex_direction: Some(FlexDirection::Row),
                        ..Default::default()
                    },
                    ..Default::default()
                }));
            }
        }

        // Wrap in Box
        if self.border_style.is_some() || self.padding > 0 {
            VNode::Box(BoxNode {
                children: rows,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Column),
                    border_style: self.border_style.clone(),
                    padding_left: Some(self.padding),
                    padding_right: Some(self.padding),
                    padding_top: Some(self.padding),
                    padding_bottom: Some(self.padding),
                    ..Default::default()
                },
                ..Default::default()
            })
        } else {
            VNode::Box(BoxNode {
                children: rows,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Column),
                    ..Default::default()
                },
                ..Default::default()
            })
        }
    }
}

// =============================================================================
// Pre-made ASCII Art Patterns
// =============================================================================

/// Common ASCII art patterns.
pub struct AsciiPatterns;

impl AsciiPatterns {
    /// Horizontal line.
    pub fn hline(width: usize, char: char) -> String {
        std::iter::repeat(char).take(width).collect()
    }

    /// Vertical line.
    pub fn vline(height: usize, char: char) -> String {
        (0..height)
            .map(|_| char.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Box shape.
    pub fn box_shape(width: usize, height: usize, double: bool) -> String {
        let (tl, tr, bl, br, h, v) = if double {
            ('╔', '╗', '╚', '╝', '═', '║')
        } else {
            ('┌', '┐', '└', '┘', '─', '│')
        };

        let top = format!(
            "{}{}{}",
            tl,
            std::iter::repeat(h)
                .take(width.saturating_sub(2))
                .collect::<String>(),
            tr
        );
        let middle = format!("{}{}{}", v, " ".repeat(width.saturating_sub(2)), v);
        let bottom = format!(
            "{}{}{}",
            bl,
            std::iter::repeat(h)
                .take(width.saturating_sub(2))
                .collect::<String>(),
            br
        );

        let mut lines = vec![top];
        for _ in 0..height.saturating_sub(2) {
            lines.push(middle.clone());
        }
        lines.push(bottom);

        lines.join("\n")
    }

    /// Diamond shape.
    pub fn diamond(size: usize) -> String {
        let half = size / 2;
        let mut lines = Vec::new();

        for i in 0..=half {
            let spaces = " ".repeat(half - i);
            let stars = "◆".repeat(i * 2 + 1);
            lines.push(format!("{}{}", spaces, stars));
        }
        for i in (0..half).rev() {
            let spaces = " ".repeat(half - i);
            let stars = "◆".repeat(i * 2 + 1);
            lines.push(format!("{}{}", spaces, stars));
        }

        lines.join("\n")
    }

    /// Arrow pointing right.
    pub fn arrow_right() -> &'static str {
        "   ▶\n  ▶▶▶\n ▶▶▶▶▶\n  ▶▶▶\n   ▶"
    }

    /// Arrow pointing left.
    pub fn arrow_left() -> &'static str {
        "   ◀\n  ◀◀◀\n ◀◀◀◀◀\n  ◀◀◀\n   ◀"
    }

    /// Star shape.
    pub fn star() -> &'static str {
        "    ★\n   ★★★\n  ★★★★★\n   ★★★\n    ★"
    }

    /// Heart shape.
    pub fn heart() -> &'static str {
        " ♥♥   ♥♥\n♥♥♥♥ ♥♥♥♥\n ♥♥♥♥♥♥♥\n  ♥♥♥♥♥\n   ♥♥♥\n    ♥"
    }

    /// Checkmark.
    pub fn checkmark() -> &'static str {
        "      ✓\n     ✓\n✓   ✓\n ✓ ✓\n  ✓"
    }

    /// Cross/X mark.
    pub fn cross() -> &'static str {
        "✗     ✗\n ✗   ✗\n  ✗ ✗\n   ✗\n  ✗ ✗\n ✗   ✗\n✗     ✗"
    }
}

/// Generate a simple text banner.
pub fn create_banner(text: &str, style: &str) -> String {
    let width = text.len() + 4;

    match style {
        "simple" => format!("{}\n  {}\n{}", "═".repeat(width), text, "═".repeat(width)),
        "box" => format!(
            "┌{}┐\n│ {} │\n└{}┘",
            "─".repeat(width - 2),
            text,
            "─".repeat(width - 2)
        ),
        "double" => format!(
            "╔{}╗\n║ {} ║\n╚{}╝",
            "═".repeat(width - 2),
            text,
            "═".repeat(width - 2)
        ),
        _ => text.to_string(),
    }
}

/// Create rainbow text effect.
pub fn rainbow_text(text: &str) -> Vec<(char, Color)> {
    let colors = [
        Color::Named(NamedColor::Red),
        Color::Named(NamedColor::Yellow),
        Color::Named(NamedColor::Green),
        Color::Named(NamedColor::Cyan),
        Color::Named(NamedColor::Blue),
        Color::Named(NamedColor::Magenta),
    ];

    text.chars()
        .enumerate()
        .map(|(i, c)| (c, colors[i % colors.len()].clone()))
        .collect()
}
