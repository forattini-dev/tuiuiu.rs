//! Checkbox - Boolean toggle with check mark
//!
//! A checkbox component for boolean selection with label support.
//!
//! # Features
//!
//! - Checked/unchecked states
//! - Indeterminate state
//! - Custom colors from theme
//! - Label support
//!
//! # Example
//!
//! ```rust
//! use tuiuiu::atoms::Checkbox;
//!
//! // Basic checkbox
//! let checkbox = Checkbox::new()
//!     .label("Accept terms")
//!     .on_change(|checked| println!("Checked: {}", checked));
//!
//! // Pre-checked
//! let checked = Checkbox::new()
//!     .checked(true)
//!     .label("Enable notifications");
//! ```

use crate::core::signals::{create_signal, ReadSignal, WriteSignal};
use crate::themes::get_theme;

// =============================================================================
// Types
// =============================================================================

/// Checkbox state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CheckboxState {
    /// Unchecked
    #[default]
    Unchecked,
    /// Checked
    Checked,
    /// Indeterminate (partial selection)
    Indeterminate,
}

impl CheckboxState {
    /// Check if checkbox is checked
    pub fn is_checked(&self) -> bool {
        matches!(self, CheckboxState::Checked)
    }

    /// Check if checkbox is indeterminate
    pub fn is_indeterminate(&self) -> bool {
        matches!(self, CheckboxState::Indeterminate)
    }
}

/// Checkbox state manager
#[derive(Clone)]
pub struct CheckboxValue {
    state: ReadSignal<CheckboxState>,
    set_state: WriteSignal<CheckboxState>,
    on_change: Option<fn(bool)>,
}

impl CheckboxValue {
    /// Create a new checkbox value
    pub fn new(initial: CheckboxState) -> Self {
        let (state, set_state) = create_signal(initial);
        Self {
            state,
            set_state,
            on_change: None,
        }
    }

    /// Set the on_change callback
    pub fn on_change(mut self, callback: fn(bool)) -> Self {
        self.on_change = Some(callback);
        self
    }

    /// Get current state
    pub fn state(&self) -> CheckboxState {
        self.state.get()
    }

    /// Check if currently checked
    pub fn is_checked(&self) -> bool {
        self.state.get().is_checked()
    }

    /// Toggle the checkbox
    pub fn toggle(&self) {
        let new_state = match self.state.get() {
            CheckboxState::Unchecked => CheckboxState::Checked,
            CheckboxState::Checked => CheckboxState::Unchecked,
            CheckboxState::Indeterminate => CheckboxState::Checked,
        };
        self.set_state.set(new_state);
        if let Some(cb) = self.on_change {
            cb(new_state.is_checked());
        }
    }

    /// Set checked state
    pub fn set_checked(&self, checked: bool) {
        let new_state = if checked {
            CheckboxState::Checked
        } else {
            CheckboxState::Unchecked
        };
        if self.state.get() != new_state {
            self.set_state.set(new_state);
            if let Some(cb) = self.on_change {
                cb(checked);
            }
        }
    }

    /// Set indeterminate state
    pub fn set_indeterminate(&self) {
        self.set_state.set(CheckboxState::Indeterminate);
    }
}

// =============================================================================
// Component
// =============================================================================

/// Checkbox component builder
pub struct Checkbox {
    /// Initial checked state
    initial_checked: bool,
    /// Initial indeterminate state
    indeterminate: bool,
    /// Label text
    label: Option<&'static str>,
    /// Is disabled
    disabled: bool,
    /// On change callback
    on_change: Option<fn(bool)>,
}

impl Default for Checkbox {
    fn default() -> Self {
        Self::new()
    }
}

impl Checkbox {
    /// Create a new checkbox
    pub fn new() -> Self {
        Self {
            initial_checked: false,
            indeterminate: false,
            label: None,
            disabled: false,
            on_change: None,
        }
    }

    /// Set initial checked state
    pub fn checked(mut self, checked: bool) -> Self {
        self.initial_checked = checked;
        self
    }

    /// Set indeterminate state
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    /// Set label
    pub fn label(mut self, label: &'static str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set on change callback
    pub fn on_change(mut self, callback: fn(bool)) -> Self {
        self.on_change = Some(callback);
        self
    }

    /// Render the checkbox to ANSI string
    pub fn render(&self) -> String {
        let theme = get_theme();
        let tokens = &theme.components.checkbox;

        let state = if self.indeterminate {
            CheckboxState::Indeterminate
        } else if self.initial_checked {
            CheckboxState::Checked
        } else {
            CheckboxState::Unchecked
        };

        let (symbol, color) = match state {
            CheckboxState::Checked => ("☑", tokens.checked_bg),
            CheckboxState::Unchecked => ("☐", tokens.border),
            CheckboxState::Indeterminate => ("☒", tokens.checked_bg),
        };

        let color_rgb = hex_to_rgb(color);
        let check_rgb = hex_to_rgb(tokens.check_color);

        let mut output = String::new();

        // Render checkbox
        if self.disabled {
            output.push_str(&format!("\x1b[2;38;2;{}m{}\x1b[0m", color_rgb, symbol));
        } else if state.is_checked() {
            // Checked: use checked background with check color
            output.push_str(&format!(
                "\x1b[38;2;{}m{}\x1b[0m",
                check_rgb, symbol
            ));
        } else {
            output.push_str(&format!("\x1b[38;2;{}m{}\x1b[0m", color_rgb, symbol));
        }

        // Add label if present
        if let Some(label) = self.label {
            if self.disabled {
                output.push_str(&format!(" \x1b[2m{}\x1b[0m", label));
            } else {
                output.push_str(&format!(" {}", label));
            }
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
    fn test_checkbox_state() {
        let value = CheckboxValue::new(CheckboxState::Unchecked);
        assert!(!value.is_checked());

        value.toggle();
        assert!(value.is_checked());

        value.set_checked(false);
        assert!(!value.is_checked());
    }

    #[test]
    fn test_checkbox_render() {
        let unchecked = Checkbox::new();
        let output = unchecked.render();
        assert!(output.contains("☐"));

        let checked = Checkbox::new().checked(true);
        let output = checked.render();
        assert!(output.contains("☑"));

        let indeterminate = Checkbox::new().indeterminate(true);
        let output = indeterminate.render();
        assert!(output.contains("☒"));
    }

    #[test]
    fn test_checkbox_with_label() {
        let checkbox = Checkbox::new()
            .label("Accept terms")
            .checked(true);
        let output = checkbox.render();
        assert!(output.contains("Accept terms"));
        assert!(output.contains("☑"));
    }
}
