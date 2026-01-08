//! VirtualList Component
//!
//! Efficiently renders large lists by only rendering visible items.
//! Supports fixed and variable item heights.

use crate::core::component::{
    BoxNode, BoxStyle, Color, EventHandlers, NamedColor, TextNode, TextStyle, VNode,
};
use crate::core::layout::{AlignItems, FlexDirection, Size};
use crate::core::signals::{create_signal, ReadSignal, WriteSignal};

/// Item renderer function type.
pub type ItemRenderer<T> = fn(&T, usize) -> VNode;

/// VirtualList component.
///
/// Efficiently displays large lists by only rendering visible items.
///
/// # Example
/// ```ignore
/// let items: Vec<String> = (0..10000).map(|i| format!("Item {}", i)).collect();
///
/// let list = VirtualList::new()
///     .items(&items)
///     .item_height(1)
///     .visible_height(20)
///     .render_item(|item, _idx| VNode::text(item))
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct VirtualList<T> {
    /// Items to render
    items: Vec<T>,
    /// Height of each item (in lines)
    item_height: u16,
    /// Visible height (in lines)
    visible_height: u16,
    /// Visible width
    width: Option<u16>,
    /// Current scroll offset (item index)
    scroll_offset: usize,
    /// Overscan (extra items to render above/below visible area)
    overscan: usize,
    /// Show scrollbar
    show_scrollbar: bool,
    /// Item renderer (for building VNodes from items)
    renderer: Option<ItemRenderer<T>>,
}

