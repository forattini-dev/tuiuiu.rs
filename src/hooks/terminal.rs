//! Terminal Hooks
//!
//! Terminal-related hooks for size detection and dimensions.
//!
//! For FPS tracking, see the dedicated `fps` module.

use crate::core::signals::{create_signal, ReadSignal};
use crate::core::terminal::get_terminal_size;

/// Get terminal size as reactive signals.
///
/// Returns a tuple of (width, height) signals that can be used reactively.
/// In a full implementation, these would update on terminal resize (SIGWINCH).
///
/// # Example
///
/// ```rust,ignore
/// let (width, height) = use_terminal_size();
/// println!("Terminal: {}x{}", width.get(), height.get());
/// ```
pub fn use_terminal_size() -> (ReadSignal<u16>, ReadSignal<u16>) {
    let (width, height) = get_terminal_size().unwrap_or((80, 24));
    let (w_signal, _) = create_signal(width);
    let (h_signal, _) = create_signal(height);

    // In a real implementation, this would listen for SIGWINCH

    (w_signal, h_signal)
}

/// Get terminal dimensions as a simple tuple.
///
/// Non-reactive version that returns current dimensions.
///
/// # Returns
///
/// Tuple of (width, height) in columns/rows.
pub fn use_dimensions() -> (u16, u16) {
    get_terminal_size().unwrap_or((80, 24))
}
