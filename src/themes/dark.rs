//! Dark Theme
//!
//! Default dark theme with blue primary and slate neutrals.

use super::colors::*;
use super::types::*;

/// Create the default dark theme
pub fn dark_theme() -> Theme {
    let palette = ThemePalette {
        primary: BLUE,
        secondary: SLATE,
        success: GREEN,
        warning: AMBER,
        danger: RED,
        neutral: SLATE,
    };

    Theme {
        name: "dark",
        mode: ThemeMode::Dark,
        meta: ThemeMeta {
            version: "1.0.0",
            author: "tuiuiu",
            description: "Default dark theme with blue primary and slate neutrals",
        },
        palette,
        background: ThemeBackground {
            lowest: SLATE.s950,
            base: SLATE.s900,
            subtle: SLATE.s800,
            surface: SLATE.s700,
            raised: SLATE.s600,
            elevated: SLATE.s500,
            popover: SLATE.s800,
            overlay: "rgba(0, 0, 0, 0.7)",
        },
        foreground: ThemeForeground {
            primary: SLATE.s50,
            secondary: SLATE.s200,
            muted: SLATE.s400,
            disabled: SLATE.s500,
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
            highlight: BLUE.s500,
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
                border: BLUE.s400,
                ring: FocusRing {
                    color: BLUE.s500,
                    width: 2,
                },
            },
            disabled: DisabledState {
                opacity: 0.4,
                bg: SLATE.s800,
                fg: SLATE.s500,
            },
            selected: SelectedState {
                bg: BLUE.s700,
                fg: WHITE,
            },
        },
        borders: ThemeBorders {
            default: SLATE.s700,
            subtle: SLATE.s800,
            strong: SLATE.s500,
            accent: BLUE.s500,
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
                    bg: BLUE.s500,
                    fg: WHITE,
                    hover_bg: BLUE.s400,
                    active_bg: BLUE.s600,
                    border: "transparent",
                },
                secondary: ButtonVariant {
                    bg: SLATE.s700,
                    fg: SLATE.s50,
                    hover_bg: SLATE.s600,
                    active_bg: SLATE.s800,
                    border: "transparent",
                },
                outline: ButtonVariant {
                    bg: "transparent",
                    fg: BLUE.s400,
                    hover_bg: "rgba(255, 255, 255, 0.05)",
                    active_bg: "rgba(255, 255, 255, 0.1)",
                    border: BLUE.s500,
                },
                ghost: ButtonVariant {
                    bg: "transparent",
                    fg: SLATE.s50,
                    hover_bg: "rgba(255, 255, 255, 0.05)",
                    active_bg: "rgba(255, 255, 255, 0.1)",
                    border: "transparent",
                },
            },
            panel: PanelTokens {
                bg: SLATE.s800,
                header_bg: SLATE.s700,
                footer_bg: SLATE.s700,
                border: SLATE.s700,
            },
            menu: MenuTokens {
                bg: SLATE.s800,
                border: SLATE.s700,
                item: MenuItemTokens {
                    fg: SLATE.s50,
                    hover_bg: SLATE.s700,
                    active_bg: BLUE.s700,
                    selected_bg: BLUE.s600,
                    disabled_fg: SLATE.s500,
                },
            },
            tabs: TabsTokens {
                bg: SLATE.s800,
                border: SLATE.s700,
                tab: TabTokens {
                    fg: SLATE.s400,
                    active_fg: SLATE.s50,
                    active_bg: SLATE.s700,
                    hover_fg: SLATE.s200,
                    indicator: BLUE.s500,
                },
            },
            dropdown: DropdownTokens {
                bg: SLATE.s800,
                border: SLATE.s700,
                item: DropdownItemTokens {
                    fg: SLATE.s50,
                    hover_bg: SLATE.s700,
                    selected_bg: BLUE.s700,
                },
            },
            input: InputTokens {
                bg: SLATE.s800,
                fg: SLATE.s50,
                placeholder: SLATE.s500,
                border: SLATE.s700,
                focus_border: BLUE.s500,
                invalid_border: RED.s500,
            },
            checkbox: CheckboxTokens {
                bg: SLATE.s800,
                border: SLATE.s600,
                check_color: WHITE,
                checked_bg: BLUE.s500,
            },
            radio: RadioTokens {
                bg: SLATE.s800,
                dot_color: BLUE.s500,
                border: SLATE.s600,
                checked_border: BLUE.s500,
            },
            tooltip: TooltipTokens {
                bg: SLATE.s700,
                fg: SLATE.s50,
            },
            modal: ModalTokens {
                bg: SLATE.s800,
                border: SLATE.s700,
                overlay: "rgba(0, 0, 0, 0.7)",
            },
            badge: BadgeTokens {
                default: BadgeVariant {
                    bg: SLATE.s700,
                    fg: SLATE.s50,
                },
                success: BadgeVariant {
                    bg: GREEN.s500,
                    fg: WHITE,
                },
                warning: BadgeVariant {
                    bg: AMBER.s500,
                    fg: SLATE.s900,
                },
                danger: BadgeVariant {
                    bg: RED.s500,
                    fg: WHITE,
                },
            },
            list: ListTokens {
                item: ListItemTokens {
                    bg: "transparent",
                    hover_bg: SLATE.s800,
                    selected_bg: BLUE.s700,
                    fg: SLATE.s50,
                },
            },
            header: HeaderTokens {
                default: HeaderVariant {
                    bg: SLATE.s800,
                    fg: SLATE.s50,
                    title_fg: SLATE.s50,
                    subtitle_fg: SLATE.s400,
                    border: SLATE.s700,
                },
                primary: HeaderVariant {
                    bg: BLUE.s600,
                    fg: WHITE,
                    title_fg: WHITE,
                    subtitle_fg: "rgba(255, 255, 255, 0.8)",
                    border: BLUE.s700,
                },
                secondary: HeaderVariant {
                    bg: SLATE.s700,
                    fg: SLATE.s50,
                    title_fg: SLATE.s50,
                    subtitle_fg: SLATE.s400,
                    border: SLATE.s600,
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
                    fg: SLATE.s900,
                    title_fg: SLATE.s900,
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
                    bg: SLATE.s800,
                    fg: SLATE.s400,
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
                    bg: AMBER.s500,
                    fg: SLATE.s900,
                },
                danger: StatusbarVariant {
                    bg: RED.s600,
                    fg: WHITE,
                },
            },
            page: PageTokens {
                default: PageVariant {
                    bg: SLATE.s900,
                    title_fg: SLATE.s50,
                    subtitle_fg: SLATE.s400,
                    border: SLATE.s700,
                },
                primary: PageVariant {
                    bg: SLATE.s900,
                    title_fg: BLUE.s400,
                    subtitle_fg: SLATE.s400,
                    border: BLUE.s700,
                },
                secondary: PageVariant {
                    bg: SLATE.s900,
                    title_fg: SLATE.s200,
                    subtitle_fg: SLATE.s500,
                    border: SLATE.s700,
                },
            },
            appshell: AppShellTokens {
                bg: SLATE.s950,
                divider_fg: SLATE.s700,
                sidebar_bg: SLATE.s900,
                aside_bg: SLATE.s900,
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
                    bg: CYAN.s900,
                    fg: CYAN.s100,
                    border: CYAN.s600,
                    icon_fg: CYAN.s400,
                },
            },
            slider: SliderTokens {
                track_bg: SLATE.s700,
                fill_bg: BLUE.s500,
                thumb: WHITE,
            },
        },
    }
}
