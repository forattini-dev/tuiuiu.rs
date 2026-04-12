//! AlertBox Component
//!
//! Inline alert messages for displaying important information.
//! Not a modal - renders inline with content.

use crate::core::component::{
    BorderStyle, BoxNode, BoxStyle, Color, NamedColor, TextNode, TextStyle, VNode,
};
use crate::core::layout::{AlignItems, FlexDirection, Size};

/// Alert variant (type of message).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AlertVariant {
    /// Informational alert (blue)
    #[default]
    Info,
    /// Success alert (green)
    Success,
    /// Warning alert (yellow/orange)
    Warning,
    /// Error/danger alert (red)
    Error,
}

impl AlertVariant {
    /// Get the icon for this variant.
    pub fn icon(&self) -> &'static str {
        match self {
            AlertVariant::Info => "ℹ",
            AlertVariant::Success => "✓",
            AlertVariant::Warning => "⚠",
            AlertVariant::Error => "✗",
        }
    }

    /// Get the ASCII icon for this variant.
    pub fn ascii_icon(&self) -> &'static str {
        match self {
            AlertVariant::Info => "[i]",
            AlertVariant::Success => "[+]",
            AlertVariant::Warning => "[!]",
            AlertVariant::Error => "[x]",
        }
    }

    /// Get the accent color for this variant.
    pub fn color(&self) -> Color {
        match self {
            AlertVariant::Info => Color::Named(NamedColor::Cyan),
            AlertVariant::Success => Color::Named(NamedColor::Green),
            AlertVariant::Warning => Color::Named(NamedColor::Yellow),
            AlertVariant::Error => Color::Named(NamedColor::Red),
        }
    }

    /// Get the background color for this variant.
    pub fn background(&self) -> Color {
        match self {
            AlertVariant::Info => Color::Rgb(20, 35, 50),
            AlertVariant::Success => Color::Rgb(20, 45, 25),
            AlertVariant::Warning => Color::Rgb(50, 40, 15),
            AlertVariant::Error => Color::Rgb(50, 20, 20),
        }
    }

    /// Get the title prefix for this variant.
    pub fn title(&self) -> &'static str {
        match self {
            AlertVariant::Info => "Info",
            AlertVariant::Success => "Success",
            AlertVariant::Warning => "Warning",
            AlertVariant::Error => "Error",
        }
    }
}

/// AlertBox component.
///
/// Displays an inline alert message with icon, title, and description.
///
/// # Example
/// ```ignore
/// let alert = AlertBox::new("Changes saved successfully")
///     .variant(AlertVariant::Success)
///     .title("Saved")
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct AlertBox {
    /// Alert message
    message: String,
    /// Optional custom title (otherwise uses variant default)
    title: Option<String>,
    /// Alert variant
    variant: AlertVariant,
    /// Show icon
    show_icon: bool,
    /// Show title
    show_title: bool,
    /// Dismissible (show close button)
    dismissible: bool,
    /// Full width
    full_width: bool,
    /// Custom width
    width: Option<u16>,
    /// Use ASCII icons
    ascii: bool,
    /// Compact mode (less padding)
    compact: bool,
}

