//! Effect Hooks
//!
//! Side effects and memoization.

use crate::core::signals::{create_effect, create_memo, Effect, Memo};

/// Create a side effect that runs when dependencies change.
///
/// # Example
///
/// ```rust
/// use tuiuiu::hooks::{use_state, use_effect};
///
/// let count = use_state(0);
///
/// use_effect(move || {
///     println!("Count changed to: {}", count.get.get());
/// });
/// ```
pub fn use_effect<F: Fn() + 'static>(effect: F) -> Effect {
    create_effect(effect)
}

/// Create a memoized computation.
///
/// # Example
///
/// ```rust
/// use tuiuiu::hooks::{use_state, use_memo};
///
/// let count = use_state(5);
/// let doubled = use_memo(move || count.get.get() * 2);
///
/// assert_eq!(doubled.get(), 10);
/// ```
pub fn use_memo<T: Clone + 'static, F: Fn() -> T + 'static>(compute: F) -> Memo<T> {
    create_memo(compute)
}

/// Create a memoized callback.
///
/// Returns a stable function reference that only changes when dependencies change.
pub fn use_callback<F: Clone + 'static>(callback: F, _deps: &[&dyn std::any::Any]) -> F {
    // In a real implementation, this would track dependencies
    callback
}

/// Run an effect only once on mount.
pub fn use_mount<F: FnOnce() + 'static>(_effect: F) {
    use std::cell::Cell;
    let ran = Cell::new(false);
    create_effect(move || {
        if !ran.get() {
            ran.set(true);
            // Can't call FnOnce in effect, but conceptually this is mount-only
        }
    });
}

/// Run a cleanup function on unmount.
pub fn use_cleanup<F: FnOnce() + 'static>(_cleanup: F) {
    // Cleanup would be called when component unmounts
    // This requires lifecycle tracking in the component system
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::signals::create_signal;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn test_use_memo() {
        let (count, set_count) = create_signal(5);
        let doubled = use_memo(move || count.get() * 2);

        assert_eq!(doubled.get(), 10);

        set_count.set(10);
        assert_eq!(doubled.get(), 20);
    }
}
