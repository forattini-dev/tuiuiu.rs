//! StatusBar Template
//!
//! Application status bar (bottom of screen) showing mode, status,
//! and contextual information.
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │ NORMAL │ main.rs │ Ln 42, Col 15 │ UTF-8 │ LF │ Rust │ 12:30 │
//! └─────────────────────────────────────────────────────────────┘
//! ```

use crate::core::component::{
    BorderStyle, BoxNode, BoxStyle, Color, NamedColor, TextNode, TextStyle, VNode,
};
use crate::core::layout::{AlignItems, FlexDirection, JustifyContent, Size};

// =============================================================================
// StatusBar Component
// =============================================================================

/// Application status bar.
///
/// A bottom bar typically showing:
/// - Mode indicator (NORMAL, INSERT, etc.)
/// - Current file/context
/// - Position information
/// - File encoding, line endings
/// - Time/date
///
/// # Example
/// ```ignore
/// let status = StatusBar::new()
///     .mode("NORMAL", Color::Named(NamedColor::Green))
///     .left("main.rs")
///     .center("Ln 42, Col 15")
///     .right("UTF-8 │ LF │ Rust")
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct StatusBar {
    /// Mode indicator (e.g., NORMAL, INSERT)
    mode: Option<StatusMode>,
    /// Left section items
    left_items: Vec<StatusItem>,
    /// Center section items
    center_items: Vec<StatusItem>,
    /// Right section items
    right_items: Vec<StatusItem>,
    /// Custom left content
    left: Option<VNode>,
    /// Custom center content
    center: Option<VNode>,
    /// Custom right content
    right: Option<VNode>,
    /// Background color
    background: Option<Color>,
    /// Default text color
    text_color: Color,
    /// Separator character
    separator: String,
    /// Show top border
    show_border: bool,
    /// Border color
    border_color: Color,
    /// Height
    height: u16,
    /// Padding
    padding: u16,
}

/// Mode indicator for status bar.
#[derive(Debug, Clone)]
pub struct StatusMode {
    /// Mode text (e.g., "NORMAL", "INSERT")
    pub text: String,
    /// Background color
    pub background: Color,
    /// Text color
    pub text_color: Color,
}

impl StatusMode {
    /// Create a new mode indicator.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            background: Color::Named(NamedColor::Blue),
            text_color: Color::Named(NamedColor::Black),
        }
    }

    /// Set background color.
    pub fn background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    /// Set text color.
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    /// Create a "NORMAL" mode (green).
    pub fn normal() -> Self {
        Self::new("NORMAL").background(Color::Named(NamedColor::Green))
    }

    /// Create an "INSERT" mode (blue).
    pub fn insert() -> Self {
        Self::new("INSERT").background(Color::Named(NamedColor::Blue))
    }

    /// Create a "VISUAL" mode (magenta).
    pub fn visual() -> Self {
        Self::new("VISUAL").background(Color::Named(NamedColor::Magenta))
    }

    /// Create a "COMMAND" mode (yellow).
    pub fn command() -> Self {
        Self::new("COMMAND").background(Color::Named(NamedColor::Yellow))
    }

    /// Create a "REPLACE" mode (red).
    pub fn replace() -> Self {
        Self::new("REPLACE").background(Color::Named(NamedColor::Red))
    }
}

/// A single item in the status bar.
#[derive(Debug, Clone)]
pub struct StatusItem {
    /// Item text
    pub text: String,
    /// Icon (optional)
    pub icon: Option<String>,
    /// Text color
    pub color: Option<Color>,
    /// Bold
    pub bold: bool,
}

impl StatusItem {
    /// Create a new status item.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon: None,
            color: None,
            bold: false,
        }
    }

    /// Set icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set bold.
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }
}

impl StatusBar {
    /// Create a new status bar.
    pub fn new() -> Self {
        Self {
            mode: None,
            left_items: Vec::new(),
            center_items: Vec::new(),
            right_items: Vec::new(),
            left: None,
            center: None,
            right: None,
            background: Some(Color::Named(NamedColor::BrightBlack)),
            text_color: Color::Named(NamedColor::White),
            separator: " │ ".to_string(),
            show_border: false,
            border_color: Color::Named(NamedColor::Gray),
            height: 1,
            padding: 1,
        }
    }

