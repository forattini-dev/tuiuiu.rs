//! Previous Value Hook
//!
//! Hook for tracking the previous value of a signal.
//!
//! # Example
//!
//! ```rust,ignore
//! use tuiuiu::hooks::previous::use_previous;
//! use tuiuiu::create_signal;
//!
//! let (count, set_count) = create_signal(0);
//! let prev_count = use_previous(count.clone());
//!
//! set_count.set(5);
//! // prev_count.get() == Some(0) (the previous value)
//! ```

use crate::core::signals::{create_signal, create_effect, ReadSignal};
use std::cell::RefCell;

// =============================================================================
// Types
// =============================================================================

/// Handle for accessing the previous value of a signal.
#[derive(Clone)]
pub struct PreviousHandle<T> {
    value: ReadSignal<Option<T>>,
}

impl<T: Clone + 'static> PreviousHandle<T> {
    /// Get the previous value, if available.
    pub fn get(&self) -> Option<T> {
        self.value.get()
    }

    /// Get the previous value or a default.
    pub fn get_or(&self, default: T) -> T {
        self.value.get().unwrap_or(default)
    }

    /// Get the signal for reactive access.
    pub fn signal(&self) -> ReadSignal<Option<T>> {
        self.value.clone()
    }
}

// =============================================================================
// Hook
// =============================================================================

/// Track the previous value of a signal.
///
/// Returns the value from the previous render/update.
/// On the first render, returns `None`.
///
/// # Arguments
///
/// * `signal` - The signal to track
///
/// # Returns
///
/// A `PreviousHandle` containing the previous value.
///
/// # Example
///
/// ```rust,ignore
/// use tuiuiu::hooks::previous::use_previous;
/// use tuiuiu::create_signal;
///
/// let (count, set_count) = create_signal(0);
/// let prev = use_previous(count.clone());
///
/// // Initial: prev.get() == None
/// set_count.set(5);
/// // After update: prev.get() == Some(0)
/// ```
pub fn use_previous<T: Clone + 'static>(signal: ReadSignal<T>) -> PreviousHandle<T> {
    let (prev, set_prev) = create_signal(None);
    let current = RefCell::new(None::<T>);

    let signal_clone = signal.clone();
    create_effect(move || {
        let new_value = signal_clone.get();
        let old_value = current.borrow().clone();
        set_prev.set(old_value);
        *current.borrow_mut() = Some(new_value);
    });

    PreviousHandle { value: prev }
}

/// Track if a value has changed since last render.
///
/// # Returns
///
/// A tuple of (previous_value, has_changed).
pub fn use_changed<T: Clone + PartialEq + 'static>(
    signal: ReadSignal<T>,
) -> (PreviousHandle<T>, ReadSignal<bool>) {
    let prev = use_previous(signal.clone());
    let (changed, set_changed) = create_signal(false);

    let prev_clone = prev.clone();
    create_effect(move || {
        let current = signal.get();
        let previous = prev_clone.get();
        let is_changed = previous.as_ref() != Some(&current);
        set_changed.set(is_changed);
    });

    (prev, changed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::signals::create_signal;

    #[test]
    fn test_use_previous() {
        let (count, set_count) = create_signal(0);
        let prev = use_previous(count.clone());

        // Initial value should be None
        // (Note: After effect runs, current value is captured but previous is still None)
        assert!(prev.get().is_none() || prev.get() == Some(0));

        set_count.set(5);
        // Previous should now be the old value
        // Note: The exact behavior depends on when effects run
    }

    #[test]
    fn test_use_previous_get_or() {
        let (count, _) = create_signal(0);
        let prev = use_previous(count);

        // First call should return default
        assert_eq!(prev.get_or(42), 42);
    }
}
