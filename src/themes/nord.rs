//! Nord Theme
//!
//! An arctic, north-bluish color palette.
//! https://www.nordtheme.com/

#![allow(dead_code)]

use super::colors::*;
use super::types::*;

// Nord color palette
// Polar Night (dark backgrounds)
const NORD0: &str = "#2e3440"; // Darkest
const NORD1: &str = "#3b4252"; // Dark
const NORD2: &str = "#434c5e"; // Medium dark
const NORD3: &str = "#4c566a"; // Light dark

// Snow Storm (light foregrounds)
const NORD4: &str = "#d8dee9"; // Dark white
const NORD5: &str = "#e5e9f0"; // White
const NORD6: &str = "#eceff4"; // Bright white

// Frost (accent blues)
const NORD7: &str = "#8fbcbb"; // Teal
const NORD8: &str = "#88c0d0"; // Cyan
const NORD9: &str = "#81a1c1"; // Light blue
const NORD10: &str = "#5e81ac"; // Blue

// Aurora (accents)
const NORD11: &str = "#bf616a"; // Red
const NORD12: &str = "#d08770"; // Orange
const NORD13: &str = "#ebcb8b"; // Yellow
const NORD14: &str = "#a3be8c"; // Green
const NORD15: &str = "#b48ead"; // Purple

