//! Theme Types
//!
//! Type definitions for the theming system.

use super::colors::ColorScale;

/// Theme mode (dark or light)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
}

/// Theme metadata
#[derive(Debug, Clone)]
pub struct ThemeMeta {
    pub version: &'static str,
    pub author: &'static str,
    pub description: &'static str,
}

/// Theme palette with semantic color scales
#[derive(Debug, Clone, Copy)]
pub struct ThemePalette {
    pub primary: ColorScale,
    pub secondary: ColorScale,
    pub success: ColorScale,
    pub warning: ColorScale,
    pub danger: ColorScale,
    pub neutral: ColorScale,
}

/// Background colors
#[derive(Debug, Clone, Copy)]
pub struct ThemeBackground {
    pub lowest: &'static str,
    pub base: &'static str,
    pub subtle: &'static str,
    pub surface: &'static str,
    pub raised: &'static str,
    pub elevated: &'static str,
    pub popover: &'static str,
    pub overlay: &'static str,
}

/// Inverse foreground colors
#[derive(Debug, Clone, Copy)]
pub struct ThemeForegroundInverse {
    pub base: &'static str,
    pub soft: &'static str,
    pub subtle: &'static str,
}

/// Foreground colors
#[derive(Debug, Clone, Copy)]
pub struct ThemeForeground {
    pub primary: &'static str,
    pub secondary: &'static str,
    pub muted: &'static str,
    pub disabled: &'static str,
    pub inverse: ThemeForegroundInverse,
}

/// Accent colors
#[derive(Debug, Clone, Copy)]
pub struct ThemeAccents {
    pub positive: &'static str,
    pub warning: &'static str,
    pub critical: &'static str,
    pub info: &'static str,
    pub highlight: &'static str,
}

/// Hover state
#[derive(Debug, Clone, Copy)]
pub struct HoverState {
    pub bg: &'static str,
    pub fg: Option<&'static str>,
}

/// Active state
#[derive(Debug, Clone, Copy)]
pub struct ActiveState {
    pub bg: &'static str,
}

/// Focus ring
#[derive(Debug, Clone, Copy)]
pub struct FocusRing {
    pub color: &'static str,
    pub width: u8,
}

/// Focus state
#[derive(Debug, Clone, Copy)]
pub struct FocusState {
    pub border: &'static str,
    pub ring: FocusRing,
}

/// Disabled state
#[derive(Debug, Clone, Copy)]
pub struct DisabledState {
    pub opacity: f32,
    pub bg: &'static str,
    pub fg: &'static str,
}

/// Selected state
#[derive(Debug, Clone, Copy)]
pub struct SelectedState {
    pub bg: &'static str,
    pub fg: &'static str,
}

/// Theme states
#[derive(Debug, Clone, Copy)]
pub struct ThemeStates {
    pub hover: HoverState,
    pub active: ActiveState,
    pub focus: FocusState,
    pub disabled: DisabledState,
    pub selected: SelectedState,
}

/// Border colors
#[derive(Debug, Clone, Copy)]
pub struct ThemeBorders {
    pub default: &'static str,
    pub subtle: &'static str,
    pub strong: &'static str,
    pub accent: &'static str,
    pub danger: &'static str,
}

/// Opacity values
#[derive(Debug, Clone, Copy)]
pub struct ThemeOpacity {
    pub disabled: f32,
    pub muted: f32,
    pub overlay: f32,
    pub ghost: f32,
}

// =============================================================================
// Component Tokens
// =============================================================================

/// Button variant colors
#[derive(Debug, Clone, Copy)]
pub struct ButtonVariant {
    pub bg: &'static str,
    pub fg: &'static str,
    pub hover_bg: &'static str,
    pub active_bg: &'static str,
    pub border: &'static str,
}

/// Button component tokens
#[derive(Debug, Clone, Copy)]
pub struct ButtonTokens {
    pub primary: ButtonVariant,
    pub secondary: ButtonVariant,
    pub outline: ButtonVariant,
    pub ghost: ButtonVariant,
}

