//! Hooks - Reactive state and effects
//!
//! React/SolidJS-inspired hooks for building reactive components:
//! - `use_state`: Local reactive state
//! - `use_effect`: Side effects
//! - `use_memo`: Memoized computations
//! - `use_input`: Keyboard input handling
//! - `use_mouse`: Mouse event handling
//! - `use_focus`: Focus management
//! - `use_app`: Application context

mod state;
mod effects;
mod input;
mod mouse;
mod focus;
mod terminal;
mod hotkeys;

pub use state::{use_state, use_reducer, use_ref, use_lazy_state, use_toggle, use_counter, State};
pub use effects::{use_effect, use_memo, use_callback, use_mount, use_cleanup};
pub use input::{use_input, use_key, dispatch_key_event, clear_input_handlers, key_matches, InputHandler};
pub use mouse::{use_mouse, dispatch_mouse_event, clear_mouse_handlers, use_mouse_position, MouseHandler, MousePosition};
pub use focus::{use_focus, use_focus_manager, FocusOptions, FocusResult};
pub use terminal::{use_terminal_size, use_fps, use_dimensions, UseFpsResult};
pub use hotkeys::{
    use_hotkeys, HotkeyBinding, HotkeyHandler, HotkeyOptions,
    register_hotkey, trigger_hotkey, get_registered_hotkeys,
    get_hotkey_scope, set_hotkey_scope, reset_hotkey_scope,
    parse_hotkeys, matches_hotkey, format_hotkey,
    format_hotkey_platform, is_mac,
};

// Re-export from core
pub use crate::core::hotkeys::parse_hotkey;

use crate::core::app::AppContext;

/// Get the application context.
pub fn use_app() -> AppContext {
    // In a real implementation, this would come from a context provider
    AppContext {
        width: 80,
        height: 24,
    }
}
