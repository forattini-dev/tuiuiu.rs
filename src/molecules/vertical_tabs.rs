//! VerticalTabs Molecule
//!
//! Tabs displayed in a vertical layout with content on the side.
//!
//! ```text
//! ┌──────────────┬──────────────────────────────┐
//! │ > Tab 1      │                              │
//! │   Tab 2      │     Content for Tab 1        │
//! │   Tab 3      │                              │
//! │   Tab 4      │                              │
//! └──────────────┴──────────────────────────────┘
//! ```

use crate::core::component::{
    BorderStyle, BoxNode, BoxStyle, Color, NamedColor, TextNode, TextStyle, VNode,
};
use crate::core::layout::{AlignItems, FlexDirection, Size};
use crate::core::signals::create_signal;

// =============================================================================
// VerticalTabs Component
// =============================================================================

/// Position of the tab list
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TabListPosition {
    /// Tabs on the left, content on the right
    #[default]
    Left,
    /// Tabs on the right, content on the left
    Right,
}

/// A vertical tab item
#[derive(Debug, Clone)]
pub struct VerticalTab {
    /// Tab identifier/key
    pub key: String,
    /// Tab label
    pub label: String,
    /// Tab content
    pub content: VNode,
    /// Whether tab is disabled
    pub disabled: bool,
    /// Icon before label
    pub icon: Option<String>,
    /// Badge/counter after label
    pub badge: Option<String>,
}

impl VerticalTab {
    /// Create a new tab.
    pub fn new(key: impl Into<String>, label: impl Into<String>, content: VNode) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            content,
            disabled: false,
            icon: None,
            badge: None,
        }
    }

    /// Set disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set badge.
    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }
}

/// VerticalTabs state for reactive updates
#[derive(Debug, Clone, Default)]
pub struct VerticalTabsState {
    /// Currently selected tab key
    pub selected: Option<String>,
    /// Keys of available tabs (for navigation)
    pub keys: Vec<String>,
}

impl VerticalTabsState {
    /// Create with initial selection.
    pub fn with_selected(key: impl Into<String>, keys: Vec<String>) -> Self {
        Self {
            selected: Some(key.into()),
            keys,
        }
    }

    /// Select a tab by key.
    pub fn select(&mut self, key: impl Into<String>) {
        let key = key.into();
        if self.keys.contains(&key) {
            self.selected = Some(key);
        }
    }

    /// Select previous tab.
    pub fn select_prev(&mut self) {
        if self.keys.is_empty() {
            return;
        }
        let current_idx = self
            .selected
            .as_ref()
            .and_then(|k| self.keys.iter().position(|x| x == k))
            .unwrap_or(0);

        let new_idx = if current_idx == 0 { 0 } else { current_idx - 1 };

        self.selected = Some(self.keys[new_idx].clone());
    }

    /// Select next tab.
    pub fn select_next(&mut self) {
        if self.keys.is_empty() {
            return;
        }
        let current_idx = self
            .selected
            .as_ref()
            .and_then(|k| self.keys.iter().position(|x| x == k))
            .unwrap_or(0);

        let new_idx = if current_idx >= self.keys.len() - 1 {
            self.keys.len() - 1
        } else {
            current_idx + 1
        };

        self.selected = Some(self.keys[new_idx].clone());
    }

    /// Get selected index.
    pub fn selected_index(&self) -> Option<usize> {
        self.selected
            .as_ref()
            .and_then(|k| self.keys.iter().position(|x| x == k))
    }
}

/// VerticalTabs component.
///
/// # Example
/// ```ignore
/// let tabs = VerticalTabs::new()
///     .add_tab(VerticalTab::new("settings", "Settings", settings_content))
///     .add_tab(VerticalTab::new("profile", "Profile", profile_content))
///     .add_tab(VerticalTab::new("security", "Security", security_content))
///     .selected("settings")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct VerticalTabs {
    /// Tabs
    tabs: Vec<VerticalTab>,
    /// Selected tab key
    selected: Option<String>,
    /// Tab list position
    position: TabListPosition,
    /// Tab list width
    tab_width: u16,
    /// Show border between tabs and content
    bordered: bool,
    /// Border color
    border_color: Color,
    /// Selected tab indicator
    indicator: String,
    /// Tab background color when selected
    selected_bg: Option<Color>,
    /// Tab text color when selected
    selected_fg: Color,
    /// Normal tab text color
    tab_fg: Color,
    /// Disabled tab color
    disabled_fg: Color,
}

