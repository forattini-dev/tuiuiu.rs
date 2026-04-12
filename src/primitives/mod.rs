//! Primitives - Basic building blocks
//!
//! The fundamental components for building UIs:
//! - `Box`: Container with flexbox layout
//! - `Text`: Text content with styling
//! - `Spacer`: Empty space
//! - `Newline`: Line break
//! - `Fragment`: Group without wrapper
//! - `Divider`: Horizontal/vertical line
//! - `Canvas`: Low-level drawing

mod box_component;
mod canvas;
mod control_flow;
mod divider;
mod fragment;
mod spacer;
mod store;
mod text;

pub use box_component::{box_, column, row, BoxComponent};
pub use canvas::{canvas, Canvas};
pub use control_flow::{each, when, Each, Slot, Static, Transform, When};
pub use divider::{divider, vdivider, Divider};
pub use fragment::{fragment, Fragment};
pub use spacer::{newline, spacer, Newline, Spacer};
pub use store::{
    applyMiddleware, apply_middleware, createLoggerMiddleware, createPersistMiddleware,
    createPersistedStore, createReactiveStore, createStore, create_logger_middleware,
    create_persist_middleware, create_persisted_store, create_reactive_store, create_store, Action,
    AnyAction, Dispatch, Middleware, MiddlewareAPI, PersistDeserializer, PersistOptions,
    PersistSerializer, PersistedStoreOptions, ReactiveStore, Reducer, Store, StoreCreator,
    StoreEnhancer, SyncStorageAdapter,
};
pub use text::{text, Text};
