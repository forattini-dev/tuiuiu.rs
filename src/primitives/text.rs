//! Text Component
//!
//! Renders text content with styling.

use crate::core::component::{VNode, TextNode, TextStyle, Color, NamedColor, WrapMode};

/// Text component builder.
#[derive(Debug, Clone, Default)]
pub struct Text {
    content: String,
    style: TextStyle,
}

impl Text {
    /// Create a new text component.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: TextStyle::default(),
        }
    }

    /// Create an empty text.
    pub fn empty() -> Self {
        Self::new("")
    }

    /// Set the text content.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    // === Colors ===

    /// Set text color.
    pub fn color(mut self, color: Color) -> Self {
        self.style.color = Some(color);
        self
    }

    /// Set text color by name.
    pub fn fg(mut self, color: NamedColor) -> Self {
        self.style.color = Some(Color::Named(color));
        self
    }

    /// Set text color as RGB.
    pub fn rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.style.color = Some(Color::Rgb(r, g, b));
        self
    }

    /// Set background color.
    pub fn background(mut self, color: Color) -> Self {
        self.style.background = Some(color);
        self
    }

    /// Set background color by name.
    pub fn bg(mut self, color: NamedColor) -> Self {
        self.style.background = Some(Color::Named(color));
        self
    }

    // === Named Color Shortcuts ===

    /// Red text.
    pub fn red(self) -> Self {
        self.fg(NamedColor::Red)
    }

    /// Green text.
    pub fn green(self) -> Self {
        self.fg(NamedColor::Green)
    }

    /// Blue text.
    pub fn blue(self) -> Self {
        self.fg(NamedColor::Blue)
    }

    /// Yellow text.
    pub fn yellow(self) -> Self {
        self.fg(NamedColor::Yellow)
    }

    /// Cyan text.
    pub fn cyan(self) -> Self {
        self.fg(NamedColor::Cyan)
    }

    /// Magenta text.
    pub fn magenta(self) -> Self {
        self.fg(NamedColor::Magenta)
    }

    /// White text.
    pub fn white(self) -> Self {
        self.fg(NamedColor::White)
    }

    /// Black text.
    pub fn black(self) -> Self {
        self.fg(NamedColor::Black)
    }

    /// Gray text.
    pub fn gray(self) -> Self {
        self.fg(NamedColor::Gray)
    }

    // === Text Styles ===

    /// Make text bold.
    pub fn bold(mut self) -> Self {
        self.style.bold = true;
        self
    }

    /// Make text italic.
    pub fn italic(mut self) -> Self {
        self.style.italic = true;
        self
    }

    /// Make text underlined.
    pub fn underline(mut self) -> Self {
        self.style.underline = true;
        self
    }

    /// Make text strikethrough.
    pub fn strikethrough(mut self) -> Self {
        self.style.strikethrough = true;
        self
    }

    /// Make text dim.
    pub fn dim(mut self) -> Self {
        self.style.dim = true;
        self
    }

    /// Inverse colors.
    pub fn inverse(mut self) -> Self {
        self.style.inverse = true;
        self
    }

    // === Wrap Mode ===

    /// Set wrap mode.
    pub fn wrap(mut self, mode: WrapMode) -> Self {
        self.style.wrap = Some(mode);
        self
    }

    /// Wrap at word boundaries.
    pub fn wrap_word(self) -> Self {
        self.wrap(WrapMode::Word)
    }

    /// Wrap at character boundaries.
    pub fn wrap_char(self) -> Self {
        self.wrap(WrapMode::Char)
    }

    /// Truncate with ellipsis.
    pub fn truncate(self) -> Self {
        self.wrap(WrapMode::Truncate)
    }

    /// Build into a VNode.
    pub fn build(self) -> VNode {
        VNode::Text(TextNode {
            content: self.content,
            style: self.style,
        })
    }
}

impl From<Text> for VNode {
    fn from(t: Text) -> VNode {
        t.build()
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Text::new(s)
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Text::new(s)
    }
}

/// Create text from a string.
pub fn text(content: impl Into<String>) -> Text {
    Text::new(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_builder() {
        let t = Text::new("Hello")
            .bold()
            .red()
            .underline();

        assert_eq!(t.content, "Hello");
        assert!(t.style.bold);
        assert!(t.style.underline);
        assert!(matches!(t.style.color, Some(Color::Named(NamedColor::Red))));
    }

    #[test]
    fn test_text_from_str() {
        let t: Text = "Hello".into();
        assert_eq!(t.content, "Hello");
    }
}
