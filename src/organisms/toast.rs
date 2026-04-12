//! Toast Component
//!
//! Notification toasts that appear briefly to inform users.
//! Supports multiple variants, positions, and auto-dismiss.

use crate::core::component::{
    BorderStyle, BoxNode, BoxStyle, Color, NamedColor, TextNode, TextStyle, VNode,
};
use crate::core::layout::{AlignItems, FlexDirection, JustifyContent, Size};

/// Toast variant (type of notification).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToastVariant {
    /// Informational toast (blue)
    #[default]
    Info,
    /// Success toast (green)
    Success,
    /// Warning toast (yellow/orange)
    Warning,
    /// Error toast (red)
    Error,
}

impl ToastVariant {
    /// Get the icon for this variant.
    pub fn icon(&self) -> &'static str {
        match self {
            ToastVariant::Info => "ℹ",
            ToastVariant::Success => "✓",
            ToastVariant::Warning => "⚠",
            ToastVariant::Error => "✗",
        }
    }

    /// Get the ASCII icon for this variant (fallback).
    pub fn ascii_icon(&self) -> &'static str {
        match self {
            ToastVariant::Info => "[i]",
            ToastVariant::Success => "[+]",
            ToastVariant::Warning => "[!]",
            ToastVariant::Error => "[x]",
        }
    }

    /// Get the color for this variant.
    pub fn color(&self) -> Color {
        match self {
            ToastVariant::Info => Color::Named(NamedColor::Cyan),
            ToastVariant::Success => Color::Named(NamedColor::Green),
            ToastVariant::Warning => Color::Named(NamedColor::Yellow),
            ToastVariant::Error => Color::Named(NamedColor::Red),
        }
    }

    /// Get the background color for this variant.
    pub fn background(&self) -> Color {
        match self {
            ToastVariant::Info => Color::Rgb(20, 40, 60),
            ToastVariant::Success => Color::Rgb(20, 60, 30),
            ToastVariant::Warning => Color::Rgb(60, 50, 20),
            ToastVariant::Error => Color::Rgb(60, 20, 20),
        }
    }
}

/// Toast position on screen.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToastPosition {
    /// Top left corner
    TopLeft,
    /// Top center
    TopCenter,
    /// Top right corner
    #[default]
    TopRight,
    /// Bottom left corner
    BottomLeft,
    /// Bottom center
    BottomCenter,
    /// Bottom right corner
    BottomRight,
}

/// A single toast notification.
///
/// # Example
/// ```ignore
/// let toast = Toast::new("File saved successfully!")
///     .variant(ToastVariant::Success)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Toast {
    /// Toast message
    message: String,
    /// Optional title
    title: Option<String>,
    /// Toast variant
    variant: ToastVariant,
    /// Show icon
    show_icon: bool,
    /// Show close button
    closable: bool,
    /// Toast width
    width: u16,
    /// Use ASCII icons (for compatibility)
    ascii: bool,
}

