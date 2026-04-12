//! FormField Molecule
//!
//! A wrapper for form inputs with label, help text, and error display.
//!
//! ```text
//! Username*
//! ┌──────────────────────┐
//! │ john.doe             │
//! └──────────────────────┘
//! Enter your username
//!
//! Email*
//! ┌──────────────────────┐
//! │ invalid-email        │
//! └──────────────────────┘
//! ⚠ Please enter a valid email address
//! ```

use crate::core::component::{BoxNode, BoxStyle, Color, NamedColor, TextNode, TextStyle, VNode};
use crate::core::layout::FlexDirection;

// =============================================================================
// FormField Component
// =============================================================================

/// Form field validation state
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum FieldState {
    /// Normal state
    #[default]
    Default,
    /// Valid input
    Valid,
    /// Invalid input with error message
    Invalid,
    /// Warning state
    Warning,
}

/// FormField component - wraps an input with label and validation.
///
/// # Example
/// ```ignore
/// let field = FormField::new("Username")
///     .required(true)
///     .help("Enter your username")
///     .input(TextInput::new().build())
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct FormField {
    /// Field label
    label: String,
    /// The input element
    input: Option<VNode>,
    /// Help text (shown below input)
    help: Option<String>,
    /// Error message
    error: Option<String>,
    /// Warning message
    warning: Option<String>,
    /// Success message
    success: Option<String>,
    /// Whether field is required
    required: bool,
    /// Required indicator character
    required_indicator: String,
    /// Field state
    state: FieldState,
    /// Label color
    label_color: Color,
    /// Error color
    error_color: Color,
    /// Warning color
    warning_color: Color,
    /// Success color
    success_color: Color,
    /// Help text color
    help_color: Color,
    /// Disabled state
    disabled: bool,
    /// Horizontal layout (label inline with input)
    horizontal: bool,
    /// Label width for horizontal layout
    label_width: Option<u16>,
}

impl Default for FormField {
    fn default() -> Self {
        Self {
            label: String::new(),
            input: None,
            help: None,
            error: None,
            warning: None,
            success: None,
            required: false,
            required_indicator: "*".to_string(),
            state: FieldState::Default,
            label_color: Color::Named(NamedColor::White),
            error_color: Color::Named(NamedColor::Red),
            warning_color: Color::Named(NamedColor::Yellow),
            success_color: Color::Named(NamedColor::Green),
            help_color: Color::Named(NamedColor::Gray),
            disabled: false,
            horizontal: false,
            label_width: None,
        }
    }
}

