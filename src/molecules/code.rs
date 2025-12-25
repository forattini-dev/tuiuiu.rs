//! Code Block Component
//!
//! Syntax-highlighted code display.

use crate::core::component::{VNode, BoxNode, BoxStyle, TextStyle, Color, NamedColor, BorderStyle};

/// Code block component.
#[derive(Debug, Clone)]
pub struct CodeBlock {
    code: String,
    language: Option<String>,
    show_line_numbers: bool,
    highlight_lines: Vec<usize>,
    start_line: usize,
    theme: CodeTheme,
}

/// Code color theme.
#[derive(Debug, Clone, Copy, Default)]
pub enum CodeTheme {
    #[default]
    Dark,
    Light,
    Monokai,
}

impl Default for CodeBlock {
    fn default() -> Self {
        Self {
            code: String::new(),
            language: None,
            show_line_numbers: true,
            highlight_lines: Vec::new(),
            start_line: 1,
            theme: CodeTheme::Dark,
        }
    }
}

impl CodeBlock {
    /// Create a new code block.
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            ..Default::default()
        }
    }

    /// Set language.
    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.language = Some(lang.into());
        self
    }

    /// Common language shortcuts.
    pub fn rust(self) -> Self { self.language("rust") }
    pub fn javascript(self) -> Self { self.language("javascript") }
    pub fn typescript(self) -> Self { self.language("typescript") }
    pub fn python(self) -> Self { self.language("python") }
    pub fn json(self) -> Self { self.language("json") }
    pub fn bash(self) -> Self { self.language("bash") }

    /// Show/hide line numbers.
    pub fn line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    /// Highlight specific lines.
    pub fn highlight<I>(mut self, lines: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        self.highlight_lines = lines.into_iter().collect();
        self
    }

    /// Set starting line number.
    pub fn start_line(mut self, line: usize) -> Self {
        self.start_line = line;
        self
    }

    /// Set theme.
    pub fn theme(mut self, theme: CodeTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Simple keyword highlighting.
    fn highlight_syntax(&self, line: &str) -> (String, Color) {
        let lang = self.language.as_deref().unwrap_or("");

        // Very basic syntax highlighting
        let keywords = match lang {
            "rust" => vec!["fn", "let", "mut", "pub", "struct", "enum", "impl", "use", "mod", "if", "else", "match", "for", "while", "loop", "return", "self", "Self", "true", "false"],
            "javascript" | "typescript" => vec!["function", "const", "let", "var", "if", "else", "for", "while", "return", "class", "import", "export", "async", "await", "true", "false", "null", "undefined"],
            "python" => vec!["def", "class", "if", "elif", "else", "for", "while", "return", "import", "from", "as", "with", "try", "except", "True", "False", "None"],
            _ => vec![],
        };

        let trimmed = line.trim();

        // Check for comments
        if trimmed.starts_with("//") || trimmed.starts_with('#') {
            return (line.to_string(), Color::Named(NamedColor::Gray));
        }

        // Check for strings
        if trimmed.starts_with('"') || trimmed.starts_with('\'') || trimmed.starts_with('`') {
            return (line.to_string(), Color::Named(NamedColor::Green));
        }

        // Check for keywords
        for kw in &keywords {
            if trimmed.starts_with(kw) &&
               (trimmed.len() == kw.len() || !trimmed.chars().nth(kw.len()).unwrap_or(' ').is_alphanumeric()) {
                return (line.to_string(), Color::Named(NamedColor::Magenta));
            }
        }

        // Default color based on theme
        let default_color = match self.theme {
            CodeTheme::Dark => Color::Named(NamedColor::White),
            CodeTheme::Light => Color::Named(NamedColor::Black),
            CodeTheme::Monokai => Color::Named(NamedColor::White),
        };

        (line.to_string(), default_color)
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let lines: Vec<&str> = self.code.lines().collect();
        let line_count = lines.len();
        let line_num_width = (self.start_line + line_count).to_string().len();

        let mut children = Vec::new();

        // Language header if provided
        if let Some(lang) = &self.language {
            children.push(VNode::styled_text(
                format!(" {} ", lang),
                TextStyle { color: Some(Color::Named(NamedColor::Gray)), dim: true, ..Default::default() }
            ));
        }

        for (idx, line) in lines.iter().enumerate() {
            let line_num = self.start_line + idx;
            let is_highlighted = self.highlight_lines.contains(&line_num);

            let (content, color) = self.highlight_syntax(line);

            let formatted = if self.show_line_numbers {
                format!("{:>width$} │ {}", line_num, content, width = line_num_width)
            } else {
                content
            };

            let style = if is_highlighted {
                TextStyle { color: Some(Color::Named(NamedColor::Yellow)), inverse: true, ..Default::default() }
            } else {
                TextStyle::color(color)
            };

            children.push(VNode::styled_text(formatted, style));
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle {
                border_style: Some(BorderStyle::Single),
                padding_left: Some(1),
                padding_right: Some(1),
                ..Default::default()
            },
            ..Default::default()
        })
    }
}

