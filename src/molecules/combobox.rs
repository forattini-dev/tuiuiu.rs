//! Combobox Molecule
//!
//! A combination of text input and dropdown selection with filtering.
//!
//! ```text
//! ┌────────────────────────┐
//! │ Search...         ▼    │
//! ├────────────────────────┤
//! │ > Option 1             │
//! │   Option 2             │
//! │   Option 3             │
//! └────────────────────────┘
//! ```

use crate::core::component::{BoxNode, BoxStyle, BorderStyle, Color, NamedColor, TextNode, TextStyle, VNode};
use crate::core::layout::{AlignItems, FlexDirection, JustifyContent};
use crate::core::signals::create_signal;

// =============================================================================
// Combobox Component
// =============================================================================

/// A combobox option
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComboboxOption {
    /// Option value
    pub value: String,
    /// Display label
    pub label: String,
    /// Whether option is disabled
    pub disabled: bool,
    /// Optional group name
    pub group: Option<String>,
    /// Optional description
    pub description: Option<String>,
}

impl ComboboxOption {
    /// Create a new option with value as label.
    pub fn new(value: impl Into<String>) -> Self {
        let v = value.into();
        Self {
            label: v.clone(),
            value: v,
            disabled: false,
            group: None,
            description: None,
        }
    }

    /// Create option with separate value and label.
    pub fn with_label(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
            group: None,
            description: None,
        }
    }

    /// Set disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set group.
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Set description.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

impl From<&str> for ComboboxOption {
    fn from(s: &str) -> Self {
        ComboboxOption::new(s)
    }
}

impl From<String> for ComboboxOption {
    fn from(s: String) -> Self {
        ComboboxOption::new(s)
    }
}

impl From<(String, String)> for ComboboxOption {
    fn from((value, label): (String, String)) -> Self {
        ComboboxOption::with_label(value, label)
    }
}

/// Combobox state for reactive updates
#[derive(Debug, Clone, Default)]
pub struct ComboboxState {
    /// Current input/search text
    pub input: String,
    /// Selected value
    pub selected: Option<String>,
    /// Whether dropdown is open
    pub open: bool,
    /// Highlighted option index
    pub highlighted: usize,
    /// Available options
    pub options: Vec<ComboboxOption>,
}

impl ComboboxState {
    /// Create with options.
    pub fn with_options(options: Vec<ComboboxOption>) -> Self {
        Self {
            input: String::new(),
            selected: None,
            open: false,
            highlighted: 0,
            options,
        }
    }

    /// Get filtered options based on input.
    pub fn filtered_options(&self) -> Vec<&ComboboxOption> {
        if self.input.is_empty() {
            self.options.iter().collect()
        } else {
            let query = self.input.to_lowercase();
            self.options
                .iter()
                .filter(|o| o.label.to_lowercase().contains(&query))
                .collect()
        }
    }

    /// Select the currently highlighted option.
    pub fn select_highlighted(&mut self) {
        // Clone values first to avoid borrow conflicts
        let selection: Option<(String, String)> = {
            let filtered = self.filtered_options();
            filtered.get(self.highlighted).and_then(|opt| {
                if !opt.disabled {
                    Some((opt.value.clone(), opt.label.clone()))
                } else {
                    None
                }
            })
        };

        if let Some((value, label)) = selection {
            self.selected = Some(value);
            self.input = label;
            self.open = false;
        }
    }

    /// Select by value.
    pub fn select_value(&mut self, value: &str) {
        if let Some(opt) = self.options.iter().find(|o| o.value == value) {
            self.selected = Some(opt.value.clone());
            self.input = opt.label.clone();
        }
    }

    /// Clear selection.
    pub fn clear(&mut self) {
        self.selected = None;
        self.input.clear();
        self.highlighted = 0;
    }

    /// Move highlight up.
    pub fn highlight_prev(&mut self) {
        let filtered = self.filtered_options();
        if filtered.is_empty() {
            self.highlighted = 0;
            return;
        }
        if self.highlighted > 0 {
            self.highlighted -= 1;
        }
    }

    /// Move highlight down.
    pub fn highlight_next(&mut self) {
        let filtered = self.filtered_options();
        if filtered.is_empty() {
            self.highlighted = 0;
            return;
        }
        if self.highlighted < filtered.len() - 1 {
            self.highlighted += 1;
        }
    }

    /// Toggle dropdown open/closed.
    pub fn toggle(&mut self) {
        self.open = !self.open;
        if self.open {
            self.highlighted = 0;
        }
    }

