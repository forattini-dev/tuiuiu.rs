//! Themes Module
//!
//! Comprehensive theming system with semantic color tokens and component styling.
//!
//! # Quick Start
//!
//! ```rust
//! use tuiuiu::themes::{get_theme, set_theme, dark_theme, light_theme};
//!
//! // Get current theme (dark by default)
//! let theme = get_theme();
//! println!("Using theme: {}", theme.name);
//!
//! // Switch to light theme
//! set_theme(light_theme());
//!
//! // Access theme colors
//! let bg = theme.background.base;
//! let fg = theme.foreground.primary;
//! ```
//!
//! # Available Themes
//!
//! - `dark` - Default dark theme with blue primary
//! - `light` - Default light theme with blue primary
//! - `dracula` - Popular dark theme with purple accents
//! - `nord` - Arctic, north-bluish color palette
//! - `monokai` - Classic editor theme with vibrant colors
//! - `tokyo-night` - Clean dark theme inspired by Tokyo's night lights
//! - `gruvbox` - Retro groove color scheme with warm colors
//! - `solarized` - Precision colors for machines and people
//! - `catppuccin` - Soothing pastel theme for the high-spirited
//! - `high-contrast` - Accessibility-focused with maximum contrast
//! - `monochrome` - Clean grayscale theme for minimalist aesthetics
//! - `orange` - Warm orange-themed dark theme with energetic vibes
//! - `pink` - Vibrant pink-themed dark theme with playful aesthetics
//!
//! # Theme Structure
//!
//! Each theme contains:
//! - **Palette**: Semantic color scales (primary, success, warning, danger)
//! - **Background**: Surface colors (base, subtle, raised, elevated)
//! - **Foreground**: Text colors (primary, muted, disabled)
//! - **States**: Interactive states (hover, active, focus, disabled)
//! - **Components**: Per-component tokens (button, input, tabs, etc.)

pub mod colors;
pub mod types;

mod catppuccin;
mod dark;
mod dracula;
mod gruvbox;
mod high_contrast;
mod light;
mod monochrome;
mod monokai;
mod nord;
mod orange;
mod pink;
mod solarized;
mod tokyo_night;

pub use colors::{
    get_color, list_colors, parse_color, ColorScale, Shade, AMBER, BLACK, BLUE, CYAN, EMERALD,
    FUCHSIA, GRAY, GREEN, INDIGO, LIME, NEUTRAL, ORANGE, PINK, PURPLE, RED, ROSE, SKY, SLATE,
    STONE, TEAL, TRANSPARENT, VIOLET, WHITE, YELLOW, ZINC,
};

pub use types::{
    ActiveState, AppShellTokens, BadgeTokens, BadgeVariant, ButtonTokens, ButtonVariant,
    CheckboxTokens, ComponentTokens, DisabledState, DropdownItemTokens, DropdownTokens, FocusRing,
    FocusState, HeaderTokens, HeaderVariant, HoverState, InputTokens, ListItemTokens, ListTokens,
    MenuItemTokens, MenuTokens, ModalTokens, PageTokens, PageVariant, PanelTokens, RadioTokens,
    SelectedState, StatusbarTokens, StatusbarVariant, TabTokens, TabsTokens, Theme, ThemeAccents,
    ThemeBackground, ThemeBorders, ThemeForeground, ThemeForegroundInverse, ThemeMeta, ThemeMode,
    ThemeOpacity, ThemePalette, ThemeStates, ToastTokens, ToastVariant, TooltipTokens,
};

pub use catppuccin::catppuccin_theme;
pub use dark::dark_theme;
pub use dracula::dracula_theme;
pub use gruvbox::gruvbox_theme;
pub use high_contrast::high_contrast_theme;
pub use light::light_theme;
pub use monochrome::monochrome_theme;
pub use monokai::monokai_theme;
pub use nord::nord_theme;
pub use orange::orange_theme;
pub use pink::pink_theme;
pub use solarized::solarized_theme;
pub use tokyo_night::tokyo_night_theme;

use std::sync::RwLock;

// =============================================================================
// Global Theme State
// =============================================================================

static CURRENT_THEME: RwLock<Option<Theme>> = RwLock::new(None);

/// Get the current theme. Returns dark theme if none is set.
pub fn get_theme() -> Theme {
    let guard = CURRENT_THEME.read().unwrap();
    guard.clone().unwrap_or_else(dark_theme)
}

/// Set the current theme
pub fn set_theme(theme: Theme) {
    let mut guard = CURRENT_THEME.write().unwrap();
    *guard = Some(theme);
}

/// Use a theme for the duration of a closure
pub fn with_theme<T, F: FnOnce(&Theme) -> T>(f: F) -> T {
    let theme = get_theme();
    f(&theme)
}

/// Create a custom theme based on an existing one
pub fn create_theme<F: FnOnce(Theme) -> Theme>(base: Theme, customize: F) -> Theme {
    customize(base)
}

// =============================================================================
// Theme Hooks (for use in components)
// =============================================================================

/// Hook to get the current theme in a component
pub fn use_theme() -> Theme {
    get_theme()
}

// =============================================================================
// Theme Registry
// =============================================================================

