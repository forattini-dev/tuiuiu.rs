//! Dracula Theme
//!
//! Popular dark theme with purple accent colors.

use super::colors::*;
use super::types::*;

// Dracula color palette
const DRACULA_BG: &str = "#282a36";
const DRACULA_CURRENT: &str = "#44475a";
const DRACULA_FG: &str = "#f8f8f2";
const DRACULA_COMMENT: &str = "#6272a4";
const DRACULA_CYAN: &str = "#8be9fd";
const DRACULA_GREEN: &str = "#50fa7b";
const DRACULA_ORANGE: &str = "#ffb86c";
const DRACULA_PINK: &str = "#ff79c6";
const DRACULA_PURPLE: &str = "#bd93f9";
const DRACULA_RED: &str = "#ff5555";
#[allow(dead_code)]
const DRACULA_YELLOW: &str = "#f1fa8c";

/// Create the Dracula theme
pub fn dracula_theme() -> Theme {
    let palette = ThemePalette {
        primary: PURPLE,
        secondary: SLATE,
        success: GREEN,
        warning: AMBER,
        danger: RED,
        neutral: SLATE,
    };

    Theme {
        name: "dracula",
        mode: ThemeMode::Dark,
        meta: ThemeMeta {
            version: "1.0.0",
            author: "tuiuiu",
            description: "Dracula theme - popular dark theme with purple accents",
        },
        palette,
        background: ThemeBackground {
            lowest: "#1e1f29",
            base: DRACULA_BG,
            subtle: DRACULA_CURRENT,
            surface: "#363848",
            raised: "#44475a",
            elevated: "#4d5066",
            popover: DRACULA_CURRENT,
            overlay: "rgba(0, 0, 0, 0.7)",
        },
        foreground: ThemeForeground {
            primary: DRACULA_FG,
            secondary: "#e0e0e0",
            muted: DRACULA_COMMENT,
            disabled: "#555770",
            inverse: ThemeForegroundInverse {
                base: WHITE,
                soft: "rgba(255, 255, 255, 0.6)",
                subtle: "rgba(255, 255, 255, 0.35)",
            },
        },
        accents: ThemeAccents {
            positive: DRACULA_GREEN,
            warning: DRACULA_ORANGE,
            critical: DRACULA_RED,
            info: DRACULA_CYAN,
            highlight: DRACULA_PURPLE,
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
                border: DRACULA_PURPLE,
                ring: FocusRing {
                    color: DRACULA_PURPLE,
                    width: 2,
                },
            },
            disabled: DisabledState {
                opacity: 0.4,
                bg: DRACULA_CURRENT,
                fg: DRACULA_COMMENT,
            },
            selected: SelectedState {
                bg: DRACULA_PURPLE,
                fg: DRACULA_BG,
            },
        },
        borders: ThemeBorders {
            default: DRACULA_CURRENT,
            subtle: "#3d3f4d",
            strong: DRACULA_COMMENT,
            accent: DRACULA_PURPLE,
            danger: DRACULA_RED,
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
                    bg: DRACULA_PURPLE,
                    fg: DRACULA_BG,
                    hover_bg: "#caa4fa",
                    active_bg: "#a67df0",
                    border: "transparent",
                },
                secondary: ButtonVariant {
                    bg: DRACULA_CURRENT,
                    fg: DRACULA_FG,
                    hover_bg: "#555770",
                    active_bg: "#3d3f4d",
                    border: "transparent",
                },
                outline: ButtonVariant {
                    bg: "transparent",
                    fg: DRACULA_PURPLE,
                    hover_bg: "rgba(189, 147, 249, 0.1)",
                    active_bg: "rgba(189, 147, 249, 0.2)",
                    border: DRACULA_PURPLE,
                },
                ghost: ButtonVariant {
                    bg: "transparent",
                    fg: DRACULA_FG,
                    hover_bg: "rgba(255, 255, 255, 0.05)",
                    active_bg: "rgba(255, 255, 255, 0.1)",
                    border: "transparent",
                },
            },
            panel: PanelTokens {
                bg: DRACULA_CURRENT,
                header_bg: "#363848",
                footer_bg: "#363848",
                border: "#555770",
            },
            menu: MenuTokens {
                bg: DRACULA_CURRENT,
                border: "#555770",
                item: MenuItemTokens {
                    fg: DRACULA_FG,
                    hover_bg: "#555770",
                    active_bg: DRACULA_PURPLE,
                    selected_bg: "#7c5db8",
                    disabled_fg: DRACULA_COMMENT,
                },
            },
            tabs: TabsTokens {
                bg: DRACULA_CURRENT,
                border: "#555770",
                tab: TabTokens {
                    fg: DRACULA_COMMENT,
                    active_fg: DRACULA_FG,
                    active_bg: "#555770",
                    hover_fg: DRACULA_FG,
                    indicator: DRACULA_PURPLE,
                },
            },
            dropdown: DropdownTokens {
                bg: DRACULA_CURRENT,
                border: "#555770",
                item: DropdownItemTokens {
                    fg: DRACULA_FG,
                    hover_bg: "#555770",
                    selected_bg: DRACULA_PURPLE,
                },
            },
            input: InputTokens {
                bg: DRACULA_BG,
                fg: DRACULA_FG,
                placeholder: DRACULA_COMMENT,
                border: DRACULA_CURRENT,
                focus_border: DRACULA_PURPLE,
                invalid_border: DRACULA_RED,
            },
            checkbox: CheckboxTokens {
                bg: DRACULA_BG,
                border: DRACULA_COMMENT,
                check_color: DRACULA_BG,
                checked_bg: DRACULA_PURPLE,
            },
            radio: RadioTokens {
                bg: DRACULA_BG,
                dot_color: DRACULA_PURPLE,
                border: DRACULA_COMMENT,
                checked_border: DRACULA_PURPLE,
            },
            tooltip: TooltipTokens {
                bg: "#555770",
                fg: DRACULA_FG,
            },
            modal: ModalTokens {
                bg: DRACULA_CURRENT,
                border: "#555770",
                overlay: "rgba(0, 0, 0, 0.7)",
            },
            badge: BadgeTokens {
                default: BadgeVariant {
                    bg: DRACULA_CURRENT,
                    fg: DRACULA_FG,
                },
                success: BadgeVariant {
                    bg: DRACULA_GREEN,
                    fg: DRACULA_BG,
                },
                warning: BadgeVariant {
                    bg: DRACULA_ORANGE,
                    fg: DRACULA_BG,
                },
                danger: BadgeVariant {
                    bg: DRACULA_RED,
                    fg: WHITE,
                },
            },
            list: ListTokens {
                item: ListItemTokens {
                    bg: "transparent",
                    hover_bg: "#555770",
                    selected_bg: DRACULA_PURPLE,
                    fg: DRACULA_FG,
                },
            },
            header: HeaderTokens {
                default: HeaderVariant {
                    bg: DRACULA_CURRENT,
                    fg: DRACULA_FG,
                    title_fg: DRACULA_FG,
                    subtitle_fg: DRACULA_COMMENT,
                    border: "#555770",
                },
                primary: HeaderVariant {
                    bg: DRACULA_PURPLE,
                    fg: DRACULA_BG,
                    title_fg: DRACULA_BG,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: "#a67df0",
                },
                secondary: HeaderVariant {
                    bg: "#555770",
                    fg: DRACULA_FG,
                    title_fg: DRACULA_FG,
                    subtitle_fg: DRACULA_COMMENT,
                    border: DRACULA_COMMENT,
                },
                success: HeaderVariant {
                    bg: DRACULA_GREEN,
                    fg: DRACULA_BG,
                    title_fg: DRACULA_BG,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: "#40d06a",
                },
                warning: HeaderVariant {
                    bg: DRACULA_ORANGE,
                    fg: DRACULA_BG,
                    title_fg: DRACULA_BG,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: "#d9a05e",
                },
                danger: HeaderVariant {
                    bg: DRACULA_RED,
                    fg: WHITE,
                    title_fg: WHITE,
                    subtitle_fg: "rgba(255, 255, 255, 0.8)",
                    border: "#d04545",
                },
            },
            statusbar: StatusbarTokens {
                default: StatusbarVariant {
                    bg: DRACULA_CURRENT,
                    fg: DRACULA_COMMENT,
                },
                primary: StatusbarVariant {
                    bg: DRACULA_PURPLE,
                    fg: DRACULA_BG,
                },
                info: StatusbarVariant {
                    bg: DRACULA_CYAN,
                    fg: DRACULA_BG,
                },
                success: StatusbarVariant {
                    bg: DRACULA_GREEN,
                    fg: DRACULA_BG,
                },
                warning: StatusbarVariant {
                    bg: DRACULA_ORANGE,
                    fg: DRACULA_BG,
                },
                danger: StatusbarVariant {
                    bg: DRACULA_RED,
                    fg: WHITE,
                },
            },
            page: PageTokens {
                default: PageVariant {
                    bg: DRACULA_BG,
                    title_fg: DRACULA_FG,
                    subtitle_fg: DRACULA_COMMENT,
                    border: DRACULA_CURRENT,
                },
                primary: PageVariant {
                    bg: DRACULA_BG,
                    title_fg: DRACULA_PURPLE,
                    subtitle_fg: DRACULA_COMMENT,
                    border: "#7c5db8",
                },
                secondary: PageVariant {
                    bg: DRACULA_BG,
                    title_fg: DRACULA_PINK,
                    subtitle_fg: DRACULA_COMMENT,
                    border: DRACULA_CURRENT,
                },
            },
            appshell: AppShellTokens {
                bg: "#1e1f29",
                divider_fg: DRACULA_CURRENT,
                sidebar_bg: DRACULA_BG,
                aside_bg: DRACULA_BG,
            },
            toast: ToastTokens {
                success: ToastVariant {
                    bg: "#1a3d2a",
                    fg: DRACULA_GREEN,
                    border: DRACULA_GREEN,
                    icon_fg: DRACULA_GREEN,
                },
                error: ToastVariant {
                    bg: "#3d1a1a",
                    fg: DRACULA_RED,
                    border: DRACULA_RED,
                    icon_fg: DRACULA_RED,
                },
                warning: ToastVariant {
                    bg: "#3d2e1a",
                    fg: DRACULA_ORANGE,
                    border: DRACULA_ORANGE,
                    icon_fg: DRACULA_ORANGE,
                },
                info: ToastVariant {
                    bg: "#1a2d3d",
                    fg: DRACULA_CYAN,
                    border: DRACULA_CYAN,
                    icon_fg: DRACULA_CYAN,
                },
            },
            slider: SliderTokens {
                track_bg: "#44475a",
                fill_bg: DRACULA_PURPLE,
                thumb: DRACULA_FG,
            },
        },
    }
}