    /// Get selected option.
    pub fn selected_option(&self) -> Option<&ComboboxOption> {
        self.selected
            .as_ref()
            .and_then(|v| self.options.iter().find(|o| &o.value == v))
    }
}

/// Combobox component - autocomplete with dropdown.
///
/// # Example
/// ```ignore
/// let combobox = Combobox::new()
///     .placeholder("Select a language...")
///     .add_option("rust")
///     .add_option("typescript")
///     .add_option("python")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Combobox {
    /// Available options
    options: Vec<ComboboxOption>,
    /// Placeholder text
    placeholder: String,
    /// Current input value
    input: String,
    /// Selected value
    selected: Option<String>,
    /// Whether dropdown is open
    open: bool,
    /// Highlighted index
    highlighted: usize,
    /// Maximum visible options
    max_visible: usize,
    /// Enable filtering
    filterable: bool,
    /// Allow clearing
    clearable: bool,
    /// Disabled state
    disabled: bool,
    /// Border color
    border_color: Color,
    /// Highlight color
    highlight_color: Color,
    /// Width
    width: Option<u16>,
}

impl Default for Combobox {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            placeholder: "Select...".to_string(),
            input: String::new(),
            selected: None,
            open: false,
            highlighted: 0,
            max_visible: 5,
            filterable: true,
            clearable: true,
            disabled: false,
            border_color: Color::Named(NamedColor::Gray),
            highlight_color: Color::Named(NamedColor::Cyan),
            width: None,
        }
    }
}

impl Combobox {
    /// Create a new combobox.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an option.
    pub fn add_option(mut self, option: impl Into<ComboboxOption>) -> Self {
        self.options.push(option.into());
        self
    }

    /// Set options.
    pub fn options(mut self, options: Vec<ComboboxOption>) -> Self {
        self.options = options;
        self
    }

    /// Set placeholder text.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set input value.
    pub fn input(mut self, input: impl Into<String>) -> Self {
        self.input = input.into();
        self
    }

    /// Set selected value.
    pub fn selected(mut self, value: impl Into<String>) -> Self {
        self.selected = Some(value.into());
        self
    }

    /// Set dropdown open state.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set highlighted index.
    pub fn highlighted(mut self, index: usize) -> Self {
        self.highlighted = index;
        self
    }

