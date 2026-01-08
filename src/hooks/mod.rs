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

mod state;
mod effects;
mod input;
mod mouse;
mod focus;
mod terminal;
mod hotkeys;
mod form;
mod timing;
mod navigation;
mod threshold;
mod format;
mod fps;
mod layout;
mod local_mouse;
mod clipboard;
mod previous;
mod animation;

pub use state::{use_state, use_reducer, use_ref, use_lazy_state, use_toggle, use_counter, State};
pub use effects::{use_effect, use_memo, use_callback, use_mount, use_cleanup};
pub use input::{use_input, use_key, dispatch_key_event, clear_input_handlers, key_matches, InputHandler};
pub use mouse::{use_mouse, dispatch_mouse_event, clear_mouse_handlers, use_mouse_position, MouseHandler, MousePosition};
pub use focus::{use_focus, use_focus_manager, FocusOptions, FocusResult};
pub use terminal::{use_terminal_size, use_dimensions};
pub use hotkeys::{
    use_hotkeys, HotkeyBinding, HotkeyHandler, HotkeyOptions,
    register_hotkey, trigger_hotkey, get_registered_hotkeys,
    get_hotkey_scope, set_hotkey_scope, reset_hotkey_scope,
    parse_hotkeys, matches_hotkey, format_hotkey,
    format_hotkey_platform, is_mac,
};
pub use form::{
    use_form, create_form,
    FormField, FormHandle, FormState, FieldValue, ValidationResult,
    validators,
};
pub use timing::{
    use_interval, use_interval_with_options, IntervalHandle, IntervalOptions,
    use_timeout, use_timeout_paused, TimeoutHandle, TimeoutState,
    use_debounce, DebounceHandle,
    use_throttle, ThrottleHandle,
};
pub use navigation::{
    use_navigation, create_navigation, simple_steps,
    NavigationState, NavigationStep, NavigationResult,
};
pub use threshold::{
    use_threshold_color, auto_threshold_color,
    ThresholdConfig, ThresholdRange, ThresholdColor,
    lerp_color, color_gradient,
    health_thresholds, percentage_thresholds, inverted_percentage_thresholds,
    temperature_thresholds, binary_thresholds,
};
pub use format::{
    use_format_bytes, use_format_bytes_si, use_format_duration, use_format_duration_compact,
    use_format_number, use_format_compact, use_format_percent, use_format_currency,
    use_format_relative, use_format_delta,
};
pub use fps::{
    use_fps, track_frame, get_fps, get_fps_metrics, reset_fps,
    FpsMetrics, FpsColor,
};
pub use layout::{
    use_layout_ref, use_layout_ref_with, create_layout_ref,
    LayoutRef, LayoutRect,
};
pub use local_mouse::{
    use_local_mouse, use_local_mouse_with,
    LocalMouseHandler, LocalMouseEvent, LocalMouseOptions,
    RawMouseEvent, MouseButton, MouseAction, Modifiers, Bounds,
};
pub use clipboard::{
    use_clipboard, copy_to_clipboard, read_clipboard, clear_clipboard,
    ClipboardHandle,
};
pub use previous::{
    use_previous, use_changed,
    PreviousHandle,
};
pub use animation::{
    use_animation, use_fade_in, use_fade_out, use_slide,
    lerp, map_range,
    AnimationHandle, AnimationState, Easing,
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
