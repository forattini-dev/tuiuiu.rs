//! Local Mouse Hook
//!
//! Hook for handling mouse events with component-relative coordinates.
//!
//! # Example
//!
//! ```rust,ignore
//! use tuiuiu::hooks::local_mouse::{use_local_mouse, Bounds, LocalMouseEvent};
//!
//! let bounds = Bounds::new(10, 5, 80, 24);
//! use_local_mouse(bounds, |event| {
//!     // event.x and event.y are relative to the bounds
//!     println!("Local: ({}, {})", event.x, event.y);
//!     println!("Global: ({}, {})", event.global_x, event.global_y);
//!     println!("Inside: {}", event.is_inside);
//! });
//! ```

use crate::hooks::layout::LayoutRect;

// =============================================================================
// Types
// =============================================================================

/// Alias for LayoutRect to represent bounds.
pub type Bounds = LayoutRect;

/// Mouse button types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    WheelUp,
    WheelDown,
    None,
}

impl Default for MouseButton {
    fn default() -> Self {
        MouseButton::None
    }
}

/// Mouse action types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseAction {
    Press,
    Release,
    Move,
    Drag,
    Scroll,
}

impl Default for MouseAction {
    fn default() -> Self {
        MouseAction::Move
    }
}

/// Modifier keys state.
#[derive(Debug, Clone, Copy, Default)]
pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}

/// Raw mouse event from terminal.
#[derive(Debug, Clone, Default)]
pub struct RawMouseEvent {
    /// Terminal X coordinate.
    pub x: u16,
    /// Terminal Y coordinate.
    pub y: u16,
    /// Which button was pressed/released.
    pub button: MouseButton,
    /// Type of mouse action.
    pub action: MouseAction,
    /// Modifier keys held during the event.
    pub modifiers: Modifiers,
}

/// Local mouse event with component-relative coordinates.
#[derive(Debug, Clone)]
pub struct LocalMouseEvent {
    /// X coordinate relative to bounds (can be negative if outside).
    pub x: i16,
    /// Y coordinate relative to bounds (can be negative if outside).
    pub y: i16,
    /// Original terminal X coordinate.
    pub global_x: u16,
    /// Original terminal Y coordinate.
    pub global_y: u16,
    /// Whether the event is inside the bounds.
    pub is_inside: bool,
    /// Which button was pressed/released.
    pub button: MouseButton,
    /// Type of mouse action.
    pub action: MouseAction,
    /// Modifier keys held during the event.
    pub modifiers: Modifiers,
}

impl LocalMouseEvent {
    /// Create a local mouse event from a raw event and bounds.
    pub fn from_raw(raw: &RawMouseEvent, bounds: &Bounds) -> Self {
        let local_x = raw.x as i16 - bounds.x as i16;
        let local_y = raw.y as i16 - bounds.y as i16;
        let is_inside = bounds.contains(raw.x, raw.y);

        Self {
            x: local_x,
            y: local_y,
            global_x: raw.x,
            global_y: raw.y,
            is_inside,
            button: raw.button,
            action: raw.action,
            modifiers: raw.modifiers,
        }
    }

    /// Check if this is a click event (press with left button).
    pub fn is_click(&self) -> bool {
        self.action == MouseAction::Press && self.button == MouseButton::Left
    }

    /// Check if this is a right-click event.
    pub fn is_right_click(&self) -> bool {
        self.action == MouseAction::Press && self.button == MouseButton::Right
    }

    /// Check if this is a scroll event.
    pub fn is_scroll(&self) -> bool {
        self.action == MouseAction::Scroll
    }

    /// Check if scrolling up.
    pub fn is_scroll_up(&self) -> bool {
        self.button == MouseButton::WheelUp
    }

    /// Check if scrolling down.
    pub fn is_scroll_down(&self) -> bool {
        self.button == MouseButton::WheelDown
    }
}

/// Options for local mouse hook.
#[derive(Debug, Clone)]
pub struct LocalMouseOptions {
    /// Only invoke callback when event is inside bounds.
    pub only_inside: bool,
    /// Only handle events when active.
    pub is_active: bool,
}

impl Default for LocalMouseOptions {
    fn default() -> Self {
        Self {
            only_inside: false,
            is_active: true,
        }
    }
}

// =============================================================================
// Local Mouse State
// =============================================================================

/// Local mouse handler state.
#[derive(Clone)]
pub struct LocalMouseHandler {
    bounds: Bounds,
    options: LocalMouseOptions,
}

impl LocalMouseHandler {
    /// Create a new local mouse handler.
    pub fn new(bounds: Bounds) -> Self {
        Self {
            bounds,
            options: LocalMouseOptions::default(),
        }
    }

    /// Create with options.
    pub fn with_options(bounds: Bounds, options: LocalMouseOptions) -> Self {
        Self { bounds, options }
    }

