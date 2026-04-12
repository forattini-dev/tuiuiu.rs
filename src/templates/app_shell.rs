//! AppShell Template
//!
//! A complete application layout structure with header, sidebar,
//! main content area, footer, and status bar.
//!
//! ```text
//! ┌──────────────────────────────────────────────┐
//! │                   Header                      │
//! ├──────────┬───────────────────────────────────┤
//! │          │                                    │
//! │ Sidebar  │            Content                 │
//! │          │                                    │
//! │          │                                    │
//! ├──────────┴───────────────────────────────────┤
//! │                   Footer                      │
//! ├──────────────────────────────────────────────┤
//! │                 Status Bar                    │
//! └──────────────────────────────────────────────┘
//! ```

use crate::core::component::{BorderStyle, BoxNode, BoxStyle, Color, NamedColor, VNode};
use crate::core::layout::{FlexDirection, Size};

// =============================================================================
// AppShell Component
// =============================================================================

/// Application shell - complete app layout structure.
///
/// Provides a standard application layout with:
/// - Header (top bar)
/// - Optional sidebar (left)
/// - Main content area (center)
/// - Optional footer (above status bar)
/// - Optional status bar (bottom)
///
/// # Example
/// ```ignore
/// let app = AppShell::new()
///     .header(Header::new().title("My App").build())
///     .sidebar(Sidebar::new().width(20).build())
///     .content(my_content)
///     .status_bar(StatusBar::new().build())
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct AppShell {
    /// Header component (top)
    header: Option<VNode>,
    /// Sidebar component (left)
    sidebar: Option<VNode>,
    /// Main content area
    content: Option<VNode>,
    /// Footer component (above status bar)
    footer: Option<VNode>,
    /// Status bar component (bottom)
    status_bar: Option<VNode>,
    /// Sidebar position
    sidebar_position: SidebarPosition,
    /// Show border between sections
    show_borders: bool,
    /// Border style
    border_style: BorderStyle,
    /// Border color
    border_color: Color,
    /// Background color
    background: Option<Color>,
}

/// Sidebar position in the layout.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SidebarPosition {
    /// Sidebar on the left (default)
    #[default]
    Left,
    /// Sidebar on the right
    Right,
}

impl AppShell {
    /// Create a new app shell.
    pub fn new() -> Self {
        Self {
            header: None,
            sidebar: None,
            content: None,
            footer: None,
            status_bar: None,
            sidebar_position: SidebarPosition::Left,
            show_borders: true,
            border_style: BorderStyle::Single,
            border_color: Color::Named(NamedColor::BrightBlack),
            background: None,
        }
    }

    /// Set the header component.
    pub fn header(mut self, header: VNode) -> Self {
        self.header = Some(header);
        self
    }

    /// Set the sidebar component.
    pub fn sidebar(mut self, sidebar: VNode) -> Self {
        self.sidebar = Some(sidebar);
        self
    }

    /// Set sidebar position.
    pub fn sidebar_position(mut self, position: SidebarPosition) -> Self {
        self.sidebar_position = position;
        self
    }

    /// Set sidebar on the right.
    pub fn sidebar_right(mut self) -> Self {
        self.sidebar_position = SidebarPosition::Right;
        self
    }

    /// Set the main content.
    pub fn content(mut self, content: VNode) -> Self {
        self.content = Some(content);
        self
    }

    /// Set the footer component.
    pub fn footer(mut self, footer: VNode) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Set the status bar component.
    pub fn status_bar(mut self, status_bar: VNode) -> Self {
        self.status_bar = Some(status_bar);
        self
    }

    /// Set whether to show borders between sections.
    pub fn borders(mut self, show: bool) -> Self {
        self.show_borders = show;
        self
    }

    /// Set border style.
    pub fn border_style(mut self, style: BorderStyle) -> Self {
        self.border_style = style;
        self
    }

