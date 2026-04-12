//! High Contrast Theme
//!
//! Accessibility-focused theme with maximum contrast ratios.
//! Designed for WCAG 2.1 AAA compliance.

#![allow(dead_code)]

use super::colors::*;
use super::types::*;

// High contrast colors - pure black and white with vivid accents
const HC_BG: &str = "#000000";
const HC_BG_RAISED: &str = "#1a1a1a";
const HC_BG_SURFACE: &str = "#262626";
const HC_FG: &str = "#ffffff";
const HC_FG_MUTED: &str = "#e0e0e0";
const HC_BORDER: &str = "#ffffff";

// Vivid accent colors (all meet WCAG AAA contrast on black)
const HC_YELLOW: &str = "#ffff00"; // Pure yellow - highest visibility
const HC_CYAN: &str = "#00ffff"; // Cyan for info
const HC_GREEN: &str = "#00ff00"; // Lime for success
const HC_RED: &str = "#ff0000"; // Pure red for danger
const HC_ORANGE: &str = "#ff8c00"; // Orange for warnings
const HC_BLUE: &str = "#00bfff"; // Deep sky blue
const HC_MAGENTA: &str = "#ff00ff"; // Magenta for highlights

/// Create the High Contrast theme
pub fn high_contrast_theme() -> Theme {
    let palette = ThemePalette {
        primary: YELLOW,
        secondary: NEUTRAL,
        success: GREEN,
        warning: AMBER,
        danger: RED,
        neutral: NEUTRAL,
    };

    Theme {
        name: "high-contrast",
        mode: ThemeMode::Dark,
        meta: ThemeMeta {
            version: "1.0.0",
            author: "tuiuiu",
            description: "High Contrast theme - accessibility-focused with maximum contrast ratios",
        },
        palette,
        background: ThemeBackground {
            lowest: HC_BG,
            base: HC_BG,
            subtle: "#0d0d0d",
            surface: HC_BG_RAISED,
            raised: HC_BG_SURFACE,
            elevated: "#333333",
            popover: HC_BG_RAISED,
            overlay: "rgba(0, 0, 0, 0.85)",
        },
        foreground: ThemeForeground {
            primary: HC_FG,
            secondary: HC_FG_MUTED,
            muted: "#b3b3b3",
            disabled: "#666666",
            inverse: ThemeForegroundInverse {
                base: HC_BG,
                soft: "rgba(0, 0, 0, 0.7)",
                subtle: "rgba(0, 0, 0, 0.5)",
            },
        },
        accents: ThemeAccents {
            positive: HC_GREEN,
            warning: HC_ORANGE,
            critical: HC_RED,
            info: HC_CYAN,
            highlight: HC_YELLOW,
        },
        states: ThemeStates {
            hover: HoverState {
                bg: HC_BG_RAISED,
                fg: Some(HC_YELLOW),
            },
            active: ActiveState { bg: HC_BG_SURFACE },
            focus: FocusState {
                border: HC_YELLOW,
                ring: FocusRing {
                    color: HC_YELLOW,
                    width: 3, // Thicker for visibility
                },
            },
            disabled: DisabledState {
                opacity: 0.5,
                bg: HC_BG_RAISED,
                fg: "#666666",
            },
            selected: SelectedState {
                bg: HC_YELLOW,
                fg: HC_BG,
            },
        },
        borders: ThemeBorders {
            default: HC_BORDER,
            subtle: "#808080",
            strong: HC_FG,
            accent: HC_YELLOW,
            danger: HC_RED,
        },
        opacity: ThemeOpacity {
            disabled: 0.5,
            muted: 0.8,
            overlay: 0.85,
            ghost: 0.3,
        },
        components: ComponentTokens {
            button: ButtonTokens {
                primary: ButtonVariant {
                    bg: HC_YELLOW,
                    fg: HC_BG,
                    hover_bg: HC_FG,
                    active_bg: "#cccc00",
                    border: HC_YELLOW,
                },
                secondary: ButtonVariant {
                    bg: HC_BG,
                    fg: HC_FG,
                    hover_bg: HC_BG_RAISED,
                    active_bg: HC_BG_SURFACE,
                    border: HC_FG,
                },
                outline: ButtonVariant {
                    bg: "transparent",
                    fg: HC_YELLOW,
                    hover_bg: HC_BG_RAISED,
                    active_bg: HC_BG_SURFACE,
                    border: HC_YELLOW,
                },
                ghost: ButtonVariant {
                    bg: "transparent",
                    fg: HC_FG,
                    hover_bg: HC_BG_RAISED,
                    active_bg: HC_BG_SURFACE,
                    border: "transparent",
                },
            },
            panel: PanelTokens {
                bg: HC_BG_RAISED,
                header_bg: HC_BG_SURFACE,
                footer_bg: HC_BG_SURFACE,
                border: HC_FG,
            },
            menu: MenuTokens {
                bg: HC_BG_RAISED,
                border: HC_FG,
                item: MenuItemTokens {
                    fg: HC_FG,
                    hover_bg: HC_BG_SURFACE,
                    active_bg: HC_YELLOW,
                    selected_bg: HC_YELLOW,
                    disabled_fg: "#666666",
                },
            },
            tabs: TabsTokens {
                bg: HC_BG_RAISED,
                border: HC_FG,
                tab: TabTokens {
                    fg: HC_FG_MUTED,
                    active_fg: HC_YELLOW,
                    active_bg: HC_BG_SURFACE,
                    hover_fg: HC_FG,
                    indicator: HC_YELLOW,
                },
            },
            dropdown: DropdownTokens {
                bg: HC_BG_RAISED,
                border: HC_FG,
                item: DropdownItemTokens {
                    fg: HC_FG,
                    hover_bg: HC_BG_SURFACE,
                    selected_bg: HC_YELLOW,
                },
            },
            input: InputTokens {
                bg: HC_BG,
                fg: HC_FG,
                placeholder: "#808080",
                border: HC_FG,
                focus_border: HC_YELLOW,
                invalid_border: HC_RED,
            },
            checkbox: CheckboxTokens {
                bg: HC_BG,
                border: HC_FG,
                check_color: HC_BG,
                checked_bg: HC_YELLOW,
            },
            radio: RadioTokens {
                bg: HC_BG,
                dot_color: HC_YELLOW,
                border: HC_FG,
                checked_border: HC_YELLOW,
            },
            tooltip: TooltipTokens {
                bg: HC_BG_SURFACE,
                fg: HC_FG,
            },
            modal: ModalTokens {
                bg: HC_BG_RAISED,
                border: HC_FG,
                overlay: "rgba(0, 0, 0, 0.85)",
            },
            badge: BadgeTokens {
                default: BadgeVariant {
                    bg: HC_BG_SURFACE,
                    fg: HC_FG,
                },
                success: BadgeVariant {
                    bg: HC_GREEN,
                    fg: HC_BG,
                },
                warning: BadgeVariant {
                    bg: HC_ORANGE,
                    fg: HC_BG,
                },
                danger: BadgeVariant {
                    bg: HC_RED,
                    fg: HC_FG,
                },
            },
            list: ListTokens {
                item: ListItemTokens {
                    bg: "transparent",
                    hover_bg: HC_BG_RAISED,
                    selected_bg: HC_YELLOW,
                    fg: HC_FG,
                },
            },
            header: HeaderTokens {
                default: HeaderVariant {
                    bg: HC_BG_RAISED,
                    fg: HC_FG,
                    title_fg: HC_FG,
                    subtitle_fg: HC_FG_MUTED,
                    border: HC_FG,
                },
                primary: HeaderVariant {
                    bg: HC_YELLOW,
                    fg: HC_BG,
                    title_fg: HC_BG,
                    subtitle_fg: "#333333",
                    border: HC_YELLOW,
                },
                secondary: HeaderVariant {
                    bg: HC_BG_SURFACE,
                    fg: HC_FG,
                    title_fg: HC_FG,
                    subtitle_fg: HC_FG_MUTED,
                    border: HC_FG,
                },
                success: HeaderVariant {
                    bg: HC_GREEN,
                    fg: HC_BG,
                    title_fg: HC_BG,
                    subtitle_fg: "#003300",
                    border: HC_GREEN,
                },
                warning: HeaderVariant {
                    bg: HC_ORANGE,
                    fg: HC_BG,
                    title_fg: HC_BG,
                    subtitle_fg: "#332200",
                    border: HC_ORANGE,
                },
                danger: HeaderVariant {
                    bg: HC_RED,
                    fg: HC_FG,
                    title_fg: HC_FG,
                    subtitle_fg: HC_FG_MUTED,
                    border: HC_RED,
                },
            },
            statusbar: StatusbarTokens {
                default: StatusbarVariant {
                    bg: HC_BG_RAISED,
                    fg: HC_FG_MUTED,
                },
                primary: StatusbarVariant {
                    bg: HC_YELLOW,
                    fg: HC_BG,
                },
                info: StatusbarVariant {
                    bg: HC_CYAN,
                    fg: HC_BG,
                },
                success: StatusbarVariant {
                    bg: HC_GREEN,
                    fg: HC_BG,
                },
                warning: StatusbarVariant {
                    bg: HC_ORANGE,
                    fg: HC_BG,
                },
                danger: StatusbarVariant {
                    bg: HC_RED,
                    fg: HC_FG,
                },
            },
            page: PageTokens {
                default: PageVariant {
                    bg: HC_BG,
                    title_fg: HC_FG,
                    subtitle_fg: HC_FG_MUTED,
                    border: HC_FG,
                },
                primary: PageVariant {
                    bg: HC_BG,
                    title_fg: HC_YELLOW,
                    subtitle_fg: HC_FG_MUTED,
                    border: HC_YELLOW,
                },
                secondary: PageVariant {
                    bg: HC_BG,
                    title_fg: HC_CYAN,
                    subtitle_fg: HC_FG_MUTED,
                    border: HC_FG,
                },
            },
            appshell: AppShellTokens {
                bg: HC_BG,
                divider_fg: HC_FG,
                sidebar_bg: HC_BG_RAISED,
                aside_bg: HC_BG_RAISED,
            },
            toast: ToastTokens {
                success: ToastVariant {
                    bg: "#003300",
                    fg: HC_GREEN,
                    border: HC_GREEN,
                    icon_fg: HC_GREEN,
                },
                error: ToastVariant {
                    bg: "#330000",
                    fg: HC_RED,
                    border: HC_RED,
                    icon_fg: HC_RED,
                },
                warning: ToastVariant {
                    bg: "#332200",
                    fg: HC_ORANGE,
                    border: HC_ORANGE,
                    icon_fg: HC_ORANGE,
                },
                info: ToastVariant {
                    bg: "#003333",
                    fg: HC_CYAN,
                    border: HC_CYAN,
                    icon_fg: HC_CYAN,
                },
            },
            slider: SliderTokens {
                track_bg: HC_BG_SURFACE,
                fill_bg: HC_YELLOW,
                thumb: HC_FG,
            },
        },
    }
}