/// Get a theme by name
pub fn get_theme_by_name(name: &str) -> Option<Theme> {
    match name.to_lowercase().replace('-', "_").as_str() {
        "dark" => Some(dark_theme()),
        "light" => Some(light_theme()),
        "dracula" => Some(dracula_theme()),
        "nord" => Some(nord_theme()),
        "monokai" => Some(monokai_theme()),
        "tokyo_night" | "tokyonight" => Some(tokyo_night_theme()),
        "gruvbox" => Some(gruvbox_theme()),
        "solarized" => Some(solarized_theme()),
        "catppuccin" => Some(catppuccin_theme()),
        "high_contrast" | "highcontrast" => Some(high_contrast_theme()),
        "monochrome" | "mono" => Some(monochrome_theme()),
        "orange" => Some(orange_theme()),
        "pink" => Some(pink_theme()),
        _ => None,
    }
}

/// List all available theme names
pub fn list_themes() -> &'static [&'static str] {
    &[
        "dark",
        "light",
        "dracula",
        "nord",
        "monokai",
        "tokyo-night",
        "gruvbox",
        "solarized",
        "catppuccin",
        "high-contrast",
        "monochrome",
        "orange",
        "pink",
    ]
}

// =============================================================================
// Color Utilities
// =============================================================================

/// Get contrast color (black or white) for a given background
pub fn get_contrast_color(bg_hex: &str) -> &'static str {
    // Parse hex color
    let hex = bg_hex.trim_start_matches('#');
    if hex.len() < 6 {
        return WHITE;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);

    // Calculate relative luminance
    let luminance = (0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64) / 255.0;

    if luminance > 0.5 {
        BLACK
    } else {
        WHITE
    }
}

/// Resolve a color string to its hex value
/// Supports: hex colors, Tailwind colors (e.g., "blue-500"), semantic colors
pub fn resolve_color<'a>(color: &'a str, theme: &'a Theme) -> &'a str {
    // If it starts with #, return as-is
    if color.starts_with('#') {
        return color;
    }

    // Try semantic color lookup
    if let Some(c) = theme.color(color) {
        return c;
    }

    // Try Tailwind color parsing
    if let Some(c) = parse_color(color) {
        return c;
    }

    // Return as-is (might be a CSS color name)
    color
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_theme() {
        let theme = get_theme();
        assert_eq!(theme.name, "dark");
        assert!(theme.is_dark());
    }

    #[test]
    fn test_set_theme() {
        set_theme(light_theme());
        let theme = get_theme();
        assert_eq!(theme.name, "light");
        assert!(theme.is_light());
        // Reset to default
        set_theme(dark_theme());
    }

    #[test]
    fn test_get_theme_by_name() {
        assert!(get_theme_by_name("dark").is_some());
        assert!(get_theme_by_name("light").is_some());
        assert!(get_theme_by_name("dracula").is_some());
        assert!(get_theme_by_name("nord").is_some());
        assert!(get_theme_by_name("monokai").is_some());
        assert!(get_theme_by_name("tokyo-night").is_some());
        assert!(get_theme_by_name("tokyo_night").is_some());
        assert!(get_theme_by_name("gruvbox").is_some());
        assert!(get_theme_by_name("solarized").is_some());
        assert!(get_theme_by_name("catppuccin").is_some());
        assert!(get_theme_by_name("high-contrast").is_some());
        assert!(get_theme_by_name("monochrome").is_some());
        assert!(get_theme_by_name("mono").is_some());
        assert!(get_theme_by_name("orange").is_some());
        assert!(get_theme_by_name("pink").is_some());
        assert!(get_theme_by_name("invalid").is_none());
    }

    #[test]
    fn test_list_themes() {
        let themes = list_themes();
        assert_eq!(themes.len(), 13);
        assert!(themes.contains(&"dark"));
        assert!(themes.contains(&"nord"));
        assert!(themes.contains(&"catppuccin"));
        assert!(themes.contains(&"monochrome"));
        assert!(themes.contains(&"orange"));
        assert!(themes.contains(&"pink"));
    }

    #[test]
    fn test_all_themes_valid() {
        // Ensure all themes can be created without panicking
        let _ = dark_theme();
        let _ = light_theme();
        let _ = dracula_theme();
        let _ = nord_theme();
        let _ = monokai_theme();
        let _ = tokyo_night_theme();
        let _ = gruvbox_theme();
        let _ = solarized_theme();
        let _ = catppuccin_theme();
        let _ = high_contrast_theme();
        let _ = monochrome_theme();
        let _ = orange_theme();
        let _ = pink_theme();
    }

    #[test]
    fn test_get_contrast_color() {
        assert_eq!(get_contrast_color("#ffffff"), BLACK);
        assert_eq!(get_contrast_color("#000000"), WHITE);
        assert_eq!(get_contrast_color("#3b82f6"), WHITE); // Blue
    }

    #[test]
    fn test_resolve_color() {
        let theme = dark_theme();

        // Hex passthrough
        assert_eq!(resolve_color("#ff0000", &theme), "#ff0000");

        // Semantic colors
        assert_eq!(resolve_color("primary", &theme), BLUE.s500);

        // Tailwind colors
        assert_eq!(resolve_color("blue-500", &theme), "#3b82f6");
    }
}