impl FormField {
    /// Create a new form field.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Default::default()
        }
    }

    /// Set the input element.
    pub fn input(mut self, input: VNode) -> Self {
        self.input = Some(input);
        self
    }

    /// Set help text.
    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Set error message.
    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self.state = FieldState::Invalid;
        self
    }

    /// Set warning message.
    pub fn warning(mut self, warning: impl Into<String>) -> Self {
        self.warning = Some(warning.into());
        self.state = FieldState::Warning;
        self
    }

    /// Set success message.
    pub fn success(mut self, success: impl Into<String>) -> Self {
        self.success = Some(success.into());
        self.state = FieldState::Valid;
        self
    }

    /// Set required.
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Set required indicator.
    pub fn required_indicator(mut self, indicator: impl Into<String>) -> Self {
        self.required_indicator = indicator.into();
        self
    }

    /// Set field state.
    pub fn state(mut self, state: FieldState) -> Self {
        self.state = state;
        self
    }

    /// Set label color.
    pub fn label_color(mut self, color: Color) -> Self {
        self.label_color = color;
        self
    }

    /// Set error color.
    pub fn error_color(mut self, color: Color) -> Self {
        self.error_color = color;
        self
    }

    /// Set disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set horizontal layout.
    pub fn horizontal(mut self, horizontal: bool) -> Self {
        self.horizontal = horizontal;
        self
    }

    /// Set label width (for horizontal layout).
    pub fn label_width(mut self, width: u16) -> Self {
        self.label_width = Some(width);
        self
    }

    /// Build the label node.
    fn build_label(&self) -> VNode {
        let mut label_text = self.label.clone();
        if self.required {
            label_text.push_str(&self.required_indicator);
        }

        let style = if let Some(width) = self.label_width {
            BoxStyle {
                min_width: Some(width),
                ..Default::default()
            }
        } else {
            BoxStyle::default()
        };

        VNode::Box(BoxNode {
            id: None,
            style,
            children: vec![VNode::Text(TextNode {
                content: label_text,
                style: TextStyle {
                    color: Some(if self.disabled {
                        Color::Named(NamedColor::Gray)
                    } else {
                        self.label_color.clone()
                    }),
                    bold: true,
                    dim: self.disabled,
                    ..Default::default()
                },
            })],
            handlers: Default::default(),
        })
    }

    /// Build the message node (error/warning/success/help).
    fn build_message(&self) -> Option<VNode> {
        let (icon, message, color) = match self.state {
            FieldState::Invalid => {
                if let Some(ref err) = self.error {
                    ("⚠ ", err.as_str(), self.error_color.clone())
                } else {
                    return None;
                }
            }
            FieldState::Warning => {
                if let Some(ref warn) = self.warning {
                    ("⚡ ", warn.as_str(), self.warning_color.clone())
                } else {
                    return None;
                }
            }
            FieldState::Valid => {
                if let Some(ref success) = self.success {
                    ("✓ ", success.as_str(), self.success_color.clone())
                } else if let Some(ref help) = self.help {
                    ("", help.as_str(), self.help_color.clone())
                } else {
                    return None;
                }
            }
            FieldState::Default => {
                if let Some(ref help) = self.help {
                    ("", help.as_str(), self.help_color.clone())
                } else {
                    return None;
                }
            }
        };

        Some(VNode::Text(TextNode {
            content: format!("{}{}", icon, message),
            style: TextStyle {
                color: Some(color),
                ..Default::default()
            },
        }))
    }

    /// Build the form field VNode.
    pub fn build(self) -> VNode {
        let label = self.build_label();
        let message = self.build_message();
        let input = self.input.clone().unwrap_or(VNode::Empty);

        if self.horizontal {
            // Horizontal layout: [Label] [Input] [Message]
            let mut row_children = vec![label, input];
            if let Some(msg) = message {
                row_children.push(msg);
            }

            VNode::Box(BoxNode {
                id: None,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Row),
                    gap: Some(2),
                    ..Default::default()
                },
                children: row_children,
                handlers: Default::default(),
            })
        } else {
            // Vertical layout
            let mut children = vec![label, input];
            if let Some(msg) = message {
                children.push(msg);
            }

            VNode::Box(BoxNode {
                id: None,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Column),
                    gap: Some(0),
                    ..Default::default()
                },
                children,
                handlers: Default::default(),
            })
        }
    }
}

impl From<FormField> for VNode {
    fn from(field: FormField) -> VNode {
        field.build()
    }
}

// =============================================================================
// FormGroup Component
// =============================================================================

/// FormGroup - groups multiple form fields.
///
/// # Example
/// ```ignore
/// let group = FormGroup::new("Account Details")
///     .add_field(username_field)
///     .add_field(email_field)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct FormGroup {
    /// Group title
    title: Option<String>,
    /// Fields in the group
    fields: Vec<VNode>,
    /// Gap between fields
    gap: u16,
    /// Show border
    bordered: bool,
    /// Border color
    border_color: Color,
    /// Description text
    description: Option<String>,
}

impl Default for FormGroup {
    fn default() -> Self {
        Self {
            title: None,
            fields: Vec::new(),
            gap: 1,
            bordered: false,
            border_color: Color::Named(NamedColor::BrightBlack),
            description: None,
        }
    }
}

