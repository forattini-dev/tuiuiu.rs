//! Page Template
//!
//! A content wrapper providing structure for page-like content with
//! title, breadcrumbs, actions, and body.
//!
//! ```text
//! ┌────────────────────────────────────────────────────┐
//! │ Home > Settings > Profile                          │
//! │                                                    │
//! │ Profile Settings                    [Save] [Cancel]│
//! │ ───────────────────────────────────────────────────│
//! │                                                    │
//! │   [Page content goes here]                         │
//! │                                                    │
//! └────────────────────────────────────────────────────┘
//! ```

use crate::core::component::{
    BoxNode, BoxStyle, Color, NamedColor, TextNode, TextStyle, VNode,
};
use crate::core::layout::{AlignItems, FlexDirection, JustifyContent};

// =============================================================================
// Page Component
// =============================================================================

/// Page content wrapper.
///
/// Provides a structured layout for page-like content including:
/// - Optional breadcrumbs navigation
/// - Title with optional subtitle
/// - Action buttons
/// - Main content area
///
/// # Example
/// ```ignore
/// let page = Page::new()
///     .breadcrumbs(vec!["Home", "Settings", "Profile"])
///     .title("Profile Settings")
///     .subtitle("Manage your profile information")
///     .actions(vec![
///         Button::new("Save").primary().build(),
///         Button::new("Cancel").build(),
///     ])
///     .content(my_form)
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct Page {
    /// Breadcrumb items
    breadcrumbs: Vec<Breadcrumb>,
    /// Page title
    title: Option<String>,
    /// Page subtitle/description
    subtitle: Option<String>,
    /// Title icon
    icon: Option<String>,
    /// Action buttons
    actions: Vec<VNode>,
    /// Main content
    content: Option<VNode>,
    /// Show title separator line
    show_separator: bool,
    /// Title color
    title_color: Color,
    /// Breadcrumb separator
    breadcrumb_sep: String,
    /// Padding
    padding: u16,
    /// Gap between sections
    gap: u16,
}

/// Breadcrumb item.
#[derive(Debug, Clone)]
pub struct Breadcrumb {
    /// Display label
    pub label: String,
    /// Whether this is the current/active item
    pub active: bool,
    /// Icon (optional)
    pub icon: Option<String>,
}

impl Breadcrumb {
    /// Create a new breadcrumb.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            active: false,
            icon: None,
        }
    }

    /// Set as active (current page).
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

impl Page {
    /// Create a new page.
    pub fn new() -> Self {
        Self {
            breadcrumbs: Vec::new(),
            title: None,
            subtitle: None,
            icon: None,
            actions: Vec::new(),
            content: None,
            show_separator: true,
            title_color: Color::Named(NamedColor::White),
            breadcrumb_sep: " > ".to_string(),
            padding: 1,
            gap: 1,
        }
    }

    /// Set breadcrumbs from Breadcrumb items.
    pub fn breadcrumb_items(mut self, items: Vec<Breadcrumb>) -> Self {
        self.breadcrumbs = items;
        self
    }

    /// Set breadcrumbs from strings (last one is active).
    pub fn breadcrumbs(mut self, items: Vec<&str>) -> Self {
        let len = items.len();
        self.breadcrumbs = items
            .into_iter()
            .enumerate()
            .map(|(i, label)| Breadcrumb::new(label).active(i == len - 1))
            .collect();
        self
    }