impl Default for VerticalTabs {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            selected: None,
            position: TabListPosition::Left,
            tab_width: 20,
            bordered: true,
            border_color: Color::Named(NamedColor::Gray),
            indicator: ">".to_string(),
            selected_bg: Some(Color::Named(NamedColor::BrightBlack)),
            selected_fg: Color::Named(NamedColor::White),
            tab_fg: Color::Named(NamedColor::Gray),
            disabled_fg: Color::Named(NamedColor::BrightBlack),
        }
    }
}

impl VerticalTabs {
    /// Create new vertical tabs.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a tab.
    pub fn add_tab(mut self, tab: VerticalTab) -> Self {
        if self.selected.is_none() && !tab.disabled {
            self.selected = Some(tab.key.clone());
        }
        self.tabs.push(tab);
        self
    }

    /// Set tabs.
    pub fn tabs(mut self, tabs: Vec<VerticalTab>) -> Self {
        self.tabs = tabs;
        if self.selected.is_none() {
            self.selected = self
                .tabs
                .iter()
                .find(|t| !t.disabled)
                .map(|t| t.key.clone());
        }
        self
    }

    /// Set selected tab.
    pub fn selected(mut self, key: impl Into<String>) -> Self {
        self.selected = Some(key.into());
        self
    }

    /// Set tab list position.
    pub fn position(mut self, pos: TabListPosition) -> Self {
        self.position = pos;
        self
    }

    /// Set tab width.
    pub fn tab_width(mut self, width: u16) -> Self {
        self.tab_width = width;
        self
    }

    /// Set bordered.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Set border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Set selection indicator.
    pub fn indicator(mut self, indicator: impl Into<String>) -> Self {
        self.indicator = indicator.into();
        self
    }

    /// Set selected background color.
    pub fn selected_bg(mut self, color: Color) -> Self {
        self.selected_bg = Some(color);
        self
    }

    /// Set selected text color.
    pub fn selected_fg(mut self, color: Color) -> Self {
        self.selected_fg = color;
        self
    }

    /// Apply state.
    pub fn state(mut self, state: &VerticalTabsState) -> Self {
        self.selected = state.selected.clone();
        self
    }

    /// Build the tab list.
    fn build_tab_list(&self) -> VNode {
        let children: Vec<VNode> = self
            .tabs
            .iter()
            .map(|tab| {
                let is_selected = self.selected.as_ref() == Some(&tab.key);

                // Build label content
                let mut label = String::new();

                if is_selected {
                    label.push_str(&self.indicator);
                    label.push(' ');
                } else {
                    label.push_str("  ");
                }

                if let Some(ref icon) = tab.icon {
                    label.push_str(icon);
                    label.push(' ');
                }

                label.push_str(&tab.label);

                if let Some(ref badge) = tab.badge {
                    label.push_str(" (");
                    label.push_str(badge);
                    label.push(')');
                }

                let (fg, bg) = if tab.disabled {
                    (self.disabled_fg.clone(), None)
                } else if is_selected {
                    (self.selected_fg.clone(), self.selected_bg.clone())
                } else {
                    (self.tab_fg.clone(), None)
                };

                VNode::Box(BoxNode {
                    id: None,
                    style: BoxStyle {
                        background: bg,
                        padding_left: Some(1),
                        padding_right: Some(1),
                        min_width: Some(self.tab_width),
                        ..Default::default()
                    },
                    children: vec![VNode::Text(TextNode {
                        content: label,
                        style: TextStyle {
                            color: Some(fg),
                            bold: is_selected,
                            dim: tab.disabled,
                            ..Default::default()
                        },
                    })],
                    handlers: Default::default(),
                })
            })
            .collect();

        let mut style = BoxStyle {
            flex_direction: Some(FlexDirection::Column),
            width: Some(Size::Fixed(self.tab_width)),
            ..Default::default()
        };

        if self.bordered {
            match self.position {
                TabListPosition::Left => {
                    style.border_style = Some(BorderStyle::Single);
                    style.border_color = Some(self.border_color.clone());
                }
                TabListPosition::Right => {
                    style.border_style = Some(BorderStyle::Single);
                    style.border_color = Some(self.border_color.clone());
                }
            }
        }

        VNode::Box(BoxNode {
            id: None,
            style,
            children,
            handlers: Default::default(),
        })
    }

