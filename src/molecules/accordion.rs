//! Accordion Molecule
//!
//! Multiple collapsible sections that can optionally auto-close others.
//!
//! ```text
//! ▼ Section 1
//! │ Content for section 1...
//!
//! ▶ Section 2
//!
//! ▶ Section 3
//! ```

use crate::core::component::{
    BorderStyle, BoxNode, BoxStyle, Color, NamedColor, TextNode, TextStyle, VNode,
};
use crate::core::layout::{AlignItems, FlexDirection};

// =============================================================================
// Accordion Component
// =============================================================================

/// An item in the accordion
#[derive(Debug, Clone)]
pub struct AccordionItem {
    /// Item title
    pub title: String,
    /// Item content
    pub content: VNode,
    /// Whether this item is expanded
    pub expanded: bool,
    /// Disabled state
    pub disabled: bool,
}

impl AccordionItem {
    /// Create a new accordion item.
    pub fn new(title: impl Into<String>, content: VNode) -> Self {
        Self {
            title: title.into(),
            content,
            expanded: false,
            disabled: false,
        }
    }

    /// Set expanded state.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Accordion state for reactive updates
#[derive(Debug, Clone)]
pub struct AccordionState {
    /// Index of expanded items (empty = all collapsed)
    pub expanded: Vec<usize>,
    /// Allow multiple items to be expanded
    pub allow_multiple: bool,
}

impl Default for AccordionState {
    fn default() -> Self {
        Self {
            expanded: Vec::new(),
            allow_multiple: false,
        }
    }
}

impl AccordionState {
    /// Create state that allows multiple expansions
    pub fn multiple() -> Self {
        Self {
            expanded: Vec::new(),
            allow_multiple: true,
        }
    }

    /// Toggle item at index
    pub fn toggle(&mut self, index: usize) {
        if self.expanded.contains(&index) {
            self.expanded.retain(|&i| i != index);
        } else {
            if !self.allow_multiple {
                self.expanded.clear();
            }
            self.expanded.push(index);
        }
    }

    /// Check if item is expanded
    pub fn is_expanded(&self, index: usize) -> bool {
        self.expanded.contains(&index)
    }

    /// Expand all items
    pub fn expand_all(&mut self, count: usize) {
        self.expanded = (0..count).collect();
    }

    /// Collapse all items
    pub fn collapse_all(&mut self) {
        self.expanded.clear();
    }
}

/// Accordion component - multiple collapsible sections.
///
/// # Example
/// ```ignore
/// let accordion = Accordion::new()
///     .add_item(AccordionItem::new("Section 1", content1).expanded(true))
///     .add_item(AccordionItem::new("Section 2", content2))
///     .add_item(AccordionItem::new("Section 3", content3))
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Accordion {
    /// Accordion items
    items: Vec<AccordionItem>,
    /// Allow multiple items expanded
    allow_multiple: bool,
    /// Gap between items
    gap: u16,
    /// Collapsed icon
    collapsed_icon: String,
    /// Expanded icon
    expanded_icon: String,
    /// Show dividers between items
    dividers: bool,
    /// Divider color
    divider_color: Color,
    /// Title color
    title_color: Color,
    /// Icon color
    icon_color: Color,
    /// Border around each item
    bordered: bool,
    /// Border color
    border_color: Color,
}

impl Default for Accordion {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            allow_multiple: false,
            gap: 0,
            collapsed_icon: "▶".to_string(),
            expanded_icon: "▼".to_string(),
            dividers: true,
            divider_color: Color::Named(NamedColor::BrightBlack),
            title_color: Color::Named(NamedColor::White),
            icon_color: Color::Named(NamedColor::Gray),
            bordered: false,
            border_color: Color::Named(NamedColor::BrightBlack),
        }
    }
}

impl Accordion {
    /// Create a new accordion.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an item.
    pub fn add_item(mut self, item: AccordionItem) -> Self {
        self.items.push(item);
        self
    }

    /// Set items.
    pub fn items(mut self, items: Vec<AccordionItem>) -> Self {
        self.items = items;
        self
    }

    /// Allow multiple items to be expanded.
    pub fn allow_multiple(mut self, allow: bool) -> Self {
        self.allow_multiple = allow;
        self
    }

    /// Set gap between items.
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Set icons.
    pub fn icons(mut self, collapsed: impl Into<String>, expanded: impl Into<String>) -> Self {
        self.collapsed_icon = collapsed.into();
        self.expanded_icon = expanded.into();
        self
    }

    /// Show dividers between items.
    pub fn dividers(mut self, show: bool) -> Self {
        self.dividers = show;
        self
    }

    /// Set divider color.
    pub fn divider_color(mut self, color: Color) -> Self {
        self.divider_color = color;
        self
    }

    /// Set title color.
    pub fn title_color(mut self, color: Color) -> Self {
        self.title_color = color;
        self
    }

    /// Set icon color.
    pub fn icon_color(mut self, color: Color) -> Self {
        self.icon_color = color;
        self
    }

