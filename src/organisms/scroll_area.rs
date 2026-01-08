//! ScrollArea Component
//!
//! A scrollable content area with customizable scrollbars.
//! Supports keyboard navigation (PageUp/PageDown, arrow keys).

use crate::core::component::{
    BoxNode, BoxStyle, Color, EventHandlers, NamedColor, TextNode, TextStyle, VNode,
};
use crate::core::layout::{AlignItems, FlexDirection, Size};
use crate::core::signals::{create_signal, ReadSignal, WriteSignal};

/// Scrollbar visibility mode.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ScrollbarVisibility {
    /// Always show scrollbar
    Always,
    /// Show only when needed (content overflows)
    #[default]
    Auto,
    /// Never show scrollbar
    Never,
    /// Show on hover only
    Hover,
}

/// ScrollArea component.
///
/// A container that allows scrolling through content that exceeds
/// the visible area.
///
/// # Example
/// ```ignore
/// let scroll_area = ScrollArea::new()
///     .height(10)
///     .content(vec![
///         VNode::text("Line 1"),
///         VNode::text("Line 2"),
///         // ... many more lines
///     ])
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct ScrollArea {
    /// Content to scroll
    content: Vec<VNode>,
    /// Visible height (in lines)
    height: u16,
    /// Visible width (in characters)
    width: Option<u16>,
    /// Current scroll offset (line)
    scroll_offset: usize,
    /// Total content height (lines) - for calculating scrollbar
    content_height: usize,
    /// Show vertical scrollbar
    show_scrollbar: ScrollbarVisibility,
    /// Use Unicode scrollbar characters
    unicode: bool,
    /// Scrollbar color
    scrollbar_color: Option<Color>,
    /// Track color
    track_color: Option<Color>,
}

