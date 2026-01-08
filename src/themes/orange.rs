//! Orange Theme
//!
//! Warm orange-themed dark theme with energetic vibes.

use super::colors::*;
use super::types::*;

/// Create the orange theme
pub fn orange_theme() -> Theme {
    let palette = ThemePalette {
        primary: ORANGE,
        secondary: STONE,
        success: GREEN,
        warning: AMBER,
        danger: RED,
        neutral: STONE,
    };

    Theme {
        name: "orange",
        mode: ThemeMode::Dark,
        meta: ThemeMeta {
            version: "1.0.0",
            author: "tuiuiu",
            description: "Warm orange-themed dark theme with energetic vibes",
        },
        palette,
        background: ThemeBackground {
            lowest: "#1a0f0a",
            base: STONE.s900,
            subtle: STONE.s800,
            surface: STONE.s700,
            raised: STONE.s600,
            elevated: STONE.s500,
            popover: STONE.s800,
            overlay: "rgba(26, 15, 10, 0.85)",
        },
        foreground: ThemeForeground {
            primary: STONE.s50,
            secondary: STONE.s200,
            muted: STONE.s400,
            disabled: STONE.s500,
            inverse: ThemeForegroundInverse {
                base: WHITE,
                soft: "rgba(255, 255, 255, 0.6)",
                subtle: "rgba(255, 255, 255, 0.35)",
            },
        },
        accents: ThemeAccents {
            positive: GREEN.s500,
            warning: AMBER.s500,
            critical: RED.s500,
            info: CYAN.s500,
            highlight: ORANGE.s500,
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
                border: ORANGE.s400,
                ring: FocusRing {
                    color: ORANGE.s500,
                    width: 2,
                },
            },
            disabled: DisabledState {
                opacity: 0.4,
                bg: STONE.s800,
                fg: STONE.s500,
            },
            selected: SelectedState {
                bg: ORANGE.s700,
                fg: WHITE,
            },
        },
        borders: ThemeBorders {
            default: STONE.s700,
            subtle: STONE.s800,
            strong: STONE.s500,
            accent: ORANGE.s500,
            danger: RED.s500,
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
                    bg: ORANGE.s500,
                    fg: WHITE,
                    hover_bg: ORANGE.s400,
                    active_bg: ORANGE.s600,
                    border: "transparent",
                },
                secondary: ButtonVariant {
                    bg: STONE.s700,
                    fg: STONE.s50,
                    hover_bg: STONE.s600,
                    active_bg: STONE.s800,
                    border: "transparent",
                },
                outline: ButtonVariant {
                    bg: "transparent",
                    fg: ORANGE.s400,
                    hover_bg: "rgba(255, 255, 255, 0.05)",
                    active_bg: "rgba(255, 255, 255, 0.1)",
                    border: ORANGE.s500,
                },
                ghost: ButtonVariant {
                    bg: "transparent",
                    fg: STONE.s50,
                    hover_bg: "rgba(255, 255, 255, 0.05)",
                    active_bg: "rgba(255, 255, 255, 0.1)",
                    border: "transparent",
                },
            },
            panel: PanelTokens {
                bg: STONE.s800,
                header_bg: STONE.s700,
                footer_bg: STONE.s700,
                border: STONE.s700,
            },
            menu: MenuTokens {
                bg: STONE.s800,
                border: STONE.s700,
                item: MenuItemTokens {
                    fg: STONE.s50,
                    hover_bg: STONE.s700,
                    active_bg: ORANGE.s700,
                    selected_bg: ORANGE.s600,
                    disabled_fg: STONE.s500,
                },
            },
            tabs: TabsTokens {
                bg: STONE.s800,
                border: STONE.s700,
                tab: TabTokens {
                    fg: STONE.s400,
                    active_fg: STONE.s50,
                    active_bg: STONE.s700,
                    hover_fg: STONE.s200,
                    indicator: ORANGE.s500,
                },
            },
            dropdown: DropdownTokens {
                bg: STONE.s800,
                border: STONE.s700,
                item: DropdownItemTokens {
                    fg: STONE.s50,
                    hover_bg: STONE.s700,
                    selected_bg: ORANGE.s700,
                },
            },
            input: InputTokens {
                bg: STONE.s800,
                fg: STONE.s50,
                placeholder: STONE.s500,
                border: STONE.s700,
                focus_border: ORANGE.s500,
                invalid_border: RED.s500,
            },
            checkbox: CheckboxTokens {
                bg: STONE.s800,
                border: STONE.s600,
                check_color: WHITE,
                checked_bg: ORANGE.s500,
            },
            radio: RadioTokens {
                bg: STONE.s800,
                dot_color: ORANGE.s500,
                border: STONE.s600,
                checked_border: ORANGE.s500,
            },
            tooltip: TooltipTokens {
                bg: STONE.s700,
                fg: STONE.s50,
            },
            modal: ModalTokens {
                bg: STONE.s800,
                border: STONE.s700,
                overlay: "rgba(26, 15, 10, 0.85)",
            },
            badge: BadgeTokens {
                default: BadgeVariant {
                    bg: STONE.s700,
                    fg: STONE.s50,
                },
                success: BadgeVariant {
                    bg: GREEN.s500,
                    fg: WHITE,
                },
                warning: BadgeVariant {
                    bg: AMBER.s500,
                    fg: STONE.s900,
                },
                danger: BadgeVariant {
                    bg: RED.s500,
                    fg: WHITE,
                },
            },
            list: ListTokens {
                item: ListItemTokens {
                    bg: "transparent",
                    hover_bg: STONE.s800,
                    selected_bg: ORANGE.s700,
                    fg: STONE.s50,
                },
            },
            header: HeaderTokens {
                default: HeaderVariant {
                    bg: STONE.s800,
                    fg: STONE.s50,
                    title_fg: STONE.s50,
                    subtitle_fg: STONE.s400,
                    border: STONE.s700,
                },
                primary: HeaderVariant {
                    bg: ORANGE.s600,
                    fg: WHITE,
                    title_fg: WHITE,
                    subtitle_fg: "rgba(255, 255, 255, 0.8)",
                    border: ORANGE.s700,
                },
                secondary: HeaderVariant {
                    bg: STONE.s700,
                    fg: STONE.s50,
                    title_fg: STONE.s50,
                    subtitle_fg: STONE.s400,
                    border: STONE.s600,
                },
                success: HeaderVariant {
                    bg: GREEN.s600,
                    fg: WHITE,
                    title_fg: WHITE,
                    subtitle_fg: "rgba(255, 255, 255, 0.8)",
                    border: GREEN.s700,
                },
                warning: HeaderVariant {
                    bg: AMBER.s500,
                    fg: STONE.s900,
                    title_fg: STONE.s900,
                    subtitle_fg: "rgba(0, 0, 0, 0.7)",
                    border: AMBER.s600,
                },
                danger: HeaderVariant {
                    bg: RED.s600,
                    fg: WHITE,
                    title_fg: WHITE,
                    subtitle_fg: "rgba(255, 255, 255, 0.8)",
                    border: RED.s700,
                },
            },
            statusbar: StatusbarTokens {
                default: StatusbarVariant {
                    bg: STONE.s800,
                    fg: STONE.s400,
                },
                primary: StatusbarVariant {
                    bg: ORANGE.s600,
                    fg: WHITE,
                },
                info: StatusbarVariant {
                    bg: CYAN.s600,
                    fg: WHITE,
                },
                success: StatusbarVariant {
                    bg: GREEN.s600,
                    fg: WHITE,
                },
                warning: StatusbarVariant {
                    bg: AMBER.s500,
                    fg: STONE.s900,
                },
                danger: StatusbarVariant {
                    bg: RED.s600,
                    fg: WHITE,
                },
            },
            page: PageTokens {
                default: PageVariant {
                    bg: STONE.s900,
                    title_fg: STONE.s50,
                    subtitle_fg: STONE.s400,
                    border: STONE.s700,
                },
                primary: PageVariant {
                    bg: STONE.s900,
                    title_fg: ORANGE.s400,
                    subtitle_fg: STONE.s400,
                    border: ORANGE.s700,
                },
                secondary: PageVariant {
                    bg: STONE.s900,
                    title_fg: STONE.s200,
                    subtitle_fg: STONE.s500,
                    border: STONE.s700,
                },
            },
            appshell: AppShellTokens {
                bg: "#1a0f0a",
                divider_fg: STONE.s700,
                sidebar_bg: STONE.s900,
                aside_bg: STONE.s900,
            },
            toast: ToastTokens {
                success: ToastVariant {
                    bg: GREEN.s900,
                    fg: GREEN.s100,
                    border: GREEN.s600,
                    icon_fg: GREEN.s400,
                },
                error: ToastVariant {
                    bg: RED.s900,
                    fg: RED.s100,
                    border: RED.s600,
                    icon_fg: RED.s400,
                },
                warning: ToastVariant {
                    bg: AMBER.s900,
                    fg: AMBER.s100,
                    border: AMBER.s500,
                    icon_fg: AMBER.s400,
                },
                info: ToastVariant {
                    bg: ORANGE.s900,
                    fg: ORANGE.s100,
                    border: ORANGE.s600,
                    icon_fg: ORANGE.s400,
                },
            },
            slider: SliderTokens {
                track_bg: STONE.s700,
                fill_bg: ORANGE.s500,
                thumb: WHITE,
            },
        },
    }
}