    /// Set bordered items.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Set border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Apply state
    pub fn state(mut self, state: &AccordionState) -> Self {
        for (i, item) in self.items.iter_mut().enumerate() {
            item.expanded = state.is_expanded(i);
        }
        self.allow_multiple = state.allow_multiple;
        self
    }

    /// Build a single accordion item.
    fn build_item(&self, item: &AccordionItem) -> VNode {
        let icon = if item.expanded {
            &self.expanded_icon
        } else {
            &self.collapsed_icon
        };

        // Header
        let header = VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                align_items: Some(AlignItems::Center),
                gap: Some(1),
                padding: Some(1),
                ..Default::default()
            },
            children: vec![
                VNode::Text(TextNode {
                    content: icon.clone(),
                    style: TextStyle {
                        color: Some(if item.disabled {
                            Color::Named(NamedColor::Gray)
                        } else {
                            self.icon_color.clone()
                        }),
                        dim: item.disabled,
                        ..Default::default()
                    },
                }),
                VNode::Text(TextNode {
                    content: item.title.clone(),
                    style: TextStyle {
                        color: Some(if item.disabled {
                            Color::Named(NamedColor::Gray)
                        } else {
                            self.title_color.clone()
                        }),
                        bold: true,
                        dim: item.disabled,
                        ..Default::default()
                    },
                }),
            ],
            handlers: Default::default(),
        });

        let mut item_children = vec![header];

        // Content (if expanded)
        if item.expanded {
            let content_box = VNode::Box(BoxNode {
                id: None,
                style: BoxStyle {
                    padding_left: Some(2),
                    padding: Some(1),
                    ..Default::default()
                },
                children: vec![item.content.clone()],
                handlers: Default::default(),
            });
            item_children.push(content_box);
        }

        // Item container
        let mut item_style = BoxStyle {
            flex_direction: Some(FlexDirection::Column),
            ..Default::default()
        };

        if self.bordered {
            item_style.border_style = Some(BorderStyle::Single);
            item_style.border_color = Some(self.border_color.clone());
        }

        VNode::Box(BoxNode {
            id: None,
            style: item_style,
            children: item_children,
            handlers: Default::default(),
        })
    }

    /// Build the accordion VNode.
    pub fn build(self) -> VNode {
        let mut children = Vec::new();
        let item_count = self.items.len();

        for (i, item) in self.items.iter().enumerate() {
            children.push(self.build_item(item));

            // Add divider (except after last item)
            if self.dividers && i < item_count - 1 && !self.bordered {
                children.push(VNode::Text(TextNode {
                    content: "─".repeat(40),
                    style: TextStyle {
                        color: Some(self.divider_color.clone()),
                        ..Default::default()
                    },
                }));
            }
        }

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                gap: Some(self.gap),
                ..Default::default()
            },
            children,
            handlers: Default::default(),
        })
    }
}

impl From<Accordion> for VNode {
    fn from(accordion: Accordion) -> VNode {
        accordion.build()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn text(s: &str) -> VNode {
        VNode::Text(TextNode {
            content: s.to_string(),
            style: Default::default(),
        })
    }

    #[test]
    fn test_accordion_empty() {
        let accordion = Accordion::new().build();
        matches!(accordion, VNode::Box(_));
    }

    #[test]
    fn test_accordion_with_items() {
        let accordion = Accordion::new()
            .add_item(AccordionItem::new("Section 1", text("Content 1")))
            .add_item(AccordionItem::new("Section 2", text("Content 2")))
            .build();

        if let VNode::Box(node) = accordion {
            // 2 items + 1 divider
            assert!(node.children.len() >= 2);
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_accordion_expanded() {
        let accordion = Accordion::new()
            .add_item(AccordionItem::new("Section 1", text("Content 1")).expanded(true))
            .add_item(AccordionItem::new("Section 2", text("Content 2")))
            .build();
        matches!(accordion, VNode::Box(_));
    }

    #[test]
    fn test_accordion_state() {
        let mut state = AccordionState::default();
        assert!(!state.is_expanded(0));

        state.toggle(0);
        assert!(state.is_expanded(0));

        state.toggle(1);
        // Single mode: should close 0
        assert!(!state.is_expanded(0));
        assert!(state.is_expanded(1));
    }

    #[test]
    fn test_accordion_state_multiple() {
        let mut state = AccordionState::multiple();
        state.toggle(0);
        state.toggle(1);

        assert!(state.is_expanded(0));
        assert!(state.is_expanded(1));
    }

    #[test]
    fn test_accordion_no_dividers() {
        let accordion = Accordion::new()
            .add_item(AccordionItem::new("Section 1", text("Content 1")))
            .add_item(AccordionItem::new("Section 2", text("Content 2")))
            .dividers(false)
            .build();

        if let VNode::Box(node) = accordion {
            // Just 2 items, no dividers
            assert_eq!(node.children.len(), 2);
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_accordion_item() {
        let item = AccordionItem::new("Title", text("Content"))
            .expanded(true)
            .disabled(false);

        assert_eq!(item.title, "Title");
        assert!(item.expanded);
    }
}
