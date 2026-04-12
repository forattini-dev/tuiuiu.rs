//! Timer Atom
//!
//! Countdown and stopwatch timer component.
//!
//! ```text
//! Countdown: 05:30    Stopwatch: 01:23:45
//! ```

use crate::core::component::{BoxNode, BoxStyle, Color, NamedColor, TextNode, TextStyle, VNode};
use crate::core::layout::FlexDirection;
use crate::core::signals::create_signal;

// =============================================================================
// Timer Component
// =============================================================================

/// Timer mode - countdown or stopwatch
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TimerMode {
    /// Counts down from a specified duration
    Countdown,
    /// Counts up from zero
    #[default]
    Stopwatch,
}

/// Timer display format
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TimerFormat {
    /// MM:SS (e.g., 05:30)
    #[default]
    MinutesSeconds,
    /// HH:MM:SS (e.g., 01:23:45)
    HoursMinutesSeconds,
    /// SS.mm (e.g., 45.23)
    SecondsMillis,
    /// Human readable (e.g., "5 min 30 sec")
    Human,
}

/// Timer state for reactive updates
#[derive(Debug, Clone)]
pub struct TimerState {
    /// Current elapsed time in milliseconds
    pub elapsed_ms: u64,
    /// Whether the timer is running
    pub running: bool,
    /// Target duration for countdown (in milliseconds)
    pub target_ms: Option<u64>,
}

impl Default for TimerState {
    fn default() -> Self {
        Self {
            elapsed_ms: 0,
            running: false,
            target_ms: None,
        }
    }
}

impl TimerState {
    /// Create state for a countdown timer
    pub fn countdown(duration_ms: u64) -> Self {
        Self {
            elapsed_ms: 0,
            running: false,
            target_ms: Some(duration_ms),
        }
    }

    /// Create state for a stopwatch
    pub fn stopwatch() -> Self {
        Self::default()
    }

    /// Get remaining time for countdown (returns 0 if stopwatch)
    pub fn remaining_ms(&self) -> u64 {
        match self.target_ms {
            Some(target) => target.saturating_sub(self.elapsed_ms),
            None => 0,
        }
    }

    /// Check if countdown has finished
    pub fn is_finished(&self) -> bool {
        match self.target_ms {
            Some(target) => self.elapsed_ms >= target,
            None => false,
        }
    }

    /// Get display time based on mode
    pub fn display_ms(&self, mode: TimerMode) -> u64 {
        match mode {
            TimerMode::Countdown => self.remaining_ms(),
            TimerMode::Stopwatch => self.elapsed_ms,
        }
    }
}

/// Timer component for countdown and stopwatch functionality.
///
/// # Example
/// ```ignore
/// let timer = Timer::new()
///     .mode(TimerMode::Countdown)
///     .duration_seconds(300) // 5 minutes
///     .format(TimerFormat::MinutesSeconds)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Timer {
    /// Timer mode
    mode: TimerMode,
    /// Display format
    format: TimerFormat,
    /// Duration for countdown (milliseconds)
    duration_ms: u64,
    /// Current state (for static display)
    state: Option<TimerState>,
    /// Show hours even if zero
    show_hours: bool,
    /// Show milliseconds
    show_millis: bool,
    /// Separator character
    separator: char,
    /// Text color
    color: Color,
    /// Color when finished/warning
    warning_color: Color,
    /// Warning threshold (remaining ms)
    warning_threshold_ms: u64,
    /// Bold text
    bold: bool,
    /// Show label
    label: Option<String>,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            mode: TimerMode::Stopwatch,
            format: TimerFormat::MinutesSeconds,
            duration_ms: 0,
            state: None,
            show_hours: false,
            show_millis: false,
            separator: ':',
            color: Color::Named(NamedColor::White),
            warning_color: Color::Named(NamedColor::Red),
            warning_threshold_ms: 30_000, // 30 seconds
            bold: true,
            label: None,
        }
    }
}

