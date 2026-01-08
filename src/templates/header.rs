//! Header Template
//!
//! Application header/top bar with title, navigation, and actions.
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │ ☰  App Title              Nav Item 1  Nav Item 2    [?] [×] │
//! └─────────────────────────────────────────────────────────────┘
//! ```

use crate::core::component::{
    BoxNode, BoxStyle, BorderStyle, Color, NamedColor, TextNode, TextStyle, VNode,
};
use crate::core::layout::{AlignItems, FlexDirection, JustifyContent, Size};

// =============================================================================
// Header Component
// =============================================================================

/// Application header/top bar.
///
/// A flexible header component with:
/// - Optional logo/icon on the left
/// - Title text
/// - Navigation items (center or left)
/// - Action buttons on the right
///
/// # Example
/// ```ignore
/// let header = Header::new()
///     .title("My Application")
///     .logo("☰")
///     .nav_items(vec!["Home", "Settings", "Help"])
///     .actions(vec![Button::new("[?]").build()])
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct Header {
    /// Logo or icon (leftmost)
    logo: Option<String>,
    /// Title text
    title: Option<String>,
    /// Subtitle/description
    subtitle: Option<String>,
    /// Navigation items
    nav_items: Vec<NavItem>,
    /// Action buttons (right side)
    actions: Vec<VNode>,
    /// Left side custom content
    left: Option<VNode>,
    /// Center custom content
    center: Option<VNode>,
    /// Right side custom content (replaces actions)
    right: Option<VNode>,
    /// Header height (in lines)
    height: u16,
    /// Background color
    background: Option<Color>,
    /// Title color
    title_color: Color,
    /// Show bottom border
    show_border: bool,
    /// Border color
    border_color: Color,
    /// Padding
    padding: u16,
}

/// Navigation item in the header.
#[derive(Debug, Clone)]
pub struct NavItem {
    /// Display label
    pub label: String,
    /// Whether this item is active/selected
    pub active: bool,
    /// Icon (optional)
    pub icon: Option<String>,
}