impl<T: Clone> VirtualList<T> {
    /// Create a new virtual list.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            item_height: 1,
            visible_height: 10,
            width: None,
            scroll_offset: 0,
            overscan: 2,
            show_scrollbar: true,
            renderer: None,
        }
    }

    /// Set the items.
    pub fn items(mut self, items: &[T]) -> Self {
        self.items = items.to_vec();
        self
    }

    /// Set item height.
    pub fn item_height(mut self, height: u16) -> Self {
        self.item_height = height;
        self
    }

    /// Set visible height.
    pub fn visible_height(mut self, height: u16) -> Self {
        self.visible_height = height;
        self
    }

    /// Set width.
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Set scroll offset.
    pub fn scroll_offset(mut self, offset: usize) -> Self {
        self.scroll_offset = offset;
        self
    }

    /// Set overscan count.
    pub fn overscan(mut self, count: usize) -> Self {
        self.overscan = count;
        self
    }

    /// Set scrollbar visibility.
    pub fn scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }

    /// Set the item renderer.
    pub fn render_item(mut self, renderer: ItemRenderer<T>) -> Self {
        self.renderer = Some(renderer);
        self
    }

    /// Build the list VNode.
    pub fn build(self) -> VNode {
        let total_items = self.items.len();
        let items_per_screen = (self.visible_height / self.item_height) as usize;

        // Calculate visible range with overscan
        let start = self.scroll_offset.saturating_sub(self.overscan);
        let end = (self.scroll_offset + items_per_screen + self.overscan).min(total_items);

        // Render visible items
        let mut visible_items: Vec<VNode> = self.items[start..end]
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let idx = start + i;
                if let Some(renderer) = self.renderer {
                    renderer(item, idx)
                } else {
                    // Default: empty box
                    VNode::Box(BoxNode::default())
                }
            })
            .collect();

        // Add top spacer for scrolled-past items
        if start > 0 {
            let spacer_height = (start * self.item_height as usize) as u16;
            visible_items.insert(
                0,
                VNode::Box(BoxNode {
                    style: BoxStyle {
                        height: Some(Size::Fixed(spacer_height)),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            );
        }

        // Add bottom spacer for items below
        if end < total_items {
            let remaining = total_items - end;
            let spacer_height = (remaining * self.item_height as usize) as u16;
            visible_items.push(VNode::Box(BoxNode {
                style: BoxStyle {
                    height: Some(Size::Fixed(spacer_height)),
                    ..Default::default()
                },
                ..Default::default()
            }));
        }

        // Content container
        let content = VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                flex_grow: Some(1.0),
                height: Some(Size::Fixed(self.visible_height)),
                width: self.width.map(Size::Fixed),
                ..Default::default()
            },
            children: visible_items,
            handlers: EventHandlers {
                focusable: true,
                ..Default::default()
            },
        });

        if !self.show_scrollbar || total_items <= items_per_screen {
            return content;
        }

        // Build scrollbar
        let scrollbar = self.build_scrollbar(items_per_screen, total_items);

        // Container with content and scrollbar
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                height: Some(Size::Fixed(self.visible_height)),
                ..Default::default()
            },
            children: vec![content, scrollbar],
            handlers: Default::default(),
        })
    }

    /// Build the scrollbar.
    fn build_scrollbar(&self, items_per_screen: usize, total_items: usize) -> VNode {
        let visible_height = self.visible_height as usize;
        let thumb_size = ((items_per_screen * visible_height) / total_items).max(1);
        let max_scroll = total_items.saturating_sub(items_per_screen);
        let thumb_pos = if max_scroll > 0 {
            (self.scroll_offset * (visible_height - thumb_size)) / max_scroll
        } else {
            0
        };

        let mut lines = Vec::new();
        for i in 0..visible_height {
            let is_thumb = i >= thumb_pos && i < thumb_pos + thumb_size;
            let (ch, color) = if is_thumb {
                ('█', Color::Named(NamedColor::White))
            } else {
                ('│', Color::Named(NamedColor::BrightBlack))
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

impl<T: Clone> Default for VirtualList<T> {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// VirtualList State
// =============================================================================

/// State manager for VirtualList.
pub struct VirtualListState {
    /// Current scroll offset (first visible item index)
    offset_read: ReadSignal<usize>,
    offset_write: WriteSignal<usize>,
    /// Total number of items
    total_items_read: ReadSignal<usize>,
    total_items_write: WriteSignal<usize>,
    /// Number of visible items
    visible_items_read: ReadSignal<usize>,
    visible_items_write: WriteSignal<usize>,
    /// Currently selected item index
    selected_read: ReadSignal<Option<usize>>,
    selected_write: WriteSignal<Option<usize>>,
}

impl VirtualListState {
    /// Create a new virtual list state.
    pub fn new(visible_items: usize) -> Self {
        let (offset_read, offset_write) = create_signal(0usize);
        let (total_items_read, total_items_write) = create_signal(0usize);
        let (visible_items_read, visible_items_write) = create_signal(visible_items);
        let (selected_read, selected_write) = create_signal(None);
        Self {
            offset_read,
            offset_write,
            total_items_read,
            total_items_write,
            visible_items_read,
            visible_items_write,
            selected_read,
            selected_write,
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

    /// Set total items count.
    pub fn set_total_items(&self, count: usize) {
        self.total_items_write.set(count);
        // Clamp offset
        let max = self.max_offset();
        if self.offset_read.get() > max {
            self.offset_write.set(max);
        }
    }

    /// Get maximum scroll offset.
    pub fn max_offset(&self) -> usize {
        let total = self.total_items_read.get();
        let visible = self.visible_items_read.get();
        total.saturating_sub(visible)
    }

    /// Scroll up by one item.
    pub fn scroll_up(&self) {
        let current = self.offset_read.get();
        if current > 0 {
            self.offset_write.set(current - 1);
        }
    }

    /// Scroll down by one item.
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
        let page = self.visible_items_read.get();
        self.offset_write.set(current.saturating_sub(page));
    }

    /// Scroll down by a page.
    pub fn page_down(&self) {
        let current = self.offset_read.get();
        let page = self.visible_items_read.get();
        let max = self.max_offset();
        self.offset_write.set((current + page).min(max));
    }

    /// Scroll to top.
    pub fn scroll_to_top(&self) {
        self.offset_write.set(0);
    }

    /// Scroll to bottom.
    pub fn scroll_to_bottom(&self) {
        self.offset_write.set(self.max_offset());
    }

    /// Get selected item index.
    pub fn selected(&self) -> Option<usize> {
        self.selected_read.get()
    }

    /// Set selected item.
    pub fn select(&self, index: Option<usize>) {
        self.selected_write.set(index);
        // Ensure selected item is visible
        if let Some(idx) = index {
            self.ensure_visible(idx);
        }
    }

    /// Select previous item.
    pub fn select_prev(&self) {
        let current = self.selected_read.get().unwrap_or(0);
        if current > 0 {
            self.select(Some(current - 1));
        }
    }

    /// Select next item.
    pub fn select_next(&self) {
        let current = self.selected_read.get().unwrap_or(0);
        let total = self.total_items_read.get();
        if current + 1 < total {
            self.select(Some(current + 1));
        }
    }

    /// Ensure an item is visible.
    pub fn ensure_visible(&self, index: usize) {
        let offset = self.offset_read.get();
        let visible = self.visible_items_read.get();

        if index < offset {
            // Item is above visible area
            self.offset_write.set(index);
        } else if index >= offset + visible {
            // Item is below visible area
            self.offset_write.set(index - visible + 1);
        }
    }

    /// Set the number of visible items (for resizing).
    pub fn set_visible_items(&self, count: usize) {
        self.visible_items_write.set(count);
        // Clamp offset if needed
        let max = self.max_offset();
        if self.offset_read.get() > max {
            self.offset_write.set(max);
        }
    }

    /// Get the offset signal.
    pub fn offset_signal(&self) -> ReadSignal<usize> {
        self.offset_read.clone()
    }

    /// Get the selected signal.
    pub fn selected_signal(&self) -> ReadSignal<Option<usize>> {
        self.selected_read.clone()
    }
}

impl Default for VirtualListState {
    fn default() -> Self {
        Self::new(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_list_empty() {
        let list: VirtualList<String> = VirtualList::new();
        let node = list.build();
        matches!(node, VNode::Box(_));
    }

    #[test]
    fn test_virtual_list_state() {
        let state = VirtualListState::new(10);
        state.set_total_items(100);

        assert_eq!(state.offset(), 0);
        assert_eq!(state.max_offset(), 90);

        state.scroll_down();
        assert_eq!(state.offset(), 1);

        state.page_down();
        assert_eq!(state.offset(), 11);

        state.scroll_to_bottom();
        assert_eq!(state.offset(), 90);
    }

    #[test]
    fn test_virtual_list_selection() {
        let state = VirtualListState::new(10);
        state.set_total_items(100);

        assert_eq!(state.selected(), None);

        state.select(Some(5));
        assert_eq!(state.selected(), Some(5));

        state.select_next();
        assert_eq!(state.selected(), Some(6));

        state.select_prev();
        assert_eq!(state.selected(), Some(5));
    }

    #[test]
    fn test_ensure_visible() {
        let state = VirtualListState::new(10);
        state.set_total_items(100);

        // Select item outside visible area
        state.select(Some(50));
        assert_eq!(state.offset(), 41); // Should scroll to make item visible
    }
}
