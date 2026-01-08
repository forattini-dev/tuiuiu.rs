//! Confirm Button Component
//!
//! A button that requires confirmation before executing the action.

use crate::core::component::{VNode, BoxNode, BoxStyle, TextStyle, Color, NamedColor, BorderStyle};
use crate::core::layout::FlexDirection;

/// Confirm button state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ConfirmState {
    #[default]
    Initial,
    Confirming,
    Confirmed,
}

/// Confirm button variant.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ConfirmVariant {
    #[default]
    Default,
    Danger,
    Warning,
}

/// State for confirm button.
#[derive(Debug, Clone)]
pub struct ConfirmButtonState {
    pub state: ConfirmState,
}

impl Default for ConfirmButtonState {
    fn default() -> Self {
        Self {
            state: ConfirmState::Initial,
        }
    }
}

impl ConfirmButtonState {
    /// Create new state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset to initial state.
    pub fn reset(&mut self) {
        self.state = ConfirmState::Initial;
    }

    /// Move to confirming state.
    pub fn confirm(&mut self) {
        self.state = ConfirmState::Confirming;
    }

    /// Execute the action.
    pub fn execute(&mut self) {
        self.state = ConfirmState::Confirmed;
    }

    /// Check if in confirming state.
    pub fn is_confirming(&self) -> bool {
        matches!(self.state, ConfirmState::Confirming)
    }
}

/// Create a new confirm button state.
pub fn create_confirm_button_state() -> ConfirmButtonState {
    ConfirmButtonState::new()
}

/// Confirm button component.
#[derive(Debug, Clone)]
pub struct ConfirmButton {
    label: String,
    confirm_label: String,
    cancel_label: String,
    state: ConfirmState,
    variant: ConfirmVariant,
    disabled: bool,
    timeout_seconds: Option<u8>,
}

impl Default for ConfirmButton {
    fn default() -> Self {
        Self {
            label: "Delete".to_string(),
            confirm_label: "Confirm?".to_string(),
            cancel_label: "Cancel".to_string(),
            state: ConfirmState::Initial,
            variant: ConfirmVariant::Default,
            disabled: false,
            timeout_seconds: Some(3),
        }
    }
}

impl ConfirmButton {
    /// Create a new confirm button.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Default::default()
        }
    }

    /// Create a danger button.
    pub fn danger(label: impl Into<String>) -> Self {
        Self::new(label).variant(ConfirmVariant::Danger)
    }

    /// Create a warning button.
    pub fn warning(label: impl Into<String>) -> Self {
        Self::new(label).variant(ConfirmVariant::Warning)
    }

    /// Set confirm label.
    pub fn confirm_label(mut self, label: impl Into<String>) -> Self {
        self.confirm_label = label.into();
        self
    }

    /// Set cancel label.
    pub fn cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = label.into();
        self
    }

    /// Set state.
    pub fn state(mut self, state: ConfirmState) -> Self {
        self.state = state;
        self
    }

    /// Set variant.
    pub fn variant(mut self, variant: ConfirmVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set timeout in seconds (None to disable).
    pub fn timeout(mut self, seconds: Option<u8>) -> Self {
        self.timeout_seconds = seconds;
        self
    }

    /// Get variant colors.
    fn get_colors(&self) -> (Color, Color) {
        match self.variant {
            ConfirmVariant::Default => (
                Color::Named(NamedColor::White),
                Color::Named(NamedColor::Cyan),
            ),
            ConfirmVariant::Danger => (
                Color::Named(NamedColor::White),
                Color::Named(NamedColor::Red),
            ),
            ConfirmVariant::Warning => (
                Color::Named(NamedColor::Black),
                Color::Named(NamedColor::Yellow),
            ),
        }
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let (fg_color, bg_color) = self.get_colors();

        match self.state {
            ConfirmState::Initial => {
                let style = if self.disabled {
                    TextStyle {
                        color: Some(Color::Named(NamedColor::Gray)),
                        dim: true,
                        ..Default::default()
                    }
                } else {
                    TextStyle {
                        color: Some(bg_color),
                        bold: true,
                        ..Default::default()
                    }
                };

                VNode::Box(BoxNode {
                    children: vec![
                        VNode::styled_text(format!(" {} ", self.label), style),
                    ],
                    style: BoxStyle {
                        border_style: Some(BorderStyle::Single),
                        padding_left: Some(1),
                        padding_right: Some(1),
                        ..Default::default()
                    },
                    ..Default::default()
                })
            }
            ConfirmState::Confirming => {
                let confirm_style = TextStyle {
                    color: Some(fg_color.clone()),
                    bold: true,
                    inverse: true,
                    ..Default::default()
                };

                let cancel_style = TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    ..Default::default()
                };

                let timeout_text = if let Some(secs) = self.timeout_seconds {
                    format!(" ({}s)", secs)
                } else {
                    String::new()
                };

                VNode::Box(BoxNode {
                    children: vec![
                        VNode::styled_text(
                            format!(" {} ", self.confirm_label),
                            confirm_style,
                        ),
                        VNode::styled_text(
                            format!(" │ {} {}", self.cancel_label, timeout_text),
                            cancel_style,
                        ),
                    ],
                    style: BoxStyle {
                        flex_direction: Some(FlexDirection::Row),
                        border_style: Some(BorderStyle::Single),
                        padding_left: Some(1),
                        padding_right: Some(1),
                        ..Default::default()
                    },
                    ..Default::default()
                })
            }
            ConfirmState::Confirmed => {
                let style = TextStyle {
                    color: Some(Color::Named(NamedColor::Green)),
                    bold: true,
                    ..Default::default()
                };

                VNode::Box(BoxNode {
                    children: vec![
                        VNode::styled_text(" ✓ Done ", style),
                    ],
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
    }
}