impl FormGroup {
    /// Create a new form group.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            ..Default::default()
        }
    }

    /// Create a form group without title.
    pub fn untitled() -> Self {
        Self::default()
    }

    /// Add a field.
    pub fn add_field(mut self, field: VNode) -> Self {
        self.fields.push(field);
        self
    }

    /// Set fields.
    pub fn fields(mut self, fields: Vec<VNode>) -> Self {
        self.fields = fields;
        self
    }

    /// Set gap between fields.
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
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

    /// Set description.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Build the form group VNode.
    pub fn build(self) -> VNode {
        let mut children = Vec::new();

        // Title
        if let Some(ref title) = self.title {
            children.push(VNode::Text(TextNode {
                content: title.clone(),
                style: TextStyle {
                    color: Some(Color::Named(NamedColor::White)),
                    bold: true,
                    ..Default::default()
                },
            }));
        }

        // Description
        if let Some(ref desc) = self.description {
            children.push(VNode::Text(TextNode {
                content: desc.clone(),
                style: TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    ..Default::default()
                },
            }));
        }

        // Fields container
        let fields_box = VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                gap: Some(self.gap),
                ..Default::default()
            },
            children: self.fields,
            handlers: Default::default(),
        });
        children.push(fields_box);

        // Group container
        let mut style = BoxStyle {
            flex_direction: Some(FlexDirection::Column),
            gap: Some(1),
            padding: if self.bordered { Some(1) } else { None },
            ..Default::default()
        };

        if self.bordered {
            style.border_style = Some(crate::core::component::BorderStyle::Single);
            style.border_color = Some(self.border_color);
        }

        VNode::Box(BoxNode {
            id: None,
            style,
            children,
            handlers: Default::default(),
        })
    }
}

impl From<FormGroup> for VNode {
    fn from(group: FormGroup) -> VNode {
        group.build()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn text_input() -> VNode {
        VNode::Text(TextNode {
            content: "[input]".to_string(),
            style: Default::default(),
        })
    }

    #[test]
    fn test_form_field_basic() {
        let field = FormField::new("Username").input(text_input()).build();
        matches!(field, VNode::Box(_));
    }

    #[test]
    fn test_form_field_required() {
        let field = FormField::new("Email")
            .required(true)
            .input(text_input())
            .build();

        if let VNode::Box(node) = field {
            // Should contain label with asterisk
            if let VNode::Box(label_box) = &node.children[0] {
                if let VNode::Text(label) = &label_box.children[0] {
                    assert!(label.content.contains("*"));
                }
            }
        }
    }

    #[test]
    fn test_form_field_error() {
        let field = FormField::new("Username")
            .input(text_input())
            .error("Username is required")
            .build();

        if let VNode::Box(node) = field {
            assert!(node.children.len() >= 3); // label + input + error
        }
    }

    #[test]
    fn test_form_field_help() {
        let field = FormField::new("Bio")
            .input(text_input())
            .help("Tell us about yourself")
            .build();

        if let VNode::Box(node) = field {
            assert!(node.children.len() >= 3);
        }
    }

    #[test]
    fn test_form_field_horizontal() {
        let field = FormField::new("Name")
            .input(text_input())
            .horizontal(true)
            .label_width(15)
            .build();
        matches!(field, VNode::Box(_));
    }

    #[test]
    fn test_form_group_basic() {
        let group = FormGroup::new("Personal Info")
            .add_field(FormField::new("Name").input(text_input()).build())
            .add_field(FormField::new("Email").input(text_input()).build())
            .build();
        matches!(group, VNode::Box(_));
    }

    #[test]
    fn test_form_group_bordered() {
        let group = FormGroup::new("Settings")
            .bordered(true)
            .description("Configure your preferences")
            .build();
        matches!(group, VNode::Box(_));
    }

    #[test]
    fn test_field_state() {
        assert_eq!(FieldState::default(), FieldState::Default);
    }
}
