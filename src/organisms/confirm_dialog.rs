//! ConfirmDialog Component
//!
//! A dialog for confirming user actions.
//! Built on top of Modal with Yes/No buttons.

use crate::core::component::{
    BoxNode, BoxStyle, BorderStyle, Color, EventHandlers, NamedColor, TextNode, TextStyle, VNode,
};
use crate::core::layout::{AlignItems, FlexDirection, JustifyContent, Size};

/// Confirm dialog variant (affects styling).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ConfirmVariant {
    /// Default confirmation
    #[default]
    Default,
    /// Destructive/dangerous action (red)
    Danger,
    /// Warning (yellow)
    Warning,
}

impl ConfirmVariant {
    /// Get the accent color for this variant.
    pub fn color(&self) -> Color {
        match self {
            ConfirmVariant::Default => Color::Named(NamedColor::Blue),
            ConfirmVariant::Danger => Color::Named(NamedColor::Red),
            ConfirmVariant::Warning => Color::Named(NamedColor::Yellow),
        }
    }
}

/// ConfirmDialog component.
///
/// Creates a modal dialog with confirm/cancel buttons.
///
/// # Example
/// ```ignore
/// let dialog = ConfirmDialog::new("Delete File?")
///     .message("Are you sure you want to delete this file? This action cannot be undone.")
///     .variant(ConfirmVariant::Danger)
///     .confirm_label("Delete")
///     .cancel_label("Keep")
///     .open(true)
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct ConfirmDialog {
    /// Whether the dialog is open
    open: bool,
    /// Dialog title
    title: String,
    /// Dialog message/description
    message: Option<String>,
    /// Confirm button label
    confirm_label: String,
    /// Cancel button label
    cancel_label: String,
    /// Dialog variant
    variant: ConfirmVariant,
    /// Dialog width
    width: u16,
    /// Show icon
    show_icon: bool,
}

impl ConfirmDialog {
    /// Create a new confirm dialog with a title.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            open: false,
            title: title.into(),
            message: None,
            confirm_label: "Yes".to_string(),
            cancel_label: "No".to_string(),
            variant: ConfirmVariant::default(),
            width: 50,
            show_icon: true,
        }
    }

    /// Set whether the dialog is open.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set the message/description.
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Set the confirm button label.
    pub fn confirm_label(mut self, label: impl Into<String>) -> Self {
        self.confirm_label = label.into();
        self
    }

    /// Set the cancel button label.
    pub fn cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = label.into();
        self
    }

    /// Set the dialog variant.
    pub fn variant(mut self, variant: ConfirmVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set as danger dialog.
    pub fn danger(self) -> Self {
        self.variant(ConfirmVariant::Danger)
    }

    /// Set as warning dialog.
    pub fn warning(self) -> Self {
        self.variant(ConfirmVariant::Warning)
    }

    /// Set the dialog width.
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Set whether to show icon.
    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    /// Build the dialog VNode.
    pub fn build(self) -> VNode {
        if !self.open {
            return VNode::Empty;
        }

        let accent_color = self.variant.color();

        // Icon based on variant
        let icon = match self.variant {
            ConfirmVariant::Default => "?",
            ConfirmVariant::Danger => "⚠",
            ConfirmVariant::Warning => "!",
        };

        // Header with icon and title
        let mut header_children = Vec::new();

        if self.show_icon {
            header_children.push(VNode::Text(TextNode {
                content: format!("{} ", icon),
                style: TextStyle {
                    color: Some(accent_color),
                    bold: true,
                    ..Default::default()
                },
            }));
        }

        header_children.push(VNode::Text(TextNode {
            content: self.title.clone(),
            style: TextStyle {
                color: Some(accent_color),
                bold: true,
                ..Default::default()
            },
        }));

        let header_row = VNode::Box(BoxNode {
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                align_items: Some(AlignItems::Center),
                ..Default::default()
            },
            children: header_children,
            ..Default::default()
        });

        // Content
        let mut content_children = vec![header_row];

        if let Some(ref message) = self.message {
            content_children.push(VNode::Spacer(crate::core::component::SpacerNode {
                x: 0,
                y: 1,
            }));
            content_children.push(VNode::Text(TextNode {
                content: message.clone(),
                style: TextStyle {
                    color: Some(Color::Named(NamedColor::White)),
                    ..Default::default()
                },
            }));
        }

        // Buttons
        let cancel_button = self.build_button(&self.cancel_label, false, accent_color);
        let confirm_button = self.build_button(&self.confirm_label, true, accent_color);

        let button_row = VNode::Box(BoxNode {
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                justify_content: Some(JustifyContent::FlexEnd),
                align_items: Some(AlignItems::Center),
                gap: Some(2),
                padding_top: Some(2),
                ..Default::default()
            },
            children: vec![cancel_button, confirm_button],
            ..Default::default()
        });

        content_children.push(button_row);

        // Dialog box
        let dialog_box = VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                width: Some(Size::Fixed(self.width)),
                border_style: Some(BorderStyle::Round),
                border_color: Some(accent_color),
                background: Some(Color::Named(NamedColor::Black)),
                padding: Some(2),
                ..Default::default()
            },
            children: content_children,
            handlers: EventHandlers {
                focusable: true,
                ..Default::default()
            },
        });

        // Center container
        let center = VNode::Box(BoxNode {
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                justify_content: Some(JustifyContent::Center),
                align_items: Some(AlignItems::Center),
                flex_grow: Some(1.0),
                ..Default::default()
            },
            children: vec![dialog_box],
            ..Default::default()
        });

        // Backdrop
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                justify_content: Some(JustifyContent::Center),
                align_items: Some(AlignItems::Center),
                flex_grow: Some(1.0),
                background: Some(Color::Rgb(0, 0, 0)),
                ..Default::default()
            },
            children: vec![center],
            handlers: Default::default(),
        })
    }

    /// Build a button.
    fn build_button(&self, label: &str, is_primary: bool, accent: Color) -> VNode {
        let (bg, fg, border) = if is_primary {
            (Some(accent), Color::Named(NamedColor::Black), Some(accent))
        } else {
            (None, Color::Named(NamedColor::White), Some(Color::Named(NamedColor::Gray)))
        };

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                padding: Some(0),
                padding_left: Some(2),
                padding_right: Some(2),
                border_style: Some(BorderStyle::Round),
                border_color: border,
                background: bg,
                ..Default::default()
            },
            children: vec![VNode::Text(TextNode {
                content: label.to_string(),
                style: TextStyle {
                    color: Some(fg),
                    bold: is_primary,
                    ..Default::default()
                },
            })],
            handlers: EventHandlers {
                focusable: true,
                on_click: true,
                ..Default::default()
            },
        })
    }
}

