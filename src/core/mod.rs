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

pub mod animation;
pub mod app;
pub mod capabilities;
pub mod command_palette;
pub mod component;
pub mod event;
pub mod focus;
pub mod graphics;
pub mod hotkeys;
pub mod key_bindings;
pub mod layout;
pub mod renderer;
pub mod screen;
pub mod signals;
pub mod terminal;
pub mod tick;
pub mod virtual_scroll;

// Re-exports for convenience
pub use app::*;
pub use command_palette::*;
pub use component::*;
pub use event::*;
pub use focus::*;
pub use layout::*;
pub use renderer::*;
pub use signals::*;
pub use terminal::*;
pub use tick::*;
