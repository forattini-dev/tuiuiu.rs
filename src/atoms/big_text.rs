//! Big Text Component
//!
//! Large ASCII art text display with multiple font styles.

use crate::core::component::{BoxNode, BoxStyle, Color, NamedColor, TextStyle, VNode};
use crate::core::layout::FlexDirection;

/// Big text font styles.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BigTextFont {
    #[default]
    Block,
    Slant,
    Small,
    Standard,
    Banner,
    Mini,
    Shadow,
    Doom,
}

/// Big text alignment.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BigTextAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// Big text component for large ASCII art text.
#[derive(Debug, Clone)]
pub struct BigText {
    text: String,
    font: BigTextFont,
    color: Option<Color>,
    gradient: Option<Vec<Color>>,
    align: BigTextAlign,
    max_width: Option<u16>,
    letter_spacing: u16,
}

impl Default for BigText {
    fn default() -> Self {
        Self {
            text: String::new(),
            font: BigTextFont::Block,
            color: None,
            gradient: None,
            align: BigTextAlign::Left,
            max_width: None,
            letter_spacing: 1,
        }
    }
}

impl BigText {
    /// Create a new big text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    /// Set the font style.
    pub fn font(mut self, font: BigTextFont) -> Self {
        self.font = font;
        self
    }

    /// Set text color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set gradient colors.
    pub fn gradient(mut self, colors: Vec<Color>) -> Self {
        self.gradient = Some(colors);
        self
    }

    /// Set alignment.
    pub fn align(mut self, align: BigTextAlign) -> Self {
        self.align = align;
        self
    }

    /// Center align.
    pub fn center(self) -> Self {
        self.align(BigTextAlign::Center)
    }

    /// Right align.
    pub fn right(self) -> Self {
        self.align(BigTextAlign::Right)
    }

    /// Set max width.
    pub fn max_width(mut self, width: u16) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Set letter spacing.
    pub fn letter_spacing(mut self, spacing: u16) -> Self {
        self.letter_spacing = spacing;
        self
    }

    /// Get character pattern for block font.
    fn get_block_char(c: char) -> [&'static str; 5] {
        match c.to_ascii_uppercase() {
            'A' => ["█████", "█   █", "█████", "█   █", "█   █"],
            'B' => ["████ ", "█   █", "████ ", "█   █", "████ "],
            'C' => ["█████", "█    ", "█    ", "█    ", "█████"],
            'D' => ["████ ", "█   █", "█   █", "█   █", "████ "],
            'E' => ["█████", "█    ", "████ ", "█    ", "█████"],
            'F' => ["█████", "█    ", "████ ", "█    ", "█    "],
            'G' => ["█████", "█    ", "█ ███", "█   █", "█████"],
            'H' => ["█   █", "█   █", "█████", "█   █", "█   █"],
            'I' => ["█████", "  █  ", "  █  ", "  █  ", "█████"],
            'J' => ["█████", "    █", "    █", "█   █", "█████"],
            'K' => ["█   █", "█  █ ", "███  ", "█  █ ", "█   █"],
            'L' => ["█    ", "█    ", "█    ", "█    ", "█████"],
            'M' => ["█   █", "██ ██", "█ █ █", "█   █", "█   █"],
            'N' => ["█   █", "██  █", "█ █ █", "█  ██", "█   █"],
            'O' => ["█████", "█   █", "█   █", "█   █", "█████"],
            'P' => ["█████", "█   █", "█████", "█    ", "█    "],
            'Q' => ["█████", "█   █", "█   █", "█  █ ", "███ █"],
            'R' => ["█████", "█   █", "█████", "█  █ ", "█   █"],
            'S' => ["█████", "█    ", "█████", "    █", "█████"],
            'T' => ["█████", "  █  ", "  █  ", "  █  ", "  █  "],
            'U' => ["█   █", "█   █", "█   █", "█   █", "█████"],
            'V' => ["█   █", "█   █", "█   █", " █ █ ", "  █  "],
            'W' => ["█   █", "█   █", "█ █ █", "██ ██", "█   █"],
            'X' => ["█   █", " █ █ ", "  █  ", " █ █ ", "█   █"],
            'Y' => ["█   █", " █ █ ", "  █  ", "  █  ", "  █  "],
            'Z' => ["█████", "   █ ", "  █  ", " █   ", "█████"],
            '0' => ["█████", "█  ██", "█ █ █", "██  █", "█████"],
            '1' => [" ██  ", "  █  ", "  █  ", "  █  ", "█████"],
            '2' => ["█████", "    █", "█████", "█    ", "█████"],
            '3' => ["█████", "    █", "█████", "    █", "█████"],
            '4' => ["█   █", "█   █", "█████", "    █", "    █"],
            '5' => ["█████", "█    ", "█████", "    █", "█████"],
            '6' => ["█████", "█    ", "█████", "█   █", "█████"],
            '7' => ["█████", "    █", "   █ ", "  █  ", "  █  "],
            '8' => ["█████", "█   █", "█████", "█   █", "█████"],
            '9' => ["█████", "█   █", "█████", "    █", "█████"],
            ' ' => ["     ", "     ", "     ", "     ", "     "],
            '.' => ["     ", "     ", "     ", "     ", "  █  "],
            '!' => ["  █  ", "  █  ", "  █  ", "     ", "  █  "],
            '?' => ["█████", "    █", "  ██ ", "     ", "  █  "],
            '-' => ["     ", "     ", "█████", "     ", "     "],
            '+' => ["     ", "  █  ", "█████", "  █  ", "     "],
            ':' => ["     ", "  █  ", "     ", "  █  ", "     "],
            _ => ["     ", "     ", "     ", "     ", "     "],
        }
    }

