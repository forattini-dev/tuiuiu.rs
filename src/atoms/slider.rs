//! Slider - Numeric value slider
//!
//! An interactive slider component for numeric value selection.
//!
//! # Features
//!
//! - Configurable min/max/step
//! - Visual track with thumb
//! - Value display option
//! - ASCII and Unicode modes
//! - State management
//!
//! # Example
//!
//! ```rust
//! use tuiuiu::atoms::Slider;
//!
//! // Basic slider
//! let slider = Slider::new()
//!     .min(0)
//!     .max(100)
//!     .value(50)
//!     .width(20);
//!
//! // Volume control with percentage
//! let volume = Slider::new()
//!     .min(0)
//!     .max(100)
//!     .value(75)
//!     .label("Volume")
//!     .show_value(true);
//! ```

use crate::core::signals::{create_signal, ReadSignal, WriteSignal};
use crate::themes::get_theme;

// =============================================================================
// Types
// =============================================================================

/// Slider render mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderMode {
    /// Unicode symbols (━━━●░░░)
    #[default]
    Unicode,
    /// ASCII symbols (====>---)
    Ascii,
}

/// Slider state manager for external state control
#[derive(Clone)]
pub struct SliderState {
    value: ReadSignal<i32>,
    set_value: WriteSignal<i32>,
    min: i32,
    max: i32,
    step: i32,
    on_change: Option<fn(i32)>,
}

impl SliderState {
    /// Create a new slider state
    pub fn new(min: i32, max: i32, initial: i32, step: i32) -> Self {
        let clamped = initial.clamp(min, max);
        let (value, set_value) = create_signal(clamped);
        Self {
            value,
            set_value,
            min,
            max,
            step,
            on_change: None,
        }
    }

    /// Set on_change callback
    pub fn on_change(mut self, callback: fn(i32)) -> Self {
        self.on_change = Some(callback);
        self
    }

    /// Get current value
    pub fn value(&self) -> i32 {
        self.value.get()
    }

    /// Get normalized value (0.0 to 1.0)
    pub fn normalized(&self) -> f64 {
        let range = self.max - self.min;
        if range <= 0 {
            return 0.0;
        }
        (self.value() - self.min) as f64 / range as f64
    }

    /// Set value directly
    pub fn set(&self, value: i32) {
        let clamped = value.clamp(self.min, self.max);
        if clamped != self.value() {
            self.set_value.set(clamped);
            if let Some(cb) = self.on_change {
                cb(clamped);
            }
        }
    }

    /// Set value from normalized (0.0 to 1.0)
    pub fn set_normalized(&self, normalized: f64) {
        let clamped = normalized.clamp(0.0, 1.0);
        let range = self.max - self.min;
        let value = self.min + (clamped * range as f64) as i32;
        self.set(value);
    }

    /// Increment by step
    pub fn increment(&self) {
        self.set(self.value() + self.step);
    }

    /// Decrement by step
    pub fn decrement(&self) {
        self.set(self.value() - self.step);
    }

    /// Increment by large step (10x)
    pub fn increment_large(&self) {
        self.set(self.value() + self.step * 10);
    }

    /// Decrement by large step (10x)
    pub fn decrement_large(&self) {
        self.set(self.value() - self.step * 10);
    }

    /// Set to minimum
    pub fn set_min(&self) {
        self.set(self.min);
    }

    /// Set to maximum
    pub fn set_max(&self) {
        self.set(self.max);
    }
}

// =============================================================================
// Component
// =============================================================================

/// Slider component builder
pub struct Slider {
    /// Minimum value
    min: i32,
    /// Maximum value
    max: i32,
    /// Step increment
    step: i32,
    /// Current value
    value: i32,
    /// Slider width in characters
    width: usize,
    /// Show value display
    show_value: bool,
    /// Show min/max labels
    show_min_max: bool,
    /// Label text
    label: Option<&'static str>,
    /// Render mode
    mode: SliderMode,
    /// Is disabled
    disabled: bool,
}

impl Default for Slider {
    fn default() -> Self {
        Self::new()
    }
}

impl Slider {
    /// Create a new slider
    pub fn new() -> Self {
        Self {
            min: 0,
            max: 100,
            step: 1,
            value: 0,
            width: 20,
            show_value: true,
            show_min_max: false,
            label: None,
            mode: SliderMode::Unicode,
            disabled: false,
        }
    }

    /// Set minimum value
    pub fn min(mut self, min: i32) -> Self {
        self.min = min;
        self
    }

    /// Set maximum value
    pub fn max(mut self, max: i32) -> Self {
        self.max = max;
        self
    }

    /// Set step increment
    pub fn step(mut self, step: i32) -> Self {
        self.step = step;
        self
    }

    /// Set current value
    pub fn value(mut self, value: i32) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    /// Set slider width in characters
    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Set whether to show value
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Set whether to show min/max labels
    pub fn show_min_max(mut self, show: bool) -> Self {
        self.show_min_max = show;
        self
    }

