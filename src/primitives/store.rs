//! Reactive Store System
//!
//! This module provides a lightweight Redux-inspired state container built on top
//! of Tuiuiu signals.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::core::signals::{create_signal, ReadSignal, WriteSignal};

/// Shared action dispatch function.
pub type Dispatch<A> = Rc<dyn Fn(A) -> A>;

/// Store reducer function.
pub type Reducer<S, A> = Rc<dyn Fn(&S, A) -> S>;

/// Store constructor used internally and by middleware enhancers.
pub type StoreCreator<S, A> = Rc<dyn Fn(Reducer<S, A>, Option<S>) -> Store<S, A>>;

/// Enhancer type for composing middleware around a store creator.
pub type StoreEnhancer<S, A> = Rc<dyn Fn(StoreCreator<S, A>) -> StoreCreator<S, A>>;

/// Middleware type.
pub type Middleware<S, A> = Rc<dyn Fn(&MiddlewareAPI<S, A>, Dispatch<A>) -> Dispatch<A>>;

/// Action type alias used by `create_store`.
pub type AnyAction = String;

/// Store action used by middleware and dispatch examples.
#[derive(Clone)]
pub struct Action<T = AnyAction> {
    /// Action type identifier.
    pub type_: T,
    /// Optional action payload.
    pub payload: Option<String>,
}

