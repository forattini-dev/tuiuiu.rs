//! Tokyo Night Theme
//!
//! A clean, dark theme inspired by the night lights of Tokyo.
//! Popular in VS Code and modern editors.

#![allow(dead_code)]

use super::colors::*;
use super::types::*;

// Tokyo Night color palette
const TN_BG: &str = "#1a1b26";
const TN_BG_DARK: &str = "#16161e";
const TN_BG_HIGHLIGHT: &str = "#292e42";
const TN_TERMINAL_BLACK: &str = "#414868";
const TN_FG: &str = "#c0caf5";
const TN_FG_DARK: &str = "#a9b1d6";
const TN_COMMENT: &str = "#565f89";

// Accent colors
const TN_CYAN: &str = "#7dcfff";
const TN_BLUE: &str = "#7aa2f7";
const TN_PURPLE: &str = "#bb9af7";
const TN_MAGENTA: &str = "#ff007c";
const TN_ORANGE: &str = "#ff9e64";
const TN_YELLOW: &str = "#e0af68";
const TN_GREEN: &str = "#9ece6a";
const TN_TEAL: &str = "#73daca";
const TN_RED: &str = "#f7768e";

/// Create the Tokyo Night theme
pub fn tokyo_night_theme() -> Theme {
    let palette = ThemePalette {
        primary: BLUE,
        secondary: SLATE,
        success: GREEN,
        warning: AMBER,
        danger: RED,
        neutral: SLATE,
    };

    Theme {
        name: "tokyo-night",
        mode: ThemeMode::Dark,
        meta: ThemeMeta {
            version: "1.0.0",
            author: "tuiuiu",
            description: "Tokyo Night theme - clean dark theme inspired by Tokyo's night lights",
        },
        palette,
        background: ThemeBackground {
            lowest: TN_BG_DARK,
            base: TN_BG,
            subtle: "#1e1f2a",
            surface: TN_BG_HIGHLIGHT,
            raised: "#2f3549",
            elevated: "#3b4261",
            popover: TN_BG_HIGHLIGHT,
            overlay: "rgba(0, 0, 0, 0.7)",
        },
        foreground: ThemeForeground {
            primary: TN_FG,
            secondary: TN_FG_DARK,
            muted: TN_COMMENT,
            disabled: TN_TERMINAL_BLACK,
            inverse: ThemeForegroundInverse {
                base: WHITE,
                soft: "rgba(255, 255, 255, 0.6)",
                subtle: "rgba(255, 255, 255, 0.35)",
            },
        },
        accents: ThemeAccents {
            positive: TN_GREEN,
            warning: TN_ORANGE,
            critical: TN_RED,
            info: TN_CYAN,
            highlight: TN_BLUE,
        },
        states: ThemeStates {
            hover: HoverState {
                bg: "rgba(122, 162, 247, 0.1)",
                fg: None,
            },
            active: ActiveState {
                bg: "rgba(122, 162, 247, 0.15)",
            },
            focus: FocusState {
                border: TN_BLUE,
                ring: FocusRing {
                    color: TN_BLUE,
                    width: 2,
                },
            },
            disabled: DisabledState {
                opacity: 0.4,
                bg: TN_BG_HIGHLIGHT,
                fg: TN_COMMENT,
            },
            selected: SelectedState {
                bg: TN_BLUE,
                fg: TN_BG,
            },
        },
        borders: ThemeBorders {
            default: TN_BG_HIGHLIGHT,
            subtle: "#24283b",
            strong: TN_TERMINAL_BLACK,
            accent: TN_BLUE,
            danger: TN_RED,
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
                    bg: TN_BLUE,
                    fg: TN_BG,
                    hover_bg: "#8ab4f8",
                    active_bg: "#6691d8",
                    border: "transparent",
                },
                secondary: ButtonVariant {
                    bg: TN_BG_HIGHLIGHT,
                    fg: TN_FG,
                    hover_bg: "#3b4261",
                    active_bg: "#24283b",
                    border: "transparent",
                },
                outline: ButtonVariant {
                    bg: "transparent",
                    fg: TN_BLUE,
                    hover_bg: "rgba(122, 162, 247, 0.1)",
                    active_bg: "rgba(122, 162, 247, 0.2)",
                    border: TN_BLUE,
                },
                ghost: ButtonVariant {
                    bg: "transparent",
                    fg: TN_FG,
                    hover_bg: "rgba(255, 255, 255, 0.05)",
                    active_bg: "rgba(255, 255, 255, 0.1)",
                    border: "transparent",
                },
            },
            panel: PanelTokens {
                bg: TN_BG_HIGHLIGHT,
                header_bg: "#2f3549",
                footer_bg: "#2f3549",
                border: TN_TERMINAL_BLACK,
            },
            menu: MenuTokens {
                bg: TN_BG_HIGHLIGHT,
                border: TN_TERMINAL_BLACK,
                item: MenuItemTokens {
                    fg: TN_FG,
                    hover_bg: "#3b4261",
                    active_bg: TN_BLUE,
                    selected_bg: "#5d71a8",
                    disabled_fg: TN_COMMENT,
                },
            },
            tabs: TabsTokens {
                bg: TN_BG_HIGHLIGHT,
                border: TN_TERMINAL_BLACK,
                tab: TabTokens {
                    fg: TN_COMMENT,
                    active_fg: TN_FG,
                    active_bg: "#3b4261",
                    hover_fg: TN_FG,
                    indicator: TN_BLUE,
                },
            },
            dropdown: DropdownTokens {
                bg: TN_BG_HIGHLIGHT,
                border: TN_TERMINAL_BLACK,
                item: DropdownItemTokens {
                    fg: TN_FG,
                    hover_bg: "#3b4261",
                    selected_bg: TN_BLUE,
                },
            },
            input: InputTokens {
                bg: TN_BG,
                fg: TN_FG,
                placeholder: TN_COMMENT,
                border: TN_BG_HIGHLIGHT,
                focus_border: TN_BLUE,
                invalid_border: TN_RED,
            },
            checkbox: CheckboxTokens {
                bg: TN_BG,
                border: TN_COMMENT,
                check_color: TN_BG,
                checked_bg: TN_TEAL,
            },
            radio: RadioTokens {
                bg: TN_BG,
                dot_color: TN_TEAL,
                border: TN_COMMENT,
                checked_border: TN_TEAL,
            },
            tooltip: TooltipTokens {
                bg: "#3b4261",
                fg: TN_FG,
            },
            modal: ModalTokens {
                bg: TN_BG_HIGHLIGHT,
                border: TN_TERMINAL_BLACK,
                overlay: "rgba(0, 0, 0, 0.7)",
            },
            badge: BadgeTokens {
                default: BadgeVariant {
                    bg: TN_BG_HIGHLIGHT,
                    fg: TN_FG,
                },
                success: BadgeVariant {
                    bg: TN_GREEN,
                    fg: TN_BG,
                },
                warning: BadgeVariant {
                    bg: TN_ORANGE,
                    fg: TN_BG,
                },
                danger: BadgeVariant {
                    bg: TN_RED,
                    fg: TN_BG,
                },
            },
            list: ListTokens {
                item: ListItemTokens {
                    bg: "transparent",
                    hover_bg: "#3b4261",
                    selected_bg: TN_BLUE,
                    fg: TN_FG,
                },
            },
            header: HeaderTokens {
                default: HeaderVariant {
                    bg: TN_BG_HIGHLIGHT,
                    fg: TN_FG,
                    title_fg: TN_FG,
                    subtitle_fg: TN_COMMENT,
                    border: "#3b4261",
                },
                primary: HeaderVariant {
                    bg: TN_BLUE,
                    fg: TN_BG,
                    title_fg: TN_BG,
                    subtitle_fg: "rgba(26, 27, 38, 0.7)",
                    border: "#6691d8",
                },
                secondary: HeaderVariant {
                    bg: "#3b4261",
                    fg: TN_FG,
                    title_fg: TN_FG,
                    subtitle_fg: TN_COMMENT,
                    border: TN_TERMINAL_BLACK,
                },
                success: HeaderVariant {
                    bg: TN_GREEN,
                    fg: TN_BG,
                    title_fg: TN_BG,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: "#7fb854",
                },
                warning: HeaderVariant {
                    bg: TN_ORANGE,
                    fg: TN_BG,
                    title_fg: TN_BG,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: "#d88954",
                },
                danger: HeaderVariant {
                    bg: TN_RED,
                    fg: TN_BG,
                    title_fg: TN_BG,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: "#d86070",
                },
            },
            statusbar: StatusbarTokens {
                default: StatusbarVariant {
                    bg: TN_BG_HIGHLIGHT,
                    fg: TN_COMMENT,
                },
                primary: StatusbarVariant {
                    bg: TN_BLUE,
                    fg: TN_BG,
                },
                info: StatusbarVariant {
                    bg: TN_CYAN,
                    fg: TN_BG,
                },
                success: StatusbarVariant {
                    bg: TN_GREEN,
                    fg: TN_BG,
                },
                warning: StatusbarVariant {
                    bg: TN_ORANGE,
                    fg: TN_BG,
                },
                danger: StatusbarVariant {
                    bg: TN_RED,
                    fg: TN_BG,
                },
            },
            page: PageTokens {
                default: PageVariant {
                    bg: TN_BG,
                    title_fg: TN_FG,
                    subtitle_fg: TN_COMMENT,
                    border: TN_BG_HIGHLIGHT,
                },
                primary: PageVariant {
                    bg: TN_BG,
                    title_fg: TN_BLUE,
                    subtitle_fg: TN_COMMENT,
                    border: TN_BLUE,
                },
                secondary: PageVariant {
                    bg: TN_BG,
                    title_fg: TN_PURPLE,
                    subtitle_fg: TN_COMMENT,
                    border: TN_BG_HIGHLIGHT,
                },
            },
            appshell: AppShellTokens {
                bg: TN_BG_DARK,
                divider_fg: TN_BG_HIGHLIGHT,
                sidebar_bg: TN_BG,
                aside_bg: TN_BG,
            },
            toast: ToastTokens {
                success: ToastVariant {
                    bg: "#1a2820",
                    fg: TN_GREEN,
                    border: TN_GREEN,
                    icon_fg: TN_GREEN,
                },
                error: ToastVariant {
                    bg: "#2a1820",
                    fg: TN_RED,
                    border: TN_RED,
                    icon_fg: TN_RED,
                },
                warning: ToastVariant {
                    bg: "#2a2818",
                    fg: TN_ORANGE,
                    border: TN_ORANGE,
                    icon_fg: TN_ORANGE,
                },
                info: ToastVariant {
                    bg: "#182028",
                    fg: TN_CYAN,
                    border: TN_CYAN,
                    icon_fg: TN_CYAN,
                },
            },
            slider: SliderTokens {
                track_bg: TN_BG_HIGHLIGHT,
                fill_bg: TN_BLUE,
                thumb: TN_FG,
            },
        },
    }
}
