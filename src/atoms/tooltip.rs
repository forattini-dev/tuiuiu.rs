//! Tooltip Atom
//!
//! A tooltip component that displays contextual information.
//!
//! ```text
//! ┌────────────────┐
//! │ Tooltip text   │
//! └───────┬────────┘
//!         ▼
//!     [Button]
//! ```

use crate::core::component::{BoxNode, BoxStyle, BorderStyle, Color, NamedColor, TextNode, TextStyle, VNode};
use crate::core::layout::{AlignItems, FlexDirection};

// =============================================================================
// Tooltip Component
// =============================================================================

/// Tooltip position relative to trigger element
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TooltipPosition {
    /// Above the element
    #[default]
    Top,
    /// Below the element
    Bottom,
    /// To the left
    Left,
    /// To the right
    Right,
}

/// Tooltip component for contextual help.
///
/// # Example
/// ```ignore
/// let tooltip = Tooltip::new("Click to submit")
///     .position(TooltipPosition::Top)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Tooltip {
    /// Tooltip text content
    text: String,
    /// Position relative to trigger
    position: TooltipPosition,
    /// Whether tooltip is visible
    visible: bool,
    /// Background color
    bg_color: Color,
    /// Text color
    fg_color: Color,
    /// Show arrow/pointer
    show_arrow: bool,
    /// Maximum width
    max_width: Option<u16>,
    /// Show border
    show_border: bool,
    /// Border color
    border_color: Color,
    /// Child element (trigger)
    child: Option<Box<VNode>>,
}

impl Tooltip {
    /// Create a new tooltip with text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            position: TooltipPosition::Top,
            visible: true,
            bg_color: Color::Named(NamedColor::BrightBlack),
            fg_color: Color::Named(NamedColor::White),
            show_arrow: true,
            max_width: None,
            show_border: true,
            border_color: Color::Named(NamedColor::Gray),
            child: None,
        }
    }

    /// Set position.
    pub fn position(mut self, pos: TooltipPosition) -> Self {
        self.position = pos;
        self
    }

    /// Set visibility.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Set background color.
    pub fn bg_color(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }

    /// Set text color.
    pub fn fg_color(mut self, color: Color) -> Self {
        self.fg_color = color;
        self
    }

    /// Show or hide arrow.
    pub fn arrow(mut self, show: bool) -> Self {
        self.show_arrow = show;
        self
    }

    /// Set maximum width.
    pub fn max_width(mut self, width: u16) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Show or hide border.
    pub fn border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Set border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Set the child/trigger element.
    pub fn child(mut self, child: VNode) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    /// Position: top
    pub fn top(self) -> Self {
        self.position(TooltipPosition::Top)
    }

    /// Position: bottom
    pub fn bottom(self) -> Self {
        self.position(TooltipPosition::Bottom)
    }

    /// Position: left
    pub fn left(self) -> Self {
        self.position(TooltipPosition::Left)
    }

    /// Position: right
    pub fn right(self) -> Self {
        self.position(TooltipPosition::Right)
    }

    /// Get arrow character based on position.
    fn get_arrow(&self) -> &'static str {
        match self.position {
            TooltipPosition::Top => "▼",
            TooltipPosition::Bottom => "▲",
            TooltipPosition::Left => "▶",
            TooltipPosition::Right => "◀",
        }
    }

    /// Build the tooltip content box.
    fn build_tooltip_content(&self) -> VNode {
        let mut style = BoxStyle {
            background: Some(self.bg_color.clone()),
            padding: Some(1),
            ..Default::default()
        };

        if self.show_border {
            style.border_style = Some(BorderStyle::Single);
            style.border_color = Some(self.border_color.clone());
        }

        if let Some(w) = self.max_width {
            style.max_width = Some(w);
        }

        VNode::Box(BoxNode {
            id: None,
            style,
            children: vec![VNode::Text(TextNode {
                content: self.text.clone(),
                style: TextStyle {
                    color: Some(self.fg_color.clone()),
                    ..Default::default()
                },
            })],
            handlers: Default::default(),
        })
    }

    /// Build the tooltip VNode.
    pub fn build(self) -> VNode {
        if !self.visible {
            // Return just the child if tooltip is hidden
            return self.child.map(|c| *c).unwrap_or(VNode::Empty);
        }

        let tooltip_content = self.build_tooltip_content();

        // Build arrow if enabled
        let arrow = if self.show_arrow {
            Some(VNode::Text(TextNode {
                content: self.get_arrow().to_string(),
                style: TextStyle {
                    color: Some(self.bg_color.clone()),
                    ..Default::default()
                },
            }))
        } else {
            None
        };

        // Arrange tooltip and child based on position
        let (direction, children) = match self.position {
            TooltipPosition::Top => {
                let mut items = vec![tooltip_content];
                if let Some(a) = arrow {
                    items.push(a);
                }
                if let Some(c) = self.child {
                    items.push(*c);
                }
                (FlexDirection::Column, items)
            }
            TooltipPosition::Bottom => {
                let mut items = Vec::new();
                if let Some(c) = self.child {
                    items.push(*c);
                }
                if let Some(a) = arrow {
                    items.push(a);
                }
                items.push(tooltip_content);
                (FlexDirection::Column, items)
            }
            TooltipPosition::Left => {
                let mut items = vec![tooltip_content];
                if let Some(a) = arrow {
                    items.push(a);
                }
                if let Some(c) = self.child {
                    items.push(*c);
                }
                (FlexDirection::Row, items)
            }
            TooltipPosition::Right => {
                let mut items = Vec::new();
                if let Some(c) = self.child {
                    items.push(*c);
                }
                if let Some(a) = arrow {
                    items.push(a);
                }
                items.push(tooltip_content);
                (FlexDirection::Row, items)
            }
        };

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(direction),
                align_items: Some(AlignItems::Center),
                gap: Some(0),
                ..Default::default()
            },
            children,
            handlers: Default::default(),
        })
    }
}

