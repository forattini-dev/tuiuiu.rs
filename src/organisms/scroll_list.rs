//! Scroll List Component
//!
//! A scrollable list with item rendering and selection.

use crate::core::component::{BorderStyle, BoxNode, BoxStyle, Color, NamedColor, TextStyle, VNode};
use crate::core::layout::{FlexDirection, Size};

/// State for scroll list.
#[derive(Debug, Clone)]
pub struct ScrollListState {
    /// Currently selected index
    pub selected: usize,
    /// Scroll offset
    pub offset: usize,
    /// Number of items
    pub count: usize,
    /// Visible height
    pub visible_height: usize,
}

impl Default for ScrollListState {
    fn default() -> Self {
        Self {
            selected: 0,
            offset: 0,
            count: 0,
            visible_height: 10,
        }
    }
}

impl ScrollListState {
    /// Create new state.
    pub fn new(count: usize) -> Self {
        Self {
            count,
            ..Default::default()
        }
    }

    /// Set visible height.
    pub fn visible_height(mut self, height: usize) -> Self {
        self.visible_height = height;
        self
    }

    /// Move selection up.
    pub fn up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            self.adjust_scroll();
        }
    }

    /// Move selection down.
    pub fn down(&mut self) {
        if self.selected < self.count.saturating_sub(1) {
            self.selected += 1;
            self.adjust_scroll();
        }
    }

    /// Move to first item.
    pub fn first(&mut self) {
        self.selected = 0;
        self.offset = 0;
    }

    /// Move to last item.
    pub fn last(&mut self) {
        self.selected = self.count.saturating_sub(1);
        self.adjust_scroll();
    }

    /// Page up.
    pub fn page_up(&mut self) {
        self.selected = self.selected.saturating_sub(self.visible_height);
        self.adjust_scroll();
    }

    /// Page down.
    pub fn page_down(&mut self) {
        self.selected = (self.selected + self.visible_height).min(self.count.saturating_sub(1));
        self.adjust_scroll();
    }

    /// Set selected index.
    pub fn select(&mut self, index: usize) {
        if index < self.count {
            self.selected = index;
            self.adjust_scroll();
        }
    }

    /// Adjust scroll to keep selection visible.
    fn adjust_scroll(&mut self) {
        if self.selected < self.offset {
            self.offset = self.selected;
        } else if self.selected >= self.offset + self.visible_height {
            self.offset = self.selected.saturating_sub(self.visible_height) + 1;
        }
    }

    /// Update item count.
    pub fn set_count(&mut self, count: usize) {
        self.count = count;
        if self.selected >= count && count > 0 {
            self.selected = count - 1;
        }
        self.adjust_scroll();
    }

    /// Get visible range.
    pub fn visible_range(&self) -> std::ops::Range<usize> {
        let end = (self.offset + self.visible_height).min(self.count);
        self.offset..end
    }
}

/// Create a new scroll list state.
pub fn create_scroll_list_state(count: usize) -> ScrollListState {
    ScrollListState::new(count)
}

/// Scroll list component.
#[derive(Debug, Clone)]
pub struct ScrollList<T, F>
where
    F: Fn(&T, usize, bool) -> VNode,
{
    items: Vec<T>,
    render_item: F,
    state: ScrollListState,
    show_scrollbar: bool,
    show_index: bool,
    highlight_selected: bool,
    border: bool,
}