    /// Add a breadcrumb item.
    pub fn add_breadcrumb(mut self, item: Breadcrumb) -> Self {
        self.breadcrumbs.push(item);
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

    /// Set the title icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set action buttons.
    pub fn actions(mut self, actions: Vec<VNode>) -> Self {
        self.actions = actions;
        self
    }

    /// Add an action button.
    pub fn add_action(mut self, action: VNode) -> Self {
        self.actions.push(action);
        self
    }

    /// Set the main content.
    pub fn content(mut self, content: VNode) -> Self {
        self.content = Some(content);
        self
    }

    /// Set whether to show separator line.
    pub fn separator(mut self, show: bool) -> Self {
        self.show_separator = show;
        self
    }

    /// Set title color.
    pub fn title_color(mut self, color: Color) -> Self {
        self.title_color = color;
        self
    }

    /// Set breadcrumb separator.
    pub fn breadcrumb_separator(mut self, sep: impl Into<String>) -> Self {
        self.breadcrumb_sep = sep.into();
        self
    }

    /// Set padding.
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Set gap between sections.
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Build the page VNode.
    pub fn build(self) -> VNode {
        let mut children = Vec::new();

        // Breadcrumbs
        if !self.breadcrumbs.is_empty() {
            children.push(self.build_breadcrumbs());
        }

        // Title row (title + actions)
        if self.title.is_some() || !self.actions.is_empty() {
            children.push(self.build_title_row());
        }

        // Separator
        if self.show_separator && self.title.is_some() {
            children.push(self.build_separator());
        }

        // Content
        if let Some(content) = self.content {
            children.push(content);
        }

        // Page container
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                flex_grow: Some(1.0),
                padding: Some(self.padding),
                gap: Some(self.gap),
                ..Default::default()
            },
            children,
            handlers: Default::default(),
        })
    }

    /// Build breadcrumbs row.
    fn build_breadcrumbs(&self) -> VNode {
        let mut children = Vec::new();

        for (i, crumb) in self.breadcrumbs.iter().enumerate() {
            if i > 0 {
                // Separator
                children.push(VNode::Text(TextNode {
                    content: self.breadcrumb_sep.clone(),
                    style: TextStyle {
                        color: Some(Color::Named(NamedColor::Gray)),
                        ..Default::default()
                    },
                }));
            }

            let mut content = String::new();
            if let Some(ref icon) = crumb.icon {
                content.push_str(icon);
                content.push(' ');
            }
            content.push_str(&crumb.label);

            children.push(VNode::Text(TextNode {
                content,
                style: TextStyle {
                    color: Some(if crumb.active {
                        Color::Named(NamedColor::White)
                    } else {
                        Color::Named(NamedColor::Cyan)
                    }),
                    bold: crumb.active,
                    underline: !crumb.active,
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

    /// Build title row with actions.
    fn build_title_row(&self) -> VNode {
        let mut row_children = Vec::new();

        // Title section (icon + title + subtitle)
        if self.title.is_some() {
            let mut title_children = Vec::new();

            // Icon
            if let Some(ref icon) = self.icon {
                title_children.push(VNode::Text(TextNode {
                    content: format!("{} ", icon),
                    style: TextStyle {
                        color: Some(self.title_color),
                        ..Default::default()
                    },
                }));
            }

            // Title
            if let Some(ref title) = self.title {
                title_children.push(VNode::Text(TextNode {
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
                title_children.push(VNode::Text(TextNode {
                    content: format!("  {}", subtitle),
                    style: TextStyle {
                        color: Some(Color::Named(NamedColor::Gray)),
                        italic: true,
                        ..Default::default()
                    },
                }));
            }

            row_children.push(VNode::Box(BoxNode {
                id: None,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Row),
                    align_items: Some(AlignItems::Center),
                    ..Default::default()
                },
                children: title_children,
                handlers: Default::default(),
            }));
        }

        // Spacer
        row_children.push(VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_grow: Some(1.0),
                ..Default::default()
            },
            children: vec![],
            handlers: Default::default(),
        }));

        // Actions
        if !self.actions.is_empty() {
            row_children.push(VNode::Box(BoxNode {
                id: None,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Row),
                    align_items: Some(AlignItems::Center),
                    gap: Some(1),
                    ..Default::default()
                },
                children: self.actions.clone(),
                handlers: Default::default(),
            }));
        }

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                align_items: Some(AlignItems::Center),
                justify_content: Some(JustifyContent::SpaceBetween),
                ..Default::default()
            },
            children: row_children,
            handlers: Default::default(),
        })
    }

    /// Build separator line.
    fn build_separator(&self) -> VNode {
        VNode::Text(TextNode {
            content: "─".repeat(60),
            style: TextStyle {
                color: Some(Color::Named(NamedColor::BrightBlack)),
                ..Default::default()
            },
        })
    }
}

impl From<Page> for VNode {
    fn from(page: Page) -> VNode {
        page.build()
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
    fn test_page_empty() {
        let page = Page::new().build();
        matches!(page, VNode::Box(_));
    }

    #[test]
    fn test_page_with_title() {
        let page = Page::new()
            .title("My Page")
            .build();

        if let VNode::Box(node) = page {
            assert!(!node.children.is_empty());
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_page_with_breadcrumbs() {
        let page = Page::new()
            .breadcrumbs(vec!["Home", "Settings", "Profile"])
            .title("Profile")
            .build();

        if let VNode::Box(node) = page {
            // Breadcrumbs + Title + Separator = 3 children
            assert!(node.children.len() >= 2);
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_breadcrumb() {
        let crumb = Breadcrumb::new("Home")
            .active(true)
            .icon("🏠");

        assert_eq!(crumb.label, "Home");
        assert!(crumb.active);
        assert_eq!(crumb.icon, Some("🏠".to_string()));
    }

    #[test]
    fn test_page_with_content() {
        let page = Page::new()
            .title("Test")
            .content(text("Hello World"))
            .build();

        if let VNode::Box(node) = page {
            // Title + Separator + Content = 3 children
            assert_eq!(node.children.len(), 3);
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_page_full() {
        let page = Page::new()
            .breadcrumbs(vec!["Home", "Settings"])
            .icon("⚙")
            .title("Settings")
            .subtitle("Configure your preferences")
            .add_action(text("[Save]"))
            .add_action(text("[Cancel]"))
            .content(text("Content here"))
            .build();

        if let VNode::Box(node) = page {
            assert!(!node.children.is_empty());
        } else {
            panic!("Expected Box");
        }
    }
}