impl From<ConfirmDialog> for VNode {
    fn from(dialog: ConfirmDialog) -> VNode {
        dialog.build()
    }
}

// =============================================================================
// ConfirmDialog State
// =============================================================================

use crate::core::signals::{create_signal, ReadSignal, WriteSignal};

/// Result of a confirm dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmResult {
    /// Dialog is still open/pending
    Pending,
    /// User confirmed
    Confirmed,
    /// User cancelled
    Cancelled,
}

/// State manager for ConfirmDialog.
pub struct ConfirmDialogState {
    open_read: ReadSignal<bool>,
    open_write: WriteSignal<bool>,
    result_read: ReadSignal<ConfirmResult>,
    result_write: WriteSignal<ConfirmResult>,
}

impl ConfirmDialogState {
    /// Create a new confirm dialog state.
    pub fn new() -> Self {
        let (open_read, open_write) = create_signal(false);
        let (result_read, result_write) = create_signal(ConfirmResult::Pending);
        Self {
            open_read,
            open_write,
            result_read,
            result_write,
        }
    }

    /// Check if dialog is open.
    pub fn is_open(&self) -> bool {
        self.open_read.get()
    }

    /// Get the result.
    pub fn result(&self) -> ConfirmResult {
        self.result_read.get()
    }

    /// Show the dialog.
    pub fn show(&self) {
        self.result_write.set(ConfirmResult::Pending);
        self.open_write.set(true);
    }

    /// Confirm the dialog.
    pub fn confirm(&self) {
        self.result_write.set(ConfirmResult::Confirmed);
        self.open_write.set(false);
    }

    /// Cancel the dialog.
    pub fn cancel(&self) {
        self.result_write.set(ConfirmResult::Cancelled);
        self.open_write.set(false);
    }

    /// Reset the state.
    pub fn reset(&self) {
        self.result_write.set(ConfirmResult::Pending);
        self.open_write.set(false);
    }

    /// Get open signal.
    pub fn open_signal(&self) -> ReadSignal<bool> {
        self.open_read.clone()
    }

    /// Get result signal.
    pub fn result_signal(&self) -> ReadSignal<ConfirmResult> {
        self.result_read.clone()
    }
}

impl Default for ConfirmDialogState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confirm_dialog_closed() {
        let dialog = ConfirmDialog::new("Test").open(false).build();
        matches!(dialog, VNode::Empty);
    }

    #[test]
    fn test_confirm_dialog_open() {
        let dialog = ConfirmDialog::new("Test").open(true).build();
        matches!(dialog, VNode::Box(_));
    }

    #[test]
    fn test_confirm_variants() {
        matches!(ConfirmVariant::Default.color(), Color::Named(NamedColor::Blue));
        matches!(ConfirmVariant::Danger.color(), Color::Named(NamedColor::Red));
        matches!(ConfirmVariant::Warning.color(), Color::Named(NamedColor::Yellow));
    }

    #[test]
    fn test_confirm_state() {
        let state = ConfirmDialogState::new();
        assert!(!state.is_open());
        assert_eq!(state.result(), ConfirmResult::Pending);

        state.show();
        assert!(state.is_open());
        assert_eq!(state.result(), ConfirmResult::Pending);

        state.confirm();
        assert!(!state.is_open());
        assert_eq!(state.result(), ConfirmResult::Confirmed);
    }
}
