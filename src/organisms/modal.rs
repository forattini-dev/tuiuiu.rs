//! Modal Component
//!
//! A dialog overlay that appears above the main content.
//! Base component for ConfirmDialog, AlertBox, and other overlays.

use crate::core::component::{
    BoxNode, BoxStyle, BorderStyle, Color, EventHandlers, NamedColor, TextNode, TextStyle, VNode,
};
use crate::core::layout::{AlignItems, FlexDirection, JustifyContent, Size};

/// Modal size variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ModalSize {
    /// Small modal (40 chars wide)
    Small,
    /// Medium modal (60 chars wide)
    #[default]
    Medium,
    /// Large modal (80 chars wide)
    Large,
    /// Full width (95% of terminal)
    Full,
    /// Custom width
    Custom(u16),
}

impl ModalSize {
    /// Get the width in characters.
    pub fn width(&self) -> u16 {
        match self {
            ModalSize::Small => 40,
            ModalSize::Medium => 60,
            ModalSize::Large => 80,
            ModalSize::Full => 100, // Will be handled specially in render
            ModalSize::Custom(w) => *w,
        }
    }
}

/// Modal component.
///
/// Creates an overlay dialog with optional title, content, and actions.
///
/// # Example
/// ```ignore
/// let modal = Modal::new()
///     .title("Confirm Action")
///     .content(vec![VNode::text("Are you sure?")])
///     .actions(vec![
///         Button::new("Cancel").outline().build(),
///         Button::new("Confirm").primary().build(),
///     ])
///     .open(true)
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct Modal {
    /// Whether the modal is open/visible
    open: bool,
    /// Modal title
    title: Option<String>,
    /// Modal content (body)
    content: Vec<VNode>,
    /// Action buttons
    actions: Vec<VNode>,
    /// Modal size
    size: ModalSize,
    /// Show close button (X)
    closable: bool,
    /// Show backdrop (dim background)
    backdrop: bool,
    /// Center content vertically
    centered: bool,
    /// Custom header color
    header_color: Option<Color>,
    /// Border style
    border_style: BorderStyle,
}

impl Modal {
    /// Create a new modal.
    pub fn new() -> Self {
        Self {
            open: false,
            title: None,
            content: Vec::new(),
            actions: Vec::new(),
            size: ModalSize::default(),
            closable: true,
            backdrop: true,
            centered: true,
            header_color: Some(Color::Named(NamedColor::Blue)),
            border_style: BorderStyle::Round,
        }
    }

    /// Set whether the modal is open.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set the modal title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the modal content.
    pub fn content(mut self, content: Vec<VNode>) -> Self {
        self.content = content;
        self
    }

    /// Add a single content node.
    pub fn add_content(mut self, node: VNode) -> Self {
        self.content.push(node);
        self
    }

