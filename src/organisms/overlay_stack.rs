//! Overlay Stack Component
//!
//! Manages stacked overlays (modals, popovers, tooltips) with z-index management
//! and focus trapping.

use crate::core::component::{BoxNode, BoxStyle, VNode};
use crate::core::layout::{AlignItems, FlexDirection, JustifyContent};

/// Overlay entry in the stack.
#[derive(Debug, Clone)]
pub struct OverlayEntry {
    /// Unique ID for this overlay
    pub id: String,
    /// The overlay content
    pub content: VNode,
    /// Z-index for ordering
    pub z_index: u32,
    /// Whether to show backdrop
    pub backdrop: bool,
    /// Whether overlay is modal (blocks interaction below)
    pub modal: bool,
    /// Whether clicking backdrop closes the overlay
    pub close_on_backdrop_click: bool,
}

impl OverlayEntry {
    /// Create a new overlay entry.
    pub fn new(id: impl Into<String>, content: VNode) -> Self {
        Self {
            id: id.into(),
            content,
            z_index: 100,
            backdrop: true,
            modal: true,
            close_on_backdrop_click: true,
        }
    }

    /// Set z-index.
    pub fn z_index(mut self, z: u32) -> Self {
        self.z_index = z;
        self
    }

    /// Set backdrop visibility.
    pub fn backdrop(mut self, show: bool) -> Self {
        self.backdrop = show;
        self
    }

    /// Set modal behavior.
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    /// Set close on backdrop click.
    pub fn close_on_backdrop_click(mut self, close: bool) -> Self {
        self.close_on_backdrop_click = close;
        self
    }
}

/// State for the overlay stack.
#[derive(Debug, Clone, Default)]
pub struct OverlayStackState {
    /// Stack of overlays
    entries: Vec<OverlayEntry>,
    /// Next z-index to assign
    next_z: u32,
}

impl OverlayStackState {
    /// Create new state.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_z: 100,
        }
    }

    /// Push a new overlay onto the stack.
    pub fn push(&mut self, mut entry: OverlayEntry) -> String {
        entry.z_index = self.next_z;
        self.next_z += 10;
        let id = entry.id.clone();
        self.entries.push(entry);
        id
    }

    /// Pop the top overlay from the stack.
    pub fn pop(&mut self) -> Option<OverlayEntry> {
        let entry = self.entries.pop();
        if self.entries.is_empty() {
            self.next_z = 100;
        }
        entry
    }

    /// Remove an overlay by ID.
    pub fn remove(&mut self, id: &str) -> Option<OverlayEntry> {
        if let Some(idx) = self.entries.iter().position(|e| e.id == id) {
            Some(self.entries.remove(idx))
        } else {
            None
        }
    }

    /// Get the top overlay.
    pub fn top(&self) -> Option<&OverlayEntry> {
        self.entries.last()
    }

    /// Check if stack is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get number of overlays.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Get all entries (for rendering).
    pub fn entries(&self) -> &[OverlayEntry] {
        &self.entries
    }

    /// Check if an overlay exists.
    pub fn contains(&self, id: &str) -> bool {
        self.entries.iter().any(|e| e.id == id)
    }

    /// Clear all overlays.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.next_z = 100;
    }
}

/// Create a new overlay stack state.
pub fn create_overlay_stack_state() -> OverlayStackState {
    OverlayStackState::new()
}

/// Overlay stack component.
#[derive(Debug, Clone)]
pub struct OverlayStack {
    state: OverlayStackState,
    base_content: Option<VNode>,
}

impl Default for OverlayStack {
    fn default() -> Self {
        Self {
            state: OverlayStackState::new(),
            base_content: None,
        }
    }
}

impl OverlayStack {
    /// Create a new overlay stack.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with state.
    pub fn with_state(state: OverlayStackState) -> Self {
        Self {
            state,
            base_content: None,
        }
    }

    /// Set the base content (underneath all overlays).
    pub fn base(mut self, content: VNode) -> Self {
        self.base_content = Some(content);
        self
    }