impl Toast {
    /// Create a new toast with a message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            title: None,
            variant: ToastVariant::default(),
            show_icon: true,
            closable: true,
            width: 50,
            ascii: false,
        }
    }

    /// Set the toast title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the toast variant.
    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set as info toast.
    pub fn info(self) -> Self {
        self.variant(ToastVariant::Info)
    }

    /// Set as success toast.
    pub fn success(self) -> Self {
        self.variant(ToastVariant::Success)
    }

    /// Set as warning toast.
    pub fn warning(self) -> Self {
        self.variant(ToastVariant::Warning)
    }

    /// Set as error toast.
    pub fn error(self) -> Self {
        self.variant(ToastVariant::Error)
    }

    /// Set whether to show icon.
    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    /// Set whether toast is closable.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set toast width.
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Use ASCII icons instead of Unicode.
    pub fn ascii(mut self, ascii: bool) -> Self {
        self.ascii = ascii;
        self
    }

    /// Build the toast VNode.
    pub fn build(self) -> VNode {
        let accent_color = self.variant.color();
        let bg_color = self.variant.background();

        let mut row_children = Vec::new();

        // Icon
        if self.show_icon {
            let icon = if self.ascii {
                self.variant.ascii_icon()
            } else {
                self.variant.icon()
            };

            row_children.push(VNode::Text(TextNode {
                content: format!("{} ", icon),
                style: TextStyle {
                    color: Some(accent_color),
                    bold: true,
                    ..Default::default()
                },
            }));
        }

        // Content column (title + message)
        let mut content_children = Vec::new();

        if let Some(ref title) = self.title {
            content_children.push(VNode::Text(TextNode {
                content: title.clone(),
                style: TextStyle {
                    color: Some(accent_color),
                    bold: true,
                    ..Default::default()
                },
            }));
        }

        content_children.push(VNode::Text(TextNode {
            content: self.message.clone(),
            style: TextStyle {
                color: Some(Color::Named(NamedColor::White)),
                ..Default::default()
            },
        }));

        let content_column = VNode::Box(BoxNode {
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                flex_grow: Some(1.0),
                gap: Some(0),
                ..Default::default()
            },
            children: content_children,
            ..Default::default()
        });

        row_children.push(content_column);

        // Close button
        if self.closable {
            row_children.push(VNode::Text(TextNode {
                content: " ×".to_string(),
                style: TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    ..Default::default()
                },
            }));
        }

        // Main row
        let main_row = VNode::Box(BoxNode {
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                align_items: Some(AlignItems::FlexStart),
                gap: Some(1),
                ..Default::default()
            },
            children: row_children,
            ..Default::default()
        });

        // Toast container
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                width: Some(Size::Fixed(self.width)),
                border_style: Some(BorderStyle::Round),
                border_color: Some(accent_color),
                background: Some(bg_color),
                padding: Some(1),
                ..Default::default()
            },
            children: vec![main_row],
            handlers: Default::default(),
        })
    }
}

impl From<Toast> for VNode {
    fn from(toast: Toast) -> VNode {
        toast.build()
    }
}

// =============================================================================
// Toast Container
// =============================================================================

/// Container for managing multiple toasts.
///
/// Positions and stacks toasts at a specific screen position.
#[derive(Debug, Clone, Default)]
pub struct ToastContainer {
    /// Toast position on screen
    position: ToastPosition,
    /// List of toasts to display
    toasts: Vec<VNode>,
    /// Gap between toasts
    gap: u16,
    /// Maximum number of visible toasts
    max_visible: usize,
}

impl ToastContainer {
    /// Create a new toast container.
    pub fn new() -> Self {
        Self {
            position: ToastPosition::default(),
            toasts: Vec::new(),
            gap: 1,
            max_visible: 5,
        }
    }

    /// Set the position.
    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    /// Set toasts.
    pub fn toasts(mut self, toasts: Vec<VNode>) -> Self {
        self.toasts = toasts;
        self
    }

    /// Add a toast.
    pub fn add(mut self, toast: VNode) -> Self {
        self.toasts.push(toast);
        self
    }

    /// Set gap between toasts.
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Set maximum visible toasts.
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = max;
        self
    }

    /// Build the container VNode.
    pub fn build(self) -> VNode {
        if self.toasts.is_empty() {
            return VNode::Empty;
        }

        // Limit visible toasts
        let visible: Vec<VNode> = self.toasts.into_iter().take(self.max_visible).collect();

        // Determine alignment based on position
        let (justify, align) = match self.position {
            ToastPosition::TopLeft => (JustifyContent::FlexStart, AlignItems::FlexStart),
            ToastPosition::TopCenter => (JustifyContent::FlexStart, AlignItems::Center),
            ToastPosition::TopRight => (JustifyContent::FlexStart, AlignItems::FlexEnd),
            ToastPosition::BottomLeft => (JustifyContent::FlexEnd, AlignItems::FlexStart),
            ToastPosition::BottomCenter => (JustifyContent::FlexEnd, AlignItems::Center),
            ToastPosition::BottomRight => (JustifyContent::FlexEnd, AlignItems::FlexEnd),
        };

        // Stack container (vertical list of toasts)
        let stack = VNode::Box(BoxNode {
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                gap: Some(self.gap),
                padding: Some(1),
                ..Default::default()
            },
            children: visible,
            ..Default::default()
        });

        // Positioning container (full screen)
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                justify_content: Some(justify),
                align_items: Some(align),
                flex_grow: Some(1.0),
                ..Default::default()
            },
            children: vec![stack],
            handlers: Default::default(),
        })
    }
}

impl From<ToastContainer> for VNode {
    fn from(container: ToastContainer) -> VNode {
        container.build()
    }
}

// =============================================================================
// Toast State Manager
// =============================================================================

use crate::core::signals::{create_signal, ReadSignal, WriteSignal};

