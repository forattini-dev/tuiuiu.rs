//! Solarized Dark Theme
//!
//! Precision colors for machines and people.
//! https://ethanschoonover.com/solarized/

#![allow(dead_code)]

use super::colors::*;
use super::types::*;

// Solarized base colors
const SOL_BASE03: &str = "#002b36"; // Background (darkest)
const SOL_BASE02: &str = "#073642"; // Background highlights
const SOL_BASE01: &str = "#586e75"; // Comments, secondary content
const SOL_BASE00: &str = "#657b83"; // (Light mode: body text)
const SOL_BASE0: &str = "#839496"; // Body text
const SOL_BASE1: &str = "#93a1a1"; // Optional emphasized content
const SOL_BASE2: &str = "#eee8d5"; // (Light mode: background highlights)
const SOL_BASE3: &str = "#fdf6e3"; // (Light mode: background)

// Solarized accent colors
const SOL_YELLOW: &str = "#b58900";
const SOL_ORANGE: &str = "#cb4b16";
const SOL_RED: &str = "#dc322f";
const SOL_MAGENTA: &str = "#d33682";
const SOL_VIOLET: &str = "#6c71c4";
const SOL_BLUE: &str = "#268bd2";
const SOL_CYAN: &str = "#2aa198";
const SOL_GREEN: &str = "#859900";

