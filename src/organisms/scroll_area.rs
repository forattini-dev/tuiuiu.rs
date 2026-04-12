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

/// Height option for scroll areas.
#[derive(Debug, Clone, Copy)]
pub enum ScrollAreaHeight {
    /// Auto height
    Auto,
    /// Fill available space
    Fill,
    /// Fixed height
    Fixed(u16),
}

/// Configuration for creating/connecting a ScrollArea state.
pub struct ScrollAreaOptions {
    pub height: Option<ScrollAreaHeight>,
    pub min_height: Option<u16>,
    pub max_height: Option<u16>,
    pub flex_grow: Option<usize>,
    pub content: Vec<VNode>,
    pub initial_scroll_top: Option<usize>,
    pub auto_scroll: bool,
    pub show_scrollbar: bool,
    pub scrollbar_color: Option<Color>,
    pub track_color: Option<Color>,
    pub wrap_width: Option<usize>,
    pub scroll_step: Option<usize>,
    pub page_size: Option<usize>,
    pub on_scroll: Option<Box<dyn Fn(usize) + Send + Sync>>,
    pub is_active: Option<bool>,
    pub width: Option<u16>,
    pub state: Option<ScrollAreaState>,
    pub id: Option<String>,
    pub auto_scroll_threshold: Option<usize>,
    pub autofocus: Option<bool>,
}

impl Default for ScrollAreaOptions {
    fn default() -> Self {
        Self {
            height: Some(ScrollAreaHeight::Auto),
            min_height: None,
            max_height: None,
            flex_grow: None,
            content: Vec::new(),
            initial_scroll_top: Some(0),
            auto_scroll: false,
            show_scrollbar: true,
            scrollbar_color: None,
            track_color: None,
            wrap_width: None,
            scroll_step: Some(1),
            page_size: None,
            on_scroll: None,
            is_active: Some(true),
            width: None,
            state: None,
            id: None,
            auto_scroll_threshold: None,
            autofocus: None,
        }
    }
}

pub type ScrollAreaProps = ScrollAreaOptions;

/// ScrollArea Component
///
/// A container that allows scrolling through content that exceeds
/// the visible area.
#[derive(Debug, Clone, Default)]
pub struct ScrollArea {
    /// Content to scroll
    content: Vec<VNode>,
    /// Visible height (in lines)
    height: u16,
    /// Current scroll offset
    scroll_offset: usize,
    /// Total content height (lines)
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
            scroll_offset: 0,
            content_height: 0,
            show_scrollbar: ScrollbarVisibility::Auto,
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
    pub fn width(self, _width: u16) -> Self {
        self
    }

    /// Set the scroll offset.
    pub fn scroll_offset(mut self, offset: usize) -> Self {
        self.scroll_offset = offset;
        self
    }

    /// Set scrollbar visibility mode.
    pub fn scrollbar(mut self, visibility: ScrollbarVisibility) -> Self {
        self.show_scrollbar = visibility;
        self
    }

    /// Set scrollbar color.
    pub fn scrollbar_color(mut self, color: Option<Color>) -> Self {
        self.scrollbar_color = color;
        self
    }

    /// Set scrollbar track color.
    pub fn track_color(mut self, color: Option<Color>) -> Self {
        self.track_color = color;
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

        let max_offset = if self.content_height > visible_height {
            self.content_height - visible_height
        } else {
            0
        };
        let scroll_offset = self.scroll_offset.min(max_offset);

        let height = self.height;
        let content_height = self.content_height;
        let unicode = self.unicode;
        let scrollbar_color = self.scrollbar_color;
        let track_color = self.track_color;

        let visible_content: Vec<VNode> = self
            .content
            .into_iter()
            .skip(scroll_offset)
            .take(visible_height)
            .collect();

        let content_box = VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                flex_grow: Some(1.0),
                height: Some(Size::Fixed(height)),
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

        let scrollbar = Self::build_scrollbar_static(
            visible_height,
            content_height,
            scroll_offset,
            unicode,
            scrollbar_color,
            track_color,
        );

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                height: Some(Size::Fixed(height)),
                ..Default::default()
            },
            children: vec![content_box, scrollbar],
            ..Default::default()
        })
    }

    /// Build scrollbar char nodes.
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

        let (thumb_char, track_char) = if unicode { ('█', '│') } else { ('#', '|') };

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
pub struct ScrollAreaState {
    offset_read: ReadSignal<usize>,
    offset_write: WriteSignal<usize>,
    visible_height_read: ReadSignal<usize>,
    visible_height_write: WriteSignal<usize>,
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

