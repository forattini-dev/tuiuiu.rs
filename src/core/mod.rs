//! Core module - The heart of Tuiuiu
//!
//! This module contains all the foundational systems:
//! - **Signals**: Fine-grained reactive primitives
//! - **Layout**: Flexbox-based layout engine
//! - **Renderer**: Terminal output rendering
//! - **Terminal**: Raw terminal I/O handling
//! - **App**: Application lifecycle management
//! - **Event**: Event system and propagation
//! - **Focus**: Focus management and navigation
//! - **Tick**: Global animation tick system
//! - **Component**: Component trait and types

pub mod signals;
pub mod layout;
pub mod renderer;
pub mod terminal;
pub mod app;
pub mod event;
pub mod focus;
pub mod tick;
pub mod component;
pub mod hotkeys;
pub mod animation;
pub mod capabilities;
pub mod graphics;
pub mod virtual_scroll;
pub mod key_bindings;
pub mod command_palette;
pub mod screen;

// Re-exports for convenience
pub use signals::*;
pub use layout::*;
pub use renderer::*;
pub use terminal::*;
pub use app::*;
pub use event::*;
pub use focus::*;
pub use tick::*;
pub use component::*;
