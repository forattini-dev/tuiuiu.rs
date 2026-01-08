//! Switch - Boolean toggle component
//!
//! On/off toggle with visual track and thumb.
//!
//! # Features
//!
//! - On/Off toggle
//! - Keyboard activation (Space, Enter, Arrow keys)
//! - Custom labels
//! - Multiple sizes (compact, normal)
//!
//! # Example
//!
//! ```rust
//! use tuiuiu::atoms::Switch;
//!
//! // Basic switch
//! let switch = Switch::new()
//!     .label("Dark mode")
//!     .on_change(|value| println!("Switch: {}", value));
//!
//! // Compact switch with custom colors
//! let switch = Switch::new()
//!     .size(SwitchSize::Compact)
//!     .color_on("#22c55e")
//!     .color_off("#64748b");
//! ```

use crate::core::signals::{create_signal, ReadSignal, WriteSignal};
use crate::themes::{get_theme, Theme};

// =============================================================================
// Types
// =============================================================================

/// Switch size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SwitchSize {
    /// Compact: [*] or [ ]
    Compact,
    /// Normal: ━●━━ or ━━○━
    #[default]
    Normal,
}

/// Switch state manager
#[derive(Clone)]
pub struct SwitchState {
    value: ReadSignal<bool>,
    set_value: WriteSignal<bool>,
    on_change: Option<fn(bool)>,
}

impl SwitchState {
    /// Create a new switch state
    pub fn new(initial: bool) -> Self {
        let (value, set_value) = create_signal(initial);
        Self {
            value,
            set_value,
            on_change: None,
        }
    }

    /// Set the on_change callback
    pub fn on_change(mut self, callback: fn(bool)) -> Self {
        self.on_change = Some(callback);
        self
    }

    /// Get current value
    pub fn value(&self) -> bool {
        self.value.get()
    }

    /// Toggle the switch
    pub fn toggle(&self) {
        let new_value = !self.value.get();
        self.set_value.set(new_value);
        if let Some(cb) = self.on_change {
            cb(new_value);
        }
    }

    /// Set switch on
    pub fn set_on(&self) {
        if !self.value.get() {
            self.set_value.set(true);
            if let Some(cb) = self.on_change {
                cb(true);
            }
        }
    }

    /// Set switch off
    pub fn set_off(&self) {
        if self.value.get() {
            self.set_value.set(false);
            if let Some(cb) = self.on_change {
                cb(false);
            }
        }
    }

    /// Set value directly
    pub fn set(&self, value: bool) {
        if self.value.get() != value {
            self.set_value.set(value);
            if let Some(cb) = self.on_change {
                cb(value);
            }
        }
    }
}

// =============================================================================
// Component
// =============================================================================

/// Switch component builder
pub struct Switch {
    /// Initial value
    initial_value: bool,
    /// Pre-created state
    state: Option<SwitchState>,
    /// On label
    on_label: &'static str,
    /// Off label
    off_label: &'static str,
    /// Show labels
    show_labels: bool,
    /// On color
    color_on: Option<&'static str>,
    /// Off color
    color_off: Option<&'static str>,
    /// Track color (background)
    background: Option<&'static str>,
    /// Size
    size: SwitchSize,
    /// Label before switch
    label: Option<&'static str>,
    /// Is disabled
    disabled: bool,
    /// On change callback
    on_change: Option<fn(bool)>,
}

impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}

impl Switch {
    /// Create a new switch
    pub fn new() -> Self {
        Self {
            initial_value: false,
            state: None,
            on_label: "ON",
            off_label: "OFF",
            show_labels: false,
            color_on: None,
            color_off: None,
            background: None,
            size: SwitchSize::Normal,
            label: None,
            disabled: false,
            on_change: None,
        }
    }

    /// Set initial value
    pub fn initial_value(mut self, value: bool) -> Self {
        self.initial_value = value;
        self
    }

    /// Set pre-created state
    pub fn state(mut self, state: SwitchState) -> Self {
        self.state = Some(state);
        self
    }

    /// Set on label
    pub fn on_label(mut self, label: &'static str) -> Self {
        self.on_label = label;
        self
    }

    /// Set off label
    pub fn off_label(mut self, label: &'static str) -> Self {
        self.off_label = label;
        self
    }

    /// Show labels
    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Set on color
    pub fn color_on(mut self, color: &'static str) -> Self {
        self.color_on = Some(color);
        self
    }

    /// Set off color
    pub fn color_off(mut self, color: &'static str) -> Self {
        self.color_off = Some(color);
        self
    }

