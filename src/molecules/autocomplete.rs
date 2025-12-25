//! Autocomplete Component
//!
//! Text input with suggestions.

use crate::core::component::{VNode, BoxNode, BoxStyle, TextStyle, Color, NamedColor, BorderStyle};

/// Autocomplete suggestion.
#[derive(Debug, Clone)]
pub struct Suggestion {
    /// Display text
    pub text: String,
    /// Value (optional, defaults to text)
    pub value: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Icon
    pub icon: Option<String>,
}

impl Suggestion {
    /// Create a new suggestion.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            value: None,
            description: None,
            icon: None,
        }
    }

    /// Set value.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set description.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Autocomplete component.
#[derive(Debug, Clone)]
pub struct Autocomplete {
    input: String,
    placeholder: String,
    suggestions: Vec<Suggestion>,
    filtered: Vec<usize>,
    selected_index: usize,
    open: bool,
    max_suggestions: usize,
}

impl Default for Autocomplete {
    fn default() -> Self {
        Self {
            input: String::new(),
            placeholder: "Type to search...".to_string(),
            suggestions: Vec::new(),
            filtered: Vec::new(),
            selected_index: 0,
            open: false,
            max_suggestions: 5,
        }
    }
}

impl Autocomplete {
    /// Create a new autocomplete.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set current input value.
    pub fn value(mut self, input: impl Into<String>) -> Self {
        self.input = input.into();
        self.filter_suggestions();
        self
    }

    /// Set placeholder.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set suggestions.
    pub fn suggestions<I>(mut self, suggestions: I) -> Self
    where
        I: IntoIterator<Item = Suggestion>,
    {
        self.suggestions = suggestions.into_iter().collect();
        self.filter_suggestions();
        self
    }

    /// Set suggestions from strings.
    pub fn items<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.suggestions = items.into_iter().map(Suggestion::new).collect();
        self.filter_suggestions();
        self
    }

    /// Set open state.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set selected index.
    pub fn selected(mut self, index: usize) -> Self {
        self.selected_index = index;
        self
    }

    /// Set max visible suggestions.
    pub fn max_suggestions(mut self, max: usize) -> Self {
        self.max_suggestions = max;
        self
    }

    /// Filter suggestions based on input.
    fn filter_suggestions(&mut self) {
        let query = self.input.to_lowercase();

        if query.is_empty() {
            self.filtered = (0..self.suggestions.len()).collect();
        } else {
            self.filtered = self.suggestions
                .iter()
                .enumerate()
                .filter(|(_, s)| s.text.to_lowercase().contains(&query))
                .map(|(i, _)| i)
                .collect();
        }
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let mut children = Vec::new();

        // Input field
        let display_value = if self.input.is_empty() {
            self.placeholder.clone()
        } else {
            self.input.clone()
        };

        let input_color = if self.input.is_empty() {
            Color::Named(NamedColor::Gray)
        } else {
            Color::Named(NamedColor::White)
        };

        children.push(VNode::styled_text(
            format!("🔍 {}_", display_value),
            TextStyle::color(input_color)
        ));

        // Suggestions dropdown
        if self.open && !self.filtered.is_empty() {
            let visible: Vec<_> = self.filtered.iter()
                .take(self.max_suggestions)
                .collect();

            for (display_idx, &suggestion_idx) in visible.iter().enumerate() {
                let suggestion = &self.suggestions[*suggestion_idx];
                let is_selected = display_idx == self.selected_index;

                let mut text = String::new();

                if let Some(icon) = &suggestion.icon {
                    text.push_str(icon);
                    text.push(' ');
                }

                text.push_str(&suggestion.text);

                if let Some(desc) = &suggestion.description {
                    text.push_str(" - ");
                    text.push_str(desc);
                }

                let style = if is_selected {
                    TextStyle {
                        color: Some(Color::Named(NamedColor::Cyan)),
                        bold: true,
                        inverse: true,
                        ..Default::default()
                    }
                } else {
                    TextStyle::color(Color::Named(NamedColor::White))
                };

                children.push(VNode::styled_text(format!("  {}", text), style));
            }

            // Show count if more items
            if self.filtered.len() > self.max_suggestions {
                let remaining = self.filtered.len() - self.max_suggestions;
                children.push(VNode::styled_text(
                    format!("  ... and {} more", remaining),
                    TextStyle { color: Some(Color::Named(NamedColor::Gray)), dim: true, ..Default::default() }
                ));
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
