//! Divider Component
//!
//! Horizontal or vertical line separator.

use crate::core::component::{VNode, TextNode, TextStyle, Color, NamedColor};

/// Divider orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Divider component.
#[derive(Debug, Clone, Default)]
pub struct Divider {
    orientation: DividerOrientation,
    char: char,
    length: Option<u16>,
    color: Option<Color>,
    title: Option<String>,
}

impl Divider {
    /// Create a new horizontal divider.
    pub fn new() -> Self {
        Self {
            char: '─',
            ..Default::default()
        }
    }

    /// Create a vertical divider.
    pub fn vertical() -> Self {
        Self {
            orientation: DividerOrientation::Vertical,
            char: '│',
            ..Default::default()
        }
    }

    /// Set the divider character.
    pub fn char(mut self, c: char) -> Self {
        self.char = c;
        self
    }

    /// Set a fixed length.
    pub fn length(mut self, len: u16) -> Self {
        self.length = Some(len);
        self
    }

    /// Set the color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set color by name.
    pub fn fg(mut self, color: NamedColor) -> Self {
        self.color = Some(Color::Named(color));
        self
    }

    /// Add a title (for horizontal dividers).
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Use dashed style.
    pub fn dashed(mut self) -> Self {
        self.char = if self.orientation == DividerOrientation::Horizontal { '╌' } else { '╎' };
        self
    }

    /// Use double style.
    pub fn double(mut self) -> Self {
        self.char = if self.orientation == DividerOrientation::Horizontal { '═' } else { '║' };
        self
    }

    /// Use bold style.
    pub fn bold(mut self) -> Self {
        self.char = if self.orientation == DividerOrientation::Horizontal { '━' } else { '┃' };
        self
    }

    /// Build into a VNode.
    pub fn build(self) -> VNode {
        let len = self.length.unwrap_or(20);
        let content = if let Some(title) = &self.title {
            let side_len = (len as usize).saturating_sub(title.len() + 2) / 2;
            let left: String = std::iter::repeat(self.char).take(side_len).collect();
            let right: String = std::iter::repeat(self.char).take(side_len).collect();
            format!("{} {} {}", left, title, right)
        } else {
            std::iter::repeat(self.char).take(len as usize).collect()
        };

        VNode::Text(TextNode {
            content,
            style: TextStyle {
                color: self.color,
                ..Default::default()
            },
        })
    }
}

impl From<Divider> for VNode {
    fn from(d: Divider) -> VNode {
        d.build()
    }
}

/// Create a horizontal divider.
pub fn divider() -> Divider {
    Divider::new()
}

/// Create a vertical divider.
pub fn vdivider() -> Divider {
    Divider::vertical()
}