    /// Set total content height.
    pub fn set_total_height(&self, height: usize) {
        self.total_height_write.set(height);
        let max = self.max_offset();
        if self.offset_read.get() > max {
            self.offset_write.set(max);
        }
    }

    /// Set visible height.
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

    /// Scroll up by one page.
    pub fn page_up(&self) {
        let current = self.offset_read.get();
        let page_size = self.visible_height_read.get();
        self.offset_write.set(current.saturating_sub(page_size));
    }

    /// Scroll down by one page.
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

    /// Scroll to specific line.
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

    /// Get scroll percentage (0..1).
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

/// Create and initialize a ScrollArea state from JS-style options.
pub fn create_scroll_area(options: ScrollAreaOptions) -> ScrollAreaState {
    let mut target_height = 10usize;
    if let Some(height) = options.height {
        target_height = match height {
            ScrollAreaHeight::Fixed(v) => v as usize,
            ScrollAreaHeight::Auto | ScrollAreaHeight::Fill => 10,
        }
    }

    let state = ScrollAreaState::new(target_height);
    state.set_total_height(options.content.len());
    if let Some(top) = options.initial_scroll_top {
        state.set_offset(top);
    }
    if let Some(visible) = options.min_height {
        state.set_visible_height(visible as usize);
    }
    state
}

/// Alias for snake/camel export style.
pub fn createScrollArea(options: ScrollAreaOptions) -> ScrollAreaState {
    create_scroll_area(options)
}

#[derive(Debug, Clone)]
pub struct ScrollableTextProps {
    pub text: String,
    pub height: u16,
    pub width: Option<u16>,
    pub show_scrollbar: bool,
    pub auto_scroll: bool,
}

impl Default for ScrollableTextProps {
    fn default() -> Self {
        Self {
            text: String::new(),
            height: 10,
            width: None,
            show_scrollbar: true,
            auto_scroll: false,
        }
    }
}

pub fn ScrollableText(props: ScrollableTextProps) -> VNode {
    let lines: Vec<VNode> = props
        .text
        .lines()
        .map(|line| VNode::text(line.to_string()))
        .collect();

    ScrollArea::new()
        .content(lines)
        .height(props.height)
        .build()
}

pub fn createScrollableText(props: ScrollableTextProps) -> VNode {
    ScrollableText(props)
}

#[derive(Debug, Clone)]
pub struct LogViewerOptions {
    pub content: String,
    pub height: u16,
    pub show_scrollbar: bool,
    pub auto_scroll: bool,
}

impl Default for LogViewerOptions {
    fn default() -> Self {
        Self {
            content: String::new(),
            height: 12,
            show_scrollbar: true,
            auto_scroll: true,
        }
    }
}

pub fn LogViewer(props: LogViewerOptions) -> VNode {
    let lines: Vec<VNode> = props
        .content
        .lines()
        .map(|line| VNode::text(line.to_string()))
        .collect();

    let mut area = ScrollArea::new().content(lines).height(props.height);

    if !props.show_scrollbar {
        area = area.scrollbar(ScrollbarVisibility::Never);
    }
    area.build()
}

pub fn createLogViewer(props: LogViewerOptions) -> VNode {
    LogViewer(props)
}

#[cfg(test)]
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
