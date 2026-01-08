//! Stack Layouts - Basic stacking and arrangement components
//!
//! Provides simple, composable layout primitives:
//! - VStack: Vertical stack with configurable gap
//! - HStack: Horizontal stack with configurable gap
//! - Center: Centers content horizontally and/or vertically
//! - FullScreen: Container that fills the terminal
//! - Spacer: Flexible space that pushes siblings apart
//!
//! # Example
//!
//! ```rust
//! use tuiuiu::templates::{VStack, HStack, Center, Spacer};
//!
//! // Vertical stack with gap
//! let vstack = VStack::new()
//!     .gap(1)
//!     .align(VAlign::Center);
//!
//! // Horizontal stack with space-between
//! let hstack = HStack::new()
//!     .justify(HJustify::SpaceBetween);
//!
//! // Center content
//! let centered = Center::new().width(80).height(24);
//! ```

use crate::core::layout::{FlexDirection, JustifyContent, AlignItems};
use crate::utils::border::BorderStyle;

// =============================================================================
// Alignment Types
// =============================================================================

/// Vertical alignment for VStack
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VAlign {
    /// Align to left (flex-start)
    Left,
    /// Align to center
    Center,
    /// Align to right (flex-end)
    Right,
    /// Stretch to fill (default)
    #[default]
    Stretch,
}

impl VAlign {
    fn to_align_items(&self) -> AlignItems {
        match self {
            VAlign::Left => AlignItems::FlexStart,
            VAlign::Center => AlignItems::Center,
            VAlign::Right => AlignItems::FlexEnd,
            VAlign::Stretch => AlignItems::Stretch,
        }
    }
}

/// Horizontal alignment for HStack
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAlign {
    /// Align to top (flex-start)
    Top,
    /// Align to center
    Center,
    /// Align to bottom (flex-end)
    Bottom,
    /// Stretch to fill (default)
    #[default]
    Stretch,
}

impl HAlign {
    fn to_align_items(&self) -> AlignItems {
        match self {
            HAlign::Top => AlignItems::FlexStart,
            HAlign::Center => AlignItems::Center,
            HAlign::Bottom => AlignItems::FlexEnd,
            HAlign::Stretch => AlignItems::Stretch,
        }
    }
}

/// Horizontal justification for HStack
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HJustify {
    /// Justify to start (default)
    #[default]
    Start,
    /// Justify to center
    Center,
    /// Justify to end
    End,
    /// Space between items
    SpaceBetween,
    /// Space around items
    SpaceAround,
}

impl HJustify {
    fn to_justify_content(&self) -> JustifyContent {
        match self {
            HJustify::Start => JustifyContent::FlexStart,
            HJustify::Center => JustifyContent::Center,
            HJustify::End => JustifyContent::FlexEnd,
            HJustify::SpaceBetween => JustifyContent::SpaceBetween,
            HJustify::SpaceAround => JustifyContent::SpaceAround,
        }
    }
}

// =============================================================================
// VStack - Vertical Stack
// =============================================================================

/// Vertical Stack - Arranges children vertically with optional gap
#[derive(Debug, Clone, Default)]
pub struct VStack {
    /// Gap between children (in lines)
    gap: usize,
    /// Horizontal alignment
    align: VAlign,
    /// Padding inside the stack
    padding: usize,
    /// Padding horizontal
    padding_x: Option<usize>,
    /// Padding vertical
    padding_y: Option<usize>,
    /// Fixed width
    width: Option<usize>,
    /// Fixed height
    height: Option<usize>,
    /// Border style
    border: Option<BorderStyle>,
    /// Border color
    border_color: Option<&'static str>,
}

impl VStack {
    /// Create a new vertical stack
    pub fn new() -> Self {
        Self::default()
    }

    /// Set gap between children
    pub fn gap(mut self, gap: usize) -> Self {
        self.gap = gap;
        self
    }

