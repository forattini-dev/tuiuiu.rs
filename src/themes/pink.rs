//! Pink Theme
//!
//! A vibrant pink-themed dark theme with playful aesthetics.

use super::colors::*;
use super::types::*;

/// Create the pink theme
pub fn pink_theme() -> Theme {
    let palette = ThemePalette {
        primary: PINK,
        secondary: SLATE,
        success: GREEN,
        warning: AMBER,
        danger: RED,
        neutral: SLATE,
    };

    Theme {
        name: "pink",
        mode: ThemeMode::Dark,
        meta: ThemeMeta {
            version: "1.0.0",
            author: "tuiuiu",
            description: "Vibrant pink-themed dark theme with playful aesthetics",
        },
        palette,
        background: ThemeBackground {
            lowest: "#1a0a14",
            base: "#1f0f18",
            subtle: "#2a1520",
            surface: "#351a2a",
            raised: "#402035",
            elevated: "#4a2540",
            popover: "#2a1520",
            overlay: "rgba(26, 10, 20, 0.85)",
        },
        foreground: ThemeForeground {
            primary: PINK.s50,
            secondary: PINK.s200,
            muted: PINK.s400,
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
            highlight: PINK.s500,
        },
        states: ThemeStates {
            hover: HoverState {
                bg: "rgba(255, 182, 193, 0.08)",
                fg: None,
            },
            active: ActiveState {
                bg: "rgba(255, 182, 193, 0.15)",
            },
            focus: FocusState {
                border: PINK.s400,
                ring: FocusRing {
                    color: PINK.s500,
                    width: 2,
                },
            },
            disabled: DisabledState {
                opacity: 0.4,
                bg: "#2a1520",
                fg: SLATE.s500,
            },
            selected: SelectedState {
                bg: PINK.s700,
                fg: WHITE,
            },
        },
        borders: ThemeBorders {
            default: "#4a2540",
            subtle: "#351a2a",
            strong: PINK.s600,
            accent: PINK.s500,
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
                    bg: PINK.s500,
                    fg: WHITE,
                    hover_bg: PINK.s400,
                    active_bg: PINK.s600,
                    border: "transparent",
                },
                secondary: ButtonVariant {
                    bg: "#4a2540",
                    fg: PINK.s50,
                    hover_bg: "#552a48",
                    active_bg: "#351a2a",
                    border: "transparent",
                },
                outline: ButtonVariant {
                    bg: "transparent",
                    fg: PINK.s400,
                    hover_bg: "rgba(255, 182, 193, 0.05)",
                    active_bg: "rgba(255, 182, 193, 0.1)",
                    border: PINK.s500,
                },
                ghost: ButtonVariant {
                    bg: "transparent",
                    fg: PINK.s200,
                    hover_bg: "rgba(255, 182, 193, 0.05)",
                    active_bg: "rgba(255, 182, 193, 0.1)",
                    border: "transparent",
                },
            },
            panel: PanelTokens {
                bg: "#2a1520",
                header_bg: "#351a2a",
                footer_bg: "#351a2a",
                border: "#4a2540",
            },
            menu: MenuTokens {
                bg: "#2a1520",
                border: "#4a2540",
                item: MenuItemTokens {
                    fg: PINK.s50,
                    hover_bg: "#351a2a",
                    active_bg: PINK.s700,
                    selected_bg: PINK.s600,
                    disabled_fg: SLATE.s500,
                },
            },
            tabs: TabsTokens {
                bg: "#2a1520",
                border: "#4a2540",
                tab: TabTokens {
                    fg: PINK.s400,
                    active_fg: PINK.s50,
                    active_bg: "#351a2a",
                    hover_fg: PINK.s200,
                    indicator: PINK.s500,
                },
            },
            dropdown: DropdownTokens {
                bg: "#2a1520",
                border: "#4a2540",
                item: DropdownItemTokens {
                    fg: PINK.s50,
                    hover_bg: "#351a2a",
                    selected_bg: PINK.s700,
                },
            },
            input: InputTokens {
                bg: "#2a1520",
                fg: PINK.s50,
                placeholder: PINK.s500,
                border: "#4a2540",
                focus_border: PINK.s500,
                invalid_border: RED.s500,
            },
            checkbox: CheckboxTokens {
                bg: "#2a1520",
                border: PINK.s600,
                check_color: WHITE,
                checked_bg: PINK.s500,
            },
            radio: RadioTokens {
                bg: "#2a1520",
                dot_color: PINK.s500,
                border: PINK.s600,
                checked_border: PINK.s500,
            },
            tooltip: TooltipTokens {
                bg: "#351a2a",
                fg: PINK.s50,
            },
            modal: ModalTokens {
                bg: "#2a1520",
                border: "#4a2540",
                overlay: "rgba(26, 10, 20, 0.85)",
            },
            badge: BadgeTokens {
                default: BadgeVariant {
                    bg: "#4a2540",
                    fg: PINK.s50,
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
                    hover_bg: "#351a2a",
                    selected_bg: PINK.s700,
                    fg: PINK.s50,
                },
            },
            header: HeaderTokens {
                default: HeaderVariant {
                    bg: "#2a1520",
                    fg: PINK.s50,
                    title_fg: PINK.s50,
                    subtitle_fg: PINK.s400,
                    border: "#4a2540",
                },
                primary: HeaderVariant {
                    bg: PINK.s600,
                    fg: WHITE,
                    title_fg: WHITE,
                    subtitle_fg: "rgba(255, 255, 255, 0.8)",
                    border: PINK.s700,
                },
                secondary: HeaderVariant {
                    bg: "#351a2a",
                    fg: PINK.s50,
                    title_fg: PINK.s50,
                    subtitle_fg: PINK.s400,
                    border: "#4a2540",
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
                    bg: "#2a1520",
                    fg: PINK.s400,
                },
                primary: StatusbarVariant {
                    bg: PINK.s600,
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
                    bg: "#1f0f18",
                    title_fg: PINK.s50,
                    subtitle_fg: PINK.s400,
                    border: "#4a2540",
                },
                primary: PageVariant {
                    bg: "#1f0f18",
                    title_fg: PINK.s400,
                    subtitle_fg: PINK.s500,
                    border: PINK.s700,
                },
                secondary: PageVariant {
                    bg: "#1f0f18",
                    title_fg: PINK.s200,
                    subtitle_fg: PINK.s500,
                    border: "#351a2a",
                },
            },
            appshell: AppShellTokens {
                bg: "#1a0a14",
                divider_fg: "#4a2540",
                sidebar_bg: "#1f0f18",
                aside_bg: "#1f0f18",
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
                    bg: PINK.s900,
                    fg: PINK.s100,
                    border: PINK.s600,
                    icon_fg: PINK.s400,
                },
            },
            slider: SliderTokens {
                track_bg: "#4a2540",
                fill_bg: PINK.s500,
                thumb: WHITE,
            },
        },
    }
}
