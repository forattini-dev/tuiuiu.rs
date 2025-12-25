//! Focus Management
//!
//! Manages keyboard focus, focus zones, and focus trapping.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// =============================================================================
// Focusable Element
// =============================================================================

/// A focusable element in the UI.
#[derive(Debug, Clone)]
pub struct Focusable {
    /// Unique ID
    pub id: u64,
    /// Tab index (lower = first)
    pub tab_index: i32,
    /// Whether currently focusable
    pub disabled: bool,
    /// Focus zone this belongs to
    pub zone_id: Option<u64>,
}

impl Focusable {
    /// Create a new focusable element.
    pub fn new(id: u64) -> Self {
        Self {
            id,
            tab_index: 0,
            disabled: false,
            zone_id: None,
        }
    }

    /// Set tab index.
    pub fn with_tab_index(mut self, index: i32) -> Self {
        self.tab_index = index;
        self
    }

    /// Set disabled state.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

// =============================================================================
// Focus Zone
// =============================================================================

/// Focus zone options.
#[derive(Debug, Clone, Default)]
pub struct FocusZoneOptions {
    /// Zone ID
    pub id: Option<u64>,
    /// Wrap focus at ends
    pub wrap: bool,
    /// Trap focus within zone
    pub trap: bool,
    /// Auto-focus first element
    pub auto_focus: bool,
    /// Restore focus on unmount
    pub restore_focus: bool,
}

/// Focus zone state.
#[derive(Debug, Clone)]
pub struct FocusZoneState {
    /// Zone ID
    pub id: u64,
    /// Currently focused element
    pub focused_id: Option<u64>,
    /// Elements in this zone
    pub elements: Vec<u64>,
    /// Options
    pub options: FocusZoneOptions,
}

/// Create a focus zone.
pub fn create_focus_zone(options: FocusZoneOptions) -> FocusZoneState {
    static mut ZONE_ID: u64 = 0;
    let id = options.id.unwrap_or_else(|| unsafe {
        ZONE_ID += 1;
        ZONE_ID
    });

    FocusZoneState {
        id,
        focused_id: None,
        elements: Vec::new(),
        options,
    }
}

/// Create a focus trap (zone that traps focus).
pub fn create_focus_trap() -> FocusZoneState {
    create_focus_zone(FocusZoneOptions {
        trap: true,
        wrap: true,
        ..Default::default()
    })
}

// =============================================================================
// Focus Manager
// =============================================================================

/// Global focus manager.
pub struct FocusManager {
    /// Registered focusable elements
    elements: HashMap<u64, Focusable>,
    /// Focus zones
    zones: HashMap<u64, FocusZoneState>,
    /// Currently focused element ID
    focused: Option<u64>,
    /// Focus history stack
    history: Vec<u64>,
    /// Focus change callbacks
    callbacks: Vec<Rc<dyn Fn(Option<u64>, Option<u64>)>>,
}

impl FocusManager {
    /// Create a new focus manager.
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            zones: HashMap::new(),
            focused: None,
            history: Vec::new(),
            callbacks: Vec::new(),
        }
    }

    /// Register a focusable element.
    pub fn register(&mut self, element: Focusable) {
        self.elements.insert(element.id, element);
    }

    /// Unregister an element.
    pub fn unregister(&mut self, id: u64) {
        self.elements.remove(&id);
        if self.focused == Some(id) {
            self.focused = None;
        }
    }

    /// Focus an element by ID.
    pub fn focus(&mut self, id: u64) -> bool {
        if let Some(element) = self.elements.get(&id) {
            if !element.disabled {
                let old = self.focused;
                if let Some(old_id) = old {
                    self.history.push(old_id);
                }
                self.focused = Some(id);
                self.notify_change(old, Some(id));
                return true;
            }
        }
        false
    }

    /// Clear focus.
    pub fn blur(&mut self) {
        let old = self.focused.take();
        self.notify_change(old, None);
    }

    /// Focus the next element.
    pub fn focus_next(&mut self) -> bool {
        let ids: Vec<_> = self.sorted_focusable_ids();
        if ids.is_empty() {
            return false;
        }

        let current_idx = self
            .focused
            .and_then(|id| ids.iter().position(|&i| i == id));

        let next_idx = match current_idx {
            Some(idx) => (idx + 1) % ids.len(),
            None => 0,
        };

        self.focus(ids[next_idx])
    }

    /// Focus the previous element.
    pub fn focus_previous(&mut self) -> bool {
        let ids: Vec<_> = self.sorted_focusable_ids();
        if ids.is_empty() {
            return false;
        }

        let current_idx = self
            .focused
            .and_then(|id| ids.iter().position(|&i| i == id));

        let prev_idx = match current_idx {
            Some(idx) if idx > 0 => idx - 1,
            Some(_) => ids.len() - 1,
            None => ids.len() - 1,
        };

        self.focus(ids[prev_idx])
    }

    /// Focus the first element.
    pub fn focus_first(&mut self) -> bool {
        let ids = self.sorted_focusable_ids();
        ids.first().map(|&id| self.focus(id)).unwrap_or(false)
    }

    /// Focus the last element.
    pub fn focus_last(&mut self) -> bool {
        let ids = self.sorted_focusable_ids();
        ids.last().map(|&id| self.focus(id)).unwrap_or(false)
    }

    /// Get the currently focused element ID.
    pub fn get_focused(&self) -> Option<u64> {
        self.focused
    }

    /// Check if an element is focused.
    pub fn is_focused(&self, id: u64) -> bool {
        self.focused == Some(id)
    }

    /// Subscribe to focus changes.
    pub fn on_change<F: Fn(Option<u64>, Option<u64>) + 'static>(&mut self, callback: F) {
        self.callbacks.push(Rc::new(callback));
    }

    fn sorted_focusable_ids(&self) -> Vec<u64> {
        let mut items: Vec<_> = self
            .elements
            .values()
            .filter(|e| !e.disabled)
            .collect();

        items.sort_by_key(|e| (e.tab_index, e.id));
        items.iter().map(|e| e.id).collect()
    }

    fn notify_change(&self, old: Option<u64>, new: Option<u64>) {
        for callback in &self.callbacks {
            callback(old, new);
        }
    }
}