/// Markdown component (basic).
#[derive(Debug, Clone)]
pub struct Markdown {
    content: String,
    width: Option<u16>,
}

impl Default for Markdown {
    fn default() -> Self {
        Self {
            content: String::new(),
            width: None,
        }
    }
}

impl Markdown {
    /// Create new markdown renderer.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    /// Set max width.
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let mut children = Vec::new();

        for line in self.content.lines() {
            let trimmed = line.trim();

            // Headers
            if trimmed.starts_with("### ") {
                children.push(VNode::styled_text(
                    format!("   {}", &trimmed[4..]),
                    TextStyle { color: Some(Color::Named(NamedColor::Cyan)), bold: true, ..Default::default() }
                ));
            } else if trimmed.starts_with("## ") {
                children.push(VNode::styled_text(
                    format!("  {}", &trimmed[3..]),
                    TextStyle { color: Some(Color::Named(NamedColor::Cyan)), bold: true, ..Default::default() }
                ));
            } else if trimmed.starts_with("# ") {
                children.push(VNode::styled_text(
                    trimmed[2..].to_string(),
                    TextStyle { color: Some(Color::Named(NamedColor::Cyan)), bold: true, ..Default::default() }
                ));
            }
            // Bullet points
            else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                children.push(VNode::styled_text(
                    format!("  • {}", &trimmed[2..]),
                    TextStyle::default()
                ));
            }
            // Numbered lists
            else if trimmed.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
                && trimmed.contains(". ") {
                children.push(VNode::styled_text(
                    format!("  {}", trimmed),
                    TextStyle::default()
                ));
            }
            // Code blocks (inline)
            else if trimmed.starts_with('`') && trimmed.ends_with('`') {
                children.push(VNode::styled_text(
                    trimmed[1..trimmed.len()-1].to_string(),
                    TextStyle::color(Color::Named(NamedColor::Green))
                ));
            }
            // Blockquotes
            else if trimmed.starts_with("> ") {
                children.push(VNode::styled_text(
                    format!("│ {}", &trimmed[2..]),
                    TextStyle { color: Some(Color::Named(NamedColor::Gray)), italic: true, ..Default::default() }
                ));
            }
            // Horizontal rule
            else if trimmed == "---" || trimmed == "***" {
                children.push(VNode::styled_text(
                    "─".repeat(self.width.unwrap_or(40) as usize),
                    TextStyle::color(Color::Named(NamedColor::Gray))
                ));
            }
            // Bold and italic (simplified - just strip markers)
            else if !trimmed.is_empty() {
                let processed = trimmed
                    .replace("**", "")
                    .replace("__", "")
                    .replace('*', "")
                    .replace('_', "");
                children.push(VNode::styled_text(processed, TextStyle::default()));
            }
            // Empty line
            else {
                children.push(VNode::styled_text(String::new(), TextStyle::default()));
            }
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle::default(),
            ..Default::default()
        })
    }
}