impl<T: Default> Default for Action<T> {
    fn default() -> Self {
        Self {
            type_: T::default(),
            payload: None,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Action<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Action")
            .field("type", &self.type_)
            .field("payload", &self.payload)
            .finish()
    }
}

/// API passed to middleware when composing dispatch.
#[derive(Clone)]
pub struct MiddlewareAPI<S, A>
where
    S: Clone + 'static,
    A: Clone + 'static,
{
    /// Current middleware dispatch.
    pub dispatch: Dispatch<A>,
    /// Read-only accessor for store state.
    pub get_state: Rc<dyn Fn() -> S>,
}

/// A synchronous sync store implementation.
#[derive(Clone)]
pub struct Store<S, A>
where
    S: Clone + 'static,
    A: Clone + 'static,
{
    state: ReadSignal<S>,
    reducer: Rc<RefCell<Reducer<S, A>>>,
    listeners: Rc<RefCell<HashMap<usize, Rc<dyn Fn()>>>>,
    next_listener_id: Rc<RefCell<usize>>,
    dispatching: Rc<Cell<bool>>,
    dispatch: RefCell<Dispatch<A>>,
}

/// Handle returned by [`Store::subscribe`]. Unsubscribe on drop.
pub struct StoreSubscription<S, A>
where
    S: Clone + 'static,
    A: Clone + 'static,
{
    listeners: Rc<RefCell<HashMap<usize, Rc<dyn Fn()>>>>,
    id: usize,
    _state: std::marker::PhantomData<(S, A)>,
}

impl<S, A> StoreSubscription<S, A>
where
    S: Clone + 'static,
    A: Clone + 'static,
{
    /// Unsubscribe this listener immediately.
    pub fn unsubscribe(&self) {
        self.listeners.borrow_mut().remove(&self.id);
    }

    /// Return the subscription id for diagnostics.
    pub fn id(&self) -> usize {
        self.id
    }
}

impl<S, A> Drop for StoreSubscription<S, A>
where
    S: Clone + 'static,
    A: Clone + 'static,
{
    fn drop(&mut self) {
        self.listeners.borrow_mut().remove(&self.id);
    }
}

impl<S, A> Store<S, A>
where
    S: Clone + Default + 'static,
    A: Clone + 'static,
{
    /// Return a signal-compatible state getter.
    pub fn state(&self) -> ReadSignal<S> {
        self.state.clone()
    }

    /// Get the current state without tracking.
    pub fn get_state(&self) -> S {
        if self.dispatching.get() {
            panic!("You may not call store.getState() while the reducer is executing.");
        }

        self.state.get_untracked()
    }

    /// Subscribe to store updates.
    pub fn subscribe<F>(&self, listener: F) -> StoreSubscription<S, A>
    where
        F: Fn() + 'static,
    {
        let mut id = self.next_listener_id.borrow_mut();
        let current_id = *id;
        *id = id.saturating_add(1);
        drop(id);

        self.listeners
            .borrow_mut()
            .insert(current_id, Rc::new(listener));

        StoreSubscription {
            listeners: Rc::clone(&self.listeners),
            id: current_id,
            _state: std::marker::PhantomData,
        }
    }

    /// Replace the reducer and keep current state.
    pub fn replace_reducer(&self, reducer: impl Fn(&S, A) -> S + 'static) {
        *self.reducer.borrow_mut() = Rc::new(reducer);
    }

    /// Dispatch an action through the composed dispatch chain.
    pub fn dispatch(&self, action: A) -> A {
        (self.dispatch.borrow())(action)
    }

    fn set_dispatch(&self, dispatch: Dispatch<A>) {
        *self.dispatch.borrow_mut() = dispatch;
    }
}

impl<S, A> Store<S, A>
where
    S: Clone + Default + 'static,
    A: Clone + 'static,
{
    fn new(reducer: Reducer<S, A>, preloaded_state: Option<S>) -> Self {
        let (state, set_state) = create_signal(preloaded_state.unwrap_or_default());
        let listeners = Rc::new(RefCell::new(HashMap::<usize, Rc<dyn Fn()>>::new()));
        let next_listener_id = Rc::new(RefCell::new(0usize));
        let dispatching = Rc::new(Cell::new(false));
        let reducer_ref = Rc::new(RefCell::new(reducer));
        let listeners_for_dispatch = Rc::clone(&listeners);
        let state_get = state.clone();
        let set_state = set_state.clone();

        let dispatch = Rc::new({
            let state_get = state_get.clone();
            let set_state = set_state.clone();
            let dispatching = Rc::clone(&dispatching);
            let listeners_for_dispatch = Rc::clone(&listeners_for_dispatch);
            let reducer_ref = Rc::clone(&reducer_ref);
            move |action: A| {
                if dispatching.get() {
                    panic!("Reducers may not dispatch actions.");
                }

                dispatching.set(true);
                let next_state = (reducer_ref.borrow())(&state_get.get_untracked(), action.clone());
                set_state.set(next_state);
                dispatching.set(false);

                let listeners = listeners_for_dispatch.borrow();
                listeners.values().for_each(|listener| (listener)());

                action
            }
        });

        Store {
            state,
            reducer: reducer_ref,
            listeners,
            next_listener_id,
            dispatching,
            dispatch: RefCell::new(dispatch),
        }
    }
}

/// Create a Redux-like store.
pub fn create_store<S, A, R>(
    reducer: R,
    preloaded_state: Option<S>,
    enhancer: Option<StoreEnhancer<S, A>>,
) -> Store<S, A>
where
    S: Clone + Default + 'static,
    A: Clone + 'static,
    R: Fn(&S, A) -> S + 'static,
{
    let creator: StoreCreator<S, A> =
        Rc::new(|reducer, preloaded_state| Store::new(reducer, preloaded_state));

    let reducer: Reducer<S, A> = Rc::new(reducer);

    match enhancer {
        Some(enhancer) => (enhancer)(creator)(reducer, preloaded_state),
        None => creator(reducer, preloaded_state),
    }
}

/// Middleware composer utility.
pub fn apply_middleware<S, A>(middlewares: Vec<Middleware<S, A>>) -> StoreEnhancer<S, A>
where
    S: Clone + Default + 'static,
    A: Clone + 'static,
{
    Rc::new(move |creator| {
        let middlewares = middlewares.clone();
        Rc::new(move |reducer, preloaded_state| {
            let store = creator(reducer, preloaded_state);
            let dispatch_cell: Rc<RefCell<Option<Dispatch<A>>>> = Rc::new(RefCell::new(None));

            let dispatch_proxy: Dispatch<A> = {
                let dispatch_cell = Rc::clone(&dispatch_cell);
                Rc::new(move |action| {
                    let dispatch = dispatch_cell
                        .borrow()
                        .as_ref()
                        .unwrap_or_else(|| panic!("Dispatch pipeline was not initialized."))
                        .clone();
                    dispatch(action)
                })
            };

            let api = MiddlewareAPI {
                dispatch: dispatch_proxy.clone(),
                get_state: {
                    let store = store.clone();
                    Rc::new(move || store.get_state())
                },
            };

            let mut dispatch: Dispatch<A> = {
                let store = store.clone();
                Rc::new(move |action| store.dispatch(action))
            };

            for middleware in middlewares.iter().rev() {
                dispatch = middleware(&api, dispatch);
            }

            *dispatch_cell.borrow_mut() = Some(Dispatch::clone(&dispatch));
            store.set_dispatch(dispatch);
            store
        })
    })
}

#[allow(non_snake_case)]
/// JS compatibility alias for `create_store`.
pub fn createStore<S, A, R>(
    reducer: R,
    preloaded_state: Option<S>,
    enhancer: Option<StoreEnhancer<S, A>>,
) -> Store<S, A>
where
    S: Clone + Default + 'static,
    A: Clone + 'static,
    R: Fn(&S, A) -> S + 'static,
{
    create_store(reducer, preloaded_state, enhancer)
}

#[allow(non_snake_case)]
/// JS compatibility alias for `apply_middleware`.
pub fn applyMiddleware<S, A>(middlewares: Vec<Middleware<S, A>>) -> StoreEnhancer<S, A>
where
    S: Clone + Default + 'static,
    A: Clone + 'static,
{
    apply_middleware(middlewares)
}

/// Logger middleware that prints actions and state transitions.
pub fn create_logger_middleware<S, A>() -> Middleware<S, A>
where
    S: Clone + Default + 'static + fmt::Debug,
    A: Clone + 'static + fmt::Debug,
{
    Rc::new(|api, next| {
        let api = api.clone();
        Rc::new(move |action: A| {
            let prev_state = (api.get_state)();
            let result = next(action.clone());
            let next_state = (api.get_state)();
            println!("  dispatching {:?}", action);
            println!("  prev state {:?}", prev_state);
            println!("  next state {:?}", next_state);
            result
        })
    })
}

#[allow(non_snake_case)]
/// JS compatibility alias for `create_logger_middleware`.
pub fn createLoggerMiddleware<S, A>() -> Middleware<S, A>
where
    S: Clone + Default + 'static + fmt::Debug,
    A: Clone + 'static + fmt::Debug,
{
    create_logger_middleware()
}

/// Storage adapter used by persistence helpers.
pub trait SyncStorageAdapter {
    /// Get serialized text stored at the key.
    fn get_item(&self, key: &str) -> Option<String>;
    /// Set serialized text for the key.
    fn set_item(&self, key: &str, value: &str);
}

/// Serialization options for persisted middleware.
pub type PersistSerializer<S> = Rc<dyn Fn(&S) -> String>;
/// Deserialization options for persisted middleware.
pub type PersistDeserializer<S> = Rc<dyn Fn(&str) -> Option<S>>;

/// Middleware-friendly persistence options.
#[derive(Clone)]
pub struct PersistOptions<S> {
    /// Deprecated path option kept for compatibility. Use `storage` and `key`.
    pub path: Option<String>,
    /// Storage key.
    pub key: String,
    /// Deprecated format option. Current implementation always serializes as plain text.
    pub format: Option<String>,
    /// Debounce time, in milliseconds.
    pub debounce: u64,
    /// Optional storage adapter.
    pub storage: Option<Rc<dyn SyncStorageAdapter>>,
    /// Serializer used by middleware.
    pub serializer: PersistSerializer<S>,
}

/// Options for creating a fully hydrated and persisted store.
#[derive(Clone)]
pub struct PersistedStoreOptions<S, A>
where
    S: Clone + 'static,
    A: Clone + 'static,
{
    /// Reducer used by the wrapped store.
    pub reducer: Reducer<S, A>,
    /// Initial state value before persistence hydration/merge.
    pub initial_state: S,
    /// Storage backend.
    pub storage: Rc<dyn SyncStorageAdapter>,
    /// Storage key.
    pub key: String,
    /// Debounce delay in milliseconds.
    pub debounce: u64,
    /// Optional migration step for loaded state.
    pub migrate: Option<Rc<dyn Fn(S) -> S>>,
    /// Optional merge strategy when both initial and persisted state exist.
    pub merge: Option<Rc<dyn Fn(&S, S) -> S>>,
    /// Serializes state into a string for persistence.
    pub serialize: PersistSerializer<S>,
    /// Deserializes persisted state from a string.
    pub deserialize: PersistDeserializer<S>,
}

/// Create persistence middleware.
pub fn create_persist_middleware<S, A>(options: PersistOptions<S>) -> Middleware<S, A>
where
    S: Clone + Default + 'static,
    A: Clone + 'static,
{
    if let Some(path) = &options.path {
        eprintln!(
            "Persist middleware ignores `path`. Use `storage` and `key` instead. path={path}"
        );
    }

    if options.format.is_some() {
        eprintln!("Persist middleware always serializes with user-provided serializer. `format` is deprecated.");
    }

    if options.storage.is_none() {
        eprintln!("Persist middleware created without storage adapter. State will not be saved.");
        return Rc::new(|_api, next| next);
    }

    let key = options.key;
    let storage = options.storage.expect("checked above");
    let serializer = options.serializer;
    let debounce = Duration::from_millis(options.debounce);
    let last_written = Rc::new(RefCell::new(Option::<Instant>::None));

    Rc::new(move |api, next| {
        let storage = Rc::clone(&storage);
        let key = key.clone();
        let serializer = Rc::clone(&serializer);
        let last_written = Rc::clone(&last_written);
        let get_state = Rc::clone(&api.get_state);
        Rc::new(move |action: A| {
            let result = next(action);
            let state = (get_state)();
            let should_write = match *last_written.borrow() {
                Some(last) => debounce.is_zero() || last.elapsed() >= debounce,
                None => true,
            };

            if should_write {
                let snapshot = serializer(&state);
                storage.set_item(&key, &snapshot);
                *last_written.borrow_mut() = Some(Instant::now());
            }

            result
        })
    })
}

#[allow(non_snake_case)]
/// JS compatibility alias for `create_persist_middleware`.
pub fn createPersistMiddleware<S, A>(options: PersistOptions<S>) -> Middleware<S, A>
where
    S: Clone + Default + 'static,
    A: Clone + 'static,
{
    create_persist_middleware(options)
}

/// Creates a store that hydrates persisted state (best-effort) before the first dispatch and
/// persists state after dispatches through `create_persist_middleware`.
pub fn create_persisted_store<S, A>(options: PersistedStoreOptions<S, A>) -> Store<S, A>
where
    S: Clone + Default + 'static,
    A: Clone + 'static,
{
    let PersistedStoreOptions {
        reducer,
        initial_state,
        storage,
        key,
        debounce,
        migrate,
        merge,
        serialize,
        deserialize,
    } = options;

    let hydrated_state = match storage.get_item(&key) {
        Some(raw_state) => match deserialize(&raw_state) {
            Some(persisted_state) => {
                let migrated_state = if let Some(migrate_fn) = migrate {
                    migrate_fn(persisted_state)
                } else {
                    persisted_state
                };
                let merge_fn = merge;
                if let Some(merge) = merge_fn {
                    merge(&initial_state, migrated_state)
                } else {
                    migrated_state
                }
            }
            None => {
                eprintln!("Failed to deserialize persisted state. Falling back to initial state.");
                initial_state.clone()
            }
        },
        None => initial_state.clone(),
    };

    let persist_middleware = create_persist_middleware(PersistOptions {
        path: None,
        key: key.clone(),
        format: None,
        debounce,
        storage: Some(storage.clone()),
        serializer: serialize,
    });

    create_store(
        move |state, action| reducer(state, action),
        Some(hydrated_state),
        Some(apply_middleware(vec![persist_middleware])),
    )
}

#[allow(non_snake_case)]
/// JS compatibility alias for `create_persisted_store`.
pub fn createPersistedStore<S, A>(options: PersistedStoreOptions<S, A>) -> Store<S, A>
where
    S: Clone + Default + 'static,
    A: Clone + 'static,
{
    create_persisted_store(options)
}

/// Reactive store based on a single root signal.
#[derive(Clone)]
pub struct ReactiveStore<T: Clone + 'static> {
    state: ReadSignal<T>,
    set_state: WriteSignal<T>,
}

