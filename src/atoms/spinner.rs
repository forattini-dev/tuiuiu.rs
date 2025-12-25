//! Spinner Component

use crate::core::component::{VNode, TextNode};
use crate::core::tick::get_frame;

const SPINNER_FRAMES: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

/// Spinner component.
#[derive(Debug, Clone)]
pub struct Spinner {
    frames: Vec<char>,
    label: Option<String>,
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            frames: SPINNER_FRAMES.to_vec(),
            label: None,
        }
    }
}

impl Spinner {
    pub fn new() -> Self { Self::default() }

    pub fn frames(mut self, frames: Vec<char>) -> Self {
        self.frames = frames;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn dots(self) -> Self { self.frames(vec!['⣾', '⣽', '⣻', '⢿', '⡿', '⣟', '⣯', '⣷']) }
    pub fn line(self) -> Self { self.frames(vec!['-', '\\', '|', '/']) }
    pub fn circle(self) -> Self { self.frames(vec!['◐', '◓', '◑', '◒']) }

    pub fn build(self) -> VNode {
        let frame_idx = get_frame(self.frames.len());
        let spinner_char = self.frames[frame_idx];
        
        let content = if let Some(label) = self.label {
            format!("{} {}", spinner_char, label)
        } else {
            spinner_char.to_string()
        };

        VNode::Text(TextNode {
            content,
            style: Default::default(),
        })
    }
}

impl From<Spinner> for VNode {
    fn from(s: Spinner) -> VNode { s.build() }
}
