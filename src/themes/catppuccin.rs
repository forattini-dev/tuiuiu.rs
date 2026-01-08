//! Catppuccin Theme (Mocha variant)
//!
//! Soothing pastel theme for the high-spirited!
//! https://catppuccin.com/

#![allow(dead_code)]

use super::colors::*;
use super::types::*;

// Catppuccin Mocha palette
const CAT_ROSEWATER: &str = "#f5e0dc";
const CAT_FLAMINGO: &str = "#f2cdcd";
const CAT_PINK: &str = "#f5c2e7";
const CAT_MAUVE: &str = "#cba6f7";
const CAT_RED: &str = "#f38ba8";
const CAT_MAROON: &str = "#eba0ac";
const CAT_PEACH: &str = "#fab387";
const CAT_YELLOW: &str = "#f9e2af";
const CAT_GREEN: &str = "#a6e3a1";
const CAT_TEAL: &str = "#94e2d5";
const CAT_SKY: &str = "#89dceb";
const CAT_SAPPHIRE: &str = "#74c7ec";
const CAT_BLUE: &str = "#89b4fa";
const CAT_LAVENDER: &str = "#b4befe";

// Base colors
const CAT_TEXT: &str = "#cdd6f4";
const CAT_SUBTEXT1: &str = "#bac2de";
const CAT_SUBTEXT0: &str = "#a6adc8";
const CAT_OVERLAY2: &str = "#9399b2";
const CAT_OVERLAY1: &str = "#7f849c";
const CAT_OVERLAY0: &str = "#6c7086";
const CAT_SURFACE2: &str = "#585b70";
const CAT_SURFACE1: &str = "#45475a";
const CAT_SURFACE0: &str = "#313244";
const CAT_BASE: &str = "#1e1e2e";
const CAT_MANTLE: &str = "#181825";
const CAT_CRUST: &str = "#11111b";