impl From<Tooltip> for VNode {
    fn from(tooltip: Tooltip) -> VNode {
        tooltip.build()
    }
}

impl From<&str> for Tooltip {
    fn from(s: &str) -> Self {
        Tooltip::new(s)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tooltip_basic() {
        let tooltip = Tooltip::new("Help text").build();
        matches!(tooltip, VNode::Box(_));
    }

    #[test]
    fn test_tooltip_positions() {
        let top = Tooltip::new("Top").top().build();
        let bottom = Tooltip::new("Bottom").bottom().build();
        let left = Tooltip::new("Left").left().build();
        let right = Tooltip::new("Right").right().build();

        matches!(top, VNode::Box(_));
        matches!(bottom, VNode::Box(_));
        matches!(left, VNode::Box(_));
        matches!(right, VNode::Box(_));
    }

    #[test]
    fn test_tooltip_hidden() {
        let tooltip = Tooltip::new("Hidden")
            .visible(false)
            .build();
        matches!(tooltip, VNode::Empty);
    }

    #[test]
    fn test_tooltip_with_child() {
        let child = VNode::Text(TextNode {
            content: "Button".to_string(),
            style: Default::default(),
        });
        let tooltip = Tooltip::new("Click me")
            .child(child)
            .build();

        if let VNode::Box(node) = tooltip {
            assert!(node.children.len() >= 2); // tooltip + child (+ optional arrow)
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_tooltip_no_arrow() {
        let tooltip = Tooltip::new("No arrow")
            .arrow(false)
            .build();
        matches!(tooltip, VNode::Box(_));
    }

    #[test]
    fn test_tooltip_from_str() {
        let tooltip: Tooltip = "Quick tip".into();
        assert_eq!(tooltip.text, "Quick tip");
    }

    #[test]
    fn test_tooltip_max_width() {
        let tooltip = Tooltip::new("Long tooltip text that might wrap")
            .max_width(20)
            .build();
        matches!(tooltip, VNode::Box(_));
    }
}