impl Default for FocusManager {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Global Focus Manager
// =============================================================================

thread_local! {
    static FOCUS_MANAGER: RefCell<FocusManager> = RefCell::new(FocusManager::new());
}

/// Get the global focus manager.
pub fn get_focus_zone_manager() -> &'static std::thread::LocalKey<RefCell<FocusManager>> {
    &FOCUS_MANAGER
}

/// Reset the global focus manager.
pub fn reset_focus_zone_manager() {
    FOCUS_MANAGER.with(|fm| {
        *fm.borrow_mut() = FocusManager::new();
    });
}

/// Focus an element.
pub fn focus_element(id: u64) -> bool {
    FOCUS_MANAGER.with(|fm| fm.borrow_mut().focus(id))
}

/// Focus the next element.
pub fn focus_next() -> bool {
    FOCUS_MANAGER.with(|fm| fm.borrow_mut().focus_next())
}

/// Focus the previous element.
pub fn focus_previous() -> bool {
    FOCUS_MANAGER.with(|fm| fm.borrow_mut().focus_previous())
}

/// Focus the first element.
pub fn focus_first() -> bool {
    FOCUS_MANAGER.with(|fm| fm.borrow_mut().focus_first())
}

/// Focus the last element.
pub fn focus_last() -> bool {
    FOCUS_MANAGER.with(|fm| fm.borrow_mut().focus_last())
}

/// Blur the current focus.
pub fn blur_focus() {
    FOCUS_MANAGER.with(|fm| fm.borrow_mut().blur());
}

/// Get the active element ID.
pub fn get_active_id() -> Option<u64> {
    FOCUS_MANAGER.with(|fm| fm.borrow().get_focused())
}

/// Check if an element is focused.
pub fn is_focused(id: u64) -> bool {
    FOCUS_MANAGER.with(|fm| fm.borrow().is_focused(id))
}

/// Register a focusable element.
pub fn register_focusable(element: Focusable) {
    FOCUS_MANAGER.with(|fm| fm.borrow_mut().register(element));
}

/// Subscribe to focus changes.
pub fn on_focus_change<F: Fn(Option<u64>, Option<u64>) + 'static>(callback: F) {
    FOCUS_MANAGER.with(|fm| fm.borrow_mut().on_change(callback));
}

// =============================================================================
// Focus Zone (Component)
// =============================================================================

/// Focus zone component options.
#[derive(Debug, Clone, Default)]
pub struct FocusZoneComponentOptions {
    /// Trap focus
    pub trap: bool,
    /// Wrap at ends
    pub wrap: bool,
    /// Auto focus first
    pub auto_focus: bool,
}

/// Focus zone state for components.
pub type FocusZoneEventData = (Option<u64>, Option<u64>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focus_manager() {
        let mut fm = FocusManager::new();

        fm.register(Focusable::new(1));
        fm.register(Focusable::new(2));
        fm.register(Focusable::new(3));

        assert!(fm.focus(1));
        assert_eq!(fm.get_focused(), Some(1));

        assert!(fm.focus_next());
        assert_eq!(fm.get_focused(), Some(2));

        assert!(fm.focus_previous());
        assert_eq!(fm.get_focused(), Some(1));
    }

    #[test]
    fn test_focus_disabled() {
        let mut fm = FocusManager::new();

        fm.register(Focusable::new(1).with_disabled(true));
        fm.register(Focusable::new(2));

        // Can't focus disabled element
        assert!(!fm.focus(1));

        // Can focus enabled element
        assert!(fm.focus(2));
    }

    #[test]
    fn test_tab_index() {
        let mut fm = FocusManager::new();

        fm.register(Focusable::new(1).with_tab_index(2));
        fm.register(Focusable::new(2).with_tab_index(1));
        fm.register(Focusable::new(3).with_tab_index(0));

        // First should be ID 3 (lowest tab_index)
        assert!(fm.focus_first());
        assert_eq!(fm.get_focused(), Some(3));
    }
}
