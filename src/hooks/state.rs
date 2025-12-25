//! State Hooks
//!
//! Local reactive state management.

use std::rc::Rc;
use crate::core::signals::{create_signal, create_reducer, ReadSignal, WriteSignal};

/// State handle returned by use_state.
#[derive(Clone)]
pub struct State<T: Clone> {
    /// Read the current value
    pub get: ReadSignal<T>,
    /// Update the value
    pub set: WriteSignal<T>,
}

impl<T: Clone> State<T> {
    /// Get the current value.
    pub fn value(&self) -> T {
        self.get.get()
    }
}

impl<T: Clone + std::fmt::Debug> std::fmt::Debug for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("value", &self.get.get_untracked())
            .finish()
    }
}

/// Create local reactive state.
///
/// Returns a `State<T>` handle with `get` and `set` methods.
///
/// # Example
///
/// ```rust
/// use tuiuiu::hooks::use_state;
///
/// let count = use_state(0);
///
/// // Read the value
/// let current = count.get.get();
///
/// // Update the value
/// count.set.set(current + 1);
///
/// // Or use update for functional updates
/// count.set.update(|c| *c += 1);
/// ```
pub fn use_state<T: Clone + 'static>(initial: T) -> State<T> {
    let (get, set) = create_signal(initial);
    State { get, set }
}

/// Create state with a reducer function.
///
/// # Example
///
/// ```rust
/// use tuiuiu::hooks::use_reducer;
///
/// #[derive(Clone)]
/// enum Action {
///     Increment,
///     Decrement,
///     Reset,
/// }
///
/// let (state, dispatch) = use_reducer(
///     |state: &i32, action: Action| match action {
///         Action::Increment => state + 1,
///         Action::Decrement => state - 1,
///         Action::Reset => 0,
///     },
///     0,
/// );
///
/// dispatch(Action::Increment);
/// ```
pub fn use_reducer<S: Clone + 'static, A: 'static>(
    reducer: impl Fn(&S, A) -> S + 'static,
    initial: S,
) -> (ReadSignal<S>, impl Fn(A) + Clone) {
    create_reducer(reducer, initial)
}

/// Create a ref (state that doesn't trigger re-renders when read).
pub fn use_ref<T: Clone + 'static>(initial: T) -> State<T> {
    use_state(initial)
}

/// Create state initialized lazily.
pub fn use_lazy_state<T: Clone + 'static, F: FnOnce() -> T>(init: F) -> State<T> {
    use_state(init())
}

/// Create boolean toggle state.
pub fn use_toggle(initial: bool) -> (ReadSignal<bool>, impl Fn() + Clone) {
    let (state, set_state) = create_signal(initial);

    let toggle = move || {
        set_state.update(|v| *v = !*v);
    };

    (state, toggle)
}

/// Create counter state with increment/decrement.
pub fn use_counter(initial: i32) -> (ReadSignal<i32>, impl Fn() + Clone, impl Fn() + Clone) {
    let (state, set_state) = create_signal(initial);

    let inc = {
        let set_state = set_state.clone();
        move || set_state.update(|v| *v += 1)
    };

    let dec = move || set_state.update(|v| *v -= 1);

    (state, inc, dec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_state() {
        let state = use_state(42);
        assert_eq!(state.get.get(), 42);

        state.set.set(100);
        assert_eq!(state.get.get(), 100);
    }

    #[test]
    fn test_use_toggle() {
        let (state, toggle) = use_toggle(false);
        assert!(!state.get());

        toggle();
        assert!(state.get());

        toggle();
        assert!(!state.get());
    }

    #[test]
    fn test_use_counter() {
        let (count, inc, dec) = use_counter(0);
        assert_eq!(count.get(), 0);

        inc();
        assert_eq!(count.get(), 1);

        dec();
        assert_eq!(count.get(), 0);
    }
}