/// Panel component tokens
#[derive(Debug, Clone, Copy)]
pub struct PanelTokens {
    pub bg: &'static str,
    pub header_bg: &'static str,
    pub footer_bg: &'static str,
    pub border: &'static str,
}

/// Menu item tokens
#[derive(Debug, Clone, Copy)]
pub struct MenuItemTokens {
    pub fg: &'static str,
    pub hover_bg: &'static str,
    pub active_bg: &'static str,
    pub selected_bg: &'static str,
    pub disabled_fg: &'static str,
}

/// Menu component tokens
#[derive(Debug, Clone, Copy)]
pub struct MenuTokens {
    pub bg: &'static str,
    pub border: &'static str,
    pub item: MenuItemTokens,
}

/// Tab tokens
#[derive(Debug, Clone, Copy)]
pub struct TabTokens {
    pub fg: &'static str,
    pub active_fg: &'static str,
    pub active_bg: &'static str,
    pub hover_fg: &'static str,
    pub indicator: &'static str,
}

/// Tabs component tokens
#[derive(Debug, Clone, Copy)]
pub struct TabsTokens {
    pub bg: &'static str,
    pub border: &'static str,
    pub tab: TabTokens,
}

/// Dropdown item tokens
#[derive(Debug, Clone, Copy)]
pub struct DropdownItemTokens {
    pub fg: &'static str,
    pub hover_bg: &'static str,
    pub selected_bg: &'static str,
}

/// Dropdown component tokens
#[derive(Debug, Clone, Copy)]
pub struct DropdownTokens {
    pub bg: &'static str,
    pub border: &'static str,
    pub item: DropdownItemTokens,
}

/// Input component tokens
#[derive(Debug, Clone, Copy)]
pub struct InputTokens {
    pub bg: &'static str,
    pub fg: &'static str,
    pub placeholder: &'static str,
    pub border: &'static str,
    pub focus_border: &'static str,
    pub invalid_border: &'static str,
}

/// Checkbox component tokens
#[derive(Debug, Clone, Copy)]
pub struct CheckboxTokens {
    pub bg: &'static str,
    pub border: &'static str,
    pub check_color: &'static str,
    pub checked_bg: &'static str,
}

/// Radio component tokens
#[derive(Debug, Clone, Copy)]
pub struct RadioTokens {
    pub bg: &'static str,
    pub dot_color: &'static str,
    pub border: &'static str,
    pub checked_border: &'static str,
}

/// Tooltip component tokens
#[derive(Debug, Clone, Copy)]
pub struct TooltipTokens {
    pub bg: &'static str,
    pub fg: &'static str,
}

/// Modal component tokens
#[derive(Debug, Clone, Copy)]
pub struct ModalTokens {
    pub bg: &'static str,
    pub border: &'static str,
    pub overlay: &'static str,
}

/// Badge variant tokens
#[derive(Debug, Clone, Copy)]
pub struct BadgeVariant {
    pub bg: &'static str,
    pub fg: &'static str,
}

/// Badge component tokens
#[derive(Debug, Clone, Copy)]
pub struct BadgeTokens {
    pub default: BadgeVariant,
    pub success: BadgeVariant,
    pub warning: BadgeVariant,
    pub danger: BadgeVariant,
}

/// List item tokens
#[derive(Debug, Clone, Copy)]
pub struct ListItemTokens {
    pub bg: &'static str,
    pub hover_bg: &'static str,
    pub selected_bg: &'static str,
    pub fg: &'static str,
}

/// List component tokens
#[derive(Debug, Clone, Copy)]
pub struct ListTokens {
    pub item: ListItemTokens,
}

/// Header variant tokens
#[derive(Debug, Clone, Copy)]
pub struct HeaderVariant {
    pub bg: &'static str,
    pub fg: &'static str,
    pub title_fg: &'static str,
    pub subtitle_fg: &'static str,
    pub border: &'static str,
}

/// Header component tokens
#[derive(Debug, Clone, Copy)]
pub struct HeaderTokens {
    pub default: HeaderVariant,
    pub primary: HeaderVariant,
    pub secondary: HeaderVariant,
    pub success: HeaderVariant,
    pub warning: HeaderVariant,
    pub danger: HeaderVariant,
}

