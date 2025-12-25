//! Utilities
//!
//! Helper functions and types for terminal manipulation.

pub mod ansi;
pub mod text;
pub mod cursor;
pub mod border;

pub use ansi::{strip_ansi, colorize, style, Color, Style};
pub use text::{measure_text, visible_width, wrap_text, truncate_text, slice_ansi};
pub use cursor::{show_cursor, hide_cursor, move_cursor, save_cursor, restore_cursor};
pub use border::{BorderStyle, BorderChars, BORDER_STYLES};