/// Create the Nord theme
pub fn nord_theme() -> Theme {
    let palette = ThemePalette {
        primary: BLUE,
        secondary: SLATE,
        success: GREEN,
        warning: AMBER,
        danger: RED,
        neutral: SLATE,
    };

    Theme {
        name: "nord",
        mode: ThemeMode::Dark,
        meta: ThemeMeta {
            version: "1.0.0",
            author: "tuiuiu",
            description: "Nord theme - an arctic, north-bluish color palette",
        },
        palette,
        background: ThemeBackground {
            lowest: "#242933",
            base: NORD0,
            subtle: NORD1,
            surface: NORD1,
            raised: NORD2,
            elevated: NORD3,
            popover: NORD2,
            overlay: "rgba(0, 0, 0, 0.6)",
        },
        foreground: ThemeForeground {
            primary: NORD4,
            secondary: NORD5,
            muted: NORD3,
            disabled: "#5c6370",
            inverse: ThemeForegroundInverse {
                base: WHITE,
                soft: "rgba(255, 255, 255, 0.6)",
                subtle: "rgba(255, 255, 255, 0.35)",
            },
        },
        accents: ThemeAccents {
            positive: NORD14,
            warning: NORD13,
            critical: NORD11,
            info: NORD8,
            highlight: NORD9,
        },
        states: ThemeStates {
            hover: HoverState {
                bg: "rgba(136, 192, 208, 0.1)",
                fg: None,
            },
            active: ActiveState {
                bg: "rgba(136, 192, 208, 0.15)",
            },
            focus: FocusState {
                border: NORD8,
                ring: FocusRing {
                    color: NORD8,
                    width: 2,
                },
            },
            disabled: DisabledState {
                opacity: 0.4,
                bg: NORD1,
                fg: NORD3,
            },
            selected: SelectedState {
                bg: NORD10,
                fg: NORD6,
            },
        },
        borders: ThemeBorders {
            default: NORD2,
            subtle: NORD1,
            strong: NORD3,
            accent: NORD9,
            danger: NORD11,
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
                    bg: NORD10,
                    fg: NORD6,
                    hover_bg: NORD9,
                    active_bg: "#4e6d8c",
                    border: "transparent",
                },
                secondary: ButtonVariant {
                    bg: NORD2,
                    fg: NORD4,
                    hover_bg: NORD3,
                    active_bg: NORD1,
                    border: "transparent",
                },
                outline: ButtonVariant {
                    bg: "transparent",
                    fg: NORD9,
                    hover_bg: "rgba(129, 161, 193, 0.1)",
                    active_bg: "rgba(129, 161, 193, 0.2)",
                    border: NORD9,
                },
                ghost: ButtonVariant {
                    bg: "transparent",
                    fg: NORD4,
                    hover_bg: "rgba(255, 255, 255, 0.05)",
                    active_bg: "rgba(255, 255, 255, 0.1)",
                    border: "transparent",
                },
            },
            panel: PanelTokens {
                bg: NORD1,
                header_bg: NORD2,
                footer_bg: NORD2,
                border: NORD3,
            },
            menu: MenuTokens {
                bg: NORD1,
                border: NORD3,
                item: MenuItemTokens {
                    fg: NORD4,
                    hover_bg: NORD2,
                    active_bg: NORD10,
                    selected_bg: NORD9,
                    disabled_fg: NORD3,
                },
            },
            tabs: TabsTokens {
                bg: NORD1,
                border: NORD3,
                tab: TabTokens {
                    fg: NORD3,
                    active_fg: NORD4,
                    active_bg: NORD2,
                    hover_fg: NORD4,
                    indicator: NORD8,
                },
            },
            dropdown: DropdownTokens {
                bg: NORD1,
                border: NORD3,
                item: DropdownItemTokens {
                    fg: NORD4,
                    hover_bg: NORD2,
                    selected_bg: NORD10,
                },
            },
            input: InputTokens {
                bg: NORD0,
                fg: NORD4,
                placeholder: NORD3,
                border: NORD2,
                focus_border: NORD8,
                invalid_border: NORD11,
            },
            checkbox: CheckboxTokens {
                bg: NORD0,
                border: NORD3,
                check_color: NORD0,
                checked_bg: NORD8,
            },
            radio: RadioTokens {
                bg: NORD0,
                dot_color: NORD8,
                border: NORD3,
                checked_border: NORD8,
            },
            tooltip: TooltipTokens {
                bg: NORD2,
                fg: NORD4,
            },
            modal: ModalTokens {
                bg: NORD1,
                border: NORD3,
                overlay: "rgba(0, 0, 0, 0.6)",
            },
            badge: BadgeTokens {
                default: BadgeVariant {
                    bg: NORD2,
                    fg: NORD4,
                },
                success: BadgeVariant {
                    bg: NORD14,
                    fg: NORD0,
                },
                warning: BadgeVariant {
                    bg: NORD13,
                    fg: NORD0,
                },
                danger: BadgeVariant {
                    bg: NORD11,
                    fg: WHITE,
                },
            },
            list: ListTokens {
                item: ListItemTokens {
                    bg: "transparent",
                    hover_bg: NORD2,
                    selected_bg: NORD10,
                    fg: NORD4,
                },
            },
            header: HeaderTokens {
                default: HeaderVariant {
                    bg: NORD1,
                    fg: NORD4,
                    title_fg: NORD4,
                    subtitle_fg: NORD3,
                    border: NORD2,
                },
                primary: HeaderVariant {
                    bg: NORD10,
                    fg: NORD6,
                    title_fg: NORD6,
                    subtitle_fg: "rgba(236, 239, 244, 0.7)",
                    border: NORD9,
                },
                secondary: HeaderVariant {
                    bg: NORD2,
                    fg: NORD4,
                    title_fg: NORD4,
                    subtitle_fg: NORD3,
                    border: NORD3,
                },
                success: HeaderVariant {
                    bg: NORD14,
                    fg: NORD0,
                    title_fg: NORD0,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: "#8fb07c",
                },
                warning: HeaderVariant {
                    bg: NORD13,
                    fg: NORD0,
                    title_fg: NORD0,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: "#d4b67b",
                },
                danger: HeaderVariant {
                    bg: NORD11,
                    fg: WHITE,
                    title_fg: WHITE,
                    subtitle_fg: "rgba(255, 255, 255, 0.8)",
                    border: "#a5515a",
                },
            },
            statusbar: StatusbarTokens {
                default: StatusbarVariant {
                    bg: NORD1,
                    fg: NORD3,
                },
                primary: StatusbarVariant {
                    bg: NORD10,
                    fg: NORD6,
                },
                info: StatusbarVariant {
                    bg: NORD8,
                    fg: NORD0,
                },
                success: StatusbarVariant {
                    bg: NORD14,
                    fg: NORD0,
                },
                warning: StatusbarVariant {
                    bg: NORD13,
                    fg: NORD0,
                },
                danger: StatusbarVariant {
                    bg: NORD11,
                    fg: WHITE,
                },
            },
            page: PageTokens {
                default: PageVariant {
                    bg: NORD0,
                    title_fg: NORD4,
                    subtitle_fg: NORD3,
                    border: NORD2,
                },
                primary: PageVariant {
                    bg: NORD0,
                    title_fg: NORD8,
                    subtitle_fg: NORD3,
                    border: NORD9,
                },
                secondary: PageVariant {
                    bg: NORD0,
                    title_fg: NORD15,
                    subtitle_fg: NORD3,
                    border: NORD2,
                },
            },
            appshell: AppShellTokens {
                bg: "#242933",
                divider_fg: NORD2,
                sidebar_bg: NORD0,
                aside_bg: NORD0,
            },
            toast: ToastTokens {
                success: ToastVariant {
                    bg: "#2a3830",
                    fg: NORD14,
                    border: NORD14,
                    icon_fg: NORD14,
                },
                error: ToastVariant {
                    bg: "#3a2830",
                    fg: NORD11,
                    border: NORD11,
                    icon_fg: NORD11,
                },
                warning: ToastVariant {
                    bg: "#3a3528",
                    fg: NORD13,
                    border: NORD13,
                    icon_fg: NORD13,
                },
                info: ToastVariant {
                    bg: "#283038",
                    fg: NORD8,
                    border: NORD8,
                    icon_fg: NORD8,
                },
            },
            slider: SliderTokens {
                track_bg: NORD2,
                fill_bg: NORD8,
                thumb: NORD4,
            },
        },
    }
}
