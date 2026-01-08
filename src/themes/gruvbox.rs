//! Gruvbox Theme
//!
//! A retro groove color scheme with warm colors.
//! https://github.com/morhetz/gruvbox

#![allow(dead_code)]

use super::colors::*;
use super::types::*;

// Gruvbox Dark palette
const GB_BG: &str = "#282828";
const GB_BG0_HARD: &str = "#1d2021";
const GB_BG0_SOFT: &str = "#32302f";
const GB_BG1: &str = "#3c3836";
const GB_BG2: &str = "#504945";
const GB_BG3: &str = "#665c54";
const GB_BG4: &str = "#7c6f64";

const GB_FG: &str = "#ebdbb2";
const GB_FG0: &str = "#fbf1c7";
const GB_FG1: &str = "#ebdbb2";
const GB_FG2: &str = "#d5c4a1";
const GB_FG3: &str = "#bdae93";
const GB_FG4: &str = "#a89984";

// Accent colors (bright variants)
const GB_RED: &str = "#fb4934";
const GB_GREEN: &str = "#b8bb26";
const GB_YELLOW: &str = "#fabd2f";
const GB_BLUE: &str = "#83a598";
const GB_PURPLE: &str = "#d3869b";
const GB_AQUA: &str = "#8ec07c";
const GB_ORANGE: &str = "#fe8019";

// Neutral colors
const GB_RED_DIM: &str = "#cc241d";
const GB_GREEN_DIM: &str = "#98971a";
const GB_AQUA_DIM: &str = "#689d6a";