impl Timer {
    /// Create a new timer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a countdown timer with duration in seconds.
    pub fn countdown(seconds: u64) -> Self {
        Self {
            mode: TimerMode::Countdown,
            duration_ms: seconds * 1000,
            ..Self::default()
        }
    }

    /// Create a stopwatch timer.
    pub fn stopwatch() -> Self {
        Self {
            mode: TimerMode::Stopwatch,
            ..Self::default()
        }
    }

    /// Set the timer mode.
    pub fn mode(mut self, mode: TimerMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set the display format.
    pub fn format(mut self, format: TimerFormat) -> Self {
        self.format = format;
        self
    }

    /// Set duration in seconds (for countdown).
    pub fn duration_seconds(mut self, seconds: u64) -> Self {
        self.duration_ms = seconds * 1000;
        self
    }

    /// Set duration in milliseconds (for countdown).
    pub fn duration_ms(mut self, ms: u64) -> Self {
        self.duration_ms = ms;
        self
    }

    /// Set current timer state.
    pub fn state(mut self, state: TimerState) -> Self {
        self.state = Some(state);
        self
    }

    /// Set elapsed time in seconds (for display).
    pub fn elapsed_seconds(mut self, seconds: u64) -> Self {
        let elapsed_ms = seconds * 1000;
        if let Some(ref mut state) = self.state {
            state.elapsed_ms = elapsed_ms;
        } else {
            self.state = Some(TimerState {
                elapsed_ms,
                running: false,
                target_ms: if self.mode == TimerMode::Countdown {
                    Some(self.duration_ms)
                } else {
                    None
                },
            });
        }
        self
    }

    /// Show hours even when zero.
    pub fn show_hours(mut self, show: bool) -> Self {
        self.show_hours = show;
        self
    }

    /// Show milliseconds.
    pub fn show_millis(mut self, show: bool) -> Self {
        self.show_millis = show;
        self
    }

    /// Set separator character.
    pub fn separator(mut self, sep: char) -> Self {
        self.separator = sep;
        self
    }

    /// Set text color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set warning color (shown when approaching deadline).
    pub fn warning_color(mut self, color: Color) -> Self {
        self.warning_color = color;
        self
    }

    /// Set warning threshold in seconds.
    pub fn warning_threshold(mut self, seconds: u64) -> Self {
        self.warning_threshold_ms = seconds * 1000;
        self
    }

    /// Set bold text.
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Set label text.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Format time in milliseconds to display string.
    fn format_time(&self, ms: u64) -> String {
        let total_seconds = ms / 1000;
        let millis = (ms % 1000) / 10; // Two digits
        let seconds = total_seconds % 60;
        let minutes = (total_seconds / 60) % 60;
        let hours = total_seconds / 3600;

        let sep = self.separator;

        match self.format {
            TimerFormat::MinutesSeconds => {
                if self.show_hours || hours > 0 {
                    if self.show_millis {
                        format!(
                            "{:02}{}{:02}{}{:02}.{:02}",
                            hours, sep, minutes, sep, seconds, millis
                        )
                    } else {
                        format!("{:02}{}{:02}{}{:02}", hours, sep, minutes, sep, seconds)
                    }
                } else if self.show_millis {
                    format!("{:02}{}{:02}.{:02}", minutes, sep, seconds, millis)
                } else {
                    format!("{:02}{}{:02}", minutes, sep, seconds)
                }
            }
            TimerFormat::HoursMinutesSeconds => {
                if self.show_millis {
                    format!(
                        "{:02}{}{:02}{}{:02}.{:02}",
                        hours, sep, minutes, sep, seconds, millis
                    )
                } else {
                    format!("{:02}{}{:02}{}{:02}", hours, sep, minutes, sep, seconds)
                }
            }
            TimerFormat::SecondsMillis => {
                format!("{:02}.{:02}", total_seconds, millis)
            }
            TimerFormat::Human => {
                if hours > 0 {
                    format!("{} hr {} min {} sec", hours, minutes, seconds)
                } else if minutes > 0 {
                    format!("{} min {} sec", minutes, seconds)
                } else {
                    format!("{} sec", seconds)
                }
            }
        }
    }

    /// Get display time based on current state.
    fn get_display_ms(&self) -> u64 {
        match &self.state {
            Some(state) => state.display_ms(self.mode),
            None => match self.mode {
                TimerMode::Countdown => self.duration_ms,
                TimerMode::Stopwatch => 0,
            },
        }
    }

    /// Check if timer is in warning state.
    fn is_warning(&self) -> bool {
        if self.mode != TimerMode::Countdown {
            return false;
        }
        let remaining = self.get_display_ms();
        remaining <= self.warning_threshold_ms && remaining > 0
    }

    /// Check if timer is finished.
    fn is_finished(&self) -> bool {
        if self.mode != TimerMode::Countdown {
            return false;
        }
        self.get_display_ms() == 0
    }

    /// Build the timer VNode.
    pub fn build(self) -> VNode {
        let display_ms = self.get_display_ms();
        let time_str = self.format_time(display_ms);

        let text_color = if self.is_finished() || self.is_warning() {
            self.warning_color
        } else {
            self.color
        };

        let time_node = VNode::Text(TextNode {
            content: time_str,
            style: TextStyle {
                color: Some(text_color),
                bold: self.bold,
                ..Default::default()
            },
        });

        match self.label {
            Some(label) => VNode::Box(BoxNode {
                id: None,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Row),
                    gap: Some(1),
                    ..Default::default()
                },
                children: vec![
                    VNode::Text(TextNode {
                        content: format!("{}: ", label),
                        style: TextStyle {
                            color: Some(Color::Named(NamedColor::Gray)),
                            ..Default::default()
                        },
                    }),
                    time_node,
                ],
                handlers: Default::default(),
            }),
            None => time_node,
        }
    }
}

