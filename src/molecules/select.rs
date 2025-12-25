//! Select Component
//!
//! Dropdown selection component.

use crate::core::component::{VNode, BoxNode, BoxStyle, TextStyle, Color, NamedColor, BorderStyle};
use crate::core::layout::FlexDirection;

/// A single select option.
#[derive(Debug, Clone)]
pub struct SelectOption {
    /// Option value
    pub value: String,
    /// Display label
    pub label: String,
    /// Whether disabled
    pub disabled: bool,
}

impl SelectOption {
    /// Create a new option.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    /// Create option with same value and label.
    pub fn simple(value: impl Into<String>) -> Self {
        let v: String = value.into();
        Self {
            label: v.clone(),
            value: v,
            disabled: false,
        }
    }

    /// Mark as disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Select component for dropdown selection.
#[derive(Debug, Clone)]
pub struct Select {
    options: Vec<SelectOption>,
    selected: Option<usize>,
    placeholder: String,
    disabled: bool,
    width: Option<u16>,
    open: bool,
    focused_index: usize,
}

impl Default for Select {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            selected: None,
            placeholder: "Select...".to_string(),
            disabled: false,
            width: None,
            open: false,
            focused_index: 0,
        }
    }
}

impl Select {
    /// Create a new select component.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add options.
    pub fn options<I>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = SelectOption>,
    {
        self.options = options.into_iter().collect();
        self
    }

    /// Add options from simple strings.
    pub fn items<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.options = items.into_iter().map(SelectOption::simple).collect();
        self
    }

    /// Set selected index.
    pub fn selected(mut self, index: usize) -> Self {
        if index < self.options.len() {
            self.selected = Some(index);
        }
        self
    }

    /// Set selected by value.
    pub fn value(mut self, value: &str) -> Self {
        if let Some(idx) = self.options.iter().position(|o| o.value == value) {
            self.selected = Some(idx);
        }
        self
    }

    /// Set placeholder text.
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set width.
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Set open state.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let display_text = if let Some(idx) = self.selected {
            self.options.get(idx)
                .map(|o| o.label.clone())
                .unwrap_or_else(|| self.placeholder.clone())
        } else {
            self.placeholder.clone()
        };

        let arrow = if self.open { "▲" } else { "▼" };
        let width = self.width.unwrap_or(20);

        let mut children = Vec::new();

        // Selected value display
        let header_text = format!(
            "{:<width$} {}",
            display_text,
            arrow,
            width = (width as usize).saturating_sub(2)
        );

        let header_color = if self.selected.is_some() {
            Color::Named(NamedColor::White)
        } else {
            Color::Named(NamedColor::Gray)
        };

        children.push(VNode::styled_text(header_text, TextStyle::color(header_color)));

        // Options list (when open)
        if self.open {
            for (idx, opt) in self.options.iter().enumerate() {
                let is_selected = self.selected == Some(idx);
                let is_focused = self.focused_index == idx;

                let prefix = if is_selected { "● " } else { "  " };
                let text = format!("{}{}", prefix, opt.label);

                let style = if opt.disabled {
                    TextStyle { color: Some(Color::Named(NamedColor::Gray)), dim: true, ..Default::default() }
                } else if is_focused {
                    TextStyle { color: Some(Color::Named(NamedColor::Cyan)), bold: true, ..Default::default() }
                } else if is_selected {
                    TextStyle::color(Color::Named(NamedColor::Green))
                } else {
                    TextStyle::default()
                };

                children.push(VNode::styled_text(text, style));
            }
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle {
                border_style: Some(BorderStyle::Single),
                padding_left: Some(1),
                padding_right: Some(1),
                ..Default::default()
            },
            ..Default::default()
        })
    }
}

/// MultiSelect component for multiple selections.
#[derive(Debug, Clone)]
pub struct MultiSelect {
    options: Vec<SelectOption>,
    selected: Vec<usize>,
    placeholder: String,
    disabled: bool,
    max_selections: Option<usize>,
    open: bool,
}

impl Default for MultiSelect {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            selected: Vec::new(),
            placeholder: "Select items...".to_string(),
            disabled: false,
            max_selections: None,
            open: false,
        }
    }
}

impl MultiSelect {
    /// Create a new multi-select.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add options.
    pub fn options<I>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = SelectOption>,
    {
        self.options = options.into_iter().collect();
        self
    }

    /// Add options from simple strings.
    pub fn items<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.options = items.into_iter().map(SelectOption::simple).collect();
        self
    }

    /// Set selected indices.
    pub fn selected<I>(mut self, indices: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        self.selected = indices.into_iter().collect();
        self
    }

    /// Set max selections.
    pub fn max(mut self, max: usize) -> Self {
        self.max_selections = Some(max);
        self
    }

    /// Set open state.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let selected_labels: Vec<_> = self.selected.iter()
            .filter_map(|&idx| self.options.get(idx).map(|o| o.label.as_str()))
            .collect();

        let display = if selected_labels.is_empty() {
            self.placeholder.clone()
        } else {
            selected_labels.join(", ")
        };

        let header_style = if self.disabled {
            TextStyle { dim: true, ..Default::default() }
        } else {
            TextStyle::default()
        };

        let mut children = vec![
            VNode::styled_text(format!("{} ▼", display), header_style)
        ];

        if self.open && !self.disabled {
            for (idx, opt) in self.options.iter().enumerate() {
                let is_selected = self.selected.contains(&idx);
                let checkbox = if is_selected { "☑" } else { "☐" };
                let text = format!("{} {}", checkbox, opt.label);

                let style = if is_selected {
                    TextStyle::color(Color::Named(NamedColor::Green))
                } else {
                    TextStyle::default()
                };

                children.push(VNode::styled_text(text, style));
            }
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle {
                border_style: Some(BorderStyle::Single),
                padding_left: Some(1),
                padding_right: Some(1),
                ..Default::default()
            },
            ..Default::default()
        })
    }
}

/// Radio group component.
#[derive(Debug, Clone)]
pub struct RadioGroup {
    options: Vec<SelectOption>,
    selected: Option<usize>,
    label: Option<String>,
    horizontal: bool,
}

impl Default for RadioGroup {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            selected: None,
            label: None,
            horizontal: false,
        }
    }
}

impl RadioGroup {
    /// Create a new radio group.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add options.
    pub fn options<I>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = SelectOption>,
    {
        self.options = options.into_iter().collect();
        self
    }

    /// Add options from simple strings.
    pub fn items<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.options = items.into_iter().map(SelectOption::simple).collect();
        self
    }

    /// Set selected index.
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }

    /// Set label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Make horizontal.
    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let mut children = Vec::new();

        if let Some(label) = &self.label {
            children.push(VNode::styled_text(label.clone(), TextStyle::bold()));
        }

        for (idx, opt) in self.options.iter().enumerate() {
            let is_selected = self.selected == Some(idx);
            let radio = if is_selected { "◉" } else { "○" };
            let text = format!("{} {}", radio, opt.label);

            let style = if opt.disabled {
                TextStyle { color: Some(Color::Named(NamedColor::Gray)), dim: true, ..Default::default() }
            } else if is_selected {
                TextStyle::color(Color::Named(NamedColor::Cyan))
            } else {
                TextStyle::default()
            };

            children.push(VNode::styled_text(text, style));
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle {
                flex_direction: Some(if self.horizontal {
                    FlexDirection::Row
                } else {
                    FlexDirection::Column
                }),
                gap: if self.horizontal { Some(2) } else { Some(0) },
                ..Default::default()
            },
            ..Default::default()
        })
    }
}
