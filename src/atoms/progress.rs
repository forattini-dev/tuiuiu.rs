//! Progress Bar Component

use crate::core::component::{VNode, TextNode, TextStyle, Color, NamedColor};

/// Progress bar component.
#[derive(Debug, Clone)]
pub struct ProgressBar {
    value: f32,
    max: f32,
    width: u16,
    show_percentage: bool,
    filled_char: char,
    empty_char: char,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            value: 0.0,
            max: 100.0,
            width: 20,
            show_percentage: true,
            filled_char: '█',
            empty_char: '░',
        }
    }
}

impl ProgressBar {
    pub fn new() -> Self { Self::default() }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    pub fn build(self) -> VNode {
        let percent = (self.value / self.max).clamp(0.0, 1.0);
        let filled = (percent * self.width as f32) as usize;
        let empty = self.width as usize - filled;

        let bar: String = std::iter::repeat(self.filled_char)
            .take(filled)
            .chain(std::iter::repeat(self.empty_char).take(empty))
            .collect();

        let content = if self.show_percentage {
            format!("{} {:3.0}%", bar, percent * 100.0)
        } else {
            bar
        };

        VNode::Text(TextNode {
            content,
            style: TextStyle {
                color: Some(Color::Named(NamedColor::Cyan)),
                ..Default::default()
            },
        })
    }
}

impl From<ProgressBar> for VNode {
    fn from(p: ProgressBar) -> VNode { p.build() }
}
