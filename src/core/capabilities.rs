//! Terminal Capabilities Detection
//!
//! Detect terminal features like color support, Unicode, etc.

use std::env;

/// Terminal capabilities.
#[derive(Debug, Clone)]
pub struct TerminalCapabilities {
    pub true_color: bool,
    pub colors_256: bool,
    pub colors_16: bool,
    pub unicode: bool,
    pub mouse: bool,
    pub bracketed_paste: bool,
    pub alternate_screen: bool,
    pub title: bool,
}

impl Default for TerminalCapabilities {
    fn default() -> Self {
        Self {
            true_color: true,
            colors_256: true,
            colors_16: true,
            unicode: true,
            mouse: true,
            bracketed_paste: true,
            alternate_screen: true,
            title: true,
        }
    }
}

/// Detect terminal capabilities.
pub fn detect_terminal_capabilities() -> TerminalCapabilities {
    let colorterm = env::var("COLORTERM").unwrap_or_default();
    let term = env::var("TERM").unwrap_or_default();

    let true_color = colorterm.contains("truecolor") || colorterm.contains("24bit");
    let colors_256 = term.contains("256color") || true_color;

    TerminalCapabilities {
        true_color,
        colors_256,
        colors_16: true,
        unicode: supports_unicode(),
        mouse: true,
        bracketed_paste: true,
        alternate_screen: true,
        title: true,
    }
}

fn supports_unicode() -> bool {
    env::var("LANG")
        .or_else(|_| env::var("LC_ALL"))
        .map(|v| v.to_lowercase().contains("utf"))
        .unwrap_or(true)
}

/// Character set for rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterSet {
    Unicode,
    Ascii,
}

/// Color support level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSupport {
    TrueColor,
    Colors256,
    Colors16,
    None,
}

/// Render mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderMode {
    #[default]
    Auto,
    Unicode,
    Ascii,
}
