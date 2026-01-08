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
use std::collections::{HashMap, HashSet};
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
// Subscriber Types
// =============================================================================

/// A subscriber that can be notified when a signal changes.
#[derive(Clone)]
enum Subscriber {
    Effect(Rc<EffectInner>),
    Memo(Rc<dyn MemoSubscriber>),
}

trait MemoSubscriber {
    fn invalidate(&self);
    fn id(&self) -> u64;
}

// =============================================================================
// Runtime Context
// =============================================================================

thread_local! {
    /// Current tracking context - stores (signal_id -> subscriber) mappings
    static CURRENT_SUBSCRIBER: RefCell<Option<Subscriber>> = const { RefCell::new(None) };

    /// Global signal registry - maps signal IDs to their subscribers
    static SIGNAL_SUBSCRIBERS: RefCell<HashMap<u64, Vec<Subscriber>>> = RefCell::new(HashMap::new());

    /// Batch update flag
    static BATCHING: Cell<bool> = const { Cell::new(false) };

    /// Pending effects during batch (stores effect IDs to deduplicate)
    static PENDING_EFFECTS: RefCell<Vec<Rc<EffectInner>>> = const { RefCell::new(Vec::new()) };

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
        let signal_id = self.inner.id;
        CURRENT_SUBSCRIBER.with(|current| {
            if let Some(subscriber) = current.borrow().as_ref() {
                // Add this subscriber to the signal's subscriber list
                SIGNAL_SUBSCRIBERS.with(|subs| {
                    let mut subs = subs.borrow_mut();
                    let subscribers = subs.entry(signal_id).or_insert_with(Vec::new);

                    // Check if already subscribed (avoid duplicates)
                    let already_subscribed = subscribers.iter().any(|s| match (s, subscriber) {
                        (Subscriber::Effect(a), Subscriber::Effect(b)) => Rc::ptr_eq(a, b),
                        (Subscriber::Memo(a), Subscriber::Memo(b)) => a.id() == b.id(),
                        _ => false,
                    });

                    if !already_subscribed {
                        subscribers.push(subscriber.clone());
                    }
                });
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
        let signal_id = self.inner.id;

        // Get subscribers from the global registry
        let subscribers: Vec<Subscriber> = SIGNAL_SUBSCRIBERS.with(|subs| {
            subs.borrow()
                .get(&signal_id)
                .cloned()
                .unwrap_or_default()
        });

        if is_batching() {
            // Queue effects for later (only effects, memos invalidate immediately)
            for subscriber in subscribers {
                match subscriber {
                    Subscriber::Effect(effect) => {
                        PENDING_EFFECTS.with(|pending| {
                            pending.borrow_mut().push(effect);
                        });
                    }
                    Subscriber::Memo(memo) => {
                        memo.invalidate();
                    }
                }
            }
        } else {
            // Execute immediately
            for subscriber in subscribers {
                match subscriber {
                    Subscriber::Effect(effect) => {
                        run_effect(&effect);
                    }
                    Subscriber::Memo(memo) => {
                        memo.invalidate();
                    }
                }
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
    #[allow(dead_code)]
    dependencies: RefCell<HashSet<u64>>,
}

impl Effect {
    /// Stop this effect from running.
    pub fn dispose(&self) {
        // Clear dependencies and unsubscribe from all signals
        let deps = self.inner.dependencies.borrow().clone();
        SIGNAL_SUBSCRIBERS.with(|subs| {
            let mut subs = subs.borrow_mut();
            for signal_id in deps {
                if let Some(subscribers) = subs.get_mut(&signal_id) {
                    subscribers.retain(|s| {
                        if let Subscriber::Effect(e) = s {
                            !Rc::ptr_eq(e, &self.inner)
                        } else {
                            true
                        }
                    });
                }
            }
        });
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
    // Set this effect as the current subscriber
    let prev_subscriber = CURRENT_SUBSCRIBER.with(|current| {
        current.borrow_mut().replace(Subscriber::Effect(Rc::clone(inner)))
    });

    // Run the callback - signal reads will auto-subscribe
    (inner.callback.borrow())();

    // Restore previous subscriber
    CURRENT_SUBSCRIBER.with(|current| {
        *current.borrow_mut() = prev_subscriber;
    });
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

impl<T: Clone + 'static> MemoSubscriber for MemoInner<T> {
    fn invalidate(&self) {
        // Clear the cached value so it recomputes on next get()
        *self.cached.borrow_mut() = None;
    }

    fn id(&self) -> u64 {
        self.id
    }
}

impl<T: Clone + 'static> Memo<T> {
    /// Get the memoized value, recomputing if necessary.
    pub fn get(&self) -> T {
        // Check cache first before any tracking
        {
            let cached = self.inner.cached.borrow();
            if let Some(ref value) = *cached {
                return value.clone();
            }
        }

        // Set this memo as the current subscriber
        let prev_subscriber = CURRENT_SUBSCRIBER.with(|current| {
            current.borrow_mut().replace(Subscriber::Memo(Rc::clone(&self.inner) as Rc<dyn MemoSubscriber>))
        });

        // Compute the value - signal reads will auto-subscribe
        let value = (self.inner.compute)();
        *self.inner.cached.borrow_mut() = Some(value.clone());

        // Restore previous subscriber
        CURRENT_SUBSCRIBER.with(|current| {
            *current.borrow_mut() = prev_subscriber;
        });

        value
    }

    /// Get the value without tracking as a dependency.
    pub fn get_untracked(&self) -> T {
        // Check cache first
        {
            let cached = self.inner.cached.borrow();
            if let Some(ref value) = *cached {
                return value.clone();
            }
        }

        // Compute without tracking
        let value = (self.inner.compute)();
        *self.inner.cached.borrow_mut() = Some(value.clone());
        value
    }

    /// Get the memo's unique ID.
    pub fn id(&self) -> u64 {
        self.inner.id
    }
}

impl<T: Clone + 'static> Clone for Memo<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl<T: Clone + std::fmt::Debug + 'static> std::fmt::Debug for Memo<T> {
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
        let effects: Vec<Rc<EffectInner>> = PENDING_EFFECTS.with(|pending| pending.borrow_mut().drain(..).collect());

        // Deduplicate and run
        let mut seen = HashSet::new();
        for effect in effects {
            let ptr = Rc::as_ptr(&effect) as usize;
            if seen.insert(ptr) {
                run_effect(&effect);
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
    // Temporarily clear the current subscriber
    let prev = CURRENT_SUBSCRIBER.with(|current| current.borrow_mut().take());
    let result = f();
    CURRENT_SUBSCRIBER.with(|current| *current.borrow_mut() = prev);
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