impl ScrollArea {
    /// Create a new scroll area.
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
            height: 10,
            width: None,
            scroll_offset: 0,
            content_height: 0,
            show_scrollbar: ScrollbarVisibility::default(),
            unicode: true,
            scrollbar_color: None,
            track_color: None,
        }
    }

    /// Set the content.
    pub fn content(mut self, content: Vec<VNode>) -> Self {
        self.content_height = content.len();
        self.content = content;
        self
    }

    /// Add a single content item.
    pub fn add(mut self, item: VNode) -> Self {
        self.content.push(item);
        self.content_height += 1;
        self
    }

    /// Set the visible height.
    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    /// Set the visible width.
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the scroll offset.
    pub fn scroll_offset(mut self, offset: usize) -> Self {
        self.scroll_offset = offset;
        self
    }

    /// Set the total content height (for external control).
    pub fn content_height(mut self, height: usize) -> Self {
        self.content_height = height;
        self
    }

    /// Set scrollbar visibility.
    pub fn scrollbar(mut self, visibility: ScrollbarVisibility) -> Self {
        self.show_scrollbar = visibility;
        self
    }

    /// Always show scrollbar.
    pub fn scrollbar_always(self) -> Self {
        self.scrollbar(ScrollbarVisibility::Always)
    }

    /// Hide scrollbar.
    pub fn scrollbar_never(self) -> Self {
        self.scrollbar(ScrollbarVisibility::Never)
    }

    /// Use ASCII characters for scrollbar.
    pub fn ascii(mut self, ascii: bool) -> Self {
        self.unicode = !ascii;
        self
    }

    /// Set scrollbar color.
    pub fn scrollbar_color(mut self, color: Color) -> Self {
        self.scrollbar_color = Some(color);
        self
    }

    /// Set track color.
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = Some(color);
        self
    }

    /// Build the scroll area VNode.
    pub fn build(self) -> VNode {
        let visible_height = self.height as usize;
        let should_show_scrollbar = match self.show_scrollbar {
            ScrollbarVisibility::Always => true,
            ScrollbarVisibility::Never => false,
            ScrollbarVisibility::Auto | ScrollbarVisibility::Hover => {
                self.content_height > visible_height
            }
        };

        // Clamp scroll offset
        let max_offset = if self.content_height > visible_height {
            self.content_height - visible_height
        } else {
            0
        };
        let scroll_offset = self.scroll_offset.min(max_offset);

        // Save values before consuming content
        let height = self.height;
        let width = self.width;
        let content_height = self.content_height;
        let unicode = self.unicode;
        let scrollbar_color = self.scrollbar_color;
        let track_color = self.track_color;

        // Get visible content
        let visible_content: Vec<VNode> = self
            .content
            .into_iter()
            .skip(scroll_offset)
            .take(visible_height)
            .collect();

        // Content column
        let content_box = VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                flex_grow: Some(1.0),
                height: Some(Size::Fixed(height)),
                width: width.map(Size::Fixed),
                ..Default::default()
            },
            children: visible_content,
            handlers: EventHandlers {
                focusable: true,
                ..Default::default()
            },
        });

        if !should_show_scrollbar {
            return content_box;
        }

        // Build scrollbar
        let scrollbar = Self::build_scrollbar_static(
            visible_height,
            content_height,
            scroll_offset,
            unicode,
            scrollbar_color,
            track_color,
        );

        // Row with content and scrollbar
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                height: Some(Size::Fixed(height)),
                width: width.map(|w| Size::Fixed(w + 1)), // +1 for scrollbar
                ..Default::default()
            },
            children: vec![content_box, scrollbar],
            handlers: Default::default(),
        })
    }

    /// Build the scrollbar (static version for ownership-safe use).
    fn build_scrollbar_static(
        visible_height: usize,
        total_height: usize,
        scroll_offset: usize,
        unicode: bool,
        scrollbar_color_opt: Option<Color>,
        track_color_opt: Option<Color>,
    ) -> VNode {
        let scrollbar_color = scrollbar_color_opt.unwrap_or(Color::Named(NamedColor::White));
        let track_color = track_color_opt.unwrap_or(Color::Named(NamedColor::BrightBlack));

        let (thumb_char, track_char) = if unicode {
            ('█', '│')
        } else {
            ('#', '|')
        };

        // Calculate thumb size and position
        let thumb_size = if total_height > 0 {
            ((visible_height * visible_height) / total_height).max(1)
        } else {
            visible_height
        };

        let thumb_position = if total_height > visible_height {
            (scroll_offset * (visible_height - thumb_size)) / (total_height - visible_height)
        } else {
            0
        };

        // Build scrollbar lines
        let mut lines = Vec::new();
        for i in 0..visible_height {
            let is_thumb = i >= thumb_position && i < thumb_position + thumb_size;
            let (ch, color) = if is_thumb {
                (thumb_char, scrollbar_color)
            } else {
                (track_char, track_color)
            };

            lines.push(VNode::Text(TextNode {
                content: ch.to_string(),
                style: TextStyle {
                    color: Some(color),
                    ..Default::default()
                },
            }));
        }

        VNode::Box(BoxNode {
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                width: Some(Size::Fixed(1)),
                align_items: Some(AlignItems::Center),
                ..Default::default()
            },
            children: lines,
            ..Default::default()
        })
    }
}

impl From<ScrollArea> for VNode {
    fn from(area: ScrollArea) -> VNode {
        area.build()
    }
}

// =============================================================================
// ScrollArea State
// =============================================================================

/// State manager for ScrollArea.
///
/// Provides reactive scroll position with methods for navigation.
pub struct ScrollAreaState {
    /// Current scroll offset
    offset_read: ReadSignal<usize>,
    offset_write: WriteSignal<usize>,
    /// Visible height
    visible_height_read: ReadSignal<usize>,
    visible_height_write: WriteSignal<usize>,
    /// Total content height
    total_height_read: ReadSignal<usize>,
    total_height_write: WriteSignal<usize>,
}

impl ScrollAreaState {
    /// Create a new scroll area state.
    pub fn new(visible_height: usize) -> Self {
        let (offset_read, offset_write) = create_signal(0usize);
        let (visible_height_read, visible_height_write) = create_signal(visible_height);
        let (total_height_read, total_height_write) = create_signal(0usize);
        Self {
            offset_read,
            offset_write,
            visible_height_read,
            visible_height_write,
            total_height_read,
            total_height_write,
        }
    }

    /// Get current scroll offset.
    pub fn offset(&self) -> usize {
        self.offset_read.get()
    }

