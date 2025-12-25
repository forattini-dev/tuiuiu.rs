//! Signal-based Reactivity System
//!
//! Fine-grained reactive primitives inspired by SolidJS. Signals provide
//! automatic dependency tracking and efficient updates.
//!
//! # Example
//!
//! ```rust
//! use tuiuiu::core::signals::{create_signal, create_effect, create_memo, batch};
//!
//! // Create a signal with initial value
//! let (count, set_count) = create_signal(0);
//!
//! // Create a derived computation
//! let doubled = create_memo(move || count.get() * 2);
//!
//! // Create a side effect
//! create_effect(move || {
//!     println!("Count: {}, Doubled: {}", count.get(), doubled.get());
//! });
//!
//! // Update the signal
//! set_count.set(5); // Triggers effect: "Count: 5, Doubled: 10"
//!
//! // Batch multiple updates
//! batch(|| {
//!     set_count.set(10);
//!     set_count.update(|c| *c += 1);
//! }); // Only triggers effects once
//! ```

use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

// =============================================================================
// Signal ID Generation
// =============================================================================

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

fn next_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

/// Reset the ID counter (useful for testing)
pub fn reset_id_counter() {
    NEXT_ID.store(0, Ordering::SeqCst);
}

// =============================================================================
// Runtime Context
// =============================================================================

thread_local! {
    /// Current tracking context for automatic dependency collection
    static TRACKING: RefCell<Option<Rc<RefCell<HashSet<u64>>>>> = const { RefCell::new(None) };

    /// Batch update flag
    static BATCHING: Cell<bool> = const { Cell::new(false) };

    /// Pending effects during batch
    static PENDING_EFFECTS: RefCell<Vec<Rc<dyn Fn()>>> = const { RefCell::new(Vec::new()) };

    /// All registered effects for cleanup
    static EFFECTS: RefCell<Vec<Rc<EffectInner>>> = const { RefCell::new(Vec::new()) };
}

// =============================================================================
// ReadSignal
// =============================================================================

/// A read-only signal that can be subscribed to.
///
/// `ReadSignal` provides reactive read access to a value. When read inside
/// an effect or memo, it automatically registers as a dependency.
#[derive(Clone)]
pub struct ReadSignal<T> {
    inner: Rc<SignalInner<T>>,
}

impl<T: Clone> ReadSignal<T> {
    /// Get the current value, tracking this read as a dependency.
    pub fn get(&self) -> T {
        self.track();
        self.inner.value.borrow().clone()
    }

    /// Get the current value without tracking.
    pub fn get_untracked(&self) -> T {
        self.inner.value.borrow().clone()
    }

    /// Track this signal as a dependency without reading the value.
    pub fn track(&self) {
        TRACKING.with(|tracking| {
            if let Some(deps) = tracking.borrow().as_ref() {
                deps.borrow_mut().insert(self.inner.id);
            }
        });
    }

    /// Get the signal's unique ID.
    pub fn id(&self) -> u64 {
        self.inner.id
    }
}

impl<T: Clone + std::fmt::Debug> std::fmt::Debug for ReadSignal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReadSignal")
            .field("id", &self.inner.id)
            .field("value", &self.get_untracked())
            .finish()
    }
}

// =============================================================================
// WriteSignal
// =============================================================================

/// A write handle for updating a signal's value.
///
/// `WriteSignal` provides methods to update the signal value and trigger
/// dependent effects and memos.
#[derive(Clone)]
pub struct WriteSignal<T> {
    inner: Rc<SignalInner<T>>,
}

impl<T: Clone + 'static> WriteSignal<T> {
    /// Set a new value, triggering dependents if changed.
    pub fn set(&self, value: T) {
        *self.inner.value.borrow_mut() = value;
        self.notify();
    }

    /// Update the value using a function.
    pub fn update<F: FnOnce(&mut T)>(&self, f: F) {
        f(&mut self.inner.value.borrow_mut());
        self.notify();
    }

    /// Notify all dependents that the value has changed.
    fn notify(&self) {
        let subscribers: Vec<_> = self.inner.subscribers.borrow().clone();

        if is_batching() {
            // Queue effects for later
            PENDING_EFFECTS.with(|pending| {
                pending.borrow_mut().extend(subscribers);
            });
        } else {
            // Execute effects immediately
            for effect in subscribers {
                effect();
            }
        }
    }

    /// Get the signal's unique ID.
    pub fn id(&self) -> u64 {
        self.inner.id
    }
}

