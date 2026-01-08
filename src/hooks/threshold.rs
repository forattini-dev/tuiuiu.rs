//! Threshold Color Hook
//!
//! Color mapping based on value thresholds for status indicators, metrics, etc.
//!
//! Features:
//! - Range-based color mapping
//! - Predefined threshold patterns (health, percentage, temperature)
//! - Custom color gradients
//! - Lerp between colors

use crate::core::component::{Color, NamedColor};

// =============================================================================
// Types
// =============================================================================

/// Threshold range as (min, max) inclusive.
#[derive(Debug, Clone, Copy)]
pub struct ThresholdRange {
    pub min: f64,
    pub max: f64,
    pub color: ThresholdColor,
}

impl ThresholdRange {
    /// Create a new threshold range.
    pub fn new(min: f64, max: f64, color: ThresholdColor) -> Self {
        Self { min, max, color }
    }

    /// Check if value is within range.
    pub fn contains(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }
}

/// Threshold color types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdColor {
    Success,
    Warning,
    Error,
    Info,
    Neutral,
    Custom(u8, u8, u8),
}

impl ThresholdColor {
    /// Convert to Color.
    pub fn to_color(&self) -> Color {
        match self {
            ThresholdColor::Success => Color::Named(NamedColor::Green),
            ThresholdColor::Warning => Color::Named(NamedColor::Yellow),
            ThresholdColor::Error => Color::Named(NamedColor::Red),
            ThresholdColor::Info => Color::Named(NamedColor::Blue),
            ThresholdColor::Neutral => Color::Named(NamedColor::Gray),
            ThresholdColor::Custom(r, g, b) => Color::Rgb(*r, *g, *b),
        }
    }
}

/// Threshold configuration for value-to-color mapping.
#[derive(Debug, Clone, Default)]
pub struct ThresholdConfig {
    ranges: Vec<ThresholdRange>,
    default_color: Option<ThresholdColor>,
}

impl ThresholdConfig {
    /// Create a new threshold config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a range.
    pub fn range(mut self, min: f64, max: f64, color: ThresholdColor) -> Self {
        self.ranges.push(ThresholdRange::new(min, max, color));
        self
    }

    /// Add success range.
    pub fn success(self, min: f64, max: f64) -> Self {
        self.range(min, max, ThresholdColor::Success)
    }

    /// Add warning range.
    pub fn warning(self, min: f64, max: f64) -> Self {
        self.range(min, max, ThresholdColor::Warning)
    }

    /// Add error range.
    pub fn error(self, min: f64, max: f64) -> Self {
        self.range(min, max, ThresholdColor::Error)
    }

    /// Add info range.
    pub fn info(self, min: f64, max: f64) -> Self {
        self.range(min, max, ThresholdColor::Info)
    }

    /// Set default color when no range matches.
    pub fn default_color(mut self, color: ThresholdColor) -> Self {
        self.default_color = Some(color);
        self
    }

    /// Get color for value.
    pub fn get_color(&self, value: f64) -> Option<Color> {
        for range in &self.ranges {
            if range.contains(value) {
                return Some(range.color.to_color());
            }
        }
        self.default_color.map(|c| c.to_color())
    }

    /// Get threshold color type for value.
    pub fn get_threshold(&self, value: f64) -> Option<ThresholdColor> {
        for range in &self.ranges {
            if range.contains(value) {
                return Some(range.color);
            }
        }
        self.default_color
    }
}

// =============================================================================
// Predefined Patterns
// =============================================================================

/// Create health-based thresholds (0-100).
/// - 0-25: Error (red)
/// - 25-50: Warning (yellow)
/// - 50-75: Info (blue)
/// - 75-100: Success (green)
pub fn health_thresholds() -> ThresholdConfig {
    ThresholdConfig::new()
        .error(0.0, 25.0)
        .warning(25.0, 50.0)
        .info(50.0, 75.0)
        .success(75.0, 100.0)
        .default_color(ThresholdColor::Neutral)
}

/// Create percentage thresholds (0-100).
/// - 0-33: Error (red)
/// - 33-66: Warning (yellow)
/// - 66-100: Success (green)
pub fn percentage_thresholds() -> ThresholdConfig {
    ThresholdConfig::new()
        .error(0.0, 33.0)
        .warning(33.0, 66.0)
        .success(66.0, 100.0)
        .default_color(ThresholdColor::Neutral)
}