/// Unique toast ID.
pub type ToastId = u64;

/// Toast entry with metadata.
#[derive(Debug, Clone)]
pub struct ToastEntry {
    /// Unique ID
    pub id: ToastId,
    /// Toast message
    pub message: String,
    /// Optional title
    pub title: Option<String>,
    /// Toast variant
    pub variant: ToastVariant,
    /// Duration in milliseconds (0 = persistent)
    pub duration: u64,
    /// Timestamp when created
    pub created_at: u64,
}

/// State manager for toasts.
///
/// Provides API for adding/removing toasts with automatic dismiss.
pub struct ToastState {
    toasts_read: ReadSignal<Vec<ToastEntry>>,
    toasts_write: WriteSignal<Vec<ToastEntry>>,
    next_id_read: ReadSignal<ToastId>,
    next_id_write: WriteSignal<ToastId>,
}

impl ToastState {
    /// Create a new toast state.
    pub fn new() -> Self {
        let (toasts_read, toasts_write) = create_signal(Vec::new());
        let (next_id_read, next_id_write) = create_signal(1u64);
        Self {
            toasts_read,
            toasts_write,
            next_id_read,
            next_id_write,
        }
    }

    /// Show a toast.
    pub fn show(&self, message: impl Into<String>, variant: ToastVariant) -> ToastId {
        let id = self.next_id_read.get();
        self.next_id_write.set(id + 1);

        let entry = ToastEntry {
            id,
            message: message.into(),
            title: None,
            variant,
            duration: 3000, // 3 seconds default
            created_at: 0,  // Would use actual timestamp
        };

        let mut toasts = self.toasts_read.get();
        toasts.push(entry);
        self.toasts_write.set(toasts);

        id
    }

    /// Show an info toast.
    pub fn info(&self, message: impl Into<String>) -> ToastId {
        self.show(message, ToastVariant::Info)
    }

    /// Show a success toast.
    pub fn success(&self, message: impl Into<String>) -> ToastId {
        self.show(message, ToastVariant::Success)
    }

    /// Show a warning toast.
    pub fn warning(&self, message: impl Into<String>) -> ToastId {
        self.show(message, ToastVariant::Warning)
    }

    /// Show an error toast.
    pub fn error(&self, message: impl Into<String>) -> ToastId {
        self.show(message, ToastVariant::Error)
    }

    /// Dismiss a toast by ID.
    pub fn dismiss(&self, id: ToastId) {
        let mut toasts = self.toasts_read.get();
        toasts.retain(|t| t.id != id);
        self.toasts_write.set(toasts);
    }

    /// Dismiss all toasts.
    pub fn dismiss_all(&self) {
        self.toasts_write.set(Vec::new());
    }

    /// Get all toast entries.
    pub fn entries(&self) -> Vec<ToastEntry> {
        self.toasts_read.get()
    }

    /// Get the toasts signal for reactive binding.
    pub fn signal(&self) -> ReadSignal<Vec<ToastEntry>> {
        self.toasts_read.clone()
    }

    /// Build VNode toasts from current state.
    pub fn build_toasts(&self) -> Vec<VNode> {
        self.toasts_read
            .get()
            .iter()
            .map(|entry| {
                let mut toast = Toast::new(&entry.message).variant(entry.variant);
                if let Some(ref title) = entry.title {
                    toast = toast.title(title);
                }
                toast.build()
            })
            .collect()
    }
}

impl Default for ToastState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toast_variants() {
        assert_eq!(ToastVariant::Info.icon(), "ℹ");
        assert_eq!(ToastVariant::Success.icon(), "✓");
        assert_eq!(ToastVariant::Warning.icon(), "⚠");
        assert_eq!(ToastVariant::Error.icon(), "✗");
    }

    #[test]
    fn test_toast_build() {
        let toast = Toast::new("Test message").success().build();
        matches!(toast, VNode::Box(_));
    }

    #[test]
    fn test_toast_container_empty() {
        let container = ToastContainer::new().build();
        matches!(container, VNode::Empty);
    }

    #[test]
    fn test_toast_state() {
        let state = ToastState::new();
        assert!(state.entries().is_empty());

        let id1 = state.info("Info message");
        let _id2 = state.success("Success message");

        assert_eq!(state.entries().len(), 2);

        state.dismiss(id1);
        assert_eq!(state.entries().len(), 1);

        state.dismiss_all();
        assert!(state.entries().is_empty());
    }
}