impl NavItem {
    /// Create a new nav item.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            active: false,
            icon: None,
        }
    }

    /// Set as active.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Set icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl Header {
    /// Create a new header.
    pub fn new() -> Self {
        Self {
            logo: None,
            title: None,
            subtitle: None,
            nav_items: Vec::new(),
            actions: Vec::new(),
            left: None,
            center: None,
            right: None,
            height: 1,
            background: None,
            title_color: Color::Named(NamedColor::White),
            show_border: true,
            border_color: Color::Named(NamedColor::BrightBlack),
            padding: 1,
        }
    }

    /// Set the logo/icon.
    pub fn logo(mut self, logo: impl Into<String>) -> Self {
        self.logo = Some(logo.into());
        self
    }

    /// Set the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the subtitle.
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Add navigation items.
    pub fn nav_items(mut self, items: Vec<NavItem>) -> Self {
        self.nav_items = items;
        self
    }

    /// Add a navigation item.
    pub fn add_nav(mut self, item: NavItem) -> Self {
        self.nav_items.push(item);
        self
    }

    /// Add navigation items from strings.
    pub fn nav(mut self, labels: Vec<&str>) -> Self {
        self.nav_items = labels.into_iter().map(NavItem::new).collect();
        self
    }

    /// Set action buttons.
    pub fn actions(mut self, actions: Vec<VNode>) -> Self {
        self.actions = actions;
        self
    }

    /// Add an action.
    pub fn add_action(mut self, action: VNode) -> Self {
        self.actions.push(action);
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

    /// Set header height.
    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    /// Set background color.
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Set title color.
    pub fn title_color(mut self, color: Color) -> Self {
        self.title_color = color;
        self
    }

    /// Set whether to show bottom border.
    pub fn border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Set border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Set padding.
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Build the header VNode.
    pub fn build(self) -> VNode {
        let mut header_children = Vec::new();

        // Left section (logo + title + subtitle)
        let left_section = self.build_left_section();
        header_children.push(left_section);

        // Center section (nav items or custom)
        if let Some(ref center) = self.center {
            header_children.push(center.clone());
        } else if !self.nav_items.is_empty() {
            header_children.push(self.build_nav());
        }

        // Spacer
        header_children.push(VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_grow: Some(1.0),
                ..Default::default()
            },
            children: vec![],
            handlers: Default::default(),
        }));

        // Right section (actions or custom)
        if let Some(ref right) = self.right {
            header_children.push(right.clone());
        } else if !self.actions.is_empty() {
            header_children.push(self.build_actions());
        }

        // Header container
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                align_items: Some(AlignItems::Center),
                justify_content: Some(JustifyContent::SpaceBetween),
                height: Some(Size::Fixed(self.height)),
                background: self.background,
                padding: Some(self.padding),
                gap: Some(2),
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
            children: header_children,
            handlers: Default::default(),
        })
    }

    /// Build the left section (logo + title).
    fn build_left_section(&self) -> VNode {
        if let Some(left) = &self.left {
            return left.clone();
        }

        let mut children = Vec::new();

        // Logo
        if let Some(ref logo) = self.logo {
            children.push(VNode::Text(TextNode {
                content: format!("{} ", logo),
                style: TextStyle {
                    color: Some(self.title_color),
                    bold: true,
                    ..Default::default()
                },
            }));
        }

        // Title
        if let Some(ref title) = self.title {
            children.push(VNode::Text(TextNode {
                content: title.clone(),
                style: TextStyle {
                    color: Some(self.title_color),
                    bold: true,
                    ..Default::default()
                },
            }));
        }

        // Subtitle
        if let Some(ref subtitle) = self.subtitle {
            children.push(VNode::Text(TextNode {
                content: format!(" - {}", subtitle),
                style: TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    ..Default::default()
                },
            }));
        }

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                align_items: Some(AlignItems::Center),
                gap: Some(0),
                ..Default::default()
            },
            children,
            handlers: Default::default(),
        })
    }

    /// Build navigation items.
    fn build_nav(&self) -> VNode {
        let nav_children: Vec<VNode> = self
            .nav_items
            .iter()
            .map(|item| {
                let mut content = String::new();

                if let Some(ref icon) = item.icon {
                    content.push_str(icon);
                    content.push(' ');
                }
                content.push_str(&item.label);

                VNode::Text(TextNode {
                    content,
                    style: TextStyle {
                        color: Some(if item.active {
                            Color::Named(NamedColor::Cyan)
                        } else {
                            Color::Named(NamedColor::White)
                        }),
                        bold: item.active,
                        underline: item.active,
                        ..Default::default()
                    },
                })
            })
            .collect();

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                align_items: Some(AlignItems::Center),
                gap: Some(3),
                ..Default::default()
            },
            children: nav_children,
            handlers: Default::default(),
        })
    }

    /// Build action buttons.
    fn build_actions(&self) -> VNode {
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                align_items: Some(AlignItems::Center),
                gap: Some(1),
                ..Default::default()
            },
            children: self.actions.clone(),
            handlers: Default::default(),
        })
    }
}

impl From<Header> for VNode {
    fn from(header: Header) -> VNode {
        header.build()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_empty() {
        let header = Header::new().build();
        matches!(header, VNode::Box(_));
    }

    #[test]
    fn test_header_with_title() {
        let header = Header::new()
            .title("My App")
            .build();

        if let VNode::Box(node) = header {
            assert!(!node.children.is_empty());
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_header_with_logo() {
        let header = Header::new()
            .logo("☰")
            .title("App")
            .build();

        matches!(header, VNode::Box(_));
    }

    #[test]
    fn test_header_with_nav() {
        let header = Header::new()
            .title("App")
            .nav(vec!["Home", "Settings", "Help"])
            .build();

        matches!(header, VNode::Box(_));
    }

    #[test]
    fn test_nav_item() {
        let item = NavItem::new("Home")
            .active(true)
            .icon("🏠");

        assert_eq!(item.label, "Home");
        assert!(item.active);
        assert_eq!(item.icon, Some("🏠".to_string()));
    }

    #[test]
    fn test_header_full() {
        let header = Header::new()
            .logo("☰")
            .title("My Application")
            .subtitle("v1.0.0")
            .add_nav(NavItem::new("Home").active(true))
            .add_nav(NavItem::new("Settings"))
            .background(Color::Named(NamedColor::Blue))
            .build();

        if let VNode::Box(node) = header {
            assert!(node.style.background.is_some());
        } else {
            panic!("Expected Box");
        }
    }
}