    /// Set the scroll offset.
    pub fn set_offset(&self, offset: usize) {
        let max = self.max_offset();
        self.offset_write.set(offset.min(max));
    }

    /// Set the total content height.
    pub fn set_total_height(&self, height: usize) {
        self.total_height_write.set(height);
        // Clamp offset if needed
        let max = self.max_offset();
        if self.offset_read.get() > max {
            self.offset_write.set(max);
        }
    }

    /// Set the visible height.
    pub fn set_visible_height(&self, height: usize) {
        self.visible_height_write.set(height);
    }

    /// Get maximum scroll offset.
    pub fn max_offset(&self) -> usize {
        let total = self.total_height_read.get();
        let visible = self.visible_height_read.get();
        if total > visible {
            total - visible
        } else {
            0
        }
    }

    /// Scroll up by one line.
    pub fn scroll_up(&self) {
        let current = self.offset_read.get();
        if current > 0 {
            self.offset_write.set(current - 1);
        }
    }

    /// Scroll down by one line.
    pub fn scroll_down(&self) {
        let current = self.offset_read.get();
        let max = self.max_offset();
        if current < max {
            self.offset_write.set(current + 1);
        }
    }

    /// Scroll up by a page.
    pub fn page_up(&self) {
        let current = self.offset_read.get();
        let page_size = self.visible_height_read.get();
        self.offset_write.set(current.saturating_sub(page_size));
    }

    /// Scroll down by a page.
    pub fn page_down(&self) {
        let current = self.offset_read.get();
        let page_size = self.visible_height_read.get();
        let max = self.max_offset();
        self.offset_write.set((current + page_size).min(max));
    }

    /// Scroll to top.
    pub fn scroll_to_top(&self) {
        self.offset_write.set(0);
    }

    /// Scroll to bottom.
    pub fn scroll_to_bottom(&self) {
        self.offset_write.set(self.max_offset());
    }

    /// Scroll to a specific line.
    pub fn scroll_to(&self, line: usize) {
        let max = self.max_offset();
        self.offset_write.set(line.min(max));
    }

    /// Get the offset signal.
    pub fn signal(&self) -> ReadSignal<usize> {
        self.offset_read.clone()
    }

    /// Check if can scroll up.
    pub fn can_scroll_up(&self) -> bool {
        self.offset_read.get() > 0
    }

    /// Check if can scroll down.
    pub fn can_scroll_down(&self) -> bool {
        self.offset_read.get() < self.max_offset()
    }

    /// Get scroll percentage (0.0 - 1.0).
    pub fn scroll_percentage(&self) -> f64 {
        let max = self.max_offset();
        if max == 0 {
            0.0
        } else {
            self.offset_read.get() as f64 / max as f64
        }
    }
}

impl Default for ScrollAreaState {
    fn default() -> Self {
        Self::new(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_area_empty() {
        let area = ScrollArea::new().height(5).build();
        matches!(area, VNode::Box(_));
    }

    #[test]
    fn test_scroll_area_with_content() {
        let content: Vec<VNode> = (0..20)
            .map(|i| VNode::text(format!("Line {}", i)))
            .collect();

        let area = ScrollArea::new()
            .height(5)
            .content(content)
            .scrollbar_always()
            .build();

        matches!(area, VNode::Box(_));
    }

    #[test]
    fn test_scroll_state() {
        let state = ScrollAreaState::new(10);
        state.set_total_height(100);

        assert_eq!(state.offset(), 0);
        assert_eq!(state.max_offset(), 90);

        state.scroll_down();
        assert_eq!(state.offset(), 1);

        state.page_down();
        assert_eq!(state.offset(), 11);

        state.scroll_to_bottom();
        assert_eq!(state.offset(), 90);

        state.scroll_to_top();
        assert_eq!(state.offset(), 0);
    }

    #[test]
    fn test_scroll_percentage() {
        let state = ScrollAreaState::new(10);
        state.set_total_height(20);

        assert_eq!(state.scroll_percentage(), 0.0);

        state.scroll_to_bottom();
        assert_eq!(state.scroll_percentage(), 1.0);

        state.scroll_to(5);
        assert!((state.scroll_percentage() - 0.5).abs() < 0.01);
    }
}