    /// Set label
    pub fn label(mut self, label: &'static str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set render mode
    pub fn mode(mut self, mode: SliderMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Render the slider to ANSI string
    pub fn render(&self) -> String {
        let theme = get_theme();
        let tokens = &theme.components.slider;

        // Calculate normalized position (0.0 to 1.0)
        let range = self.max - self.min;
        let normalized = if range > 0 {
            (self.value.clamp(self.min, self.max) - self.min) as f64 / range as f64
        } else {
            0.0
        };

        // Calculate thumb position
        let thumb_pos = ((normalized * (self.width - 1) as f64).round() as usize).min(self.width - 1);

        // Get colors
        let track_color = hex_to_rgb(tokens.track_bg);
        let fill_color = hex_to_rgb(tokens.fill_bg);
        let thumb_color = hex_to_rgb(tokens.thumb);

        // Choose characters based on mode
        let (filled_char, thumb_char, empty_char) = match self.mode {
            SliderMode::Unicode => ('━', '●', '░'),
            SliderMode::Ascii => ('=', '>', '-'),
        };

        let mut output = String::new();

        // Add label if present
        if let Some(label) = self.label {
            if self.disabled {
                output.push_str(&format!("\x1b[2m{}\x1b[0m ", label));
            } else {
                output.push_str(&format!("{} ", label));
            }
        }

        // Add min label if enabled
        if self.show_min_max {
            output.push_str(&format!("\x1b[2m{}\x1b[0m ", self.min));
        }

        // Build track
        for i in 0..self.width {
            if i < thumb_pos {
                // Filled portion
                if self.disabled {
                    output.push_str(&format!("\x1b[2m{}\x1b[0m", filled_char));
                } else {
                    output.push_str(&format!("\x1b[38;2;{}m{}\x1b[0m", fill_color, filled_char));
                }
            } else if i == thumb_pos {
                // Thumb
                if self.disabled {
                    output.push_str(&format!("\x1b[2m{}\x1b[0m", thumb_char));
                } else {
                    output.push_str(&format!("\x1b[1;38;2;{}m{}\x1b[0m", thumb_color, thumb_char));
                }
            } else {
                // Empty portion
                if self.disabled {
                    output.push_str(&format!("\x1b[2m{}\x1b[0m", empty_char));
                } else {
                    output.push_str(&format!("\x1b[2;38;2;{}m{}\x1b[0m", track_color, empty_char));
                }
            }
        }

        // Add max label if enabled
        if self.show_min_max {
            output.push_str(&format!(" \x1b[2m{}\x1b[0m", self.max));
        }

        // Add value display
        if self.show_value {
            if self.disabled {
                output.push_str(&format!(" \x1b[2m{}\x1b[0m", self.value));
            } else {
                output.push_str(&format!(" \x1b[38;2;{}m{}\x1b[0m", fill_color, self.value));
            }
        }

        output
    }

    /// Create a state object for external control
    pub fn create_state(&self) -> SliderState {
        SliderState::new(self.min, self.max, self.value, self.step)
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
    fn test_slider_default() {
        let slider = Slider::new();
        let output = slider.render();
        // Should contain thumb symbol
        assert!(output.contains('●') || output.contains('>'));
    }

    #[test]
    fn test_slider_with_value() {
        let slider = Slider::new()
            .min(0)
            .max(100)
            .value(50)
            .width(10);
        let output = slider.render();
        assert!(output.contains("50")); // Value should be shown
    }

    #[test]
    fn test_slider_ascii_mode() {
        let slider = Slider::new()
            .mode(SliderMode::Ascii)
            .value(50);
        let output = slider.render();
        assert!(output.contains('>'));
        assert!(output.contains('=') || output.contains('-'));
    }

    #[test]
    fn test_slider_with_label() {
        let slider = Slider::new()
            .label("Volume")
            .value(75);
        let output = slider.render();
        assert!(output.contains("Volume"));
    }

    #[test]
    fn test_slider_state() {
        let state = SliderState::new(0, 100, 50, 1);
        assert_eq!(state.value(), 50);

        state.increment();
        assert_eq!(state.value(), 51);

        state.decrement();
        assert_eq!(state.value(), 50);

        state.set(200); // Should clamp
        assert_eq!(state.value(), 100);
    }

    #[test]
    fn test_slider_normalized() {
        let state = SliderState::new(0, 100, 50, 1);
        assert!((state.normalized() - 0.5).abs() < 0.01);

        state.set_normalized(0.75);
        assert_eq!(state.value(), 75);
    }

    #[test]
    fn test_slider_min_max() {
        let slider = Slider::new()
            .min(10)
            .max(20)
            .value(15)
            .show_min_max(true);
        let output = slider.render();
        assert!(output.contains("10"));
        assert!(output.contains("20"));
    }
}