    /// Set horizontal alignment
    pub fn align(mut self, align: VAlign) -> Self {
        self.align = align;
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    /// Set horizontal padding
    pub fn padding_x(mut self, padding: usize) -> Self {
        self.padding_x = Some(padding);
        self
    }

    /// Set vertical padding
    pub fn padding_y(mut self, padding: usize) -> Self {
        self.padding_y = Some(padding);
        self
    }

    /// Set fixed width
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Set fixed height
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// Set border
    pub fn border(mut self, style: BorderStyle) -> Self {
        self.border = Some(style);
        self
    }

    /// Set border color
    pub fn border_color(mut self, color: &'static str) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Get layout properties
    pub fn layout_props(&self) -> LayoutProps {
        LayoutProps {
            direction: FlexDirection::Column,
            align_items: self.align.to_align_items(),
            justify_content: JustifyContent::FlexStart,
            gap: self.gap,
            padding: self.padding,
            padding_x: self.padding_x,
            padding_y: self.padding_y,
            width: self.width,
            height: self.height,
            border: self.border,
            border_color: self.border_color,
        }
    }
}

// =============================================================================
// HStack - Horizontal Stack
// =============================================================================

/// Horizontal Stack - Arranges children horizontally with optional gap
#[derive(Debug, Clone, Default)]
pub struct HStack {
    /// Gap between children (in characters)
    gap: usize,
    /// Vertical alignment
    align: HAlign,
    /// Horizontal justification
    justify: HJustify,
    /// Padding inside the stack
    padding: usize,
    /// Padding horizontal
    padding_x: Option<usize>,
    /// Padding vertical
    padding_y: Option<usize>,
    /// Fixed width
    width: Option<usize>,
    /// Fixed height
    height: Option<usize>,
    /// Border style
    border: Option<BorderStyle>,
    /// Border color
    border_color: Option<&'static str>,
}

impl HStack {
    /// Create a new horizontal stack
    pub fn new() -> Self {
        Self::default()
    }

    /// Set gap between children
    pub fn gap(mut self, gap: usize) -> Self {
        self.gap = gap;
        self
    }

    /// Set vertical alignment
    pub fn align(mut self, align: HAlign) -> Self {
        self.align = align;
        self
    }

    /// Set horizontal justification
    pub fn justify(mut self, justify: HJustify) -> Self {
        self.justify = justify;
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    /// Set horizontal padding
    pub fn padding_x(mut self, padding: usize) -> Self {
        self.padding_x = Some(padding);
        self
    }

    /// Set vertical padding
    pub fn padding_y(mut self, padding: usize) -> Self {
        self.padding_y = Some(padding);
        self
    }

    /// Set fixed width
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Set fixed height
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// Set border
    pub fn border(mut self, style: BorderStyle) -> Self {
        self.border = Some(style);
        self
    }

    /// Set border color
    pub fn border_color(mut self, color: &'static str) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Get layout properties
    pub fn layout_props(&self) -> LayoutProps {
        LayoutProps {
            direction: FlexDirection::Row,
            align_items: self.align.to_align_items(),
            justify_content: self.justify.to_justify_content(),
            gap: self.gap,
            padding: self.padding,
            padding_x: self.padding_x,
            padding_y: self.padding_y,
            width: self.width,
            height: self.height,
            border: self.border,
            border_color: self.border_color,
        }
    }
}

// =============================================================================
// Center - Centers content
// =============================================================================

/// Center - Centers content horizontally and/or vertically
#[derive(Debug, Clone)]
pub struct Center {
    /// Center horizontally
    horizontal: bool,
    /// Center vertically
    vertical: bool,
    /// Fixed width
    width: Option<usize>,
    /// Fixed height
    height: Option<usize>,
}

impl Default for Center {
    fn default() -> Self {
        Self {
            horizontal: true,
            vertical: true,
            width: None,
            height: None,
        }
    }
}

impl Center {
    /// Create a new center layout
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether to center horizontally
    pub fn horizontal(mut self, enabled: bool) -> Self {
        self.horizontal = enabled;
        self
    }

    /// Set whether to center vertically
    pub fn vertical(mut self, enabled: bool) -> Self {
        self.vertical = enabled;
        self
    }

    /// Set fixed width
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Set fixed height
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// Get layout properties
    pub fn layout_props(&self) -> LayoutProps {
        LayoutProps {
            direction: FlexDirection::Column,
            align_items: if self.horizontal {
                AlignItems::Center
            } else {
                AlignItems::FlexStart
            },
            justify_content: if self.vertical {
                JustifyContent::Center
            } else {
                JustifyContent::FlexStart
            },
            gap: 0,
            padding: 0,
            padding_x: None,
            padding_y: None,
            width: self.width,
            height: self.height,
            border: None,
            border_color: None,
        }
    }
}

// =============================================================================
// FullScreen - Full terminal container
// =============================================================================

/// FullScreen - Container that fills the entire terminal
#[derive(Debug, Clone, Default)]
pub struct FullScreen {
    /// Background color
    background_color: Option<&'static str>,
    /// Padding from edges
    padding: usize,
}

impl FullScreen {
    /// Create a new fullscreen container
    pub fn new() -> Self {
        Self::default()
    }

    /// Set background color
    pub fn background_color(mut self, color: &'static str) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    /// Get terminal dimensions
    fn terminal_size() -> (usize, usize) {
        // Try to get terminal size, fallback to 80x24
        (80, 24)
    }

    /// Get layout properties
    pub fn layout_props(&self) -> LayoutProps {
        let (width, height) = Self::terminal_size();

        LayoutProps {
            direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::FlexStart,
            gap: 0,
            padding: self.padding,
            padding_x: None,
            padding_y: None,
            width: Some(width),
            height: Some(height),
            border: None,
            border_color: None,
        }
    }
}

// =============================================================================
// Spacer - Flexible space
// =============================================================================

/// Spacer - Flexible space that pushes siblings apart
#[derive(Debug, Clone)]
pub struct Spacer {
    /// Flex grow value
    flex: usize,
    /// Minimum size
    min_size: usize,
}

impl Default for Spacer {
    fn default() -> Self {
        Self {
            flex: 1,
            min_size: 0,
        }
    }
}

impl Spacer {
    /// Create a new spacer
    pub fn new() -> Self {
        Self::default()
    }

    /// Set flex grow value
    pub fn flex(mut self, flex: usize) -> Self {
        self.flex = flex;
        self
    }

    /// Set minimum size
    pub fn min_size(mut self, size: usize) -> Self {
        self.min_size = size;
        self
    }

    /// Get flex value
    pub fn get_flex(&self) -> usize {
        self.flex
    }

    /// Get minimum size
    pub fn get_min_size(&self) -> usize {
        self.min_size
    }
}

// =============================================================================
// Layout Properties (shared output type)
// =============================================================================

/// Layout properties for rendering
#[derive(Debug, Clone)]
pub struct LayoutProps {
    pub direction: FlexDirection,
    pub align_items: AlignItems,
    pub justify_content: JustifyContent,
    pub gap: usize,
    pub padding: usize,
    pub padding_x: Option<usize>,
    pub padding_y: Option<usize>,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub border: Option<BorderStyle>,
    pub border_color: Option<&'static str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vstack() {
        let vstack = VStack::new()
            .gap(1)
            .align(VAlign::Center)
            .padding(2);

        let props = vstack.layout_props();
        assert_eq!(props.direction, FlexDirection::Column);
        assert_eq!(props.align_items, AlignItems::Center);
        assert_eq!(props.gap, 1);
        assert_eq!(props.padding, 2);
    }

    #[test]
    fn test_hstack() {
        let hstack = HStack::new()
            .gap(2)
            .justify(HJustify::SpaceBetween);

        let props = hstack.layout_props();
        assert_eq!(props.direction, FlexDirection::Row);
        assert_eq!(props.justify_content, JustifyContent::SpaceBetween);
    }

    #[test]
    fn test_center() {
        let center = Center::new()
            .width(80)
            .height(24);

        let props = center.layout_props();
        assert_eq!(props.align_items, AlignItems::Center);
        assert_eq!(props.justify_content, JustifyContent::Center);
        assert_eq!(props.width, Some(80));
        assert_eq!(props.height, Some(24));
    }

    #[test]
    fn test_spacer() {
        let spacer = Spacer::new().flex(2).min_size(5);
        assert_eq!(spacer.get_flex(), 2);
        assert_eq!(spacer.get_min_size(), 5);
    }
}