/// Statusbar variant tokens
#[derive(Debug, Clone, Copy)]
pub struct StatusbarVariant {
    pub bg: &'static str,
    pub fg: &'static str,
}

/// Statusbar component tokens
#[derive(Debug, Clone, Copy)]
pub struct StatusbarTokens {
    pub default: StatusbarVariant,
    pub primary: StatusbarVariant,
    pub info: StatusbarVariant,
    pub success: StatusbarVariant,
    pub warning: StatusbarVariant,
    pub danger: StatusbarVariant,
}

/// Page variant tokens
#[derive(Debug, Clone, Copy)]
pub struct PageVariant {
    pub bg: &'static str,
    pub title_fg: &'static str,
    pub subtitle_fg: &'static str,
    pub border: &'static str,
}

/// Page component tokens
#[derive(Debug, Clone, Copy)]
pub struct PageTokens {
    pub default: PageVariant,
    pub primary: PageVariant,
    pub secondary: PageVariant,
}

/// AppShell component tokens
#[derive(Debug, Clone, Copy)]
pub struct AppShellTokens {
    pub bg: &'static str,
    pub divider_fg: &'static str,
    pub sidebar_bg: &'static str,
    pub aside_bg: &'static str,
}

/// Toast variant tokens
#[derive(Debug, Clone, Copy)]
pub struct ToastVariant {
    pub bg: &'static str,
    pub fg: &'static str,
    pub border: &'static str,
    pub icon_fg: &'static str,
}

/// Toast component tokens
#[derive(Debug, Clone, Copy)]
pub struct ToastTokens {
    pub success: ToastVariant,
    pub error: ToastVariant,
    pub warning: ToastVariant,
    pub info: ToastVariant,
}

/// Slider component tokens
#[derive(Debug, Clone, Copy)]
pub struct SliderTokens {
    /// Track background (unfilled portion)
    pub track_bg: &'static str,
    /// Fill background (filled portion)
    pub fill_bg: &'static str,
    /// Thumb color
    pub thumb: &'static str,
}

/// All component tokens
#[derive(Debug, Clone, Copy)]
pub struct ComponentTokens {
    pub button: ButtonTokens,
    pub panel: PanelTokens,
    pub menu: MenuTokens,
    pub tabs: TabsTokens,
    pub dropdown: DropdownTokens,
    pub input: InputTokens,
    pub checkbox: CheckboxTokens,
    pub radio: RadioTokens,
    pub tooltip: TooltipTokens,
    pub modal: ModalTokens,
    pub badge: BadgeTokens,
    pub list: ListTokens,
    pub header: HeaderTokens,
    pub statusbar: StatusbarTokens,
    pub page: PageTokens,
    pub appshell: AppShellTokens,
    pub toast: ToastTokens,
    pub slider: SliderTokens,
}

// =============================================================================
// Main Theme Structure
// =============================================================================

/// Complete theme definition
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    pub mode: ThemeMode,
    pub meta: ThemeMeta,
    pub palette: ThemePalette,
    pub background: ThemeBackground,
    pub foreground: ThemeForeground,
    pub accents: ThemeAccents,
    pub states: ThemeStates,
    pub borders: ThemeBorders,
    pub opacity: ThemeOpacity,
    pub components: ComponentTokens,
}

impl Theme {
    /// Get a color by semantic name
    pub fn color(&self, name: &str) -> Option<&'static str> {
        match name {
            "primary" => Some(self.palette.primary.s500),
            "secondary" => Some(self.palette.secondary.s500),
            "success" => Some(self.palette.success.s500),
            "warning" => Some(self.palette.warning.s500),
            "danger" => Some(self.palette.danger.s500),
            "bg" => Some(self.background.base),
            "fg" => Some(self.foreground.primary),
            "muted" => Some(self.foreground.muted),
            "border" => Some(self.borders.default),
            _ => None,
        }
    }

    /// Check if theme is dark mode
    pub fn is_dark(&self) -> bool {
        matches!(self.mode, ThemeMode::Dark)
    }

    /// Check if theme is light mode
    pub fn is_light(&self) -> bool {
        matches!(self.mode, ThemeMode::Light)
    }
}