    /// Set track background color
    pub fn background(mut self, color: &'static str) -> Self {
        self.background = Some(color);
        self
    }

    /// Set size
    pub fn size(mut self, size: SwitchSize) -> Self {
        self.size = size;
        self
    }

    /// Set label
    pub fn label(mut self, label: &'static str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set on change callback
    pub fn on_change(mut self, callback: fn(bool)) -> Self {
        self.on_change = Some(callback);
        self
    }

    /// Get or create the state
    fn get_state(&self) -> SwitchState {
        if let Some(ref state) = self.state {
            state.clone()
        } else {
            let mut state = SwitchState::new(self.initial_value);
            if let Some(cb) = self.on_change {
                state = state.on_change(cb);
            }
            state
        }
    }

    /// Render the switch to a string
    pub fn render(&self) -> String {
        let theme = get_theme();
        let state = self.get_state();
        let is_on = state.value();

        let color_on = self.color_on.unwrap_or(theme.accents.positive);
        let color_off = self.color_off.unwrap_or(theme.foreground.muted);
        let background = self.background.unwrap_or(theme.borders.default);

        self.render_with_colors(is_on, color_on, color_off, background, &theme)
    }

    /// Render with specific colors
    fn render_with_colors(
        &self,
        is_on: bool,
        color_on: &str,
        color_off: &str,
        background: &str,
        _theme: &Theme,
    ) -> String {
        let mut output = String::new();

        // Add label if present
        if let Some(label) = self.label {
            if self.disabled {
                output.push_str(&format!("\x1b[2m{}\x1b[0m ", label));
            } else {
                output.push_str(&format!("{} ", label));
            }
        }

        // Add off label if showing and off
        if self.show_labels && !is_on {
            output.push_str(&format!(
                "\x1b[38;2;{}m{}\x1b[0m ",
                hex_to_rgb(color_off),
                self.off_label
            ));
        }

        // Render switch visual based on size
        let switch_visual = match self.size {
            SwitchSize::Compact => {
                if is_on {
                    format!("\x1b[38;2;{}m●\x1b[2;38;2;{}m━\x1b[0m",
                        hex_to_rgb(color_on), hex_to_rgb(background))
                } else {
                    format!("\x1b[2;38;2;{}m━\x1b[0m\x1b[38;2;{}m○\x1b[0m",
                        hex_to_rgb(background), hex_to_rgb(color_off))
                }
            }
            SwitchSize::Normal => {
                if is_on {
                    // ━━━●
                    format!(
                        "\x1b[2;38;2;{}m━━━\x1b[0m\x1b[38;2;{}m●\x1b[0m",
                        hex_to_rgb(background),
                        hex_to_rgb(color_on)
                    )
                } else {
                    // ○━━━
                    format!(
                        "\x1b[38;2;{}m○\x1b[0m\x1b[2;38;2;{}m━━━\x1b[0m",
                        hex_to_rgb(color_off),
                        hex_to_rgb(background)
                    )
                }
            }
        };

        output.push_str(&switch_visual);

        // Add on label if showing and on
        if self.show_labels && is_on {
            output.push_str(&format!(
                " \x1b[38;2;{}m{}\x1b[0m",
                hex_to_rgb(color_on),
                self.on_label
            ));
        }

        output
    }
}

/// Convert hex color to RGB format for ANSI
fn hex_to_rgb(hex: &str) -> String {
    let hex = hex.trim_start_matches('#');
    if hex.len() < 6 {
        return "255;255;255".to_string();
    }

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);

    format!("{};{};{}", r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_state() {
        let state = SwitchState::new(false);
        assert!(!state.value());

        state.toggle();
        assert!(state.value());

        state.set_off();
        assert!(!state.value());

        state.set_on();
        assert!(state.value());
    }

    #[test]
    fn test_switch_render() {
        let switch = Switch::new().initial_value(false);
        let output = switch.render();
        assert!(output.contains("○")); // Off indicator

        let switch_on = Switch::new().initial_value(true);
        let output_on = switch_on.render();
        assert!(output_on.contains("●")); // On indicator
    }

    #[test]
    fn test_switch_with_label() {
        let switch = Switch::new()
            .label("Dark mode")
            .initial_value(true);
        let output = switch.render();
        assert!(output.contains("Dark mode"));
    }

    #[test]
    fn test_switch_compact() {
        let switch = Switch::new()
            .size(SwitchSize::Compact)
            .initial_value(true);
        let output = switch.render();
        assert!(output.contains("●"));
    }
}
