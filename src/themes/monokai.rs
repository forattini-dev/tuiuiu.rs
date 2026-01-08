//! Monokai Theme
//!
//! Classic code editor theme with vibrant colors.

#![allow(dead_code)]

use super::colors::*;
use super::types::*;

// Monokai color palette
const MONOKAI_BG: &str = "#272822";
const MONOKAI_BG_HIGHLIGHT: &str = "#3e3d32";
const MONOKAI_FG: &str = "#f8f8f2";
const MONOKAI_COMMENT: &str = "#75715e";

const MONOKAI_YELLOW: &str = "#e6db74";
const MONOKAI_ORANGE: &str = "#fd971f";
const MONOKAI_RED: &str = "#f92672";
const MONOKAI_MAGENTA: &str = "#fd5ff0";
const MONOKAI_VIOLET: &str = "#ae81ff";
const MONOKAI_BLUE: &str = "#66d9ef";
const MONOKAI_CYAN: &str = "#a1efe4";
const MONOKAI_GREEN: &str = "#a6e22e";

/// Create the Monokai theme
pub fn monokai_theme() -> Theme {
    let palette = ThemePalette {
        primary: PINK,
        secondary: SLATE,
        success: GREEN,
        warning: AMBER,
        danger: RED,
        neutral: SLATE,
    };

    Theme {
        name: "monokai",
        mode: ThemeMode::Dark,
        meta: ThemeMeta {
            version: "1.0.0",
            author: "tuiuiu",
            description: "Monokai theme - classic editor theme with vibrant colors",
        },
        palette,
        background: ThemeBackground {
            lowest: "#1e1f1c",
            base: MONOKAI_BG,
            subtle: "#2d2e27",
            surface: MONOKAI_BG_HIGHLIGHT,
            raised: "#49483e",
            elevated: "#5c5b4f",
            popover: MONOKAI_BG_HIGHLIGHT,
            overlay: "rgba(0, 0, 0, 0.7)",
        },
        foreground: ThemeForeground {
            primary: MONOKAI_FG,
            secondary: "#cfcfc2",
            muted: MONOKAI_COMMENT,
            disabled: "#5c5b4f",
            inverse: ThemeForegroundInverse {
                base: WHITE,
                soft: "rgba(255, 255, 255, 0.6)",
                subtle: "rgba(255, 255, 255, 0.35)",
            },
        },
        accents: ThemeAccents {
            positive: MONOKAI_GREEN,
            warning: MONOKAI_ORANGE,
            critical: MONOKAI_RED,
            info: MONOKAI_BLUE,
            highlight: MONOKAI_YELLOW,
        },
        states: ThemeStates {
            hover: HoverState {
                bg: "rgba(255, 255, 255, 0.05)",
                fg: None,
            },
            active: ActiveState {
                bg: "rgba(255, 255, 255, 0.1)",
            },
            focus: FocusState {
                border: MONOKAI_BLUE,
                ring: FocusRing {
                    color: MONOKAI_BLUE,
                    width: 2,
                },
            },
            disabled: DisabledState {
                opacity: 0.4,
                bg: MONOKAI_BG_HIGHLIGHT,
                fg: MONOKAI_COMMENT,
            },
            selected: SelectedState {
                bg: MONOKAI_RED,
                fg: MONOKAI_FG,
            },
        },
        borders: ThemeBorders {
            default: MONOKAI_BG_HIGHLIGHT,
            subtle: "#2d2e27",
            strong: MONOKAI_COMMENT,
            accent: MONOKAI_RED,
            danger: MONOKAI_RED,
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
                    bg: MONOKAI_RED,
                    fg: MONOKAI_FG,
                    hover_bg: "#ff4d8f",
                    active_bg: "#d91f5f",
                    border: "transparent",
                },
                secondary: ButtonVariant {
                    bg: MONOKAI_BG_HIGHLIGHT,
                    fg: MONOKAI_FG,
                    hover_bg: "#49483e",
                    active_bg: "#2d2e27",
                    border: "transparent",
                },
                outline: ButtonVariant {
                    bg: "transparent",
                    fg: MONOKAI_RED,
                    hover_bg: "rgba(249, 38, 114, 0.1)",
                    active_bg: "rgba(249, 38, 114, 0.2)",
                    border: MONOKAI_RED,
                },
                ghost: ButtonVariant {
                    bg: "transparent",
                    fg: MONOKAI_FG,
                    hover_bg: "rgba(255, 255, 255, 0.05)",
                    active_bg: "rgba(255, 255, 255, 0.1)",
                    border: "transparent",
                },
            },
            panel: PanelTokens {
                bg: MONOKAI_BG_HIGHLIGHT,
                header_bg: "#49483e",
                footer_bg: "#49483e",
                border: "#5c5b4f",
            },
            menu: MenuTokens {
                bg: MONOKAI_BG_HIGHLIGHT,
                border: "#5c5b4f",
                item: MenuItemTokens {
                    fg: MONOKAI_FG,
                    hover_bg: "#49483e",
                    active_bg: MONOKAI_RED,
                    selected_bg: "#c41f5d",
                    disabled_fg: MONOKAI_COMMENT,
                },
            },
            tabs: TabsTokens {
                bg: MONOKAI_BG_HIGHLIGHT,
                border: "#5c5b4f",
                tab: TabTokens {
                    fg: MONOKAI_COMMENT,
                    active_fg: MONOKAI_FG,
                    active_bg: "#49483e",
                    hover_fg: MONOKAI_FG,
                    indicator: MONOKAI_RED,
                },
            },
            dropdown: DropdownTokens {
                bg: MONOKAI_BG_HIGHLIGHT,
                border: "#5c5b4f",
                item: DropdownItemTokens {
                    fg: MONOKAI_FG,
                    hover_bg: "#49483e",
                    selected_bg: MONOKAI_RED,
                },
            },
            input: InputTokens {
                bg: MONOKAI_BG,
                fg: MONOKAI_FG,
                placeholder: MONOKAI_COMMENT,
                border: MONOKAI_BG_HIGHLIGHT,
                focus_border: MONOKAI_BLUE,
                invalid_border: MONOKAI_RED,
            },
            checkbox: CheckboxTokens {
                bg: MONOKAI_BG,
                border: MONOKAI_COMMENT,
                check_color: MONOKAI_FG,
                checked_bg: MONOKAI_GREEN,
            },
            radio: RadioTokens {
                bg: MONOKAI_BG,
                dot_color: MONOKAI_GREEN,
                border: MONOKAI_COMMENT,
                checked_border: MONOKAI_GREEN,
            },
            tooltip: TooltipTokens {
                bg: "#49483e",
                fg: MONOKAI_FG,
            },
            modal: ModalTokens {
                bg: MONOKAI_BG_HIGHLIGHT,
                border: "#5c5b4f",
                overlay: "rgba(0, 0, 0, 0.7)",
            },
            badge: BadgeTokens {
                default: BadgeVariant {
                    bg: MONOKAI_BG_HIGHLIGHT,
                    fg: MONOKAI_FG,
                },
                success: BadgeVariant {
                    bg: MONOKAI_GREEN,
                    fg: MONOKAI_BG,
                },
                warning: BadgeVariant {
                    bg: MONOKAI_ORANGE,
                    fg: MONOKAI_BG,
                },
                danger: BadgeVariant {
                    bg: MONOKAI_RED,
                    fg: MONOKAI_FG,
                },
            },
            list: ListTokens {
                item: ListItemTokens {
                    bg: "transparent",
                    hover_bg: "#49483e",
                    selected_bg: MONOKAI_RED,
                    fg: MONOKAI_FG,
                },
            },
            header: HeaderTokens {
                default: HeaderVariant {
                    bg: MONOKAI_BG_HIGHLIGHT,
                    fg: MONOKAI_FG,
                    title_fg: MONOKAI_FG,
                    subtitle_fg: MONOKAI_COMMENT,
                    border: "#49483e",
                },
                primary: HeaderVariant {
                    bg: MONOKAI_RED,
                    fg: MONOKAI_FG,
                    title_fg: MONOKAI_FG,
                    subtitle_fg: "rgba(255, 255, 255, 0.7)",
                    border: "#d91f5f",
                },
                secondary: HeaderVariant {
                    bg: "#49483e",
                    fg: MONOKAI_FG,
                    title_fg: MONOKAI_FG,
                    subtitle_fg: MONOKAI_COMMENT,
                    border: "#5c5b4f",
                },
                success: HeaderVariant {
                    bg: MONOKAI_GREEN,
                    fg: MONOKAI_BG,
                    title_fg: MONOKAI_BG,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: "#8cc21e",
                },
                warning: HeaderVariant {
                    bg: MONOKAI_ORANGE,
                    fg: MONOKAI_BG,
                    title_fg: MONOKAI_BG,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: "#d07f19",
                },
                danger: HeaderVariant {
                    bg: MONOKAI_RED,
                    fg: MONOKAI_FG,
                    title_fg: MONOKAI_FG,
                    subtitle_fg: "rgba(255, 255, 255, 0.8)",
                    border: "#d91f5f",
                },
            },
            statusbar: StatusbarTokens {
                default: StatusbarVariant {
                    bg: MONOKAI_BG_HIGHLIGHT,
                    fg: MONOKAI_COMMENT,
                },
                primary: StatusbarVariant {
                    bg: MONOKAI_RED,
                    fg: MONOKAI_FG,
                },
                info: StatusbarVariant {
                    bg: MONOKAI_BLUE,
                    fg: MONOKAI_BG,
                },
                success: StatusbarVariant {
                    bg: MONOKAI_GREEN,
                    fg: MONOKAI_BG,
                },
                warning: StatusbarVariant {
                    bg: MONOKAI_ORANGE,
                    fg: MONOKAI_BG,
                },
                danger: StatusbarVariant {
                    bg: MONOKAI_RED,
                    fg: MONOKAI_FG,
                },
            },
            page: PageTokens {
                default: PageVariant {
                    bg: MONOKAI_BG,
                    title_fg: MONOKAI_FG,
                    subtitle_fg: MONOKAI_COMMENT,
                    border: MONOKAI_BG_HIGHLIGHT,
                },
                primary: PageVariant {
                    bg: MONOKAI_BG,
                    title_fg: MONOKAI_RED,
                    subtitle_fg: MONOKAI_COMMENT,
                    border: "#c41f5d",
                },
                secondary: PageVariant {
                    bg: MONOKAI_BG,
                    title_fg: MONOKAI_BLUE,
                    subtitle_fg: MONOKAI_COMMENT,
                    border: MONOKAI_BG_HIGHLIGHT,
                },
            },
            appshell: AppShellTokens {
                bg: "#1e1f1c",
                divider_fg: MONOKAI_BG_HIGHLIGHT,
                sidebar_bg: MONOKAI_BG,
                aside_bg: MONOKAI_BG,
            },
            toast: ToastTokens {
                success: ToastVariant {
                    bg: "#2a3820",
                    fg: MONOKAI_GREEN,
                    border: MONOKAI_GREEN,
                    icon_fg: MONOKAI_GREEN,
                },
                error: ToastVariant {
                    bg: "#3a2028",
                    fg: MONOKAI_RED,
                    border: MONOKAI_RED,
                    icon_fg: MONOKAI_RED,
                },
                warning: ToastVariant {
                    bg: "#3a3020",
                    fg: MONOKAI_ORANGE,
                    border: MONOKAI_ORANGE,
                    icon_fg: MONOKAI_ORANGE,
                },
                info: ToastVariant {
                    bg: "#203038",
                    fg: MONOKAI_BLUE,
                    border: MONOKAI_BLUE,
                    icon_fg: MONOKAI_BLUE,
                },
            },
            slider: SliderTokens {
                track_bg: MONOKAI_BG_HIGHLIGHT,
                fill_bg: MONOKAI_RED,
                thumb: MONOKAI_FG,
            },
        },
    }
}