/// Create inverted percentage thresholds (lower is better).
/// - 0-33: Success (green)
/// - 33-66: Warning (yellow)
/// - 66-100: Error (red)
pub fn inverted_percentage_thresholds() -> ThresholdConfig {
    ThresholdConfig::new()
        .success(0.0, 33.0)
        .warning(33.0, 66.0)
        .error(66.0, 100.0)
        .default_color(ThresholdColor::Neutral)
}

/// Create temperature thresholds (Celsius).
/// - Below 0: Blue
/// - 0-15: Info (light blue)
/// - 15-25: Success (green)
/// - 25-35: Warning (yellow)
/// - Above 35: Error (red)
pub fn temperature_thresholds() -> ThresholdConfig {
    ThresholdConfig::new()
        .range(-100.0, 0.0, ThresholdColor::Custom(100, 149, 237)) // Cornflower blue
        .info(0.0, 15.0)
        .success(15.0, 25.0)
        .warning(25.0, 35.0)
        .error(35.0, 100.0)
        .default_color(ThresholdColor::Neutral)
}

/// Create binary thresholds (on/off, true/false).
/// - 0: Error (red)
/// - 1: Success (green)
pub fn binary_thresholds() -> ThresholdConfig {
    ThresholdConfig::new()
        .error(0.0, 0.5)
        .success(0.5, 1.0)
        .default_color(ThresholdColor::Neutral)
}

// =============================================================================
// Color Utilities
// =============================================================================

/// Linearly interpolate between two colors.
pub fn lerp_color(from: Color, to: Color, t: f64) -> Color {
    let t = t.clamp(0.0, 1.0);

    let (r1, g1, b1) = color_to_rgb(&from);
    let (r2, g2, b2) = color_to_rgb(&to);

    let r = ((r1 as f64) * (1.0 - t) + (r2 as f64) * t) as u8;
    let g = ((g1 as f64) * (1.0 - t) + (g2 as f64) * t) as u8;
    let b = ((b1 as f64) * (1.0 - t) + (b2 as f64) * t) as u8;

    Color::Rgb(r, g, b)
}

/// Create a gradient of colors.
pub fn color_gradient(from: Color, to: Color, steps: usize) -> Vec<Color> {
    if steps == 0 {
        return vec![];
    }
    if steps == 1 {
        return vec![from];
    }

    (0..steps)
        .map(|i| {
            let t = i as f64 / (steps - 1) as f64;
            lerp_color(from.clone(), to.clone(), t)
        })
        .collect()
}

/// Convert Color to RGB tuple.
fn color_to_rgb(color: &Color) -> (u8, u8, u8) {
    match color {
        Color::Rgb(r, g, b) => (*r, *g, *b),
        Color::Named(named) => match named {
            NamedColor::Black => (0, 0, 0),
            NamedColor::Red => (255, 0, 0),
            NamedColor::Green => (0, 255, 0),
            NamedColor::Yellow => (255, 255, 0),
            NamedColor::Blue => (0, 0, 255),
            NamedColor::Magenta => (255, 0, 255),
            NamedColor::Cyan => (0, 255, 255),
            NamedColor::White => (255, 255, 255),
            NamedColor::Gray => (128, 128, 128),
            NamedColor::BrightBlack => (64, 64, 64),
            NamedColor::BrightRed => (255, 100, 100),
            NamedColor::BrightGreen => (100, 255, 100),
            NamedColor::BrightYellow => (255, 255, 100),
            NamedColor::BrightBlue => (100, 100, 255),
            NamedColor::BrightMagenta => (255, 100, 255),
            NamedColor::BrightCyan => (100, 255, 255),
            NamedColor::BrightWhite => (255, 255, 255),
        },
        Color::Default => (200, 200, 200), // Default terminal color
        Color::Ansi256(_) => (128, 128, 128), // Fallback for ANSI256
    }
}

// =============================================================================
// Hook Function
// =============================================================================

/// Create a threshold color hook.
pub fn use_threshold_color(value: f64, config: &ThresholdConfig) -> Option<Color> {
    config.get_color(value)
}

/// Create a threshold color with auto-detection based on value range.
pub fn auto_threshold_color(value: f64, min: f64, max: f64) -> Color {
    let range = max - min;
    let normalized = if range.abs() < f64::EPSILON {
        0.5
    } else {
        (value - min) / range
    };

    match normalized {
        n if n < 0.33 => Color::Named(NamedColor::Red),
        n if n < 0.66 => Color::Named(NamedColor::Yellow),
        _ => Color::Named(NamedColor::Green),
    }
}
