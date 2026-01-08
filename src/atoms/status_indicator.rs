//! Status Indicator Component
//!
//! Visual indicator for status states (online, offline, busy, away, etc).

use crate::core::component::{VNode, TextStyle, Color, NamedColor};

/// Status indicator states.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StatusState {
    #[default]
    Offline,
    Online,
    Busy,
    Away,
    DoNotDisturb,
    Invisible,
    Custom,
}

/// Status indicator size.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StatusSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Status indicator component.
#[derive(Debug, Clone)]
pub struct StatusIndicator {
    state: StatusState,
    size: StatusSize,
    label: Option<String>,
    show_label: bool,
    custom_color: Option<Color>,
    custom_icon: Option<char>,
    pulse: bool,
}

impl Default for StatusIndicator {
    fn default() -> Self {
        Self {
            state: StatusState::Offline,
            size: StatusSize::Medium,
            label: None,
            show_label: false,
            custom_color: None,
            custom_icon: None,
            pulse: false,
        }
    }
}

impl StatusIndicator {
    /// Create a new status indicator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create online status.
    pub fn online() -> Self {
        Self::new().state(StatusState::Online)
    }

    /// Create offline status.
    pub fn offline() -> Self {
        Self::new().state(StatusState::Offline)
    }

    /// Create busy status.
    pub fn busy() -> Self {
        Self::new().state(StatusState::Busy)
    }

    /// Create away status.
    pub fn away() -> Self {
        Self::new().state(StatusState::Away)
    }

    /// Create do not disturb status.
    pub fn dnd() -> Self {
        Self::new().state(StatusState::DoNotDisturb)
    }

    /// Set state.
    pub fn state(mut self, state: StatusState) -> Self {
        self.state = state;
        self
    }

    /// Set size.
    pub fn size(mut self, size: StatusSize) -> Self {
        self.size = size;
        self
    }

    /// Set small size.
    pub fn small(self) -> Self {
        self.size(StatusSize::Small)
    }

    /// Set large size.
    pub fn large(self) -> Self {
        self.size(StatusSize::Large)
    }

    /// Set label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self.show_label = true;
        self
    }

    /// Show/hide label.
    pub fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    /// Set custom color for Custom state.
    pub fn custom_color(mut self, color: Color) -> Self {
        self.custom_color = Some(color);
        self
    }

    /// Set custom icon.
    pub fn custom_icon(mut self, icon: char) -> Self {
        self.custom_icon = Some(icon);
        self
    }

    /// Enable pulse animation (visual only).
    pub fn pulse(mut self, pulse: bool) -> Self {
        self.pulse = pulse;
        self
    }

    /// Get the icon for the state.
    fn get_icon(&self) -> &str {
        if let Some(icon) = self.custom_icon {
            // Return static str for common custom icons
            return match icon {
                '●' => "●",
                '○' => "○",
                '◉' => "◉",
                '◎' => "◎",
                '⬤' => "⬤",
                '🟢' => "🟢",
                '🔴' => "🔴",
                '🟡' => "🟡",
                _ => "●",
            };
        }

        match self.size {
            StatusSize::Small => match self.state {
                StatusState::Online => "•",
                StatusState::Offline => "○",
                StatusState::Busy => "•",
                StatusState::Away => "◌",
                StatusState::DoNotDisturb => "⊘",
                StatusState::Invisible => "◌",
                StatusState::Custom => "•",
            },
            StatusSize::Medium => match self.state {
                StatusState::Online => "●",
                StatusState::Offline => "○",
                StatusState::Busy => "●",
                StatusState::Away => "◐",
                StatusState::DoNotDisturb => "⊘",
                StatusState::Invisible => "◌",
                StatusState::Custom => "●",
            },
            StatusSize::Large => match self.state {
                StatusState::Online => "⬤",
                StatusState::Offline => "○",
                StatusState::Busy => "⬤",
                StatusState::Away => "◐",
                StatusState::DoNotDisturb => "⊘",
                StatusState::Invisible => "◌",
                StatusState::Custom => "⬤",
            },
        }
    }

    /// Get color for the state.
    fn get_color(&self) -> Color {
        if let Some(color) = &self.custom_color {
            return color.clone();
        }

        match self.state {
            StatusState::Online => Color::Named(NamedColor::Green),
            StatusState::Offline => Color::Named(NamedColor::Gray),
            StatusState::Busy => Color::Named(NamedColor::Red),
            StatusState::Away => Color::Named(NamedColor::Yellow),
            StatusState::DoNotDisturb => Color::Named(NamedColor::Red),
            StatusState::Invisible => Color::Named(NamedColor::Gray),
            StatusState::Custom => Color::Named(NamedColor::Cyan),
        }
    }

    /// Get default label for state.
    fn get_default_label(&self) -> &str {
        match self.state {
            StatusState::Online => "Online",
            StatusState::Offline => "Offline",
            StatusState::Busy => "Busy",
            StatusState::Away => "Away",
            StatusState::DoNotDisturb => "Do Not Disturb",
            StatusState::Invisible => "Invisible",
            StatusState::Custom => "Status",
        }
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let icon = self.get_icon();
        let color = self.get_color();

        let icon_style = TextStyle {
            color: Some(color.clone()),
            bold: self.pulse,
            ..Default::default()
        };

        if self.show_label {
            let label_text = self.label.as_deref().unwrap_or_else(|| self.get_default_label());
            let text = format!("{} {}", icon, label_text);
            VNode::styled_text(text, icon_style)
        } else {
            VNode::styled_text(icon.to_string(), icon_style)
        }
    }
}

/// Create a status indicator from a boolean.
impl From<bool> for StatusIndicator {
    fn from(online: bool) -> Self {
        if online {
            StatusIndicator::online()
        } else {
            StatusIndicator::offline()
        }
    }
}
