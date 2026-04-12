//! Digits Component
//!
//! LCD-style digital number display.

use crate::core::component::{BoxNode, BoxStyle, Color, NamedColor, TextStyle, VNode};
use crate::core::layout::FlexDirection;

/// Digit display style.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DigitStyle {
    #[default]
    SevenSegment,
    Block,
    Braille,
    Ascii,
}

/// Digits component for LCD-style number display.
#[derive(Debug, Clone)]
pub struct Digits {
    value: String,
    style: DigitStyle,
    color: Color,
    dim_color: Color,
    show_colon: bool,
    pad_zeros: Option<usize>,
}

impl Default for Digits {
    fn default() -> Self {
        Self {
            value: String::from("0"),
            style: DigitStyle::SevenSegment,
            color: Color::Named(NamedColor::Green),
            dim_color: Color::Named(NamedColor::Gray),
            show_colon: false,
            pad_zeros: None,
        }
    }
}

impl Digits {
    /// Create new digits display.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }

    /// Create from number.
    pub fn number(n: i64) -> Self {
        Self::new(n.to_string())
    }

    /// Create from float.
    pub fn float(n: f64, decimals: usize) -> Self {
        Self::new(format!("{:.prec$}", n, prec = decimals))
    }

    /// Create time display (HH:MM:SS).
    pub fn time(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self::new(format!("{:02}:{:02}:{:02}", hours, minutes, seconds)).colon(true)
    }

    /// Set display style.
    pub fn style(mut self, style: DigitStyle) -> Self {
        self.style = style;
        self
    }

    /// Set color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set dim color for inactive segments.
    pub fn dim_color(mut self, color: Color) -> Self {
        self.dim_color = color;
        self
    }

    /// Show/hide colon for time display.
    pub fn colon(mut self, show: bool) -> Self {
        self.show_colon = show;
        self
    }

    /// Pad with zeros to specified length.
    pub fn pad(mut self, length: usize) -> Self {
        self.pad_zeros = Some(length);
        self
    }

    /// Get seven-segment pattern for a digit.
    /// Returns 3 lines of characters.
    fn get_segment_pattern(c: char, style: DigitStyle) -> [&'static str; 3] {
        match style {
            DigitStyle::SevenSegment => Self::get_seven_segment(c),
            DigitStyle::Block => Self::get_block_digit(c),
            DigitStyle::Braille => Self::get_braille_digit(c),
            DigitStyle::Ascii => Self::get_ascii_digit(c),
        }
    }

    fn get_seven_segment(c: char) -> [&'static str; 3] {
        match c {
            '0' => ["┌─┐", "│ │", "└─┘"],
            '1' => ["  ╷", "  │", "  ╵"],
            '2' => ["╶─┐", "┌─┘", "└─╴"],
            '3' => ["╶─┐", "╶─┤", "╶─┘"],
            '4' => ["╷ ╷", "└─┤", "  ╵"],
            '5' => ["┌─╴", "└─┐", "╶─┘"],
            '6' => ["┌─╴", "├─┐", "└─┘"],
            '7' => ["╶─┐", "  │", "  ╵"],
            '8' => ["┌─┐", "├─┤", "└─┘"],
            '9' => ["┌─┐", "└─┤", "╶─┘"],
            ':' => [" ", "•", "•"],
            '.' => ["   ", "   ", " • "],
            '-' => ["   ", "───", "   "],
            ' ' => ["   ", "   ", "   "],
            _ => ["   ", "   ", "   "],
        }
    }