/// Create the Gruvbox Dark theme
pub fn gruvbox_theme() -> Theme {
    let palette = ThemePalette {
        primary: AMBER,
        secondary: STONE,
        success: GREEN,
        warning: AMBER,
        danger: RED,
        neutral: STONE,
    };

    Theme {
        name: "gruvbox",
        mode: ThemeMode::Dark,
        meta: ThemeMeta {
            version: "1.0.0",
            author: "tuiuiu",
            description: "Gruvbox theme - retro groove color scheme with warm colors",
        },
        palette,
        background: ThemeBackground {
            lowest: GB_BG0_HARD,
            base: GB_BG,
            subtle: GB_BG0_SOFT,
            surface: GB_BG1,
            raised: GB_BG2,
            elevated: GB_BG3,
            popover: GB_BG1,
            overlay: "rgba(0, 0, 0, 0.6)",
        },
        foreground: ThemeForeground {
            primary: GB_FG,
            secondary: GB_FG2,
            muted: GB_FG4,
            disabled: GB_BG4,
            inverse: ThemeForegroundInverse {
                base: GB_FG0,
                soft: "rgba(251, 241, 199, 0.6)",
                subtle: "rgba(251, 241, 199, 0.35)",
            },
        },
        accents: ThemeAccents {
            positive: GB_GREEN,
            warning: GB_YELLOW,
            critical: GB_RED,
            info: GB_BLUE,
            highlight: GB_ORANGE,
        },
        states: ThemeStates {
            hover: HoverState {
                bg: "rgba(254, 128, 25, 0.15)",
                fg: None,
            },
            active: ActiveState {
                bg: "rgba(254, 128, 25, 0.25)",
            },
            focus: FocusState {
                border: GB_ORANGE,
                ring: FocusRing {
                    color: GB_ORANGE,
                    width: 2,
                },
            },
            disabled: DisabledState {
                opacity: 0.4,
                bg: GB_BG1,
                fg: GB_FG4,
            },
            selected: SelectedState {
                bg: GB_ORANGE,
                fg: GB_BG,
            },
        },
        borders: ThemeBorders {
            default: GB_BG2,
            subtle: GB_BG1,
            strong: GB_BG4,
            accent: GB_ORANGE,
            danger: GB_RED,
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
                    bg: GB_ORANGE,
                    fg: GB_BG,
                    hover_bg: GB_YELLOW,
                    active_bg: "#d57014",
                    border: "transparent",
                },
                secondary: ButtonVariant {
                    bg: GB_BG2,
                    fg: GB_FG,
                    hover_bg: GB_BG3,
                    active_bg: GB_BG1,
                    border: "transparent",
                },
                outline: ButtonVariant {
                    bg: "transparent",
                    fg: GB_ORANGE,
                    hover_bg: "rgba(254, 128, 25, 0.15)",
                    active_bg: "rgba(254, 128, 25, 0.25)",
                    border: GB_ORANGE,
                },
                ghost: ButtonVariant {
                    bg: "transparent",
                    fg: GB_FG,
                    hover_bg: "rgba(235, 219, 178, 0.05)",
                    active_bg: "rgba(235, 219, 178, 0.1)",
                    border: "transparent",
                },
            },
            panel: PanelTokens {
                bg: GB_BG1,
                header_bg: GB_BG2,
                footer_bg: GB_BG2,
                border: GB_BG3,
            },
            menu: MenuTokens {
                bg: GB_BG1,
                border: GB_BG3,
                item: MenuItemTokens {
                    fg: GB_FG,
                    hover_bg: GB_BG2,
                    active_bg: GB_ORANGE,
                    selected_bg: "#c87018",
                    disabled_fg: GB_FG4,
                },
            },
            tabs: TabsTokens {
                bg: GB_BG1,
                border: GB_BG3,
                tab: TabTokens {
                    fg: GB_FG4,
                    active_fg: GB_FG,
                    active_bg: GB_BG2,
                    hover_fg: GB_FG,
                    indicator: GB_ORANGE,
                },
            },
            dropdown: DropdownTokens {
                bg: GB_BG1,
                border: GB_BG3,
                item: DropdownItemTokens {
                    fg: GB_FG,
                    hover_bg: GB_BG2,
                    selected_bg: GB_ORANGE,
                },
            },
            input: InputTokens {
                bg: GB_BG,
                fg: GB_FG,
                placeholder: GB_FG4,
                border: GB_BG2,
                focus_border: GB_ORANGE,
                invalid_border: GB_RED,
            },
            checkbox: CheckboxTokens {
                bg: GB_BG,
                border: GB_BG4,
                check_color: GB_BG,
                checked_bg: GB_GREEN,
            },
            radio: RadioTokens {
                bg: GB_BG,
                dot_color: GB_GREEN,
                border: GB_BG4,
                checked_border: GB_GREEN,
            },
            tooltip: TooltipTokens {
                bg: GB_BG2,
                fg: GB_FG,
            },
            modal: ModalTokens {
                bg: GB_BG1,
                border: GB_BG3,
                overlay: "rgba(0, 0, 0, 0.6)",
            },
            badge: BadgeTokens {
                default: BadgeVariant {
                    bg: GB_BG2,
                    fg: GB_FG,
                },
                success: BadgeVariant {
                    bg: GB_GREEN,
                    fg: GB_BG,
                },
                warning: BadgeVariant {
                    bg: GB_YELLOW,
                    fg: GB_BG,
                },
                danger: BadgeVariant {
                    bg: GB_RED,
                    fg: GB_BG,
                },
            },
            list: ListTokens {
                item: ListItemTokens {
                    bg: "transparent",
                    hover_bg: GB_BG2,
                    selected_bg: GB_ORANGE,
                    fg: GB_FG,
                },
            },
            header: HeaderTokens {
                default: HeaderVariant {
                    bg: GB_BG1,
                    fg: GB_FG,
                    title_fg: GB_FG,
                    subtitle_fg: GB_FG4,
                    border: GB_BG2,
                },
                primary: HeaderVariant {
                    bg: GB_ORANGE,
                    fg: GB_BG,
                    title_fg: GB_BG,
                    subtitle_fg: "rgba(40, 40, 40, 0.7)",
                    border: "#d57014",
                },
                secondary: HeaderVariant {
                    bg: GB_BG2,
                    fg: GB_FG,
                    title_fg: GB_FG,
                    subtitle_fg: GB_FG4,
                    border: GB_BG3,
                },
                success: HeaderVariant {
                    bg: GB_GREEN,
                    fg: GB_BG,
                    title_fg: GB_BG,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: GB_GREEN_DIM,
                },
                warning: HeaderVariant {
                    bg: GB_YELLOW,
                    fg: GB_BG,
                    title_fg: GB_BG,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: "#d5a021",
                },
                danger: HeaderVariant {
                    bg: GB_RED,
                    fg: GB_BG,
                    title_fg: GB_BG,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: GB_RED_DIM,
                },
            },
            statusbar: StatusbarTokens {
                default: StatusbarVariant {
                    bg: GB_BG1,
                    fg: GB_FG4,
                },
                primary: StatusbarVariant {
                    bg: GB_ORANGE,
                    fg: GB_BG,
                },
                info: StatusbarVariant {
                    bg: GB_BLUE,
                    fg: GB_BG,
                },
                success: StatusbarVariant {
                    bg: GB_GREEN,
                    fg: GB_BG,
                },
                warning: StatusbarVariant {
                    bg: GB_YELLOW,
                    fg: GB_BG,
                },
                danger: StatusbarVariant {
                    bg: GB_RED,
                    fg: GB_BG,
                },
            },
            page: PageTokens {
                default: PageVariant {
                    bg: GB_BG,
                    title_fg: GB_FG,
                    subtitle_fg: GB_FG4,
                    border: GB_BG2,
                },
                primary: PageVariant {
                    bg: GB_BG,
                    title_fg: GB_ORANGE,
                    subtitle_fg: GB_FG4,
                    border: GB_ORANGE,
                },
                secondary: PageVariant {
                    bg: GB_BG,
                    title_fg: GB_AQUA,
                    subtitle_fg: GB_FG4,
                    border: GB_BG2,
                },
            },
            appshell: AppShellTokens {
                bg: GB_BG0_HARD,
                divider_fg: GB_BG2,
                sidebar_bg: GB_BG,
                aside_bg: GB_BG,
            },
            toast: ToastTokens {
                success: ToastVariant {
                    bg: "#2a3020",
                    fg: GB_GREEN,
                    border: GB_GREEN,
                    icon_fg: GB_GREEN,
                },
                error: ToastVariant {
                    bg: "#3a2020",
                    fg: GB_RED,
                    border: GB_RED,
                    icon_fg: GB_RED,
                },
                warning: ToastVariant {
                    bg: "#3a3020",
                    fg: GB_YELLOW,
                    border: GB_YELLOW,
                    icon_fg: GB_YELLOW,
                },
                info: ToastVariant {
                    bg: "#203030",
                    fg: GB_BLUE,
                    border: GB_BLUE,
                    icon_fg: GB_BLUE,
                },
            },
            slider: SliderTokens {
                track_bg: GB_BG2,
                fill_bg: GB_ORANGE,
                thumb: GB_FG,
            },
        },
    }
}
