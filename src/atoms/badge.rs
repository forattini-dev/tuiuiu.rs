//! Badge - Small status indicator with label
//!
//! A compact label component for showing status, counts, or categories.
//!
//! # Features
//!
//! - Semantic variants (success, warning, danger, etc.)
//! - Auto-contrast text color
//! - Custom colors support
//!
//! # Example
//!
//! ```rust
//! use tuiuiu::atoms::{Badge, BadgeVariant};
//!
//! // Semantic variants
//! let success = Badge::new("SUCCESS").variant(BadgeVariant::Success);
//! let error = Badge::new("ERROR").variant(BadgeVariant::Danger);
//!
//! // Custom color
//! let custom = Badge::new("CUSTOM").color("#ff6600");
//! ```

use crate::themes::{get_theme, get_contrast_color};

// =============================================================================
// Types
// =============================================================================

/// Badge variant - semantic color variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    /// Primary theme color
    Primary,
    /// Secondary theme color
    Secondary,
    /// Success/positive (green)
    Success,
    /// Warning (amber/yellow)
    Warning,
    /// Danger/error (red)
    Danger,
    /// Default neutral
    #[default]
    Default,
}

// =============================================================================
// Component
// =============================================================================

/// Badge component builder
pub struct Badge {
    /// Label text
    label: String,
    /// Semantic variant
    variant: BadgeVariant,
    /// Custom color (overrides variant)
    color: Option<&'static str>,
    /// Bold text
    bold: bool,
}

impl Badge {
    /// Create a new badge with label
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: BadgeVariant::Default,
            color: None,
            bold: true,
        }
    }

    /// Set semantic variant
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set custom background color (auto-calculates contrast)
    pub fn color(mut self, color: &'static str) -> Self {
        self.color = Some(color);
        self
    }

    /// Set bold text
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Render the badge to ANSI string
    pub fn render(&self) -> String {
        let theme = get_theme();

        let (bg, fg) = if let Some(custom_color) = self.color {
            // Custom color: auto-calculate contrast
            let text_color = get_contrast_color(custom_color);
            (custom_color, text_color)
        } else {
            // Use component tokens for badges
            let tokens = &theme.components.badge;
            match self.variant {
                BadgeVariant::Success => (tokens.success.bg, tokens.success.fg),
                BadgeVariant::Warning => (tokens.warning.bg, tokens.warning.fg),
                BadgeVariant::Danger => (tokens.danger.bg, tokens.danger.fg),
                BadgeVariant::Primary => (theme.palette.primary.s500, theme.foreground.inverse.base),
                BadgeVariant::Secondary => (theme.palette.secondary.s500, theme.foreground.inverse.base),
                BadgeVariant::Default => (tokens.default.bg, tokens.default.fg),
            }
        };

        let bold_code = if self.bold { "1;" } else { "" };
        let fg_rgb = hex_to_rgb(fg);
        let bg_rgb = hex_to_rgb(bg);

        format!(
            "\x1b[{}38;2;{};48;2;{}m {} \x1b[0m",
            bold_code,
            fg_rgb,
            bg_rgb,
            self.label
        )
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
    fn test_badge_default() {
        let badge = Badge::new("TEST");
        let output = badge.render();
        assert!(output.contains("TEST"));
        assert!(output.contains("\x1b[")); // Has ANSI codes
    }

    #[test]
    fn test_badge_variants() {
        let success = Badge::new("OK").variant(BadgeVariant::Success);
        let danger = Badge::new("ERR").variant(BadgeVariant::Danger);

        let success_out = success.render();
        let danger_out = danger.render();

        // Both should render with ANSI codes
        assert!(success_out.contains("OK"));
        assert!(danger_out.contains("ERR"));
    }

    #[test]
    fn test_badge_custom_color() {
        let badge = Badge::new("CUSTOM").color("#ff6600");
        let output = badge.render();
        assert!(output.contains("CUSTOM"));
        // Should have orange background (255;102;0)
        assert!(output.contains("255;102;0"));
    }
}