    /// Build a backdrop VNode.
    fn build_backdrop(&self, _for_modal: bool) -> VNode {
        VNode::Box(BoxNode {
            children: vec![],
            style: BoxStyle {
                flex_grow: Some(1.0),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let mut children: Vec<VNode> = Vec::new();

        // Add base content if present
        if let Some(ref base) = self.base_content {
            children.push(base.clone());
        }

        // Add overlays in order
        for entry in self.state.entries.iter() {
            // Add backdrop if needed
            if entry.backdrop {
                children.push(self.build_backdrop(entry.modal));
            }

            // Add the overlay content
            children.push(VNode::Box(BoxNode {
                children: vec![entry.content.clone()],
                style: BoxStyle {
                    justify_content: Some(JustifyContent::Center),
                    align_items: Some(AlignItems::Center),
                    ..Default::default()
                },
                ..Default::default()
            }));
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                flex_grow: Some(1.0),
                ..Default::default()
            },
            ..Default::default()
        })
    }
}

// =============================================================================
// JS-Compatible API
// =============================================================================

/// Overlay priority levels.
#[derive(Debug, Clone, Copy, Default)]
pub enum OverlayPriority {
    /// Lowest priority.
    Low,
    /// Normal priority.
    #[default]
    Normal,
    /// High priority.
    High,
    /// Critical priority.
    Critical,
}

/// Render function type for an overlay.
pub type OverlayRenderer = fn() -> VNode;

/// Component used by `OverlayContainer` as optional backdrop renderer.
pub type OverlayBackdropRenderer = fn(&OverlayEntry) -> Option<VNode>;

/// Configuration used to describe an overlay before mounting.
#[derive(Debug, Clone)]
pub struct OverlayConfig {
    /// Overlay id.
    pub id: String,
    /// Overlay component.
    pub component: OverlayRenderer,
    /// Priority.
    pub priority: OverlayPriority,
    /// Close when escape is pressed.
    pub close_on_escape: bool,
    /// Close when clicking outside.
    pub close_on_click_outside: bool,
    /// Show backdrop.
    pub show_backdrop: bool,
}

impl OverlayConfig {
    /// Build an [`OverlayEntry`] from this config.
    pub fn into_entry(self) -> OverlayEntry {
        let mut entry = OverlayEntry::new(self.id, (self.component)());
        entry.backdrop = self.show_backdrop;
        entry.close_on_backdrop_click = self.close_on_click_outside;
        // Use modal only when backdrop is active.
        entry.modal = self.show_backdrop && self.close_on_escape;
        entry
    }
}

/// Props for [`OverlayContainer`].
pub struct OverlayContainerProps {
    /// Overlay state.
    pub stack: OverlayStackState,
    /// Optional backdrop renderer.
    pub render_backdrop: Option<OverlayBackdropRenderer>,
}

/// Options for a lightweight overlay input hook.
pub struct UseOverlayInputOptions {
    /// Target stack.
    pub stack: OverlayStackState,
    /// Handle escape key.
    pub handle_escape: bool,
}

/// Create a new overlay stack.
pub fn createOverlayStack() -> OverlayStackState {
    create_overlay_stack_state()
}

/// Snake case alias.
pub fn create_overlay_stack() -> OverlayStackState {
    create_overlay_stack_state()
}

/// Keep input handling blocked while any overlay is active.
pub fn shouldBlockInput(stack: &OverlayStackState) -> bool {
    !stack.is_empty()
}

/// Close the top overlay on escape if applicable.
pub fn handleOverlayEscape(stack: &mut OverlayStackState) -> bool {
    if stack.is_empty() {
        return false;
    }

    stack.pop();
    true
}

/// Container that renders overlays in order.
pub fn OverlayContainer(props: OverlayContainerProps) -> VNode {
    let mut children: Vec<VNode> = Vec::new();

    for entry in props.stack.entries() {
        if entry.backdrop {
            let backdrop = props.render_backdrop.map(|render| render(entry));
            if let Some(Some(node)) = backdrop {
                children.push(node);
            }
        }
        children.push(VNode::Box(BoxNode {
            children: vec![entry.content.clone()],
            style: BoxStyle {
                justify_content: Some(JustifyContent::Center),
                align_items: Some(AlignItems::Center),
                ..Default::default()
            },
            ..Default::default()
        }));
    }

    VNode::Box(BoxNode {
        children,
        style: BoxStyle {
            flex_direction: Some(FlexDirection::Column),
            ..Default::default()
        },
        ..Default::default()
    })
}

/// Create common modal overlay config.
pub fn createModalOverlay(options: OverlayConfigBuilder) -> OverlayConfig {
    OverlayConfig {
        id: options.id,
        component: options.component,
        priority: options.priority.unwrap_or(OverlayPriority::Normal),
        close_on_escape: options.close_on_escape.unwrap_or(true),
        close_on_click_outside: options.close_on_click_outside.unwrap_or(false),
        show_backdrop: options.show_backdrop.unwrap_or(true),
    }
}

/// Create toast overlay config.
pub fn createToastOverlay(options: OverlayConfigBuilder) -> OverlayConfig {
    OverlayConfig {
        id: options.id,
        component: options.component,
        priority: OverlayPriority::Low,
        close_on_escape: false,
        close_on_click_outside: false,
        show_backdrop: false,
    }
}

/// Create critical overlay config.
pub fn createCriticalOverlay(options: OverlayConfigBuilder) -> OverlayConfig {
    OverlayConfig {
        id: options.id,
        component: options.component,
        priority: OverlayPriority::Critical,
        close_on_escape: options.close_on_escape.unwrap_or(false),
        close_on_click_outside: options.close_on_click_outside.unwrap_or(false),
        show_backdrop: options.show_backdrop.unwrap_or(true),
    }
}

/// Helper for constructing overlay configs without repetitive literals.
pub struct OverlayConfigBuilder {
    pub id: String,
    pub component: OverlayRenderer,
    pub priority: Option<OverlayPriority>,
    pub close_on_escape: Option<bool>,
    pub close_on_click_outside: Option<bool>,
    pub show_backdrop: Option<bool>,
}

impl OverlayConfigBuilder {
    /// Build from simple options.
    pub fn new(id: impl Into<String>, component: OverlayRenderer) -> Self {
        Self {
            id: id.into(),
            component,
            priority: None,
            close_on_escape: None,
            close_on_click_outside: None,
            show_backdrop: None,
        }
    }
}