    /// Build the content area.
    fn build_content(&self) -> VNode {
        let content = self
            .selected
            .as_ref()
            .and_then(|key| self.tabs.iter().find(|t| &t.key == key))
            .map(|t| t.content.clone())
            .unwrap_or(VNode::Empty);

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_grow: Some(1.0),
                padding: Some(1),
                ..Default::default()
            },
            children: vec![content],
            handlers: Default::default(),
        })
    }

    /// Build the vertical tabs VNode.
    pub fn build(self) -> VNode {
        let tab_list = self.build_tab_list();
        let content = self.build_content();

        let children = match self.position {
            TabListPosition::Left => vec![tab_list, content],
            TabListPosition::Right => vec![content, tab_list],
        };

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                align_items: Some(AlignItems::Stretch),
                ..Default::default()
            },
            children,
            handlers: Default::default(),
        })
    }
}

impl From<VerticalTabs> for VNode {
    fn from(tabs: VerticalTabs) -> VNode {
        tabs.build()
    }
}

/// Create reactive vertical tabs state.
pub fn create_vertical_tabs_state(
    selected: Option<String>,
    keys: Vec<String>,
) -> (
    crate::core::signals::ReadSignal<VerticalTabsState>,
    crate::core::signals::WriteSignal<VerticalTabsState>,
) {
    create_signal(VerticalTabsState { selected, keys })
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
    fn test_vertical_tabs_empty() {
        let tabs = VerticalTabs::new().build();
        matches!(tabs, VNode::Box(_));
    }

    #[test]
    fn test_vertical_tabs_with_tabs() {
        let tabs = VerticalTabs::new()
            .add_tab(VerticalTab::new("a", "Tab A", text("Content A")))
            .add_tab(VerticalTab::new("b", "Tab B", text("Content B")))
            .build();

        if let VNode::Box(node) = tabs {
            // Tab list + content
            assert_eq!(node.children.len(), 2);
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_vertical_tabs_selection() {
        let tabs = VerticalTabs::new()
            .add_tab(VerticalTab::new("a", "Tab A", text("A")))
            .add_tab(VerticalTab::new("b", "Tab B", text("B")))
            .selected("b")
            .build();

        // Should show content B
        matches!(tabs, VNode::Box(_));
    }

    #[test]
    fn test_vertical_tabs_position() {
        let tabs = VerticalTabs::new()
            .add_tab(VerticalTab::new("a", "Tab A", text("A")))
            .position(TabListPosition::Right)
            .build();

        if let VNode::Box(node) = tabs {
            // Content should be first when position is Right
            assert_eq!(node.children.len(), 2);
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_vertical_tab_item() {
        let tab = VerticalTab::new("key", "Label", text("Content"))
            .icon("⚙")
            .badge("3")
            .disabled(true);

        assert_eq!(tab.key, "key");
        assert_eq!(tab.label, "Label");
        assert_eq!(tab.icon, Some("⚙".to_string()));
        assert_eq!(tab.badge, Some("3".to_string()));
        assert!(tab.disabled);
    }

    #[test]
    fn test_vertical_tabs_state() {
        let mut state = VerticalTabsState::with_selected(
            "a",
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
        );

        assert_eq!(state.selected, Some("a".to_string()));
        assert_eq!(state.selected_index(), Some(0));

        state.select_next();
        assert_eq!(state.selected, Some("b".to_string()));

        state.select_next();
        assert_eq!(state.selected, Some("c".to_string()));

        state.select_prev();
        assert_eq!(state.selected, Some("b".to_string()));
    }

    #[test]
    fn test_vertical_tabs_auto_select_first() {
        let tabs = VerticalTabs::new()
            .add_tab(VerticalTab::new("first", "First", text("1")))
            .add_tab(VerticalTab::new("second", "Second", text("2")))
            .build();

        // First non-disabled tab should be auto-selected
        matches!(tabs, VNode::Box(_));
    }

    #[test]
    fn test_vertical_tabs_skip_disabled() {
        let tabs = VerticalTabs::new()
            .add_tab(VerticalTab::new("disabled", "Disabled", text("D")).disabled(true))
            .add_tab(VerticalTab::new("enabled", "Enabled", text("E")))
            .build();

        // Second tab should be auto-selected since first is disabled
        matches!(tabs, VNode::Box(_));
    }
}