/// Create the Catppuccin Mocha theme
pub fn catppuccin_theme() -> Theme {
    let palette = ThemePalette {
        primary: PURPLE,
        secondary: SLATE,
        success: GREEN,
        warning: AMBER,
        danger: RED,
        neutral: SLATE,
    };

    Theme {
        name: "catppuccin",
        mode: ThemeMode::Dark,
        meta: ThemeMeta {
            version: "1.0.0",
            author: "tuiuiu",
            description: "Catppuccin Mocha theme - soothing pastel theme for the high-spirited",
        },
        palette,
        background: ThemeBackground {
            lowest: CAT_CRUST,
            base: CAT_BASE,
            subtle: CAT_MANTLE,
            surface: CAT_SURFACE0,
            raised: CAT_SURFACE1,
            elevated: CAT_SURFACE2,
            popover: CAT_SURFACE0,
            overlay: "rgba(0, 0, 0, 0.6)",
        },
        foreground: ThemeForeground {
            primary: CAT_TEXT,
            secondary: CAT_SUBTEXT1,
            muted: CAT_OVERLAY1,
            disabled: CAT_OVERLAY0,
            inverse: ThemeForegroundInverse {
                base: CAT_CRUST,
                soft: "rgba(17, 17, 27, 0.6)",
                subtle: "rgba(17, 17, 27, 0.35)",
            },
        },
        accents: ThemeAccents {
            positive: CAT_GREEN,
            warning: CAT_YELLOW,
            critical: CAT_RED,
            info: CAT_SAPPHIRE,
            highlight: CAT_MAUVE,
        },
        states: ThemeStates {
            hover: HoverState {
                bg: "rgba(203, 166, 247, 0.1)",
                fg: None,
            },
            active: ActiveState {
                bg: "rgba(203, 166, 247, 0.2)",
            },
            focus: FocusState {
                border: CAT_LAVENDER,
                ring: FocusRing {
                    color: CAT_LAVENDER,
                    width: 2,
                },
            },
            disabled: DisabledState {
                opacity: 0.4,
                bg: CAT_SURFACE0,
                fg: CAT_OVERLAY0,
            },
            selected: SelectedState {
                bg: CAT_MAUVE,
                fg: CAT_CRUST,
            },
        },
        borders: ThemeBorders {
            default: CAT_SURFACE1,
            subtle: CAT_SURFACE0,
            strong: CAT_SURFACE2,
            accent: CAT_MAUVE,
            danger: CAT_RED,
        },
        opacity: ThemeOpacity {
            disabled: 0.4,
            muted: 0.7,
            overlay: 0.5,
            ghost: 0.2,
        },
        components: ComponentTokens {
            button: ButtonTokens {
                primary: ButtonVariant {
                    bg: CAT_MAUVE,
                    fg: CAT_CRUST,
                    hover_bg: CAT_LAVENDER,
                    active_bg: "#a88be0",
                    border: "transparent",
                },
                secondary: ButtonVariant {
                    bg: CAT_SURFACE1,
                    fg: CAT_TEXT,
                    hover_bg: CAT_SURFACE2,
                    active_bg: CAT_SURFACE0,
                    border: "transparent",
                },
                outline: ButtonVariant {
                    bg: "transparent",
                    fg: CAT_MAUVE,
                    hover_bg: "rgba(203, 166, 247, 0.1)",
                    active_bg: "rgba(203, 166, 247, 0.2)",
                    border: CAT_MAUVE,
                },
                ghost: ButtonVariant {
                    bg: "transparent",
                    fg: CAT_TEXT,
                    hover_bg: "rgba(205, 214, 244, 0.05)",
                    active_bg: "rgba(205, 214, 244, 0.1)",
                    border: "transparent",
                },
            },
            panel: PanelTokens {
                bg: CAT_SURFACE0,
                header_bg: CAT_SURFACE1,
                footer_bg: CAT_SURFACE1,
                border: CAT_SURFACE2,
            },
            menu: MenuTokens {
                bg: CAT_SURFACE0,
                border: CAT_SURFACE2,
                item: MenuItemTokens {
                    fg: CAT_TEXT,
                    hover_bg: CAT_SURFACE1,
                    active_bg: CAT_MAUVE,
                    selected_bg: "#a88be0",
                    disabled_fg: CAT_OVERLAY0,
                },
            },
            tabs: TabsTokens {
                bg: CAT_SURFACE0,
                border: CAT_SURFACE2,
                tab: TabTokens {
                    fg: CAT_OVERLAY1,
                    active_fg: CAT_TEXT,
                    active_bg: CAT_SURFACE1,
                    hover_fg: CAT_TEXT,
                    indicator: CAT_MAUVE,
                },
            },
            dropdown: DropdownTokens {
                bg: CAT_SURFACE0,
                border: CAT_SURFACE2,
                item: DropdownItemTokens {
                    fg: CAT_TEXT,
                    hover_bg: CAT_SURFACE1,
                    selected_bg: CAT_MAUVE,
                },
            },
            input: InputTokens {
                bg: CAT_BASE,
                fg: CAT_TEXT,
                placeholder: CAT_OVERLAY1,
                border: CAT_SURFACE1,
                focus_border: CAT_LAVENDER,
                invalid_border: CAT_RED,
            },
            checkbox: CheckboxTokens {
                bg: CAT_BASE,
                border: CAT_SURFACE2,
                check_color: CAT_CRUST,
                checked_bg: CAT_GREEN,
            },
            radio: RadioTokens {
                bg: CAT_BASE,
                dot_color: CAT_GREEN,
                border: CAT_SURFACE2,
                checked_border: CAT_GREEN,
            },
            tooltip: TooltipTokens {
                bg: CAT_SURFACE1,
                fg: CAT_TEXT,
            },
            modal: ModalTokens {
                bg: CAT_SURFACE0,
                border: CAT_SURFACE2,
                overlay: "rgba(0, 0, 0, 0.6)",
            },
            badge: BadgeTokens {
                default: BadgeVariant {
                    bg: CAT_SURFACE1,
                    fg: CAT_TEXT,
                },
                success: BadgeVariant {
                    bg: CAT_GREEN,
                    fg: CAT_CRUST,
                },
                warning: BadgeVariant {
                    bg: CAT_YELLOW,
                    fg: CAT_CRUST,
                },
                danger: BadgeVariant {
                    bg: CAT_RED,
                    fg: CAT_CRUST,
                },
            },
            list: ListTokens {
                item: ListItemTokens {
                    bg: "transparent",
                    hover_bg: CAT_SURFACE1,
                    selected_bg: CAT_MAUVE,
                    fg: CAT_TEXT,
                },
            },
            header: HeaderTokens {
                default: HeaderVariant {
                    bg: CAT_SURFACE0,
                    fg: CAT_TEXT,
                    title_fg: CAT_TEXT,
                    subtitle_fg: CAT_OVERLAY1,
                    border: CAT_SURFACE1,
                },
                primary: HeaderVariant {
                    bg: CAT_MAUVE,
                    fg: CAT_CRUST,
                    title_fg: CAT_CRUST,
                    subtitle_fg: "rgba(17, 17, 27, 0.7)",
                    border: "#a88be0",
                },
                secondary: HeaderVariant {
                    bg: CAT_SURFACE1,
                    fg: CAT_TEXT,
                    title_fg: CAT_TEXT,
                    subtitle_fg: CAT_OVERLAY1,
                    border: CAT_SURFACE2,
                },
                success: HeaderVariant {
                    bg: CAT_GREEN,
                    fg: CAT_CRUST,
                    title_fg: CAT_CRUST,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: "#8bcc86",
                },
                warning: HeaderVariant {
                    bg: CAT_YELLOW,
                    fg: CAT_CRUST,
                    title_fg: CAT_CRUST,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: "#dcc899",
                },
                danger: HeaderVariant {
                    bg: CAT_RED,
                    fg: CAT_CRUST,
                    title_fg: CAT_CRUST,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: "#d6768f",
                },
            },
            statusbar: StatusbarTokens {
                default: StatusbarVariant {
                    bg: CAT_SURFACE0,
                    fg: CAT_OVERLAY1,
                },
                primary: StatusbarVariant {
                    bg: CAT_MAUVE,
                    fg: CAT_CRUST,
                },
                info: StatusbarVariant {
                    bg: CAT_SAPPHIRE,
                    fg: CAT_CRUST,
                },
                success: StatusbarVariant {
                    bg: CAT_GREEN,
                    fg: CAT_CRUST,
                },
                warning: StatusbarVariant {
                    bg: CAT_YELLOW,
                    fg: CAT_CRUST,
                },
                danger: StatusbarVariant {
                    bg: CAT_RED,
                    fg: CAT_CRUST,
                },
            },
            page: PageTokens {
                default: PageVariant {
                    bg: CAT_BASE,
                    title_fg: CAT_TEXT,
                    subtitle_fg: CAT_OVERLAY1,
                    border: CAT_SURFACE1,
                },
                primary: PageVariant {
                    bg: CAT_BASE,
                    title_fg: CAT_MAUVE,
                    subtitle_fg: CAT_OVERLAY1,
                    border: CAT_MAUVE,
                },
                secondary: PageVariant {
                    bg: CAT_BASE,
                    title_fg: CAT_PINK,
                    subtitle_fg: CAT_OVERLAY1,
                    border: CAT_SURFACE1,
                },
            },
            appshell: AppShellTokens {
                bg: CAT_CRUST,
                divider_fg: CAT_SURFACE0,
                sidebar_bg: CAT_BASE,
                aside_bg: CAT_BASE,
            },
            toast: ToastTokens {
                success: ToastVariant {
                    bg: "#1e2d2a",
                    fg: CAT_GREEN,
                    border: CAT_GREEN,
                    icon_fg: CAT_GREEN,
                },
                error: ToastVariant {
                    bg: "#2d1e24",
                    fg: CAT_RED,
                    border: CAT_RED,
                    icon_fg: CAT_RED,
                },
                warning: ToastVariant {
                    bg: "#2d2a1e",
                    fg: CAT_YELLOW,
                    border: CAT_YELLOW,
                    icon_fg: CAT_YELLOW,
                },
                info: ToastVariant {
                    bg: "#1e242d",
                    fg: CAT_SAPPHIRE,
                    border: CAT_SAPPHIRE,
                    icon_fg: CAT_SAPPHIRE,
                },
            },
            slider: SliderTokens {
                track_bg: CAT_SURFACE1,
                fill_bg: CAT_MAUVE,
                thumb: CAT_TEXT,
            },
        },
    }
}