    /// Set maximum visible options.
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = max;
        self
    }

    /// Set filterable.
    pub fn filterable(mut self, filterable: bool) -> Self {
        self.filterable = filterable;
        self
    }

    /// Set clearable.
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    /// Set disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Set highlight color.
    pub fn highlight_color(mut self, color: Color) -> Self {
        self.highlight_color = color;
        self
    }

    /// Set width.
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Apply state.
    pub fn state(mut self, state: &ComboboxState) -> Self {
        self.options = state.options.clone();
        self.input = state.input.clone();
        self.selected = state.selected.clone();
        self.open = state.open;
        self.highlighted = state.highlighted;
        self
    }

    /// Get filtered options.
    fn filtered_options(&self) -> Vec<&ComboboxOption> {
        if !self.filterable || self.input.is_empty() {
            self.options.iter().collect()
        } else {
            let query = self.input.to_lowercase();
            self.options
                .iter()
                .filter(|o| o.label.to_lowercase().contains(&query))
                .collect()
        }
    }

    /// Build input row.
    fn build_input(&self) -> VNode {
        let display = if self.input.is_empty() {
            &self.placeholder
        } else {
            &self.input
        };

        let text_color = if self.input.is_empty() {
            Color::Named(NamedColor::Gray)
        } else {
            Color::Named(NamedColor::White)
        };

        let arrow = if self.open { "▲" } else { "▼" };

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                justify_content: Some(JustifyContent::SpaceBetween),
                align_items: Some(AlignItems::Center),
                padding: Some(1),
                border_style: Some(BorderStyle::Single),
                border_color: Some(self.border_color.clone()),
                min_width: self.width,
                ..Default::default()
            },
            children: vec![
                VNode::Text(TextNode {
                    content: display.to_string(),
                    style: TextStyle {
                        color: Some(text_color),
                        dim: self.disabled,
                        ..Default::default()
                    },
                }),
                VNode::Text(TextNode {
                    content: arrow.to_string(),
                    style: TextStyle {
                        color: Some(Color::Named(NamedColor::Gray)),
                        ..Default::default()
                    },
                }),
            ],
            handlers: Default::default(),
        })
    }

    /// Build dropdown options.
    fn build_dropdown(&self) -> VNode {
        let filtered = self.filtered_options();
        let visible: Vec<_> = filtered.into_iter().take(self.max_visible).collect();

        let children: Vec<VNode> = visible
            .iter()
            .enumerate()
            .map(|(i, opt)| {
                let is_highlighted = i == self.highlighted;
                let is_selected = self.selected.as_ref() == Some(&opt.value);

                let prefix = if is_highlighted { "> " } else { "  " };
                let suffix = if is_selected { " ✓" } else { "" };

                let bg = if is_highlighted {
                    Some(self.highlight_color.clone())
                } else {
                    None
                };

                let fg = if opt.disabled {
                    Color::Named(NamedColor::Gray)
                } else if is_highlighted {
                    Color::Named(NamedColor::Black)
                } else {
                    Color::Named(NamedColor::White)
                };

                VNode::Text(TextNode {
                    content: format!("{}{}{}", prefix, opt.label, suffix),
                    style: TextStyle {
                        color: Some(fg),
                        background: bg,
                        dim: opt.disabled,
                        ..Default::default()
                    },
                })
            })
            .collect();

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                border_style: Some(BorderStyle::Single),
                border_color: Some(self.border_color.clone()),
                min_width: self.width,
                ..Default::default()
            },
            children,
            handlers: Default::default(),
        })
    }

    /// Build the combobox VNode.
    pub fn build(self) -> VNode {
        let mut children = vec![self.build_input()];

        if self.open && !self.disabled {
            children.push(self.build_dropdown());
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

impl From<Combobox> for VNode {
    fn from(cb: Combobox) -> VNode {
        cb.build()
    }
}

/// Create reactive combobox state.
pub fn create_combobox_state(options: Vec<ComboboxOption>) -> (
    crate::core::signals::ReadSignal<ComboboxState>,
    crate::core::signals::WriteSignal<ComboboxState>,
) {
    create_signal(ComboboxState::with_options(options))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combobox_empty() {
        let cb = Combobox::new().build();
        matches!(cb, VNode::Box(_));
    }

    #[test]
    fn test_combobox_with_options() {
        let cb = Combobox::new()
            .add_option("rust")
            .add_option("go")
            .add_option("python")
            .build();

        if let VNode::Box(node) = cb {
            // Just input when closed
            assert_eq!(node.children.len(), 1);
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_combobox_open() {
        let cb = Combobox::new()
            .add_option("rust")
            .add_option("go")
            .open(true)
            .build();

        if let VNode::Box(node) = cb {
            // Input + dropdown when open
            assert_eq!(node.children.len(), 2);
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_combobox_option() {
        let opt = ComboboxOption::new("value")
            .disabled(true)
            .group("group1")
            .description("A description");

        assert_eq!(opt.value, "value");
        assert_eq!(opt.label, "value");
        assert!(opt.disabled);
        assert_eq!(opt.group, Some("group1".to_string()));
    }

    #[test]
    fn test_combobox_option_with_label() {
        let opt = ComboboxOption::with_label("us", "United States");
        assert_eq!(opt.value, "us");
        assert_eq!(opt.label, "United States");
    }

    #[test]
    fn test_combobox_state() {
        let mut state = ComboboxState::with_options(vec![
            ComboboxOption::new("a"),
            ComboboxOption::new("b"),
            ComboboxOption::new("c"),
        ]);

        assert_eq!(state.filtered_options().len(), 3);

        state.input = "a".to_string();
        assert_eq!(state.filtered_options().len(), 1);
    }

    #[test]
    fn test_combobox_state_navigation() {
        let mut state = ComboboxState::with_options(vec![
            ComboboxOption::new("a"),
            ComboboxOption::new("b"),
            ComboboxOption::new("c"),
        ]);

        assert_eq!(state.highlighted, 0);

        state.highlight_next();
        assert_eq!(state.highlighted, 1);

        state.highlight_next();
        assert_eq!(state.highlighted, 2);

        state.highlight_next(); // Should stay at max
        assert_eq!(state.highlighted, 2);

        state.highlight_prev();
        assert_eq!(state.highlighted, 1);
    }

    #[test]
    fn test_combobox_state_selection() {
        let mut state = ComboboxState::with_options(vec![
            ComboboxOption::new("rust"),
            ComboboxOption::new("go"),
        ]);

        state.highlighted = 1;
        state.select_highlighted();

        assert_eq!(state.selected, Some("go".to_string()));
        assert_eq!(state.input, "go");
    }

    #[test]
    fn test_combobox_disabled() {
        let cb = Combobox::new()
            .add_option("rust")
            .open(true)
            .disabled(true)
            .build();

        if let VNode::Box(node) = cb {
            // No dropdown when disabled
            assert_eq!(node.children.len(), 1);
        } else {
            panic!("Expected Box");
        }
    }
}