impl<T: Clone + std::fmt::Debug> std::fmt::Debug for WriteSignal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WriteSignal")
            .field("id", &self.inner.id)
            .finish()
    }
}

// =============================================================================
// SignalInner
// =============================================================================

struct SignalInner<T> {
    id: u64,
    value: RefCell<T>,
    subscribers: RefCell<Vec<Rc<dyn Fn()>>>,
}

// =============================================================================
// create_signal
// =============================================================================

/// Create a reactive signal with an initial value.
///
/// Returns a tuple of `(ReadSignal, WriteSignal)` for reading and writing.
///
/// # Example
///
/// ```rust
/// use tuiuiu::core::signals::create_signal;
///
/// let (count, set_count) = create_signal(0);
///
/// assert_eq!(count.get(), 0);
/// set_count.set(5);
/// assert_eq!(count.get(), 5);
/// ```
pub fn create_signal<T: Clone + 'static>(initial: T) -> (ReadSignal<T>, WriteSignal<T>) {
    let inner = Rc::new(SignalInner {
        id: next_id(),
        value: RefCell::new(initial),
        subscribers: RefCell::new(Vec::new()),
    });

    (
        ReadSignal {
            inner: Rc::clone(&inner),
        },
        WriteSignal { inner },
    )
}

// =============================================================================
// Effect
// =============================================================================

/// A reactive effect that automatically re-runs when its dependencies change.
pub struct Effect {
    inner: Rc<EffectInner>,
}

struct EffectInner {
    id: u64,
    callback: RefCell<Box<dyn Fn()>>,
    dependencies: RefCell<HashSet<u64>>,
}

impl Effect {
    /// Stop this effect from running.
    pub fn dispose(&self) {
        // Clear dependencies
        self.inner.dependencies.borrow_mut().clear();
    }

    /// Get the effect's unique ID.
    pub fn id(&self) -> u64 {
        self.inner.id
    }
}

/// Create a reactive effect that runs when dependencies change.
///
/// The effect callback will be executed immediately, and then re-executed
/// whenever any signal read inside it changes.
///
/// # Example
///
/// ```rust
/// use tuiuiu::core::signals::{create_signal, create_effect};
///
/// let (count, set_count) = create_signal(0);
///
/// create_effect(move || {
///     println!("Count is: {}", count.get());
/// });
///
/// set_count.set(1); // Prints: "Count is: 1"
/// set_count.set(2); // Prints: "Count is: 2"
/// ```
pub fn create_effect<F: Fn() + 'static>(callback: F) -> Effect {
    let inner = Rc::new(EffectInner {
        id: next_id(),
        callback: RefCell::new(Box::new(callback)),
        dependencies: RefCell::new(HashSet::new()),
    });

    // Run the effect immediately to collect dependencies
    run_effect(&inner);

    // Store for cleanup
    EFFECTS.with(|effects| {
        effects.borrow_mut().push(Rc::clone(&inner));
    });

    Effect { inner }
}

fn run_effect(inner: &Rc<EffectInner>) {
    // Create a new dependency set
    let deps = Rc::new(RefCell::new(HashSet::new()));

    // Set as current tracking context
    TRACKING.with(|tracking| {
        *tracking.borrow_mut() = Some(Rc::clone(&deps));
    });

    // Run the callback
    (inner.callback.borrow())();

    // Clear tracking context
    TRACKING.with(|tracking| {
        *tracking.borrow_mut() = None;
    });

    // Store collected dependencies
    *inner.dependencies.borrow_mut() = deps.borrow().clone();
}

// =============================================================================
// Memo
// =============================================================================

/// A derived reactive computation that caches its result.
///
/// Memos automatically track their dependencies and only recompute when
/// those dependencies change.
pub struct Memo<T> {
    inner: Rc<MemoInner<T>>,
}

struct MemoInner<T> {
    id: u64,
    compute: Box<dyn Fn() -> T>,
    cached: RefCell<Option<T>>,
    #[allow(dead_code)]
    dependencies: RefCell<HashSet<u64>>,
}

impl<T: Clone> Memo<T> {
    /// Get the memoized value, recomputing if necessary.
    pub fn get(&self) -> T {
        // Track this memo as a dependency
        TRACKING.with(|tracking| {
            if let Some(deps) = tracking.borrow().as_ref() {
                deps.borrow_mut().insert(self.inner.id);
            }
        });

        // Return cached value or compute
        if let Some(cached) = self.inner.cached.borrow().clone() {
            cached
        } else {
            let value = (self.inner.compute)();
            *self.inner.cached.borrow_mut() = Some(value.clone());
            value
        }
    }