    /// Set mode indicator.
    pub fn mode(mut self, mode: StatusMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Set mode from text and color.
    pub fn mode_text(mut self, text: impl Into<String>, background: Color) -> Self {
        self.mode = Some(StatusMode::new(text).background(background));
        self
    }

    /// Add left items.
    pub fn left_items(mut self, items: Vec<StatusItem>) -> Self {
        self.left_items = items;
        self
    }

    /// Add a left item.
    pub fn add_left(mut self, item: StatusItem) -> Self {
        self.left_items.push(item);
        self
    }

    /// Add left item from string.
    pub fn left_text(mut self, text: impl Into<String>) -> Self {
        self.left_items.push(StatusItem::new(text));
        self
    }

    /// Add center items.
    pub fn center_items(mut self, items: Vec<StatusItem>) -> Self {
        self.center_items = items;
        self
    }

    /// Add a center item.
    pub fn add_center(mut self, item: StatusItem) -> Self {
        self.center_items.push(item);
        self
    }

    /// Add center item from string.
    pub fn center_text(mut self, text: impl Into<String>) -> Self {
        self.center_items.push(StatusItem::new(text));
        self
    }

    /// Add right items.
    pub fn right_items(mut self, items: Vec<StatusItem>) -> Self {
        self.right_items = items;
        self
    }

    /// Add a right item.
    pub fn add_right(mut self, item: StatusItem) -> Self {
        self.right_items.push(item);
        self
    }

    /// Add right item from string.
    pub fn right_text(mut self, text: impl Into<String>) -> Self {
        self.right_items.push(StatusItem::new(text));
        self
    }

    /// Set custom left content.
    pub fn left(mut self, content: VNode) -> Self {
        self.left = Some(content);
        self
    }

    /// Set custom center content.
    pub fn center(mut self, content: VNode) -> Self {
        self.center = Some(content);
        self
    }

    /// Set custom right content.
    pub fn right(mut self, content: VNode) -> Self {
        self.right = Some(content);
        self
    }

    /// Set background color.
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Set text color.
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    /// Set separator.
    pub fn separator(mut self, sep: impl Into<String>) -> Self {
        self.separator = sep.into();
        self
    }

    /// Set whether to show border.
    pub fn border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Set border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Set height.
    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    /// Set padding.
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Build the status bar VNode.
    pub fn build(self) -> VNode {
        let mut children = Vec::new();

        // Mode indicator
        if let Some(mode) = &self.mode {
            children.push(self.build_mode(mode));
        }

        // Left section
        let left_section = if let Some(left) = &self.left {
            left.clone()
        } else {
            self.build_items(&self.left_items)
        };
        children.push(left_section);

        // Spacer
        children.push(VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_grow: Some(1.0),
                ..Default::default()
            },
            children: vec![],
            handlers: Default::default(),
        }));

        // Center section
        if self.center.is_some() || !self.center_items.is_empty() {
            let center_section = if let Some(center) = &self.center {
                center.clone()
            } else {
                self.build_items(&self.center_items)
            };
            children.push(center_section);

            // Another spacer
            children.push(VNode::Box(BoxNode {
                id: None,
                style: BoxStyle {
                    flex_grow: Some(1.0),
                    ..Default::default()
                },
                children: vec![],
                handlers: Default::default(),
            }));
        }

        // Right section
        let right_section = if let Some(right) = &self.right {
            right.clone()
        } else {
            self.build_items(&self.right_items)
        };
        children.push(right_section);

        // Status bar container
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                align_items: Some(AlignItems::Center),
                justify_content: Some(JustifyContent::SpaceBetween),
                height: Some(Size::Fixed(self.height)),
                width: Some(Size::Percent(100.0)),
                background: self.background,
                padding: Some(self.padding),
                gap: Some(1),
                border_style: if self.show_border {
                    Some(BorderStyle::Single)
                } else {
                    None
                },
                border_color: if self.show_border {
                    Some(self.border_color)
                } else {
                    None
                },
                ..Default::default()
            },
            children,
            handlers: Default::default(),
        })
    }

    /// Build the mode indicator.
    fn build_mode(&self, mode: &StatusMode) -> VNode {
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                background: Some(mode.background),
                padding_left: Some(1),
                padding_right: Some(1),
                ..Default::default()
            },
            children: vec![VNode::Text(TextNode {
                content: mode.text.clone(),
                style: TextStyle {
                    color: Some(mode.text_color),
                    bold: true,
                    ..Default::default()
                },
            })],
            handlers: Default::default(),
        })
    }

    /// Build a section of items.
    fn build_items(&self, items: &[StatusItem]) -> VNode {
        if items.is_empty() {
            return VNode::Empty;
        }

        let mut children = Vec::new();

        for (i, item) in items.iter().enumerate() {
            if i > 0 {
                // Add separator
                children.push(VNode::Text(TextNode {
                    content: self.separator.clone(),
                    style: TextStyle {
                        color: Some(Color::Named(NamedColor::Gray)),
                        ..Default::default()
                    },
                }));
            }

            let mut content = String::new();
            if let Some(ref icon) = item.icon {
                content.push_str(icon);
                content.push(' ');
            }
            content.push_str(&item.text);

            children.push(VNode::Text(TextNode {
                content,
                style: TextStyle {
                    color: item.color.or(Some(self.text_color)),
                    bold: item.bold,
                    ..Default::default()
                },
            }));
        }

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                align_items: Some(AlignItems::Center),
                ..Default::default()
            },
            children,
            handlers: Default::default(),
        })
    }
}

impl From<StatusBar> for VNode {
    fn from(bar: StatusBar) -> VNode {
        bar.build()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_bar_empty() {
        let bar = StatusBar::new().build();
        matches!(bar, VNode::Box(_));
    }

    #[test]
    fn test_status_bar_with_mode() {
        let bar = StatusBar::new().mode(StatusMode::normal()).build();

        if let VNode::Box(node) = bar {
            assert!(!node.children.is_empty());
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_status_mode_presets() {
        let normal = StatusMode::normal();
        assert_eq!(normal.text, "NORMAL");

        let insert = StatusMode::insert();
        assert_eq!(insert.text, "INSERT");

        let visual = StatusMode::visual();
        assert_eq!(visual.text, "VISUAL");
    }

    #[test]
    fn test_status_item() {
        let item = StatusItem::new("test")
            .icon("📁")
            .color(Color::Named(NamedColor::Green))
            .bold(true);

        assert_eq!(item.text, "test");
        assert!(item.bold);
    }

    #[test]
    fn test_status_bar_full() {
        let bar = StatusBar::new()
            .mode(StatusMode::normal())
            .left_text("main.rs")
            .center_text("Ln 42, Col 15")
            .add_right(StatusItem::new("UTF-8"))
            .add_right(StatusItem::new("LF"))
            .add_right(StatusItem::new("Rust").color(Color::Named(NamedColor::Yellow)))
            .build();

        if let VNode::Box(node) = bar {
            assert!(node.style.background.is_some());
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_status_bar_custom_separator() {
        let bar = StatusBar::new()
            .separator(" | ")
            .add_left(StatusItem::new("A"))
            .add_left(StatusItem::new("B"))
            .build();

        matches!(bar, VNode::Box(_));
    }
}