impl From<Timer> for VNode {
    fn from(timer: Timer) -> VNode {
        timer.build()
    }
}

/// Create a reactive timer state with signals.
pub fn create_timer_state(
    _mode: TimerMode,
    duration_ms: Option<u64>,
) -> (
    crate::core::signals::ReadSignal<TimerState>,
    crate::core::signals::WriteSignal<TimerState>,
) {
    let initial = TimerState {
        elapsed_ms: 0,
        running: false,
        target_ms: duration_ms,
    };
    create_signal(initial)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_default() {
        let timer = Timer::new().build();
        if let VNode::Text(node) = timer {
            assert_eq!(node.content, "00:00");
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_timer_countdown() {
        let timer = Timer::countdown(300).build(); // 5 minutes
        if let VNode::Text(node) = timer {
            assert_eq!(node.content, "05:00");
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_timer_with_elapsed() {
        let timer = Timer::stopwatch()
            .elapsed_seconds(125) // 2:05
            .build();
        if let VNode::Text(node) = timer {
            assert_eq!(node.content, "02:05");
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_timer_hours_format() {
        let timer = Timer::stopwatch()
            .elapsed_seconds(3665) // 1:01:05
            .format(TimerFormat::HoursMinutesSeconds)
            .build();
        if let VNode::Text(node) = timer {
            assert_eq!(node.content, "01:01:05");
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_timer_human_format() {
        let timer = Timer::countdown(125).format(TimerFormat::Human).build();
        if let VNode::Text(node) = timer {
            assert_eq!(node.content, "2 min 5 sec");
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_timer_with_label() {
        let timer = Timer::countdown(60).label("Time left").build();
        matches!(timer, VNode::Box(_));
    }

    #[test]
    fn test_timer_state() {
        let state = TimerState::countdown(60_000); // 1 minute
        assert_eq!(state.remaining_ms(), 60_000);
        assert!(!state.is_finished());

        let mut finished = state.clone();
        finished.elapsed_ms = 60_000;
        assert!(finished.is_finished());
        assert_eq!(finished.remaining_ms(), 0);
    }

    #[test]
    fn test_timer_separator() {
        let timer = Timer::countdown(90).separator('.').build();
        if let VNode::Text(node) = timer {
            assert_eq!(node.content, "01.30");
        } else {
            panic!("Expected Text node");
        }
    }
}
