//! ANSI Escape Code Utilities

/// Strip ANSI escape codes from a string.
pub fn strip_ansi(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1B' {
            // Skip escape sequence
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c.is_ascii_alphabetic() || c == 'm' {
                        break;
                    }
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// Color for styling.
pub use crate::core::component::Color;

/// Style builder.
#[derive(Debug, Clone, Default)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub inverse: bool,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub fn inverse(mut self) -> Self {
        self.inverse = true;
        self
    }

    /// Apply style to a string.
    pub fn apply(&self, s: &str) -> String {
        let mut codes: Vec<String> = Vec::new();

        if self.bold { codes.push("1".to_string()); }
        if self.dim { codes.push("2".to_string()); }
        if self.italic { codes.push("3".to_string()); }
        if self.underline { codes.push("4".to_string()); }
        if self.inverse { codes.push("7".to_string()); }

        if let Some(fg) = &self.fg {
            codes.push(color_to_fg_code(fg));
        }
        if let Some(bg) = &self.bg {
            codes.push(color_to_bg_code(bg));
        }

        if codes.is_empty() {
            s.to_string()
        } else {
            format!("\x1B[{}m{}\x1B[0m", codes.join(";"), s)
        }
    }
}

fn color_to_fg_code(color: &Color) -> String {
    match color {
        Color::Default => String::new(),
        Color::Named(c) => format!("{}", named_to_fg(*c)),
        Color::Ansi256(n) => format!("38;5;{}", n),
        Color::Rgb(r, g, b) => format!("38;2;{};{};{}", r, g, b),
    }
}

fn color_to_bg_code(color: &Color) -> String {
    match color {
        Color::Default => String::new(),
        Color::Named(c) => format!("{}", named_to_fg(*c) + 10),
        Color::Ansi256(n) => format!("48;5;{}", n),
        Color::Rgb(r, g, b) => format!("48;2;{};{};{}", r, g, b),
    }
}

fn named_to_fg(c: crate::core::component::NamedColor) -> u8 {
    use crate::core::component::NamedColor::*;
    match c {
        Black => 30,
        Red => 31,
        Green => 32,
        Yellow => 33,
        Blue => 34,
        Magenta => 35,
        Cyan => 36,
        White => 37,
        BrightBlack | Gray => 90,
        BrightRed => 91,
        BrightGreen => 92,
        BrightYellow => 93,
        BrightBlue => 94,
        BrightMagenta => 95,
        BrightCyan => 96,
        BrightWhite => 97,
    }
}

/// Colorize a string with foreground color.
pub fn colorize(s: &str, color: Color) -> String {
    Style::new().fg(color).apply(s)
}

/// Create a style builder.
pub fn style() -> Style {
    Style::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi() {
        let s = "\x1B[31mRed\x1B[0m text";
        assert_eq!(strip_ansi(s), "Red text");
    }

    #[test]
    fn test_style_apply() {
        let styled = Style::new().bold().apply("Hello");
        assert!(styled.contains("\x1B[1m"));
    }
}