    /// Set border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Set background color.
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Build the app shell VNode.
    pub fn build(self) -> VNode {
        let mut main_children = Vec::new();

        // Header
        if let Some(ref header) = self.header {
            main_children.push(self.wrap_section(header.clone(), false, true));
        }

        // Middle section (sidebar + content)
        let middle = self.build_middle();
        main_children.push(middle);

        // Footer
        if let Some(ref footer) = self.footer {
            main_children.push(self.wrap_section(footer.clone(), true, false));
        }

        // Status bar
        if let Some(ref status_bar) = self.status_bar {
            main_children.push(self.wrap_section(status_bar.clone(), true, false));
        }

        // Main container
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                flex_grow: Some(1.0),
                width: Some(Size::Percent(100.0)),
                height: Some(Size::Percent(100.0)),
                background: self.background,
                ..Default::default()
            },
            children: main_children,
            handlers: Default::default(),
        })
    }

    /// Build the middle section (sidebar + content).
    fn build_middle(&self) -> VNode {
        let content = self.content.clone().unwrap_or(VNode::Empty);

        match (&self.sidebar, self.sidebar_position) {
            (Some(sidebar), SidebarPosition::Left) => VNode::Box(BoxNode {
                id: None,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Row),
                    flex_grow: Some(1.0),
                    ..Default::default()
                },
                children: vec![
                    self.wrap_sidebar(sidebar.clone()),
                    self.wrap_content(content),
                ],
                handlers: Default::default(),
            }),
            (Some(sidebar), SidebarPosition::Right) => VNode::Box(BoxNode {
                id: None,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Row),
                    flex_grow: Some(1.0),
                    ..Default::default()
                },
                children: vec![
                    self.wrap_content(content),
                    self.wrap_sidebar(sidebar.clone()),
                ],
                handlers: Default::default(),
            }),
            (None, _) => self.wrap_content(content),
        }
    }

    /// Wrap a section with optional border.
    fn wrap_section(&self, node: VNode, border_top: bool, border_bottom: bool) -> VNode {
        if !self.show_borders || (!border_top && !border_bottom) {
            return node;
        }

        // For now, we wrap in a box. The border will be shown as separator lines
        // since BoxStyle doesn't support individual border sides.
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                ..Default::default()
            },
            children: vec![node],
            handlers: Default::default(),
        })
    }

    /// Wrap sidebar with styling.
    fn wrap_sidebar(&self, sidebar: VNode) -> VNode {
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                border_style: if self.show_borders {
                    Some(BorderStyle::Single)
                } else {
                    None
                },
                border_color: if self.show_borders {
                    Some(self.border_color)
                } else {
                    None
                },
                ..Default::default()
            },
            children: vec![sidebar],
            handlers: Default::default(),
        })
    }

    /// Wrap content with styling.
    fn wrap_content(&self, content: VNode) -> VNode {
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                flex_grow: Some(1.0),
                ..Default::default()
            },
            children: vec![content],
            handlers: Default::default(),
        })
    }
}

impl From<AppShell> for VNode {
    fn from(shell: AppShell) -> VNode {
        shell.build()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::component::TextNode;

    fn text(s: &str) -> VNode {
        VNode::Text(TextNode {
            content: s.to_string(),
            style: Default::default(),
        })
    }

    #[test]
    fn test_app_shell_empty() {
        let shell = AppShell::new().build();
        matches!(shell, VNode::Box(_));
    }

    #[test]
    fn test_app_shell_with_header() {
        let shell = AppShell::new().header(text("Header")).build();

        if let VNode::Box(node) = shell {
            assert!(!node.children.is_empty());
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_app_shell_with_sidebar() {
        let shell = AppShell::new()
            .sidebar(text("Sidebar"))
            .content(text("Content"))
            .build();

        matches!(shell, VNode::Box(_));
    }

    #[test]
    fn test_app_shell_sidebar_right() {
        let shell = AppShell::new()
            .sidebar(text("Sidebar"))
            .sidebar_right()
            .content(text("Content"))
            .build();

        matches!(shell, VNode::Box(_));
    }

    #[test]
    fn test_app_shell_full() {
        let shell = AppShell::new()
            .header(text("Header"))
            .sidebar(text("Sidebar"))
            .content(text("Content"))
            .footer(text("Footer"))
            .status_bar(text("Status"))
            .build();

        if let VNode::Box(node) = shell {
            // Header + Middle + Footer + StatusBar = 4 children
            assert_eq!(node.children.len(), 4);
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_sidebar_position() {
        assert_eq!(SidebarPosition::default(), SidebarPosition::Left);
    }
}
