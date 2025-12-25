//! Tabs Component
//!
//! Tabbed interface for organizing content.

use crate::core::component::{VNode, BoxNode, BoxStyle, TextStyle, Color, NamedColor};
use crate::core::layout::FlexDirection;

/// A single tab.
#[derive(Debug, Clone)]
pub struct Tab {
    /// Tab label
    pub label: String,
    /// Tab content (as VNode)
    pub content: Option<VNode>,
    /// Whether disabled
    pub disabled: bool,
    /// Icon (optional)
    pub icon: Option<String>,
}

impl Tab {
    /// Create a new tab.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            content: None,
            disabled: false,
            icon: None,
        }
    }

    /// Set tab content.
    pub fn content(mut self, content: VNode) -> Self {
        self.content = Some(content);
        self
    }

    /// Set icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Mark as disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Tab style.
#[derive(Debug, Clone, Copy, Default)]
pub enum TabStyle {
    #[default]
    Underline,
    Boxed,
    Pills,
    Minimal,
}

/// Tabs component.
#[derive(Debug, Clone)]
pub struct Tabs {
    tabs: Vec<Tab>,
    active: usize,
    style: TabStyle,
    active_color: Color,
    inactive_color: Color,
}

impl Default for Tabs {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            active: 0,
            style: TabStyle::Underline,
            active_color: Color::Named(NamedColor::Cyan),
            inactive_color: Color::Named(NamedColor::Gray),
        }
    }
}

impl Tabs {
    /// Create new tabs.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add tabs.
    pub fn tabs<I>(mut self, tabs: I) -> Self
    where
        I: IntoIterator<Item = Tab>,
    {
        self.tabs = tabs.into_iter().collect();
        self
    }

    /// Add tabs from labels.
    pub fn items<I, S>(mut self, labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tabs = labels.into_iter().map(Tab::new).collect();
        self
    }

    /// Set active tab index.
    pub fn active(mut self, index: usize) -> Self {
        self.active = index;
        self
    }

    /// Set tab style.
    pub fn style(mut self, style: TabStyle) -> Self {
        self.style = style;
        self
    }

    /// Use underline style.
    pub fn underline(self) -> Self {
        self.style(TabStyle::Underline)
    }

    /// Use boxed style.
    pub fn boxed(self) -> Self {
        self.style(TabStyle::Boxed)
    }

    /// Use pills style.
    pub fn pills(self) -> Self {
        self.style(TabStyle::Pills)
    }

    /// Set active color.
    pub fn active_color(mut self, color: Color) -> Self {
        self.active_color = color;
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let mut tab_bar_children = Vec::new();

        for (idx, tab) in self.tabs.iter().enumerate() {
            let is_active = idx == self.active;

            let label = if let Some(icon) = &tab.icon {
                format!("{} {}", icon, tab.label)
            } else {
                tab.label.clone()
            };

            let (text, color, bold) = match self.style {
                TabStyle::Underline => {
                    let underline = if is_active { "─".repeat(label.len()) } else { String::new() };
                    let text = format!(" {} \n {}", label, underline);
                    let color = if is_active { self.active_color } else { self.inactive_color };
                    (text, color, is_active)
                }
                TabStyle::Boxed => {
                    let text = format!("│ {} │", label);
                    let color = if is_active { self.active_color } else { self.inactive_color };
                    (text, color, is_active)
                }
                TabStyle::Pills => {
                    let text = if is_active {
                        format!("[{}]", label)
                    } else {
                        format!(" {} ", label)
                    };
                    let color = if is_active { self.active_color } else { self.inactive_color };
                    (text, color, false)
                }
                TabStyle::Minimal => {
                    let text = format!(" {} ", label);
                    let color = if is_active { self.active_color } else { self.inactive_color };
                    (text, color, is_active)
                }
            };

            let text_color = if tab.disabled {
                Color::Named(NamedColor::BrightBlack)
            } else {
                color
            };

            tab_bar_children.push(VNode::styled_text(
                text,
                TextStyle { color: Some(text_color), bold, dim: tab.disabled, ..Default::default() }
            ));
        }

        let tab_bar = VNode::Box(BoxNode {
            children: tab_bar_children,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                gap: Some(1),
                ..Default::default()
            },
            ..Default::default()
        });

        // Content area
        let content = self.tabs.get(self.active)
            .and_then(|t| t.content.clone())
            .unwrap_or(VNode::Empty);

        VNode::Box(BoxNode {
            children: vec![tab_bar, content],
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                gap: Some(1),
                ..Default::default()
            },
            ..Default::default()
        })
    }
}