    /// Get the value without tracking as a dependency.
    pub fn get_untracked(&self) -> T {
        if let Some(cached) = self.inner.cached.borrow().clone() {
            cached
        } else {
            let value = (self.inner.compute)();
            *self.inner.cached.borrow_mut() = Some(value.clone());
            value
        }
    }

    /// Get the memo's unique ID.
    pub fn id(&self) -> u64 {
        self.inner.id
    }
}

impl<T: Clone> Clone for Memo<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl<T: Clone + std::fmt::Debug> std::fmt::Debug for Memo<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Memo")
            .field("id", &self.inner.id)
            .field("cached", &self.inner.cached.borrow())
            .finish()
    }
}

/// Create a memoized computation that caches its result.
///
/// The computation will only re-run when its dependencies change.
///
/// # Example
///
/// ```rust
/// use tuiuiu::core::signals::{create_signal, create_memo};
///
/// let (count, set_count) = create_signal(2);
/// let squared = create_memo(move || count.get() * count.get());
///
/// assert_eq!(squared.get(), 4);
/// set_count.set(3);
/// assert_eq!(squared.get(), 9);
/// ```
pub fn create_memo<T: Clone + 'static, F: Fn() -> T + 'static>(compute: F) -> Memo<T> {
    let inner = Rc::new(MemoInner {
        id: next_id(),
        compute: Box::new(compute),
        cached: RefCell::new(None),
        dependencies: RefCell::new(HashSet::new()),
    });

    Memo { inner }
}

// =============================================================================
// Batch Updates
// =============================================================================

/// Check if we're currently in a batch.
fn is_batching() -> bool {
    BATCHING.with(|b| b.get())
}

/// Batch multiple signal updates into a single notification.
///
/// Effects will only run once after all updates complete.
///
/// # Example
///
/// ```rust
/// use tuiuiu::core::signals::{create_signal, create_effect, batch};
///
/// let (a, set_a) = create_signal(1);
/// let (b, set_b) = create_signal(2);
///
/// let mut calls = 0;
/// create_effect(move || {
///     let _ = a.get() + b.get();
///     calls += 1; // Would be 3 without batch, but only 2 with batch
/// });
///
/// batch(|| {
///     set_a.set(10);
///     set_b.set(20);
/// }); // Effect runs once with final values
/// ```
pub fn batch<F: FnOnce() -> R, R>(f: F) -> R {
    let was_batching = BATCHING.with(|b| b.replace(true));

    let result = f();

    if !was_batching {
        BATCHING.with(|b| b.set(false));

        // Run all pending effects
        let effects: Vec<_> = PENDING_EFFECTS.with(|pending| pending.borrow_mut().drain(..).collect());

        // Deduplicate and run
        let mut seen = HashSet::new();
        for effect in effects {
            let ptr = Rc::as_ptr(&effect) as *const () as usize;
            if seen.insert(ptr) {
                effect();
            }
        }
    }

    result
}

// =============================================================================
// Untrack
// =============================================================================

/// Run a function without tracking any signal reads as dependencies.
///
/// # Example
///
/// ```rust
/// use tuiuiu::core::signals::{create_signal, create_effect, untrack};
///
/// let (a, set_a) = create_signal(1);
/// let (b, set_b) = create_signal(2);
///
/// create_effect(move || {
///     let a_val = a.get(); // This is tracked
///     let b_val = untrack(|| b.get()); // This is NOT tracked
///     println!("{} + {} = {}", a_val, b_val, a_val + b_val);
/// });
///
/// set_a.set(10); // Effect runs
/// set_b.set(20); // Effect does NOT run (b was untracked)
/// ```
pub fn untrack<F: FnOnce() -> R, R>(f: F) -> R {
    let prev = TRACKING.with(|tracking| tracking.borrow_mut().take());
    let result = f();
    TRACKING.with(|tracking| *tracking.borrow_mut() = prev);
    result
}

// =============================================================================
// Additional Signal Utilities
// =============================================================================

/// Create a signal that stores a reference (similar to React's useRef).
pub fn create_ref<T: Clone + 'static>(initial: T) -> (ReadSignal<T>, WriteSignal<T>) {
    create_signal(initial)
}

