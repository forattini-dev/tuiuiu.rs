//! Scrollbar - Visual scroll indicator
//!
//! A scrollbar component for displaying scroll position in scrollable content.
//!
//! # Features
//!
//! - Proportional thumb size based on content
//! - ASCII and Unicode modes
//! - Customizable colors
//! - Vertical orientation (horizontal TBD)
//!
//! # Example
//!
//! ```rust
//! use tuiuiu::atoms::Scrollbar;
//!
//! // Basic scrollbar
//! let scrollbar = Scrollbar::new()
//!     .height(10)
//!     .total(100)
//!     .current(25);
//!
//! // Custom colors
//! let custom = Scrollbar::new()
//!     .height(20)
//!     .total(200)
//!     .current(50)
//!     .thumb_color("#00ff00")
//!     .track_color("#333333");
//! ```

use crate::themes::get_theme;

// =============================================================================
// Types
// =============================================================================

/// Scrollbar render mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollbarMode {
    /// Unicode symbols (█ for thumb, │ for track)
    #[default]
    Unicode,
    /// ASCII symbols (# for thumb, | for track)
    Ascii,
}

// =============================================================================
// Component
// =============================================================================

/// Scrollbar component builder
pub struct Scrollbar {
    /// Height of scrollbar area (number of lines)
    height: usize,
    /// Total content height
    total: usize,
    /// Current scroll position
    current: usize,
    /// Thumb color
    thumb_color: Option<&'static str>,
    /// Track color
    track_color: Option<&'static str>,
    /// Render mode
    mode: ScrollbarMode,
    /// Custom thumb character
    thumb_char: Option<char>,
    /// Custom track character
    track_char: Option<char>,
}

impl Default for Scrollbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Scrollbar {
    /// Create a new scrollbar
    pub fn new() -> Self {
        Self {
            height: 10,
            total: 100,
            current: 0,
            thumb_color: None,
            track_color: None,
            mode: ScrollbarMode::Unicode,
            thumb_char: None,
            track_char: None,
        }
    }

    /// Set visible height (number of lines)
    pub fn height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }

    /// Set total content height
    pub fn total(mut self, total: usize) -> Self {
        self.total = total;
        self
    }

    /// Set current scroll position (0 to total - height)
    pub fn current(mut self, current: usize) -> Self {
        self.current = current;
        self
    }

    /// Set thumb color
    pub fn thumb_color(mut self, color: &'static str) -> Self {
        self.thumb_color = Some(color);
        self
    }

    /// Set track color
    pub fn track_color(mut self, color: &'static str) -> Self {
        self.track_color = Some(color);
        self
    }

    /// Set render mode
    pub fn mode(mut self, mode: ScrollbarMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set custom thumb character
    pub fn thumb_char(mut self, c: char) -> Self {
        self.thumb_char = Some(c);
        self
    }

    /// Set custom track character
    pub fn track_char(mut self, c: char) -> Self {
        self.track_char = Some(c);
        self
    }

    /// Calculate scrollbar geometry
    fn calculate(&self) -> ScrollbarGeometry {
        // If content fits in view, no scrollbar needed
        if self.total <= self.height {
            return ScrollbarGeometry {
                thumb_size: self.height,
                thumb_pos: 0,
                max_scroll: 0,
            };
        }

        let max_scroll = self.total.saturating_sub(self.height);

        // Thumb size is proportional to visible area
        let thumb_size = ((self.height as f64 / self.total as f64) * self.height as f64)
            .floor()
            .max(1.0) as usize;

        let available_track = self.height.saturating_sub(thumb_size);

        // Calculate thumb position
        let effective_scroll = self.current.min(max_scroll);
        let scroll_ratio = if max_scroll > 0 {
            effective_scroll as f64 / max_scroll as f64
        } else {
            0.0
        };

        let thumb_pos = (available_track as f64 * scroll_ratio).floor() as usize;

        ScrollbarGeometry {
            thumb_size,
            thumb_pos,
            max_scroll,
        }
    }

    /// Render the scrollbar to an array of ANSI strings (one per line)
    pub fn render_lines(&self) -> Vec<String> {
        let theme = get_theme();

        // Get colors
        let thumb_color = self.thumb_color.unwrap_or(theme.palette.primary.s500);
        let track_color = self.track_color.unwrap_or(theme.background.subtle);

        let thumb_rgb = hex_to_rgb(thumb_color);
        let track_rgb = hex_to_rgb(track_color);

        // Get characters based on mode
        let (thumb_char, track_char) = match self.mode {
            ScrollbarMode::Unicode => (
                self.thumb_char.unwrap_or('█'),
                self.track_char.unwrap_or('│'),
            ),
            ScrollbarMode::Ascii => (
                self.thumb_char.unwrap_or('#'),
                self.track_char.unwrap_or('|'),
            ),
        };

        let geo = self.calculate();
        let mut lines = Vec::with_capacity(self.height);

        for i in 0..self.height {
            let is_thumb = i >= geo.thumb_pos && i < geo.thumb_pos + geo.thumb_size;

            if is_thumb {
                lines.push(format!("\x1b[38;2;{}m{}\x1b[0m", thumb_rgb, thumb_char));
            } else {
                lines.push(format!("\x1b[2;38;2;{}m{}\x1b[0m", track_rgb, track_char));
            }
        }

        lines
    }

    /// Render as a single string with newlines
    pub fn render(&self) -> String {
        self.render_lines().join("\n")
    }

    /// Check if scrollbar is needed (content exceeds viewport)
    pub fn is_needed(&self) -> bool {
        self.total > self.height
    }
}

/// Scrollbar geometry calculation result
struct ScrollbarGeometry {
    thumb_size: usize,
    thumb_pos: usize,
    #[allow(dead_code)]
    max_scroll: usize,
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
    fn test_scrollbar_default() {
        let scrollbar = Scrollbar::new();
        let lines = scrollbar.render_lines();
        assert_eq!(lines.len(), 10); // Default height
    }

    #[test]
    fn test_scrollbar_not_needed() {
        let scrollbar = Scrollbar::new().height(20).total(10); // Total less than height

        assert!(!scrollbar.is_needed());
    }

    #[test]
    fn test_scrollbar_needed() {
        let scrollbar = Scrollbar::new().height(10).total(100);

        assert!(scrollbar.is_needed());
    }

    #[test]
    fn test_scrollbar_thumb_position() {
        // At start
        let start = Scrollbar::new().height(10).total(100).current(0);
        let lines = start.render_lines();
        // First line should contain thumb
        assert!(lines[0].contains('█') || lines[0].contains('#'));

        // At end
        let end = Scrollbar::new().height(10).total(100).current(90);
        let lines = end.render_lines();
        // Last line should contain thumb
        assert!(lines[9].contains('█') || lines[9].contains('#'));
    }

    #[test]
    fn test_scrollbar_ascii_mode() {
        let scrollbar = Scrollbar::new().mode(ScrollbarMode::Ascii).current(0);
        let lines = scrollbar.render_lines();

        // Should use ASCII characters
        let output = lines.join("");
        assert!(output.contains('#') || output.contains('|'));
    }

    #[test]
    fn test_scrollbar_custom_chars() {
        let scrollbar = Scrollbar::new().thumb_char('▓').track_char('░');
        let lines = scrollbar.render_lines();
        let output = lines.join("");

        assert!(output.contains('▓') || output.contains('░'));
    }
}
