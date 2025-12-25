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
mod text;
mod spacer;
mod fragment;
mod divider;
mod canvas;
mod control_flow;

pub use box_component::BoxComponent;
pub use text::Text;
pub use spacer::{Spacer, Newline};
pub use fragment::Fragment;
pub use divider::Divider;
pub use canvas::Canvas;
pub use control_flow::{When, Each, Transform, Static, Slot};