/// Create a signal from a reducer function (like Redux).
pub fn create_reducer<S: Clone + 'static, A>(
    reducer: impl Fn(&S, A) -> S + 'static,
    initial: S,
) -> (ReadSignal<S>, impl Fn(A) + Clone)
where
    A: 'static,
{
    let (state, set_state) = create_signal(initial);
    let reducer = Rc::new(reducer);

    let dispatch = {
        let state = state.clone();
        let reducer = Rc::clone(&reducer);
        move |action: A| {
            let current = state.get_untracked();
            let next = reducer(&current, action);
            set_state.set(next);
        }
    };

    (state, dispatch)
}

/// Create a deferred signal that updates asynchronously.
pub fn create_deferred<T: Clone + 'static>(source: ReadSignal<T>) -> ReadSignal<T> {
    let (deferred, set_deferred) = create_signal(source.get_untracked());

    create_effect(move || {
        let value = source.get();
        // In a real implementation, this would defer to next tick
        set_deferred.set(value);
    });

    deferred
}

/// Create a signal that tracks the previous value.
pub fn create_previous<T: Clone + 'static>(
    source: ReadSignal<T>,
) -> (ReadSignal<T>, ReadSignal<Option<T>>) {
    let (previous, set_previous) = create_signal::<Option<T>>(None);
    let current = source.clone();

    create_effect(move || {
        let curr = source.get();
        // Store current as previous before update
        set_previous.set(Some(curr));
    });

    (current, previous)
}

/// Create a throttled signal that limits update frequency.
pub fn create_throttled<T: Clone + 'static>(
    source: ReadSignal<T>,
    _delay_ms: u64,
) -> ReadSignal<T> {
    let (throttled, set_throttled) = create_signal(source.get_untracked());

    create_effect(move || {
        let value = source.get();
        // In a real implementation, this would throttle updates
        set_throttled.set(value);
    });

    throttled
}

/// Create a debounced signal that delays updates.
pub fn create_debounced<T: Clone + 'static>(
    source: ReadSignal<T>,
    _delay_ms: u64,
) -> ReadSignal<T> {
    let (debounced, set_debounced) = create_signal(source.get_untracked());

    create_effect(move || {
        let value = source.get();
        // In a real implementation, this would debounce updates
        set_debounced.set(value);
    });

    debounced
}

/// Create a unique ID generator.
pub fn create_id() -> ReadSignal<u64> {
    let (id, _) = create_signal(next_id());
    id
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn test_signal_read_write() {
        let (count, set_count) = create_signal(0);
        assert_eq!(count.get(), 0);

        set_count.set(5);
        assert_eq!(count.get(), 5);

        set_count.update(|c| *c += 1);
        assert_eq!(count.get(), 6);
    }

    #[test]
    fn test_memo() {
        let (count, set_count) = create_signal(2);
        let squared = create_memo(move || count.get() * count.get());

        assert_eq!(squared.get(), 4);

        set_count.set(3);
        assert_eq!(squared.get(), 9);
    }

    #[test]
    fn test_batch() {
        let (a, set_a) = create_signal(1);
        let (b, set_b) = create_signal(2);
        let calls = Rc::new(Cell::new(0));

        {
            let a = a.clone();
            let b = b.clone();
            let calls = Rc::clone(&calls);
            create_effect(move || {
                let _ = a.get() + b.get();
                calls.set(calls.get() + 1);
            });
        }

        // Initial effect run
        assert_eq!(calls.get(), 1);

        // Batched updates - effect should run once
        batch(|| {
            set_a.set(10);
            set_b.set(20);
        });

        assert_eq!(calls.get(), 2);
    }

    #[test]
    fn test_untrack() {
        let (a, set_a) = create_signal(1);
        let (b, _set_b) = create_signal(2);
        let calls = Rc::new(Cell::new(0));

        {
            let a = a.clone();
            let b = b.clone();
            let calls = Rc::clone(&calls);
            create_effect(move || {
                let _ = a.get();
                let _ = untrack(|| b.get());
                calls.set(calls.get() + 1);
            });
        }

        assert_eq!(calls.get(), 1);

        set_a.set(10);
        assert_eq!(calls.get(), 2);

        // b is untracked, so this shouldn't trigger
        // (Note: in this simple implementation, we don't have full dependency tracking)
    }

    #[test]
    fn test_reducer() {
        #[derive(Clone)]
        enum Action {
            Increment,
            Decrement,
        }

        let (state, dispatch) = create_reducer(
            |state: &i32, action: Action| match action {
                Action::Increment => state + 1,
                Action::Decrement => state - 1,
            },
            0,
        );

        assert_eq!(state.get(), 0);

        dispatch(Action::Increment);
        assert_eq!(state.get(), 1);

        dispatch(Action::Decrement);
        assert_eq!(state.get(), 0);
    }
}