    /// Get character pattern for mini font.
    fn get_mini_char(c: char) -> [&'static str; 3] {
        match c.to_ascii_uppercase() {
            'A' => ["▄█▄", "█▀█", "▀ ▀"],
            'B' => ["██▄", "█▄█", "██▀"],
            'C' => ["▄█▄", "█  ", "▀█▀"],
            'D' => ["██▄", "█ █", "██▀"],
            'E' => ["███", "█▄ ", "███"],
            'F' => ["███", "█▄ ", "█  "],
            'G' => ["▄██", "█ █", "▀██"],
            'H' => ["█ █", "███", "█ █"],
            'I' => ["███", " █ ", "███"],
            'J' => ["███", "  █", "▀█▀"],
            'K' => ["█▄█", "██ ", "█ █"],
            'L' => ["█  ", "█  ", "███"],
            'M' => ["█▄█", "█▀█", "█ █"],
            'N' => ["█▄█", "█▀█", "█ █"],
            'O' => ["▄█▄", "█ █", "▀█▀"],
            'P' => ["██▄", "█▀ ", "█  "],
            'Q' => ["▄█▄", "█ █", "▀██"],
            'R' => ["██▄", "█▀ ", "█ █"],
            'S' => ["▄██", " █ ", "██▀"],
            'T' => ["███", " █ ", " █ "],
            'U' => ["█ █", "█ █", "▀█▀"],
            'V' => ["█ █", "█ █", " █ "],
            'W' => ["█ █", "█▄█", "█▀█"],
            'X' => ["█ █", " █ ", "█ █"],
            'Y' => ["█ █", " █ ", " █ "],
            'Z' => ["██▄", " █ ", "▀██"],
            '0' => ["▄█▄", "█ █", "▀█▀"],
            '1' => ["▄█ ", " █ ", "▄█▄"],
            '2' => ["▀█▄", " ▄▀", "▄█▄"],
            '3' => ["▀█▄", " █ ", "▄█▀"],
            '4' => ["█ █", "▀█▀", "  █"],
            '5' => ["▄██", "▀█▄", "▄█▀"],
            '6' => ["▄█▄", "█▄ ", "▀█▀"],
            '7' => ["███", "  █", "  █"],
            '8' => ["▄█▄", "▀█▀", "▀█▀"],
            '9' => ["▄█▄", "▀█▀", "▄█▀"],
            ' ' => ["   ", "   ", "   "],
            _ => ["   ", "   ", "   "],
        }
    }

    /// Render text to lines.
    fn render_lines(&self) -> Vec<String> {
        let height = match self.font {
            BigTextFont::Block
            | BigTextFont::Standard
            | BigTextFont::Banner
            | BigTextFont::Shadow
            | BigTextFont::Doom => 5,
            BigTextFont::Mini | BigTextFont::Slant | BigTextFont::Small => 3,
        };

        let mut lines: Vec<String> = vec![String::new(); height];
        let spacing = " ".repeat(self.letter_spacing as usize);

        for (idx, c) in self.text.chars().enumerate() {
            let patterns: Vec<&str> = match self.font {
                BigTextFont::Block
                | BigTextFont::Standard
                | BigTextFont::Banner
                | BigTextFont::Shadow
                | BigTextFont::Doom => Self::get_block_char(c).to_vec(),
                BigTextFont::Mini | BigTextFont::Slant | BigTextFont::Small => {
                    Self::get_mini_char(c).to_vec()
                }
            };

            for (line_idx, pattern) in patterns.iter().enumerate() {
                if idx > 0 {
                    lines[line_idx].push_str(&spacing);
                }
                lines[line_idx].push_str(pattern);
            }
        }

        lines
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let lines = self.render_lines();
        let base_color = self
            .color
            .clone()
            .unwrap_or(Color::Named(NamedColor::White));

        let mut children: Vec<VNode> = Vec::new();

        for (idx, line) in lines.iter().enumerate() {
            let color = if let Some(gradient) = &self.gradient {
                if !gradient.is_empty() {
                    let ratio = idx as f32 / (lines.len() - 1).max(1) as f32;
                    let color_idx = ((gradient.len() - 1) as f32 * ratio) as usize;
                    gradient
                        .get(color_idx)
                        .cloned()
                        .unwrap_or(base_color.clone())
                } else {
                    base_color.clone()
                }
            } else {
                base_color.clone()
            };

            let style = TextStyle {
                color: Some(color),
                bold: true,
                ..Default::default()
            };

            children.push(VNode::styled_text(line.clone(), style));
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                ..Default::default()
            },
            ..Default::default()
        })
    }
}