impl<T, F> ScrollList<T, F>
where
    F: Fn(&T, usize, bool) -> VNode,
{
    /// Create a new scroll list.
    pub fn new<I>(items: I, render_item: F) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let items: Vec<T> = items.into_iter().collect();
        let count = items.len();
        Self {
            items,
            render_item,
            state: ScrollListState::new(count),
            show_scrollbar: true,
            show_index: false,
            highlight_selected: true,
            border: false,
        }
    }

    /// Set state.
    pub fn state(mut self, state: ScrollListState) -> Self {
        self.state = state;
        self
    }

    /// Set visible height.
    pub fn height(mut self, height: usize) -> Self {
        self.state.visible_height = height;
        self
    }

    /// Show/hide scrollbar.
    pub fn scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }

    /// Show/hide index numbers.
    pub fn show_index(mut self, show: bool) -> Self {
        self.show_index = show;
        self
    }

    /// Enable/disable selection highlighting.
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight_selected = highlight;
        self
    }

    /// Show border.
    pub fn border(mut self, show: bool) -> Self {
        self.border = show;
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let range = self.state.visible_range();
        let mut children: Vec<VNode> = Vec::new();

        for idx in range.clone() {
            if let Some(item) = self.items.get(idx) {
                let is_selected = idx == self.state.selected;
                let item_node = (self.render_item)(item, idx, is_selected);

                let row = if self.show_index {
                    let index_style = TextStyle {
                        color: Some(Color::Named(NamedColor::Gray)),
                        dim: true,
                        ..Default::default()
                    };
                    VNode::Box(BoxNode {
                        children: vec![
                            VNode::styled_text(format!("{:3} ", idx + 1), index_style),
                            item_node,
                        ],
                        style: BoxStyle {
                            flex_direction: Some(FlexDirection::Row),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                } else {
                    item_node
                };

                // Wrap with selection highlighting
                let row = if self.highlight_selected && is_selected {
                    VNode::Box(BoxNode {
                        children: vec![row],
                        style: BoxStyle {
                            // Selection styling handled by render_item
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                } else {
                    row
                };

                children.push(row);
            }
        }

        // Add scrollbar
        if self.show_scrollbar && self.state.count > self.state.visible_height {
            let scrollbar = self.build_scrollbar();
            let content = VNode::Box(BoxNode {
                children,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Column),
                    flex_grow: Some(1.0),
                    ..Default::default()
                },
                ..Default::default()
            });

            children = vec![VNode::Box(BoxNode {
                children: vec![content, scrollbar],
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Row),
                    ..Default::default()
                },
                ..Default::default()
            })];
        }

        let mut container_style = BoxStyle {
            flex_direction: Some(FlexDirection::Column),
            height: Some(Size::Fixed(self.state.visible_height as u16)),
            ..Default::default()
        };

        if self.border {
            container_style.border_style = Some(BorderStyle::Single);
        }

        VNode::Box(BoxNode {
            children,
            style: container_style,
            ..Default::default()
        })
    }

    /// Build scrollbar VNode.
    fn build_scrollbar(&self) -> VNode {
        let total = self.state.count;
        let visible = self.state.visible_height;
        let offset = self.state.offset;

        if total <= visible {
            return VNode::text(String::new());
        }

        let track_height = visible;
        let thumb_height = ((visible as f32 / total as f32) * track_height as f32)
            .ceil()
            .max(1.0) as usize;
        let thumb_pos = ((offset as f32 / (total - visible) as f32)
            * (track_height - thumb_height) as f32) as usize;

        let mut scrollbar_chars = String::new();
        for i in 0..track_height {
            if i >= thumb_pos && i < thumb_pos + thumb_height {
                scrollbar_chars.push('█');
            } else {
                scrollbar_chars.push('░');
            }
            scrollbar_chars.push('\n');
        }
        scrollbar_chars.pop(); // Remove trailing newline

        VNode::styled_text(
            scrollbar_chars,
            TextStyle {
                color: Some(Color::Named(NamedColor::Gray)),
                ..Default::default()
            },
        )
    }
}

/// Convenience function to create a simple string list.
pub fn simple_scroll_list<I, S>(
    items: I,
) -> ScrollList<String, impl Fn(&String, usize, bool) -> VNode>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let items: Vec<String> = items.into_iter().map(|s| s.into()).collect();
    ScrollList::new(items, |item: &String, _idx: usize, selected: bool| {
        let style = if selected {
            TextStyle {
                color: Some(Color::Named(NamedColor::Cyan)),
                bold: true,
                inverse: true,
                ..Default::default()
            }
        } else {
            TextStyle::default()
        };
        VNode::styled_text(item.clone(), style)
    })
}

// =============================================================================
// JS-Compatible API
// =============================================================================

/// Options for `useScrollList`.
pub struct UseScrollListOptions {
    /// Inverted scrolling direction.
    pub inverted: bool,
}

impl Default for UseScrollListOptions {
    fn default() -> Self {
        Self { inverted: false }
    }
}

/// Return type for `useScrollList`.
#[derive(Clone)]
pub struct UseScrollListReturn {
    /// Backing state.
    pub state: ScrollListState,
}

impl UseScrollListReturn {
    /// Scroll to the bottom of the list.
    pub fn scroll_to_bottom(&self) {
        // Placeholder for hook-oriented API compatibility.
        let _ = self;
    }

    /// Scroll to the top of the list.
    pub fn scroll_to_top(&self) {
        let _ = self;
    }

    /// Scroll by a delta.
    pub fn scroll_by(&self, _delta: isize) {
        let _ = self;
    }

    /// Get the current scroll top.
    pub fn scroll_top(&self) -> usize {
        self.state.offset
    }

    /// Get max scroll.
    pub fn max_scroll(&self) -> usize {
        self.state.count.saturating_sub(self.state.visible_height)
    }

    /// Bind helper payload.
    pub fn bind(&self) -> ScrollListState {
        self.state.clone()
    }
}

/// Create a scroll list state.
pub fn createScrollList(count: usize) -> ScrollListState {
    create_scroll_list_state(count)
}

/// Snake_case alias.
pub fn create_scroll_list(count: usize) -> ScrollListState {
    create_scroll_list_state(count)
}

/// Hook-style API wrapper.
pub fn useScrollList(_options: UseScrollListOptions) -> UseScrollListReturn {
    UseScrollListReturn {
        state: create_scroll_list_state(0),
    }
}

/// Cached list clearing no-op compatibility hook.
pub fn clearScrollListCache() {}

/// Invalidate a cached list item compatibility hook.
pub fn invalidateScrollListItem<T>(_item: &T) {
    let _ = _item;
}

/// Chat list alias.
pub struct ChatListProps {
    pub items: Vec<String>,
}

fn chat_list_item(item: &String, _idx: usize, _selected: bool) -> VNode {
    VNode::styled_text(format!("{}", item), TextStyle::default())
}

/// Create a chat-style list from string items.
pub fn ChatList<I, S>(items: I) -> ScrollList<String, fn(&String, usize, bool) -> VNode>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let items: Vec<String> = items.into_iter().map(|s| s.into()).collect();
    ScrollList::new(items, chat_list_item)
}

/// Type aliases for compat with JS export shape.
pub type ScrollListProps<T> = ScrollList<T, fn(&T, usize, bool) -> VNode>;
pub type ScrollListRenderer<T> = fn(&T, usize, bool) -> VNode;
pub type ChatListRenderer = fn(&String, usize, bool) -> VNode;
pub type ChatListFn = fn(Vec<String>) -> ScrollList<String, ChatListRenderer>;