    /// Set the modal text content (convenience method).
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.content = vec![VNode::text(text)];
        self
    }

    /// Set the action buttons.
    pub fn actions(mut self, actions: Vec<VNode>) -> Self {
        self.actions = actions;
        self
    }

    /// Add a single action.
    pub fn add_action(mut self, action: VNode) -> Self {
        self.actions.push(action);
        self
    }

    /// Set the modal size.
    pub fn size(mut self, size: ModalSize) -> Self {
        self.size = size;
        self
    }

    /// Set small size.
    pub fn small(self) -> Self {
        self.size(ModalSize::Small)
    }

    /// Set medium size.
    pub fn medium(self) -> Self {
        self.size(ModalSize::Medium)
    }

    /// Set large size.
    pub fn large(self) -> Self {
        self.size(ModalSize::Large)
    }

    /// Set full width.
    pub fn full(self) -> Self {
        self.size(ModalSize::Full)
    }

    /// Set custom width.
    pub fn width(self, width: u16) -> Self {
        self.size(ModalSize::Custom(width))
    }

    /// Set whether to show close button.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set whether to show backdrop.
    pub fn backdrop(mut self, backdrop: bool) -> Self {
        self.backdrop = backdrop;
        self
    }

    /// Set whether to center content vertically.
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// Set header color.
    pub fn header_color(mut self, color: Color) -> Self {
        self.header_color = Some(color);
        self
    }

    /// Set border style.
    pub fn border(mut self, style: BorderStyle) -> Self {
        self.border_style = style;
        self
    }

    /// Build the modal VNode.
    pub fn build(self) -> VNode {
        // If not open, return empty
        if !self.open {
            return VNode::Empty;
        }

        let modal_width = self.size.width();

        // Build header row (title + close button)
        let header = self.build_header(modal_width);

        // Build content area
        let content_area = self.build_content();

        // Build actions row
        let actions_area = self.build_actions();

        // Combine all parts
        let mut modal_children = Vec::new();

        if let Some(header) = header {
            modal_children.push(header);
        }

        if !self.content.is_empty() {
            modal_children.push(content_area);
        }

        if !self.actions.is_empty() {
            modal_children.push(actions_area);
        }

        // Modal box
        let modal_box = VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                width: Some(Size::Fixed(modal_width)),
                border_style: Some(self.border_style),
                border_color: self.header_color,
                background: Some(Color::Named(NamedColor::Black)),
                padding: Some(1),
                gap: Some(1),
                ..Default::default()
            },
            children: modal_children,
            handlers: EventHandlers {
                focusable: true,
                ..Default::default()
            },
        });

        // Center container
        let center_container = VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                justify_content: Some(JustifyContent::Center),
                align_items: Some(AlignItems::Center),
                flex_grow: Some(1.0),
                ..Default::default()
            },
            children: vec![modal_box],
            handlers: Default::default(),
        });

        // Backdrop overlay (full screen)
        if self.backdrop {
            VNode::Box(BoxNode {
                id: None,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Column),
                    justify_content: Some(JustifyContent::Center),
                    align_items: Some(AlignItems::Center),
                    flex_grow: Some(1.0),
                    background: Some(Color::Rgb(0, 0, 0)), // Semi-transparent black effect
                    ..Default::default()
                },
                children: vec![center_container],
                handlers: Default::default(),
            })
        } else {
            center_container
        }
    }

    /// Build the header row.
    fn build_header(&self, _width: u16) -> Option<VNode> {
        if self.title.is_none() && !self.closable {
            return None;
        }

        let mut header_children = Vec::new();

        // Title
        if let Some(ref title) = self.title {
            header_children.push(VNode::Text(TextNode {
                content: title.clone(),
                style: TextStyle {
                    bold: true,
                    color: self.header_color,
                    ..Default::default()
                },
            }));
        }

        // Spacer (push close button to the right)
        if self.closable {
            header_children.push(VNode::Box(BoxNode {
                style: BoxStyle {
                    flex_grow: Some(1.0),
                    ..Default::default()
                },
                ..Default::default()
            }));

            // Close button [X]
            header_children.push(VNode::Text(TextNode {
                content: "[X]".to_string(),
                style: TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    ..Default::default()
                },
            }));
        }

        Some(VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                justify_content: Some(JustifyContent::SpaceBetween),
                align_items: Some(AlignItems::Center),
                ..Default::default()
            },
            children: header_children,
            handlers: Default::default(),
        }))
    }

    /// Build the content area.
    fn build_content(&self) -> VNode {
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                gap: Some(1),
                padding_top: Some(1),
                padding_bottom: Some(1),
                ..Default::default()
            },
            children: self.content.clone(),
            handlers: Default::default(),
        })
    }

    /// Build the actions row.
    fn build_actions(&self) -> VNode {
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                justify_content: Some(JustifyContent::FlexEnd),
                align_items: Some(AlignItems::Center),
                gap: Some(2),
                padding_top: Some(1),
                ..Default::default()
            },
            children: self.actions.clone(),
            handlers: Default::default(),
        })
    }
}

impl From<Modal> for VNode {
    fn from(modal: Modal) -> VNode {
        modal.build()
    }
}

// =============================================================================
// Modal State
// =============================================================================

use crate::core::signals::{create_signal, ReadSignal, WriteSignal};

/// State manager for Modal.
///
/// Provides reactive open/close state with signals.
pub struct ModalState {
    open_read: ReadSignal<bool>,
    open_write: WriteSignal<bool>,
}

impl ModalState {
    /// Create a new modal state.
    pub fn new() -> Self {
        let (read, write) = create_signal(false);
        Self {
            open_read: read,
            open_write: write,
        }
    }

    /// Create starting open.
    pub fn open() -> Self {
        let (read, write) = create_signal(true);
        Self {
            open_read: read,
            open_write: write,
        }
    }

    /// Check if modal is open.
    pub fn is_open(&self) -> bool {
        self.open_read.get()
    }

    /// Open the modal.
    pub fn show(&self) {
        self.open_write.set(true);
    }

    /// Close the modal.
    pub fn hide(&self) {
        self.open_write.set(false);
    }

    /// Toggle the modal.
    pub fn toggle(&self) {
        self.open_write.set(!self.open_read.get());
    }

    /// Get the open signal for reactive binding.
    pub fn signal(&self) -> ReadSignal<bool> {
        self.open_read.clone()
    }
}

impl Default for ModalState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_closed_returns_empty() {
        let modal = Modal::new().title("Test").open(false).build();
        matches!(modal, VNode::Empty);
    }

    #[test]
    fn test_modal_open_returns_box() {
        let modal = Modal::new().title("Test").open(true).build();
        matches!(modal, VNode::Box(_));
    }

    #[test]
    fn test_modal_sizes() {
        assert_eq!(ModalSize::Small.width(), 40);
        assert_eq!(ModalSize::Medium.width(), 60);
        assert_eq!(ModalSize::Large.width(), 80);
        assert_eq!(ModalSize::Custom(100).width(), 100);
    }

    #[test]
    fn test_modal_state() {
        let state = ModalState::new();
        assert!(!state.is_open());

        state.show();
        assert!(state.is_open());

        state.hide();
        assert!(!state.is_open());

        state.toggle();
        assert!(state.is_open());
    }
}