impl<T: Clone + 'static> ReactiveStore<T> {
    /// Borrow current state reactively.
    pub fn state(&self) -> ReadSignal<T> {
        self.state.clone()
    }

    /// Replace the whole state.
    pub fn set_state(&self, value: T) {
        self.set_state.set(value)
    }

    /// Read current state without tracking.
    pub fn get_state(&self) -> T {
        self.state.get_untracked()
    }

    /// Update current state with a closure.
    pub fn update_state<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        self.set_state.update(f)
    }
}

/// Create a reactive, signal-backed store.
pub fn create_reactive_store<T: Clone + 'static>(initial: T) -> ReactiveStore<T> {
    let (state, set_state) = create_signal(initial);
    ReactiveStore { state, set_state }
}

#[allow(non_snake_case)]
/// JS compatibility alias for `create_reactive_store`.
pub fn createReactiveStore<T: Clone + 'static>(initial: T) -> ReactiveStore<T> {
    create_reactive_store(initial)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    enum TestAction {
        Increment,
        Set(i32),
    }

    impl Copy for TestAction {}

    fn reducer(count: &i32, action: TestAction) -> i32 {
        match action {
            TestAction::Increment => count + 1,
            TestAction::Set(value) => value,
        }
    }

    #[test]
    fn test_create_store_and_dispatch() {
        let store = create_store(reducer, Some(0), None);
        assert_eq!(store.get_state(), 0);
        store.dispatch(TestAction::Increment);
        assert_eq!(store.get_state(), 1);
    }

    #[test]
    fn test_store_subscribe() {
        use std::cell::Cell;
        use std::rc::Rc;
        let calls = Cell::new(0u32);
        let store = create_store(reducer, Some(0), None);
        let calls = Rc::new(calls);
        let _sub = store.subscribe({
            let calls = Rc::clone(&calls);
            move || calls.set(calls.get() + 1)
        });
        store.dispatch(TestAction::Set(3));
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn test_apply_middleware() {
        fn double_incr(count: &i32, _action: TestAction) -> i32 {
            count + 2
        }

        let middleware = Rc::new(
            |_api: &MiddlewareAPI<i32, TestAction>, next: Dispatch<TestAction>| {
                let next = next.clone();
                Rc::new(move |action| match action {
                    TestAction::Increment => next(TestAction::Increment),
                    value => next(value),
                }) as Dispatch<TestAction>
            },
        ) as Middleware<i32, TestAction>;

        let store = create_store(
            double_incr,
            Some(0),
            Some(apply_middleware(vec![middleware])),
        );

        store.dispatch(TestAction::Increment);
        assert_eq!(store.get_state(), 2);
    }

    #[test]
    fn test_reactive_store_updates() {
        let store = create_reactive_store(10);
        assert_eq!(store.get_state(), 10);

        store.set_state(30);
        assert_eq!(store.get_state(), 30);

        store.update_state(|value| *value += 5);
        assert_eq!(store.get_state(), 35);
    }

    #[derive(Clone)]
    struct MemoryStorage {
        inner: Rc<RefCell<HashMap<String, String>>>,
    }

    impl MemoryStorage {
        fn new() -> Self {
            Self {
                inner: Rc::new(RefCell::new(HashMap::new())),
            }
        }
    }

    impl SyncStorageAdapter for MemoryStorage {
        fn get_item(&self, key: &str) -> Option<String> {
            self.inner.borrow().get(key).cloned()
        }

        fn set_item(&self, key: &str, value: &str) {
            self.inner
                .borrow_mut()
                .insert(key.to_owned(), value.to_owned());
        }
    }

    #[test]
    fn test_persisted_store_roundtrip() {
        let storage: Rc<dyn SyncStorageAdapter> = Rc::new(MemoryStorage::new());
        let initial = 10;

        let store = create_persisted_store(PersistedStoreOptions {
            reducer: Rc::new(|state: &i32, action: TestAction| match action {
                TestAction::Increment => state + 1,
                TestAction::Set(value) => value,
            }),
            initial_state: initial,
            storage: Rc::clone(&storage),
            key: "root".to_owned(),
            debounce: 0,
            migrate: None,
            merge: None,
            serialize: Rc::new(|value| value.to_string()),
            deserialize: Rc::new(|serialized| serialized.parse::<i32>().ok()),
        });

        store.dispatch(TestAction::Set(15));
        assert_eq!(storage.get_item("root"), Some("15".to_owned()));

        let _rehydrated = create_persisted_store(PersistedStoreOptions {
            reducer: Rc::new(|state: &i32, action: TestAction| match action {
                TestAction::Increment => state + 1,
                TestAction::Set(value) => value,
            }),
            initial_state: initial,
            storage,
            key: "root".to_owned(),
            debounce: 0,
            migrate: None,
            merge: None,
            serialize: Rc::new(|value| value.to_string()),
            deserialize: Rc::new(|serialized| serialized.parse::<i32>().ok()),
        });

        assert_eq!(store.get_state(), 15);
    }
}
