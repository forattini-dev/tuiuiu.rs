//! Collapsible Molecule
//!
//! An expandable/collapsible section component.
//!
//! ```text
//! ▶ Section Title (collapsed)
//!
//! ▼ Section Title (expanded)
//! │ Content here...
//! │ More content...
//! └─────────────────
//! ```

use crate::core::component::{BoxNode, BoxStyle, BorderStyle, Color, NamedColor, TextNode, TextStyle, VNode};
use crate::core::layout::{AlignItems, FlexDirection};
use crate::core::signals::create_signal;

// =============================================================================
// Collapsible Component
// =============================================================================

/// Collapsible state for reactive updates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollapsibleState {
    /// Whether the section is expanded
    pub expanded: bool,
}

impl Default for CollapsibleState {
    fn default() -> Self {
        Self { expanded: false }
    }
}

impl CollapsibleState {
    /// Create expanded state
    pub fn expanded() -> Self {
        Self { expanded: true }
    }

    /// Create collapsed state
    pub fn collapsed() -> Self {
        Self { expanded: false }
    }

    /// Toggle the state
    pub fn toggle(&mut self) {
        self.expanded = !self.expanded;
    }
}

/// Collapsible section component.
///
/// # Example
/// ```ignore
/// let collapsible = Collapsible::new("Advanced Settings")
///     .expanded(true)
///     .content(settings_content)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Collapsible {
    /// Section title
    title: String,
    /// Whether expanded
    expanded: bool,
    /// Content when expanded
    content: Option<VNode>,
    /// Icon for collapsed state
    collapsed_icon: String,
    /// Icon for expanded state
    expanded_icon: String,
    /// Show border around content
    bordered: bool,
    /// Border color
    border_color: Color,
    /// Title color
    title_color: Color,
    /// Icon color
    icon_color: Color,
    /// Disabled state
    disabled: bool,
    /// Animate (hint for renderers)
    animated: bool,
}

impl Default for Collapsible {
    fn default() -> Self {
        Self {
            title: String::new(),
            expanded: false,
            content: None,
            collapsed_icon: "▶".to_string(),
            expanded_icon: "▼".to_string(),
            bordered: false,
            border_color: Color::Named(NamedColor::BrightBlack),
            title_color: Color::Named(NamedColor::White),
            icon_color: Color::Named(NamedColor::Gray),
            disabled: false,
            animated: true,
        }
    }
}

impl Collapsible {
    /// Create a new collapsible section.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Set expanded state.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set content.
    pub fn content(mut self, content: VNode) -> Self {
        self.content = Some(content);
        self
    }

    /// Set collapsed icon.
    pub fn collapsed_icon(mut self, icon: impl Into<String>) -> Self {
        self.collapsed_icon = icon.into();
        self
    }

    /// Set expanded icon.
    pub fn expanded_icon(mut self, icon: impl Into<String>) -> Self {
        self.expanded_icon = icon.into();
        self
    }

    /// Set icons for both states.
    pub fn icons(mut self, collapsed: impl Into<String>, expanded: impl Into<String>) -> Self {
        self.collapsed_icon = collapsed.into();
        self.expanded_icon = expanded.into();
        self
    }

    /// Set bordered content.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Set border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
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

    /// Set disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set animated hint.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Apply state from CollapsibleState
    pub fn state(mut self, state: CollapsibleState) -> Self {
        self.expanded = state.expanded;
        self
    }

    /// Build the collapsible VNode.
    pub fn build(self) -> VNode {
        let icon = if self.expanded {
            &self.expanded_icon
        } else {
            &self.collapsed_icon
        };

        // Header row
        let header = VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                align_items: Some(AlignItems::Center),
                gap: Some(1),
                ..Default::default()
            },
            children: vec![
                VNode::Text(TextNode {
                    content: icon.clone(),
                    style: TextStyle {
                        color: Some(if self.disabled {
                            Color::Named(NamedColor::Gray)
                        } else {
                            self.icon_color.clone()
                        }),
                        dim: self.disabled,
                        ..Default::default()
                    },
                }),
                VNode::Text(TextNode {
                    content: self.title.clone(),
                    style: TextStyle {
                        color: Some(if self.disabled {
                            Color::Named(NamedColor::Gray)
                        } else {
                            self.title_color.clone()
                        }),
                        bold: true,
                        dim: self.disabled,
                        ..Default::default()
                    },
                }),
            ],
            handlers: Default::default(),
        });

        // Build content section
        let content = if self.expanded {
            self.content.clone()
        } else {
            None
        };

        // Container
        let mut children = vec![header];

        if let Some(c) = content {
            let content_box = if self.bordered {
                VNode::Box(BoxNode {
                    id: None,
                    style: BoxStyle {
                        padding_left: Some(2),
                        padding: Some(1),
                        border_style: Some(BorderStyle::Single),
                        border_color: Some(self.border_color.clone()),
                        ..Default::default()
                    },
                    children: vec![c],
                    handlers: Default::default(),
                })
            } else {
                VNode::Box(BoxNode {
                    id: None,
                    style: BoxStyle {
                        padding_left: Some(2),
                        ..Default::default()
                    },
                    children: vec![c],
                    handlers: Default::default(),
                })
            };
            children.push(content_box);
        }

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                ..Default::default()
            },
            children,
            handlers: Default::default(),
        })
    }
}

impl From<Collapsible> for VNode {
    fn from(c: Collapsible) -> VNode {
        c.build()
    }
}

/// Create reactive collapsible state
pub fn create_collapsible_state(expanded: bool) -> (
    crate::core::signals::ReadSignal<CollapsibleState>,
    crate::core::signals::WriteSignal<CollapsibleState>,
) {
    create_signal(CollapsibleState { expanded })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collapsible_basic() {
        let collapsible = Collapsible::new("Section").build();
        matches!(collapsible, VNode::Box(_));
    }

    #[test]
    fn test_collapsible_expanded() {
        let content = VNode::Text(TextNode {
            content: "Content".to_string(),
            style: Default::default(),
        });
        let collapsible = Collapsible::new("Section")
            .expanded(true)
            .content(content)
            .build();

        if let VNode::Box(node) = collapsible {
            assert_eq!(node.children.len(), 2); // header + content
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_collapsible_collapsed() {
        let content = VNode::Text(TextNode {
            content: "Content".to_string(),
            style: Default::default(),
        });
        let collapsible = Collapsible::new("Section")
            .expanded(false)
            .content(content)
            .build();

        if let VNode::Box(node) = collapsible {
            assert_eq!(node.children.len(), 1); // header only
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_collapsible_state() {
        let mut state = CollapsibleState::collapsed();
        assert!(!state.expanded);
        state.toggle();
        assert!(state.expanded);
    }

    #[test]
    fn test_collapsible_bordered() {
        let content = VNode::Text(TextNode {
            content: "Content".to_string(),
            style: Default::default(),
        });
        let collapsible = Collapsible::new("Section")
            .expanded(true)
            .bordered(true)
            .content(content)
            .build();
        matches!(collapsible, VNode::Box(_));
    }

    #[test]
    fn test_collapsible_custom_icons() {
        let collapsible = Collapsible::new("Section")
            .icons("+", "-")
            .build();
        matches!(collapsible, VNode::Box(_));
    }
}