impl AlertBox {
    /// Create a new alert with a message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            title: None,
            variant: AlertVariant::default(),
            show_icon: true,
            show_title: true,
            dismissible: false,
            full_width: false,
            width: None,
            ascii: false,
            compact: false,
        }
    }

    /// Set custom title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the alert variant.
    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set as info alert.
    pub fn info(self) -> Self {
        self.variant(AlertVariant::Info)
    }

    /// Set as success alert.
    pub fn success(self) -> Self {
        self.variant(AlertVariant::Success)
    }

    /// Set as warning alert.
    pub fn warning(self) -> Self {
        self.variant(AlertVariant::Warning)
    }

    /// Set as error alert.
    pub fn error(self) -> Self {
        self.variant(AlertVariant::Error)
    }

    /// Set whether to show icon.
    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    /// Set whether to show title.
    pub fn show_title(mut self, show: bool) -> Self {
        self.show_title = show;
        self
    }

    /// Set whether alert is dismissible.
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Set full width mode.
    pub fn full_width(mut self, full: bool) -> Self {
        self.full_width = full;
        self
    }

    /// Set custom width.
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Use ASCII icons.
    pub fn ascii(mut self, ascii: bool) -> Self {
        self.ascii = ascii;
        self
    }

    /// Enable compact mode.
    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    /// Build the alert VNode.
    pub fn build(self) -> VNode {
        let accent_color = self.variant.color();
        let bg_color = self.variant.background();
        let padding = if self.compact { 0 } else { 1 };

        let mut content_children = Vec::new();

        // Icon
        if self.show_icon {
            let icon = if self.ascii {
                self.variant.ascii_icon()
            } else {
                self.variant.icon()
            };

            content_children.push(VNode::Text(TextNode {
                content: format!("{} ", icon),
                style: TextStyle {
                    color: Some(accent_color),
                    bold: true,
                    ..Default::default()
                },
            }));
        }

        // Title and message column
        let mut text_column = Vec::new();

        // Title row
        if self.show_title {
            let title_text = self
                .title
                .as_ref()
                .map(|t| t.as_str())
                .unwrap_or_else(|| self.variant.title());

            text_column.push(VNode::Text(TextNode {
                content: title_text.to_string(),
                style: TextStyle {
                    color: Some(accent_color),
                    bold: true,
                    ..Default::default()
                },
            }));
        }

        // Message
        text_column.push(VNode::Text(TextNode {
            content: self.message.clone(),
            style: TextStyle {
                color: Some(Color::Named(NamedColor::White)),
                ..Default::default()
            },
        }));

        let text_box = VNode::Box(BoxNode {
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                flex_grow: Some(1.0),
                gap: Some(0),
                ..Default::default()
            },
            children: text_column,
            ..Default::default()
        });

        content_children.push(text_box);

        // Close button
        if self.dismissible {
            content_children.push(VNode::Text(TextNode {
                content: " ×".to_string(),
                style: TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    ..Default::default()
                },
            }));
        }

        // Content row
        let content_row = VNode::Box(BoxNode {
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                align_items: Some(AlignItems::FlexStart),
                gap: Some(1),
                ..Default::default()
            },
            children: content_children,
            ..Default::default()
        });

        // Determine width
        let width_style = if self.full_width {
            Some(Size::Percent(100.0))
        } else {
            self.width.map(Size::Fixed)
        };

        // Alert container
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                width: width_style,
                border_style: Some(BorderStyle::Round),
                border_color: Some(accent_color),
                background: Some(bg_color),
                padding: Some(padding),
                padding_left: Some(padding + 1),
                padding_right: Some(padding + 1),
                ..Default::default()
            },
            children: vec![content_row],
            handlers: Default::default(),
        })
    }
}

impl From<AlertBox> for VNode {
    fn from(alert: AlertBox) -> VNode {
        alert.build()
    }
}

// =============================================================================
// Quick Alert Constructors
// =============================================================================

/// Create a quick info alert.
pub fn info_alert(message: impl Into<String>) -> VNode {
    AlertBox::new(message).info().build()
}

/// Create a quick success alert.
pub fn success_alert(message: impl Into<String>) -> VNode {
    AlertBox::new(message).success().build()
}

/// Create a quick warning alert.
pub fn warning_alert(message: impl Into<String>) -> VNode {
    AlertBox::new(message).warning().build()
}

/// Create a quick error alert.
pub fn error_alert(message: impl Into<String>) -> VNode {
    AlertBox::new(message).error().build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_variants() {
        assert_eq!(AlertVariant::Info.icon(), "ℹ");
        assert_eq!(AlertVariant::Success.icon(), "✓");
        assert_eq!(AlertVariant::Warning.icon(), "⚠");
        assert_eq!(AlertVariant::Error.icon(), "✗");
    }

    #[test]
    fn test_alert_build() {
        let alert = AlertBox::new("Test message").success().build();
        matches!(alert, VNode::Box(_));
    }

    #[test]
    fn test_quick_alerts() {
        matches!(info_alert("Info"), VNode::Box(_));
        matches!(success_alert("Success"), VNode::Box(_));
        matches!(warning_alert("Warning"), VNode::Box(_));
        matches!(error_alert("Error"), VNode::Box(_));
    }
}
