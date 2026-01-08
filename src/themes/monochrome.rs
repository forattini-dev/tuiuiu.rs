//! Monochrome Theme
//!
//! A clean, grayscale theme for minimalist aesthetics.

use super::colors::*;
use super::types::*;

/// Create the monochrome theme
pub fn monochrome_theme() -> Theme {
    let palette = ThemePalette {
        primary: GRAY,
        secondary: NEUTRAL,
        success: GRAY,
        warning: GRAY,
        danger: GRAY,
        neutral: NEUTRAL,
    };

    Theme {
        name: "monochrome",
        mode: ThemeMode::Dark,
        meta: ThemeMeta {
            version: "1.0.0",
            author: "tuiuiu",
            description: "Clean grayscale theme for minimalist aesthetics",
        },
        palette,
        background: ThemeBackground {
            lowest: BLACK,
            base: NEUTRAL.s950,
            subtle: NEUTRAL.s900,
            surface: NEUTRAL.s800,
            raised: NEUTRAL.s700,
            elevated: NEUTRAL.s600,
            popover: NEUTRAL.s800,
            overlay: "rgba(0, 0, 0, 0.8)",
        },
        foreground: ThemeForeground {
            primary: NEUTRAL.s100,
            secondary: NEUTRAL.s300,
            muted: NEUTRAL.s500,
            disabled: NEUTRAL.s600,
            inverse: ThemeForegroundInverse {
                base: BLACK,
                soft: "rgba(0, 0, 0, 0.7)",
                subtle: "rgba(0, 0, 0, 0.4)",
            },
        },
        accents: ThemeAccents {
            positive: NEUTRAL.s300,
            warning: NEUTRAL.s400,
            critical: NEUTRAL.s200,
            info: NEUTRAL.s400,
            highlight: WHITE,
        },
        states: ThemeStates {
            hover: HoverState {
                bg: "rgba(255, 255, 255, 0.08)",
                fg: None,
            },
            active: ActiveState {
                bg: "rgba(255, 255, 255, 0.15)",
            },
            focus: FocusState {
                border: NEUTRAL.s400,
                ring: FocusRing {
                    color: WHITE,
                    width: 1,
                },
            },
            disabled: DisabledState {
                opacity: 0.35,
                bg: NEUTRAL.s900,
                fg: NEUTRAL.s600,
            },
            selected: SelectedState {
                bg: NEUTRAL.s600,
                fg: WHITE,
            },
        },
        borders: ThemeBorders {
            default: NEUTRAL.s700,
            subtle: NEUTRAL.s800,
            strong: NEUTRAL.s500,
            accent: NEUTRAL.s400,
            danger: NEUTRAL.s300,
        },
        opacity: ThemeOpacity {
            disabled: 0.35,
            muted: 0.6,
            overlay: 0.6,
            ghost: 0.15,
        },
        components: ComponentTokens {
            button: ButtonTokens {
                primary: ButtonVariant {
                    bg: WHITE,
                    fg: BLACK,
                    hover_bg: NEUTRAL.s200,
                    active_bg: NEUTRAL.s300,
                    border: "transparent",
                },
                secondary: ButtonVariant {
                    bg: NEUTRAL.s700,
                    fg: NEUTRAL.s100,
                    hover_bg: NEUTRAL.s600,
                    active_bg: NEUTRAL.s800,
                    border: "transparent",
                },
                outline: ButtonVariant {
                    bg: "transparent",
                    fg: NEUTRAL.s200,
                    hover_bg: "rgba(255, 255, 255, 0.05)",
                    active_bg: "rgba(255, 255, 255, 0.1)",
                    border: NEUTRAL.s500,
                },
                ghost: ButtonVariant {
                    bg: "transparent",
                    fg: NEUTRAL.s200,
                    hover_bg: "rgba(255, 255, 255, 0.05)",
                    active_bg: "rgba(255, 255, 255, 0.1)",
                    border: "transparent",
                },
            },
            panel: PanelTokens {
                bg: NEUTRAL.s900,
                header_bg: NEUTRAL.s800,
                footer_bg: NEUTRAL.s800,
                border: NEUTRAL.s700,
            },
            menu: MenuTokens {
                bg: NEUTRAL.s900,
                border: NEUTRAL.s700,
                item: MenuItemTokens {
                    fg: NEUTRAL.s200,
                    hover_bg: NEUTRAL.s800,
                    active_bg: NEUTRAL.s700,
                    selected_bg: NEUTRAL.s600,
                    disabled_fg: NEUTRAL.s600,
                },
            },
            tabs: TabsTokens {
                bg: NEUTRAL.s900,
                border: NEUTRAL.s700,
                tab: TabTokens {
                    fg: NEUTRAL.s500,
                    active_fg: WHITE,
                    active_bg: NEUTRAL.s800,
                    hover_fg: NEUTRAL.s300,
                    indicator: WHITE,
                },
            },
            dropdown: DropdownTokens {
                bg: NEUTRAL.s900,
                border: NEUTRAL.s700,
                item: DropdownItemTokens {
                    fg: NEUTRAL.s200,
                    hover_bg: NEUTRAL.s800,
                    selected_bg: NEUTRAL.s700,
                },
            },
            input: InputTokens {
                bg: NEUTRAL.s900,
                fg: NEUTRAL.s100,
                placeholder: NEUTRAL.s600,
                border: NEUTRAL.s700,
                focus_border: WHITE,
                invalid_border: NEUTRAL.s300,
            },
            checkbox: CheckboxTokens {
                bg: NEUTRAL.s900,
                border: NEUTRAL.s600,
                check_color: BLACK,
                checked_bg: WHITE,
            },
            radio: RadioTokens {
                bg: NEUTRAL.s900,
                dot_color: WHITE,
                border: NEUTRAL.s600,
                checked_border: WHITE,
            },
            tooltip: TooltipTokens {
                bg: NEUTRAL.s800,
                fg: NEUTRAL.s100,
            },
            modal: ModalTokens {
                bg: NEUTRAL.s900,
                border: NEUTRAL.s700,
                overlay: "rgba(0, 0, 0, 0.8)",
            },
            badge: BadgeTokens {
                default: BadgeVariant {
                    bg: NEUTRAL.s700,
                    fg: NEUTRAL.s100,
                },
                success: BadgeVariant {
                    bg: NEUTRAL.s500,
                    fg: WHITE,
                },
                warning: BadgeVariant {
                    bg: NEUTRAL.s600,
                    fg: WHITE,
                },
                danger: BadgeVariant {
                    bg: NEUTRAL.s400,
                    fg: BLACK,
                },
            },
            list: ListTokens {
                item: ListItemTokens {
                    bg: "transparent",
                    hover_bg: NEUTRAL.s800,
                    selected_bg: NEUTRAL.s700,
                    fg: NEUTRAL.s200,
                },
            },
            header: HeaderTokens {
                default: HeaderVariant {
                    bg: NEUTRAL.s900,
                    fg: NEUTRAL.s100,
                    title_fg: WHITE,
                    subtitle_fg: NEUTRAL.s500,
                    border: NEUTRAL.s700,
                },
                primary: HeaderVariant {
                    bg: WHITE,
                    fg: BLACK,
                    title_fg: BLACK,
                    subtitle_fg: NEUTRAL.s600,
                    border: NEUTRAL.s300,
                },
                secondary: HeaderVariant {
                    bg: NEUTRAL.s800,
                    fg: NEUTRAL.s100,
                    title_fg: NEUTRAL.s100,
                    subtitle_fg: NEUTRAL.s500,
                    border: NEUTRAL.s700,
                },
                success: HeaderVariant {
                    bg: NEUTRAL.s700,
                    fg: WHITE,
                    title_fg: WHITE,
                    subtitle_fg: NEUTRAL.s400,
                    border: NEUTRAL.s600,
                },
                warning: HeaderVariant {
                    bg: NEUTRAL.s600,
                    fg: WHITE,
                    title_fg: WHITE,
                    subtitle_fg: NEUTRAL.s300,
                    border: NEUTRAL.s500,
                },
                danger: HeaderVariant {
                    bg: NEUTRAL.s500,
                    fg: BLACK,
                    title_fg: BLACK,
                    subtitle_fg: NEUTRAL.s800,
                    border: NEUTRAL.s400,
                },
            },
            statusbar: StatusbarTokens {
                default: StatusbarVariant {
                    bg: NEUTRAL.s900,
                    fg: NEUTRAL.s500,
                },
                primary: StatusbarVariant {
                    bg: WHITE,
                    fg: BLACK,
                },
                info: StatusbarVariant {
                    bg: NEUTRAL.s700,
                    fg: WHITE,
                },
                success: StatusbarVariant {
                    bg: NEUTRAL.s600,
                    fg: WHITE,
                },
                warning: StatusbarVariant {
                    bg: NEUTRAL.s500,
                    fg: BLACK,
                },
                danger: StatusbarVariant {
                    bg: NEUTRAL.s400,
                    fg: BLACK,
                },
            },
            page: PageTokens {
                default: PageVariant {
                    bg: NEUTRAL.s950,
                    title_fg: WHITE,
                    subtitle_fg: NEUTRAL.s500,
                    border: NEUTRAL.s700,
                },
                primary: PageVariant {
                    bg: NEUTRAL.s950,
                    title_fg: WHITE,
                    subtitle_fg: NEUTRAL.s400,
                    border: NEUTRAL.s600,
                },
                secondary: PageVariant {
                    bg: NEUTRAL.s950,
                    title_fg: NEUTRAL.s300,
                    subtitle_fg: NEUTRAL.s600,
                    border: NEUTRAL.s800,
                },
            },
            appshell: AppShellTokens {
                bg: BLACK,
                divider_fg: NEUTRAL.s800,
                sidebar_bg: NEUTRAL.s950,
                aside_bg: NEUTRAL.s950,
            },
            toast: ToastTokens {
                success: ToastVariant {
                    bg: NEUTRAL.s800,
                    fg: NEUTRAL.s100,
                    border: NEUTRAL.s600,
                    icon_fg: WHITE,
                },
                error: ToastVariant {
                    bg: NEUTRAL.s800,
                    fg: NEUTRAL.s100,
                    border: NEUTRAL.s500,
                    icon_fg: NEUTRAL.s300,
                },
                warning: ToastVariant {
                    bg: NEUTRAL.s800,
                    fg: NEUTRAL.s100,
                    border: NEUTRAL.s600,
                    icon_fg: NEUTRAL.s400,
                },
                info: ToastVariant {
                    bg: NEUTRAL.s800,
                    fg: NEUTRAL.s100,
                    border: NEUTRAL.s700,
                    icon_fg: NEUTRAL.s400,
                },
            },
            slider: SliderTokens {
                track_bg: NEUTRAL.s800,
                fill_bg: WHITE,
                thumb: WHITE,
            },
        },
    }
}