/// Create the Solarized Dark theme
pub fn solarized_theme() -> Theme {
    let palette = ThemePalette {
        primary: CYAN,
        secondary: SLATE,
        success: GREEN,
        warning: AMBER,
        danger: RED,
        neutral: SLATE,
    };

    Theme {
        name: "solarized",
        mode: ThemeMode::Dark,
        meta: ThemeMeta {
            version: "1.0.0",
            author: "tuiuiu",
            description: "Solarized Dark theme - precision colors for machines and people",
        },
        palette,
        background: ThemeBackground {
            lowest: "#001e26",
            base: SOL_BASE03,
            subtle: SOL_BASE02,
            surface: SOL_BASE02,
            raised: "#0a4050",
            elevated: "#0d4d5e",
            popover: SOL_BASE02,
            overlay: "rgba(0, 0, 0, 0.6)",
        },
        foreground: ThemeForeground {
            primary: SOL_BASE0,
            secondary: SOL_BASE1,
            muted: SOL_BASE01,
            disabled: "#4a5d64",
            inverse: ThemeForegroundInverse {
                base: SOL_BASE3,
                soft: "rgba(253, 246, 227, 0.6)",
                subtle: "rgba(253, 246, 227, 0.35)",
            },
        },
        accents: ThemeAccents {
            positive: SOL_GREEN,
            warning: SOL_YELLOW,
            critical: SOL_RED,
            info: SOL_BLUE,
            highlight: SOL_CYAN,
        },
        states: ThemeStates {
            hover: HoverState {
                bg: "rgba(38, 139, 210, 0.15)",
                fg: None,
            },
            active: ActiveState {
                bg: "rgba(38, 139, 210, 0.25)",
            },
            focus: FocusState {
                border: SOL_BLUE,
                ring: FocusRing {
                    color: SOL_BLUE,
                    width: 2,
                },
            },
            disabled: DisabledState {
                opacity: 0.4,
                bg: SOL_BASE02,
                fg: SOL_BASE01,
            },
            selected: SelectedState {
                bg: SOL_BLUE,
                fg: SOL_BASE03,
            },
        },
        borders: ThemeBorders {
            default: SOL_BASE02,
            subtle: "#04323f",
            strong: SOL_BASE01,
            accent: SOL_CYAN,
            danger: SOL_RED,
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
                    bg: SOL_BLUE,
                    fg: SOL_BASE03,
                    hover_bg: "#3a9be5",
                    active_bg: "#1f78b4",
                    border: "transparent",
                },
                secondary: ButtonVariant {
                    bg: SOL_BASE02,
                    fg: SOL_BASE0,
                    hover_bg: "#0a4050",
                    active_bg: "#04323f",
                    border: "transparent",
                },
                outline: ButtonVariant {
                    bg: "transparent",
                    fg: SOL_CYAN,
                    hover_bg: "rgba(42, 161, 152, 0.15)",
                    active_bg: "rgba(42, 161, 152, 0.25)",
                    border: SOL_CYAN,
                },
                ghost: ButtonVariant {
                    bg: "transparent",
                    fg: SOL_BASE0,
                    hover_bg: "rgba(131, 148, 150, 0.1)",
                    active_bg: "rgba(131, 148, 150, 0.2)",
                    border: "transparent",
                },
            },
            panel: PanelTokens {
                bg: SOL_BASE02,
                header_bg: "#0a4050",
                footer_bg: "#0a4050",
                border: SOL_BASE01,
            },
            menu: MenuTokens {
                bg: SOL_BASE02,
                border: SOL_BASE01,
                item: MenuItemTokens {
                    fg: SOL_BASE0,
                    hover_bg: "#0a4050",
                    active_bg: SOL_BLUE,
                    selected_bg: SOL_CYAN,
                    disabled_fg: SOL_BASE01,
                },
            },
            tabs: TabsTokens {
                bg: SOL_BASE02,
                border: SOL_BASE01,
                tab: TabTokens {
                    fg: SOL_BASE01,
                    active_fg: SOL_BASE0,
                    active_bg: "#0a4050",
                    hover_fg: SOL_BASE0,
                    indicator: SOL_CYAN,
                },
            },
            dropdown: DropdownTokens {
                bg: SOL_BASE02,
                border: SOL_BASE01,
                item: DropdownItemTokens {
                    fg: SOL_BASE0,
                    hover_bg: "#0a4050",
                    selected_bg: SOL_BLUE,
                },
            },
            input: InputTokens {
                bg: SOL_BASE03,
                fg: SOL_BASE0,
                placeholder: SOL_BASE01,
                border: SOL_BASE02,
                focus_border: SOL_BLUE,
                invalid_border: SOL_RED,
            },
            checkbox: CheckboxTokens {
                bg: SOL_BASE03,
                border: SOL_BASE01,
                check_color: SOL_BASE03,
                checked_bg: SOL_CYAN,
            },
            radio: RadioTokens {
                bg: SOL_BASE03,
                dot_color: SOL_CYAN,
                border: SOL_BASE01,
                checked_border: SOL_CYAN,
            },
            tooltip: TooltipTokens {
                bg: "#0a4050",
                fg: SOL_BASE0,
            },
            modal: ModalTokens {
                bg: SOL_BASE02,
                border: SOL_BASE01,
                overlay: "rgba(0, 0, 0, 0.6)",
            },
            badge: BadgeTokens {
                default: BadgeVariant {
                    bg: SOL_BASE02,
                    fg: SOL_BASE0,
                },
                success: BadgeVariant {
                    bg: SOL_GREEN,
                    fg: SOL_BASE03,
                },
                warning: BadgeVariant {
                    bg: SOL_YELLOW,
                    fg: SOL_BASE03,
                },
                danger: BadgeVariant {
                    bg: SOL_RED,
                    fg: SOL_BASE3,
                },
            },
            list: ListTokens {
                item: ListItemTokens {
                    bg: "transparent",
                    hover_bg: "#0a4050",
                    selected_bg: SOL_BLUE,
                    fg: SOL_BASE0,
                },
            },
            header: HeaderTokens {
                default: HeaderVariant {
                    bg: SOL_BASE02,
                    fg: SOL_BASE0,
                    title_fg: SOL_BASE0,
                    subtitle_fg: SOL_BASE01,
                    border: "#0a4050",
                },
                primary: HeaderVariant {
                    bg: SOL_BLUE,
                    fg: SOL_BASE03,
                    title_fg: SOL_BASE03,
                    subtitle_fg: "rgba(0, 43, 54, 0.7)",
                    border: "#1f78b4",
                },
                secondary: HeaderVariant {
                    bg: "#0a4050",
                    fg: SOL_BASE0,
                    title_fg: SOL_BASE0,
                    subtitle_fg: SOL_BASE01,
                    border: SOL_BASE01,
                },
                success: HeaderVariant {
                    bg: SOL_GREEN,
                    fg: SOL_BASE03,
                    title_fg: SOL_BASE03,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: "#6e7a00",
                },
                warning: HeaderVariant {
                    bg: SOL_YELLOW,
                    fg: SOL_BASE03,
                    title_fg: SOL_BASE03,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: "#9a7400",
                },
                danger: HeaderVariant {
                    bg: SOL_RED,
                    fg: SOL_BASE3,
                    title_fg: SOL_BASE3,
                    subtitle_fg: "rgba(255, 255, 255, 0.8)",
                    border: "#b82825",
                },
            },
            statusbar: StatusbarTokens {
                default: StatusbarVariant {
                    bg: SOL_BASE02,
                    fg: SOL_BASE01,
                },
                primary: StatusbarVariant {
                    bg: SOL_BLUE,
                    fg: SOL_BASE03,
                },
                info: StatusbarVariant {
                    bg: SOL_CYAN,
                    fg: SOL_BASE03,
                },
                success: StatusbarVariant {
                    bg: SOL_GREEN,
                    fg: SOL_BASE03,
                },
                warning: StatusbarVariant {
                    bg: SOL_YELLOW,
                    fg: SOL_BASE03,
                },
                danger: StatusbarVariant {
                    bg: SOL_RED,
                    fg: SOL_BASE3,
                },
            },
            page: PageTokens {
                default: PageVariant {
                    bg: SOL_BASE03,
                    title_fg: SOL_BASE0,
                    subtitle_fg: SOL_BASE01,
                    border: SOL_BASE02,
                },
                primary: PageVariant {
                    bg: SOL_BASE03,
                    title_fg: SOL_CYAN,
                    subtitle_fg: SOL_BASE01,
                    border: SOL_CYAN,
                },
                secondary: PageVariant {
                    bg: SOL_BASE03,
                    title_fg: SOL_VIOLET,
                    subtitle_fg: SOL_BASE01,
                    border: SOL_BASE02,
                },
            },
            appshell: AppShellTokens {
                bg: "#001e26",
                divider_fg: SOL_BASE02,
                sidebar_bg: SOL_BASE03,
                aside_bg: SOL_BASE03,
            },
            toast: ToastTokens {
                success: ToastVariant {
                    bg: "#002820",
                    fg: SOL_GREEN,
                    border: SOL_GREEN,
                    icon_fg: SOL_GREEN,
                },
                error: ToastVariant {
                    bg: "#280020",
                    fg: SOL_RED,
                    border: SOL_RED,
                    icon_fg: SOL_RED,
                },
                warning: ToastVariant {
                    bg: "#282000",
                    fg: SOL_YELLOW,
                    border: SOL_YELLOW,
                    icon_fg: SOL_YELLOW,
                },
                info: ToastVariant {
                    bg: "#002028",
                    fg: SOL_BLUE,
                    border: SOL_BLUE,
                    icon_fg: SOL_BLUE,
                },
            },
            slider: SliderTokens {
                track_bg: SOL_BASE02,
                fill_bg: SOL_CYAN,
                thumb: SOL_BASE0,
            },
        },
    }
}
