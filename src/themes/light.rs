//! Light Theme
//!
//! Default light theme with blue primary and slate neutrals.

use super::colors::*;
use super::types::*;

/// Create the default light theme
pub fn light_theme() -> Theme {
    let palette = ThemePalette {
        primary: BLUE,
        secondary: SLATE,
        success: GREEN,
        warning: AMBER,
        danger: RED,
        neutral: SLATE,
    };

    Theme {
        name: "light",
        mode: ThemeMode::Light,
        meta: ThemeMeta {
            version: "1.0.0",
            author: "tuiuiu",
            description: "Default light theme with blue primary and slate neutrals",
        },
        palette,
        background: ThemeBackground {
            lowest: WHITE,
            base: SLATE.s50,
            subtle: SLATE.s100,
            surface: SLATE.s200,
            raised: SLATE.s300,
            elevated: SLATE.s400,
            popover: WHITE,
            overlay: "rgba(0, 0, 0, 0.4)",
        },
        foreground: ThemeForeground {
            primary: SLATE.s900,
            secondary: SLATE.s700,
            muted: SLATE.s500,
            disabled: SLATE.s400,
            inverse: ThemeForegroundInverse {
                base: BLACK,
                soft: "rgba(0, 0, 0, 0.6)",
                subtle: "rgba(0, 0, 0, 0.35)",
            },
        },
        accents: ThemeAccents {
            positive: GREEN.s600,
            warning: AMBER.s600,
            critical: RED.s600,
            info: CYAN.s600,
            highlight: BLUE.s600,
        },
        states: ThemeStates {
            hover: HoverState {
                bg: "rgba(0, 0, 0, 0.05)",
                fg: None,
            },
            active: ActiveState {
                bg: "rgba(0, 0, 0, 0.1)",
            },
            focus: FocusState {
                border: BLUE.s500,
                ring: FocusRing {
                    color: BLUE.s400,
                    width: 2,
                },
            },
            disabled: DisabledState {
                opacity: 0.4,
                bg: SLATE.s100,
                fg: SLATE.s400,
            },
            selected: SelectedState {
                bg: BLUE.s100,
                fg: BLUE.s900,
            },
        },
        borders: ThemeBorders {
            default: SLATE.s300,
            subtle: SLATE.s200,
            strong: SLATE.s400,
            accent: BLUE.s500,
            danger: RED.s500,
        },
        opacity: ThemeOpacity {
            disabled: 0.4,
            muted: 0.7,
            overlay: 0.4,
            ghost: 0.1,
        },
        components: ComponentTokens {
            button: ButtonTokens {
                primary: ButtonVariant {
                    bg: BLUE.s600,
                    fg: WHITE,
                    hover_bg: BLUE.s700,
                    active_bg: BLUE.s800,
                    border: "transparent",
                },
                secondary: ButtonVariant {
                    bg: SLATE.s200,
                    fg: SLATE.s900,
                    hover_bg: SLATE.s300,
                    active_bg: SLATE.s400,
                    border: "transparent",
                },
                outline: ButtonVariant {
                    bg: "transparent",
                    fg: BLUE.s600,
                    hover_bg: "rgba(0, 0, 0, 0.05)",
                    active_bg: "rgba(0, 0, 0, 0.1)",
                    border: BLUE.s500,
                },
                ghost: ButtonVariant {
                    bg: "transparent",
                    fg: SLATE.s900,
                    hover_bg: "rgba(0, 0, 0, 0.05)",
                    active_bg: "rgba(0, 0, 0, 0.1)",
                    border: "transparent",
                },
            },
            panel: PanelTokens {
                bg: WHITE,
                header_bg: SLATE.s100,
                footer_bg: SLATE.s100,
                border: SLATE.s300,
            },
            menu: MenuTokens {
                bg: WHITE,
                border: SLATE.s300,
                item: MenuItemTokens {
                    fg: SLATE.s900,
                    hover_bg: SLATE.s100,
                    active_bg: BLUE.s100,
                    selected_bg: BLUE.s200,
                    disabled_fg: SLATE.s400,
                },
            },
            tabs: TabsTokens {
                bg: WHITE,
                border: SLATE.s300,
                tab: TabTokens {
                    fg: SLATE.s500,
                    active_fg: SLATE.s900,
                    active_bg: WHITE,
                    hover_fg: SLATE.s700,
                    indicator: BLUE.s500,
                },
            },
            dropdown: DropdownTokens {
                bg: WHITE,
                border: SLATE.s300,
                item: DropdownItemTokens {
                    fg: SLATE.s900,
                    hover_bg: SLATE.s100,
                    selected_bg: BLUE.s100,
                },
            },
            input: InputTokens {
                bg: WHITE,
                fg: SLATE.s900,
                placeholder: SLATE.s400,
                border: SLATE.s300,
                focus_border: BLUE.s500,
                invalid_border: RED.s500,
            },
            checkbox: CheckboxTokens {
                bg: WHITE,
                border: SLATE.s400,
                check_color: WHITE,
                checked_bg: BLUE.s600,
            },
            radio: RadioTokens {
                bg: WHITE,
                dot_color: BLUE.s600,
                border: SLATE.s400,
                checked_border: BLUE.s600,
            },
            tooltip: TooltipTokens {
                bg: SLATE.s900,
                fg: WHITE,
            },
            modal: ModalTokens {
                bg: WHITE,
                border: SLATE.s300,
                overlay: "rgba(0, 0, 0, 0.4)",
            },
            badge: BadgeTokens {
                default: BadgeVariant {
                    bg: SLATE.s200,
                    fg: SLATE.s900,
                },
                success: BadgeVariant {
                    bg: GREEN.s100,
                    fg: GREEN.s800,
                },
                warning: BadgeVariant {
                    bg: AMBER.s100,
                    fg: AMBER.s800,
                },
                danger: BadgeVariant {
                    bg: RED.s100,
                    fg: RED.s800,
                },
            },
            list: ListTokens {
                item: ListItemTokens {
                    bg: "transparent",
                    hover_bg: SLATE.s100,
                    selected_bg: BLUE.s100,
                    fg: SLATE.s900,
                },
            },
            header: HeaderTokens {
                default: HeaderVariant {
                    bg: WHITE,
                    fg: SLATE.s900,
                    title_fg: SLATE.s900,
                    subtitle_fg: SLATE.s500,
                    border: SLATE.s300,
                },
                primary: HeaderVariant {
                    bg: BLUE.s600,
                    fg: WHITE,
                    title_fg: WHITE,
                    subtitle_fg: "rgba(255, 255, 255, 0.8)",
                    border: BLUE.s700,
                },
                secondary: HeaderVariant {
                    bg: SLATE.s100,
                    fg: SLATE.s900,
                    title_fg: SLATE.s900,
                    subtitle_fg: SLATE.s500,
                    border: SLATE.s300,
                },
                success: HeaderVariant {
                    bg: GREEN.s600,
                    fg: WHITE,
                    title_fg: WHITE,
                    subtitle_fg: "rgba(255, 255, 255, 0.8)",
                    border: GREEN.s700,
                },
                warning: HeaderVariant {
                    bg: AMBER.s400,
                    fg: AMBER.s900,
                    title_fg: AMBER.s900,
                    subtitle_fg: "rgba(0, 0, 0, 0.6)",
                    border: AMBER.s500,
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
                    bg: SLATE.s100,
                    fg: SLATE.s600,
                },
                primary: StatusbarVariant {
                    bg: BLUE.s600,
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
                    bg: AMBER.s400,
                    fg: AMBER.s900,
                },
                danger: StatusbarVariant {
                    bg: RED.s600,
                    fg: WHITE,
                },
            },
            page: PageTokens {
                default: PageVariant {
                    bg: WHITE,
                    title_fg: SLATE.s900,
                    subtitle_fg: SLATE.s500,
                    border: SLATE.s300,
                },
                primary: PageVariant {
                    bg: WHITE,
                    title_fg: BLUE.s700,
                    subtitle_fg: SLATE.s500,
                    border: BLUE.s300,
                },
                secondary: PageVariant {
                    bg: SLATE.s50,
                    title_fg: SLATE.s800,
                    subtitle_fg: SLATE.s500,
                    border: SLATE.s300,
                },
            },
            appshell: AppShellTokens {
                bg: SLATE.s50,
                divider_fg: SLATE.s300,
                sidebar_bg: WHITE,
                aside_bg: WHITE,
            },
            toast: ToastTokens {
                success: ToastVariant {
                    bg: GREEN.s50,
                    fg: GREEN.s900,
                    border: GREEN.s300,
                    icon_fg: GREEN.s600,
                },
                error: ToastVariant {
                    bg: RED.s50,
                    fg: RED.s900,
                    border: RED.s300,
                    icon_fg: RED.s600,
                },
                warning: ToastVariant {
                    bg: AMBER.s50,
                    fg: AMBER.s900,
                    border: AMBER.s300,
                    icon_fg: AMBER.s600,
                },
                info: ToastVariant {
                    bg: CYAN.s50,
                    fg: CYAN.s900,
                    border: CYAN.s300,
                    icon_fg: CYAN.s600,
                },
            },
            slider: SliderTokens {
                track_bg: SLATE.s200,
                fill_bg: BLUE.s500,
                thumb: SLATE.s800,
            },
        },
    }
}
