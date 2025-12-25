//! Hotkey Hooks
//!
//! Keyboard shortcut management.

use std::cell::RefCell;
use std::collections::HashMap;
use crate::core::terminal::{Key, KeyModifiers};
use crate::core::hotkeys::{parse_hotkey, ParsedHotkey};

/// Hotkey binding.
#[derive(Debug, Clone)]
pub struct HotkeyBinding {
    /// Hotkey pattern (e.g., "ctrl+s")
    pub pattern: String,
    /// Parsed hotkey
    pub parsed: ParsedHotkey,
    /// Description
    pub description: Option<String>,
    /// Scope
    pub scope: Option<String>,
}

/// Hotkey options.
#[derive(Debug, Clone, Default)]
pub struct HotkeyOptions {
    /// Only active in this scope
    pub scope: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Enabled
    pub enabled: bool,
}

/// Hotkey handler type.
pub type HotkeyHandler = Box<dyn Fn()>;

thread_local! {
    static HOTKEYS: RefCell<HashMap<String, (HotkeyBinding, HotkeyHandler)>> =
        RefCell::new(HashMap::new());
    static CURRENT_SCOPE: RefCell<Option<String>> = RefCell::new(None);
}

/// Register multiple hotkeys.
pub fn use_hotkeys<I, F>(bindings: I)
where
    I: IntoIterator<Item = (String, F)>,
    F: Fn() + 'static,
{
    for (pattern, handler) in bindings {
        register_hotkey(&pattern, handler, HotkeyOptions::default());
    }
}

/// Register a hotkey.
pub fn register_hotkey<F: Fn() + 'static>(
    pattern: &str,
    handler: F,
    options: HotkeyOptions,
) {
    let binding = HotkeyBinding {
        pattern: pattern.to_string(),
        parsed: parse_hotkey(pattern),
        description: options.description,
        scope: options.scope,
    };

    HOTKEYS.with(|hotkeys| {
        hotkeys.borrow_mut().insert(
            pattern.to_string(),
            (binding, Box::new(handler)),
        );
    });
}

/// Trigger a hotkey by pattern.
pub fn trigger_hotkey(pattern: &str) -> bool {
    HOTKEYS.with(|hotkeys| {
        if let Some((_, handler)) = hotkeys.borrow().get(pattern) {
            handler();
            true
        } else {
            false
        }
    })
}

/// Get all registered hotkeys.
pub fn get_registered_hotkeys() -> Vec<HotkeyBinding> {
    HOTKEYS.with(|hotkeys| {
        hotkeys.borrow().values().map(|(b, _)| b.clone()).collect()
    })
}

/// Get current hotkey scope.
pub fn get_hotkey_scope() -> Option<String> {
    CURRENT_SCOPE.with(|scope| scope.borrow().clone())
}

/// Set hotkey scope.
pub fn set_hotkey_scope(scope: Option<String>) {
    CURRENT_SCOPE.with(|s| *s.borrow_mut() = scope);
}

/// Reset hotkey scope.
pub fn reset_hotkey_scope() {
    set_hotkey_scope(None);
}

/// Parse multiple hotkey patterns.
pub fn parse_hotkeys(patterns: &[&str]) -> Vec<ParsedHotkey> {
    patterns.iter().map(|p| parse_hotkey(p)).collect()
}

/// Check if a key matches a hotkey.
pub fn matches_hotkey(key: &Key, mods: &KeyModifiers, pattern: &str) -> bool {
    let parsed = parse_hotkey(pattern);
    key == &parsed.key
        && mods.ctrl == parsed.modifiers.ctrl
        && mods.alt == parsed.modifiers.alt
        && mods.shift == parsed.modifiers.shift
}

/// Format a hotkey for display.
pub fn format_hotkey(pattern: &str) -> String {
    pattern
        .replace("ctrl", "Ctrl")
        .replace("alt", "Alt")
        .replace("shift", "Shift")
        .replace("cmd", "⌘")
        .replace("+", " + ")
}

/// Format a hotkey for the current platform.
pub fn format_hotkey_platform(pattern: &str) -> String {
    if is_mac() {
        pattern
            .replace("ctrl", "⌃")
            .replace("alt", "⌥")
            .replace("shift", "⇧")
            .replace("cmd", "⌘")
            .replace("+", "")
    } else {
        format_hotkey(pattern)
    }
}

/// Check if running on macOS.
pub fn is_mac() -> bool {
    cfg!(target_os = "macos")
}
