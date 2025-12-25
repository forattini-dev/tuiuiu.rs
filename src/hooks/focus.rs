//! Focus Hooks
//!
//! Focus management for components.

use crate::core::focus::{
    Focusable, focus_element, focus_next, focus_previous, 
    is_focused, register_focusable, get_active_id,
};
use crate::core::signals::{create_signal, ReadSignal};

/// Focus options.
#[derive(Debug, Clone, Default)]
pub struct FocusOptions {
    /// Initial focus state
    pub auto_focus: bool,
    /// Tab index
    pub tab_index: i32,
    /// Whether focusable
    pub disabled: bool,
}

/// Focus result from use_focus.
pub struct FocusResult {
    /// Whether this element is focused
    pub is_focused: ReadSignal<bool>,
    /// Focus this element
    pub focus: Box<dyn Fn()>,
    /// Blur this element
    pub blur: Box<dyn Fn()>,
    /// Element ID
    pub id: u64,
}

impl std::fmt::Debug for FocusResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FocusResult")
            .field("is_focused", &self.is_focused)
            .field("focus", &"<fn>")
            .field("blur", &"<fn>")
            .field("id", &self.id)
            .finish()
    }
}

/// Create focus management for a component.
///
/// # Example
///
/// ```rust
/// use tuiuiu::hooks::{use_focus, FocusOptions};
///
/// let focus = use_focus(FocusOptions {
///     auto_focus: true,
///     ..Default::default()
/// });
///
/// if focus.is_focused.get() {
///     println!("I'm focused!");
/// }
/// ```
pub fn use_focus(options: FocusOptions) -> FocusResult {
    static mut NEXT_ID: u64 = 1000;
    let id = unsafe {
        NEXT_ID += 1;
        NEXT_ID
    };

    let (is_focused_signal, set_focused) = create_signal(false);

    // Register as focusable
    let focusable = Focusable::new(id)
        .with_tab_index(options.tab_index)
        .with_disabled(options.disabled);
    register_focusable(focusable);

    // Auto focus if requested
    if options.auto_focus {
        focus_element(id);
    }

    let focus_fn = {
        let id = id;
        move || { focus_element(id); }
    };

    let blur_fn = move || {
        set_focused.set(false);
    };

    FocusResult {
        is_focused: is_focused_signal,
        focus: Box::new(focus_fn),
        blur: Box::new(blur_fn),
        id,
    }
}

/// Get the focus manager hook.
pub fn use_focus_manager() -> FocusManagerHook {
    FocusManagerHook
}

/// Focus manager hook result.
pub struct FocusManagerHook;

impl FocusManagerHook {
    /// Focus the next element.
    pub fn focus_next(&self) -> bool {
        focus_next()
    }

    /// Focus the previous element.
    pub fn focus_previous(&self) -> bool {
        focus_previous()
    }

    /// Check if an element is focused.
    pub fn is_focused(&self, id: u64) -> bool {
        is_focused(id)
    }

    /// Get the currently focused element ID.
    pub fn active_id(&self) -> Option<u64> {
        get_active_id()
    }
}
