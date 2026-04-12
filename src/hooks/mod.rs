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
//! - `use_form`: Form state management
//! - `use_interval`: Periodic callbacks
//! - `use_timeout`: Delayed callbacks
//! - `use_debounce`: Debounced callbacks
//! - `use_throttle`: Throttled callbacks
//! - `use_navigation`: Wizard-style step navigation
//! - `use_threshold_color`: Threshold-based color mapping
//! - `use_format_*`: Reactive formatting hooks
//! - `use_fps`: FPS tracking
//! - `use_layout_ref`: Layout measurement
//! - `use_local_mouse`: Component-relative mouse events
//! - `use_clipboard`: Clipboard operations
//! - `use_previous`: Track previous values
//! - `use_animation`: Value animations and transitions

mod animation;
mod clipboard;
mod effects;
mod focus;
mod form;
mod format;
mod fps;
mod hotkeys;
mod input;
mod layout;
mod local_mouse;
mod mouse;
mod navigation;
mod previous;
mod state;
mod terminal;
mod threshold;
mod timing;

pub use animation::{
    lerp, map_range, use_animation, use_fade_in, use_fade_out, use_slide, AnimationHandle,
    AnimationState, Easing,
};
pub use clipboard::{
    clear_clipboard, copy_to_clipboard, read_clipboard, use_clipboard, ClipboardHandle,
};
pub use effects::{use_callback, use_cleanup, use_effect, use_memo, use_mount};
pub use focus::{use_focus, use_focus_manager, FocusOptions, FocusResult};
pub use form::{
    create_form, use_form, validators, FieldValue, FormField, FormHandle, FormState,
    ValidationResult,
};
pub use format::{
    use_format_bytes, use_format_bytes_si, use_format_compact, use_format_currency,
    use_format_delta, use_format_duration, use_format_duration_compact, use_format_number,
    use_format_percent, use_format_relative,
};
pub use fps::{get_fps, get_fps_metrics, reset_fps, track_frame, use_fps, FpsColor, FpsMetrics};
pub use hotkeys::{
    format_hotkey, format_hotkey_platform, get_hotkey_scope, get_registered_hotkeys, is_mac,
    matches_hotkey, parse_hotkeys, register_hotkey, reset_hotkey_scope, set_hotkey_scope,
    trigger_hotkey, use_hotkeys, HotkeyBinding, HotkeyHandler, HotkeyOptions,
};
pub use input::{
    clear_input_handlers, dispatch_key_event, key_matches, use_input, use_key, InputHandler,
};
pub use layout::{create_layout_ref, use_layout_ref, use_layout_ref_with, LayoutRect, LayoutRef};
pub use local_mouse::{
    use_local_mouse, use_local_mouse_with, Bounds, LocalMouseEvent, LocalMouseHandler,
    LocalMouseOptions, Modifiers, MouseAction, MouseButton, RawMouseEvent,
};
pub use mouse::{
    clear_mouse_handlers, dispatch_mouse_event, use_mouse, use_mouse_position, MouseHandler,
    MousePosition,
};
pub use navigation::{
    create_navigation, simple_steps, use_navigation, NavigationResult, NavigationState,
    NavigationStep,
};
pub use previous::{use_changed, use_previous, PreviousHandle};
pub use state::{use_counter, use_lazy_state, use_reducer, use_ref, use_state, use_toggle, State};
pub use terminal::{use_dimensions, use_terminal_size};
pub use threshold::{
    auto_threshold_color, binary_thresholds, color_gradient, health_thresholds,
    inverted_percentage_thresholds, lerp_color, percentage_thresholds, temperature_thresholds,
    use_threshold_color, ThresholdColor, ThresholdConfig, ThresholdRange,
};
pub use timing::{
    use_debounce, use_interval, use_interval_with_options, use_throttle, use_timeout,
    use_timeout_paused, DebounceHandle, IntervalHandle, IntervalOptions, ThrottleHandle,
    TimeoutHandle, TimeoutState,
};

// Re-export from core
pub use crate::core::hotkeys::parse_hotkey;

/// Compatibility helper that keeps the JS entrypoint name.
pub fn parse_keypress(s: &str) -> crate::core::hotkeys::ParsedHotkey {
    parse_hotkey(s)
}

/// Cancel an interval handle.
pub fn cleanup_interval(handle: &IntervalHandle) {
    handle.stop();
}

/// Cancel a timeout handle.
pub fn cleanup_timeout(handle: &TimeoutHandle) {
    handle.cancel();
}

use crate::core::app::AppContext;

/// Get the application context.
pub fn use_app() -> AppContext {
    // In a real implementation, this would come from a context provider
    AppContext {
        width: 80,
        height: 24,
    }
}