    fn get_block_digit(c: char) -> [&'static str; 3] {
        match c {
            '0' => ["███", "█ █", "███"],
            '1' => [" █ ", " █ ", " █ "],
            '2' => ["███", " ██", "██ "],
            '3' => ["███", " ██", "███"],
            '4' => ["█ █", "███", "  █"],
            '5' => ["███", "██ ", "███"],
            '6' => ["███", "██ ", "███"],
            '7' => ["███", "  █", "  █"],
            '8' => ["███", "███", "███"],
            '9' => ["███", "███", "  █"],
            ':' => [" ", "█", "█"],
            '.' => ["   ", "   ", " █ "],
            '-' => ["   ", "███", "   "],
            ' ' => ["   ", "   ", "   "],
            _ => ["   ", "   ", "   "],
        }
    }

    fn get_braille_digit(c: char) -> [&'static str; 3] {
        match c {
            '0' => ["⣀⣀⣀", "⡇ ⢸", "⠉⠉⠉"],
            '1' => [" ⢀ ", " ⡇ ", " ⠁ "],
            '2' => ["⠉⠉⣹", "⢀⡠⠃", "⠉⠉⠁"],
            '3' => ["⠉⠉⣹", "⠉⠉⣹", "⠉⠉⠁"],
            '4' => ["⢀ ⢀", "⠉⠉⣹", "   ⠁"],
            '5' => ["⣀⣀⡀", "⠉⠉⣹", "⠉⠉⠁"],
            '6' => ["⣀⣀⡀", "⣏⣉⣹", "⠉⠉⠁"],
            '7' => ["⠉⠉⣹", "   ⡇", "   ⠁"],
            '8' => ["⣀⣀⣀", "⣏⣉⣹", "⠉⠉⠁"],
            '9' => ["⣀⣀⣀", "⠉⠉⣹", "⠉⠉⠁"],
            ':' => [" ", "⡇", "⠁"],
            '.' => ["   ", "   ", " ⠁ "],
            '-' => ["   ", "⠉⠉⠁", "   "],
            ' ' => ["   ", "   ", "   "],
            _ => ["   ", "   ", "   "],
        }
    }

    fn get_ascii_digit(c: char) -> [&'static str; 3] {
        match c {
            '0' => [" _ ", "| |", "|_|"],
            '1' => ["   ", "  |", "  |"],
            '2' => [" _ ", " _|", "|_ "],
            '3' => [" _ ", " _|", " _|"],
            '4' => ["   ", "|_|", "  |"],
            '5' => [" _ ", "|_ ", " _|"],
            '6' => [" _ ", "|_ ", "|_|"],
            '7' => [" _ ", "  |", "  |"],
            '8' => [" _ ", "|_|", "|_|"],
            '9' => [" _ ", "|_|", " _|"],
            ':' => [" ", ".", "."],
            '.' => ["   ", "   ", " . "],
            '-' => ["   ", "---", "   "],
            ' ' => ["   ", "   ", "   "],
            _ => ["   ", "   ", "   "],
        }
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let mut display_value = self.value.clone();

        // Pad with zeros if needed
        if let Some(length) = self.pad_zeros {
            let num_len = display_value.chars().filter(|c| c.is_ascii_digit()).count();
            if num_len < length {
                let padding = "0".repeat(length - num_len);
                display_value = format!("{}{}", padding, display_value);
            }
        }

        let mut line0 = String::new();
        let mut line1 = String::new();
        let mut line2 = String::new();

        for (idx, c) in display_value.chars().enumerate() {
            let pattern = Self::get_segment_pattern(c, self.style);

            if idx > 0 && c != ':' && c != '.' {
                line0.push(' ');
                line1.push(' ');
                line2.push(' ');
            }

            line0.push_str(pattern[0]);
            line1.push_str(pattern[1]);
            line2.push_str(pattern[2]);
        }

        let style = TextStyle {
            color: Some(self.color.clone()),
            bold: true,
            ..Default::default()
        };

        VNode::Box(BoxNode {
            children: vec![
                VNode::styled_text(line0, style.clone()),
                VNode::styled_text(line1, style.clone()),
                VNode::styled_text(line2, style),
            ],
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                ..Default::default()
            },
            ..Default::default()
        })
    }
}