    /// Update the bounds.
    pub fn set_bounds(&mut self, bounds: Bounds) {
        self.bounds = bounds;
    }

    /// Update options.
    pub fn set_options(&mut self, options: LocalMouseOptions) {
        self.options = options;
    }

    /// Process a raw mouse event.
    ///
    /// Returns `Some(LocalMouseEvent)` if the event should be handled,
    /// `None` if it should be filtered out.
    pub fn process(&self, raw: &RawMouseEvent) -> Option<LocalMouseEvent> {
        if !self.options.is_active {
            return None;
        }

        let local_event = LocalMouseEvent::from_raw(raw, &self.bounds);

        if self.options.only_inside && !local_event.is_inside {
            return None;
        }

        Some(local_event)
    }

    /// Check if a point is inside the bounds.
    pub fn contains(&self, x: u16, y: u16) -> bool {
        self.bounds.contains(x, y)
    }

    /// Get the current bounds.
    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}

// =============================================================================
// Hook
// =============================================================================

/// Create a local mouse handler with the given bounds.
///
/// This hook creates a handler that transforms terminal-absolute mouse
/// coordinates into component-relative coordinates.
///
/// # Arguments
///
/// * `bounds` - The component's bounds
///
/// # Returns
///
/// A `LocalMouseHandler` that can process raw mouse events.
///
/// # Example
///
/// ```rust,ignore
/// use tuiuiu::hooks::local_mouse::{use_local_mouse, Bounds, RawMouseEvent};
///
/// let bounds = Bounds::new(10, 5, 80, 24);
/// let handler = use_local_mouse(bounds);
///
/// // When a raw mouse event arrives:
/// if let Some(local_event) = handler.process(&raw_event) {
///     if local_event.is_inside && local_event.is_click() {
///         // Handle click inside component
///     }
/// }
/// ```
pub fn use_local_mouse(bounds: Bounds) -> LocalMouseHandler {
    LocalMouseHandler::new(bounds)
}

/// Create a local mouse handler with custom options.
pub fn use_local_mouse_with(bounds: Bounds, options: LocalMouseOptions) -> LocalMouseHandler {
    LocalMouseHandler::with_options(bounds, options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_mouse_event() {
        let bounds = Bounds::new(10, 10, 20, 20);
        let raw = RawMouseEvent {
            x: 15,
            y: 15,
            button: MouseButton::Left,
            action: MouseAction::Press,
            modifiers: Modifiers::default(),
        };

        let local = LocalMouseEvent::from_raw(&raw, &bounds);
        assert_eq!(local.x, 5); // 15 - 10
        assert_eq!(local.y, 5); // 15 - 10
        assert_eq!(local.global_x, 15);
        assert_eq!(local.global_y, 15);
        assert!(local.is_inside);
        assert!(local.is_click());
    }

    #[test]
    fn test_local_mouse_outside() {
        let bounds = Bounds::new(10, 10, 20, 20);
        let raw = RawMouseEvent {
            x: 5,
            y: 5,
            button: MouseButton::Left,
            action: MouseAction::Press,
            modifiers: Modifiers::default(),
        };

        let local = LocalMouseEvent::from_raw(&raw, &bounds);
        assert_eq!(local.x, -5); // 5 - 10
        assert_eq!(local.y, -5); // 5 - 10
        assert!(!local.is_inside);
    }

    #[test]
    fn test_handler_filter() {
        let bounds = Bounds::new(10, 10, 20, 20);
        let handler = use_local_mouse_with(
            bounds,
            LocalMouseOptions {
                only_inside: true,
                is_active: true,
            },
        );

        // Event inside bounds
        let inside = RawMouseEvent {
            x: 15,
            y: 15,
            ..Default::default()
        };
        assert!(handler.process(&inside).is_some());

        // Event outside bounds
        let outside = RawMouseEvent {
            x: 5,
            y: 5,
            ..Default::default()
        };
        assert!(handler.process(&outside).is_none()); // Filtered out
    }

    #[test]
    fn test_handler_inactive() {
        let bounds = Bounds::new(0, 0, 100, 100);
        let handler = use_local_mouse_with(
            bounds,
            LocalMouseOptions {
                only_inside: false,
                is_active: false,
            },
        );

        let event = RawMouseEvent {
            x: 50,
            y: 50,
            ..Default::default()
        };
        assert!(handler.process(&event).is_none()); // Handler is inactive
    }

    #[test]
    fn test_scroll_detection() {
        let bounds = Bounds::new(0, 0, 100, 100);
        let raw_up = RawMouseEvent {
            x: 50,
            y: 50,
            button: MouseButton::WheelUp,
            action: MouseAction::Scroll,
            modifiers: Modifiers::default(),
        };

        let local = LocalMouseEvent::from_raw(&raw_up, &bounds);
        assert!(local.is_scroll());
        assert!(local.is_scroll_up());
        assert!(!local.is_scroll_down());
    }
}
