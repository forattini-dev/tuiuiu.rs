//! Syntax Highlighter
//!
//! Token-based syntax highlighting for code display in terminal.
//! Supports multiple languages with customizable themes.

use crate::core::component::{Color, NamedColor};

// =============================================================================
// Token Types
// =============================================================================

/// Token type for syntax highlighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// Plain text
    Text,
    /// Language keyword (fn, let, if, etc.)
    Keyword,
    /// Type name (String, Vec, etc.)
    Type,
    /// Function name
    Function,
    /// String literal
    String,
    /// Number literal
    Number,
    /// Comment
    Comment,
    /// Operator (+, -, *, etc.)
    Operator,
    /// Punctuation (, ; : etc.)
    Punctuation,
    /// Variable/identifier
    Variable,
    /// Constant (TRUE, FALSE, etc.)
    Constant,
    /// Attribute/decorator (@something)
    Attribute,
    /// Macro (rust macros, etc.)
    Macro,
    /// Lifetime ('a, 'static)
    Lifetime,
    /// Label
    Label,
    /// Error/invalid token
    Error,
}

/// A single token.
#[derive(Debug, Clone)]
pub struct Token {
    /// Token text
    pub text: String,
    /// Token type
    pub token_type: TokenType,
    /// Start position in original text
    pub start: usize,
    /// End position in original text
    pub end: usize,
}

impl Token {
    /// Create a new token.
    pub fn new(text: impl Into<String>, token_type: TokenType, start: usize, end: usize) -> Self {
        Self {
            text: text.into(),
            token_type,
            start,
            end,
        }
    }
}

// =============================================================================
// Highlight Theme
// =============================================================================

/// Color theme for syntax highlighting.
#[derive(Debug, Clone)]
pub struct HighlightTheme {
    pub text: Color,
    pub keyword: Color,
    pub type_name: Color,
    pub function: Color,
    pub string: Color,
    pub number: Color,
    pub comment: Color,
    pub operator: Color,
    pub punctuation: Color,
    pub variable: Color,
    pub constant: Color,
    pub attribute: Color,
    pub macro_color: Color,
    pub lifetime: Color,
    pub label: Color,
    pub error: Color,
}

impl Default for HighlightTheme {
    fn default() -> Self {
        Self::dark()
    }
}

impl HighlightTheme {
    /// Dark theme (default).
    pub fn dark() -> Self {
        Self {
            text: Color::Named(NamedColor::White),
            keyword: Color::Named(NamedColor::Magenta),
            type_name: Color::Named(NamedColor::Yellow),
            function: Color::Named(NamedColor::Blue),
            string: Color::Named(NamedColor::Green),
            number: Color::Named(NamedColor::Cyan),
            comment: Color::Named(NamedColor::Gray),
            operator: Color::Named(NamedColor::White),
            punctuation: Color::Named(NamedColor::Gray),
            variable: Color::Named(NamedColor::White),
            constant: Color::Named(NamedColor::Yellow),
            attribute: Color::Named(NamedColor::Yellow),
            macro_color: Color::Named(NamedColor::BrightMagenta),
            lifetime: Color::Named(NamedColor::Cyan),
            label: Color::Named(NamedColor::Yellow),
            error: Color::Named(NamedColor::Red),
        }
    }

    /// Light theme.
    pub fn light() -> Self {
        Self {
            text: Color::Named(NamedColor::Black),
            keyword: Color::Named(NamedColor::Magenta),
            type_name: Color::Named(NamedColor::Blue),
            function: Color::Named(NamedColor::Blue),
            string: Color::Named(NamedColor::Green),
            number: Color::Named(NamedColor::Cyan),
            comment: Color::Named(NamedColor::Gray),
            operator: Color::Named(NamedColor::Black),
            punctuation: Color::Named(NamedColor::Gray),
            variable: Color::Named(NamedColor::Black),
            constant: Color::Named(NamedColor::Magenta),
            attribute: Color::Named(NamedColor::Yellow),
            macro_color: Color::Named(NamedColor::Magenta),
            lifetime: Color::Named(NamedColor::Cyan),
            label: Color::Named(NamedColor::Yellow),
            error: Color::Named(NamedColor::Red),
        }
    }

    /// Monokai theme.
    pub fn monokai() -> Self {
        Self {
            text: Color::Rgb(248, 248, 242),
            keyword: Color::Rgb(249, 38, 114),
            type_name: Color::Rgb(166, 226, 46),
            function: Color::Rgb(166, 226, 46),
            string: Color::Rgb(230, 219, 116),
            number: Color::Rgb(174, 129, 255),
            comment: Color::Rgb(117, 113, 94),
            operator: Color::Rgb(249, 38, 114),
            punctuation: Color::Rgb(248, 248, 242),
            variable: Color::Rgb(248, 248, 242),
            constant: Color::Rgb(174, 129, 255),
            attribute: Color::Rgb(166, 226, 46),
            macro_color: Color::Rgb(102, 217, 239),
            lifetime: Color::Rgb(102, 217, 239),
            label: Color::Rgb(166, 226, 46),
            error: Color::Rgb(249, 38, 114),
        }
    }

    /// Dracula theme.
    pub fn dracula() -> Self {
        Self {
            text: Color::Rgb(248, 248, 242),
            keyword: Color::Rgb(255, 121, 198),
            type_name: Color::Rgb(139, 233, 253),
            function: Color::Rgb(80, 250, 123),
            string: Color::Rgb(241, 250, 140),
            number: Color::Rgb(189, 147, 249),
            comment: Color::Rgb(98, 114, 164),
            operator: Color::Rgb(255, 121, 198),
            punctuation: Color::Rgb(248, 248, 242),
            variable: Color::Rgb(248, 248, 242),
            constant: Color::Rgb(189, 147, 249),
            attribute: Color::Rgb(80, 250, 123),
            macro_color: Color::Rgb(139, 233, 253),
            lifetime: Color::Rgb(255, 121, 198),
            label: Color::Rgb(241, 250, 140),
            error: Color::Rgb(255, 85, 85),
        }
    }

    /// Get color for a token type.
    pub fn color_for(&self, token_type: TokenType) -> Color {
        match token_type {
            TokenType::Text => self.text,
            TokenType::Keyword => self.keyword,
            TokenType::Type => self.type_name,
            TokenType::Function => self.function,
            TokenType::String => self.string,
            TokenType::Number => self.number,
            TokenType::Comment => self.comment,
            TokenType::Operator => self.operator,
            TokenType::Punctuation => self.punctuation,
            TokenType::Variable => self.variable,
            TokenType::Constant => self.constant,
            TokenType::Attribute => self.attribute,
            TokenType::Macro => self.macro_color,
            TokenType::Lifetime => self.lifetime,
            TokenType::Label => self.label,
            TokenType::Error => self.error,
        }
    }
}

// =============================================================================
// Language Definition
// =============================================================================

/// Language definition for highlighting.
#[derive(Debug, Clone)]
pub struct LanguageDefinition {
    /// Language name
    pub name: String,
    /// Keywords
    pub keywords: Vec<&'static str>,
    /// Types
    pub types: Vec<&'static str>,
    /// Built-in functions
    pub builtins: Vec<&'static str>,
    /// Constants
    pub constants: Vec<&'static str>,
    /// Line comment prefix
    pub line_comment: Option<&'static str>,
    /// Block comment (start, end)
    pub block_comment: Option<(&'static str, &'static str)>,
    /// String delimiters
    pub string_delimiters: Vec<char>,
    /// Char delimiter
    pub char_delimiter: Option<char>,
    /// Raw string prefix (for languages like Rust r#"..."#)
    pub raw_string_prefix: Option<&'static str>,
    /// Attribute prefix (@ for Python, # for Rust attributes)
    pub attribute_prefix: Option<&'static str>,
    /// Macro suffix (! for Rust)
    pub macro_suffix: Option<char>,
    /// Supports lifetimes ('a syntax)
    pub has_lifetimes: bool,
}

impl LanguageDefinition {
    /// Create Rust language definition.
    pub fn rust() -> Self {
        Self {
            name: "rust".to_string(),
            keywords: vec![
                "as", "async", "await", "break", "const", "continue", "crate", "dyn",
                "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in",
                "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
                "self", "Self", "static", "struct", "super", "trait", "true", "type",
                "unsafe", "use", "where", "while",
            ],
            types: vec![
                "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128",
                "isize", "str", "u8", "u16", "u32", "u64", "u128", "usize",
                "String", "Vec", "Option", "Result", "Box", "Rc", "Arc", "Cell",
                "RefCell", "HashMap", "HashSet", "BTreeMap", "BTreeSet",
            ],
            builtins: vec![
                "println", "print", "eprintln", "eprint", "format", "panic", "assert",
                "debug_assert", "todo", "unimplemented", "unreachable",
            ],
            constants: vec!["None", "Some", "Ok", "Err", "true", "false"],
            line_comment: Some("//"),
            block_comment: Some(("/*", "*/")),
            string_delimiters: vec!['"'],
            char_delimiter: Some('\''),
            raw_string_prefix: Some("r#"),
            attribute_prefix: Some("#"),
            macro_suffix: Some('!'),
            has_lifetimes: true,
        }
    }

    /// Create JavaScript/TypeScript language definition.
    pub fn javascript() -> Self {
        Self {
            name: "javascript".to_string(),
            keywords: vec![
                "async", "await", "break", "case", "catch", "class", "const",
                "continue", "debugger", "default", "delete", "do", "else", "export",
                "extends", "finally", "for", "function", "if", "import", "in",
                "instanceof", "let", "new", "return", "static", "super", "switch",
                "this", "throw", "try", "typeof", "var", "void", "while", "with", "yield",
            ],
            types: vec![
                "string", "number", "boolean", "object", "symbol", "bigint", "undefined",
                "null", "any", "unknown", "never", "void", "Array", "Promise", "Map", "Set",
            ],
            builtins: vec![
                "console", "Math", "JSON", "Object", "Array", "String", "Number",
                "Boolean", "Date", "RegExp", "Error", "Promise", "setTimeout",
                "setInterval", "fetch", "require",
            ],
            constants: vec!["true", "false", "null", "undefined", "NaN", "Infinity"],
            line_comment: Some("//"),
            block_comment: Some(("/*", "*/")),
            string_delimiters: vec!['"', '\'', '`'],
            char_delimiter: None,
            raw_string_prefix: None,
            attribute_prefix: Some("@"),
            macro_suffix: None,
            has_lifetimes: false,
        }
    }

    /// Create Python language definition.
    pub fn python() -> Self {
        Self {
            name: "python".to_string(),
            keywords: vec![
                "and", "as", "assert", "async", "await", "break", "class", "continue",
                "def", "del", "elif", "else", "except", "finally", "for", "from",
                "global", "if", "import", "in", "is", "lambda", "nonlocal", "not",
                "or", "pass", "raise", "return", "try", "while", "with", "yield",
            ],
            types: vec![
                "int", "float", "str", "bool", "list", "dict", "tuple", "set",
                "bytes", "bytearray", "complex", "frozenset", "range", "slice",
            ],
            builtins: vec![
                "print", "len", "range", "type", "isinstance", "input", "open",
                "sum", "min", "max", "abs", "round", "sorted", "reversed", "enumerate",
                "zip", "map", "filter", "any", "all", "dir", "help", "id", "hash",
            ],
            constants: vec!["True", "False", "None"],
            line_comment: Some("#"),
            block_comment: None,
            string_delimiters: vec!['"', '\''],
            char_delimiter: None,
            raw_string_prefix: Some("r"),
            attribute_prefix: Some("@"),
            macro_suffix: None,
            has_lifetimes: false,
        }
    }

    /// Create Go language definition.
    pub fn go() -> Self {
        Self {
            name: "go".to_string(),
            keywords: vec![
                "break", "case", "chan", "const", "continue", "default", "defer",
                "else", "fallthrough", "for", "func", "go", "goto", "if", "import",
                "interface", "map", "package", "range", "return", "select", "struct",
                "switch", "type", "var",
            ],
            types: vec![
                "bool", "byte", "complex64", "complex128", "error", "float32", "float64",
                "int", "int8", "int16", "int32", "int64", "rune", "string",
                "uint", "uint8", "uint16", "uint32", "uint64", "uintptr",
            ],
            builtins: vec![
                "append", "cap", "close", "complex", "copy", "delete", "imag", "len",
                "make", "new", "panic", "print", "println", "real", "recover",
            ],
            constants: vec!["true", "false", "nil", "iota"],
            line_comment: Some("//"),
            block_comment: Some(("/*", "*/")),
            string_delimiters: vec!['"', '`'],
            char_delimiter: Some('\''),
            raw_string_prefix: None,
            attribute_prefix: None,
            macro_suffix: None,
            has_lifetimes: false,
        }
    }

    /// Create JSON definition (no keywords, just structure).
    pub fn json() -> Self {
        Self {
            name: "json".to_string(),
            keywords: vec![],
            types: vec![],
            builtins: vec![],
            constants: vec!["true", "false", "null"],
            line_comment: None,
            block_comment: None,
            string_delimiters: vec!['"'],
            char_delimiter: None,
            raw_string_prefix: None,
            attribute_prefix: None,
            macro_suffix: None,
            has_lifetimes: false,
        }
    }

    /// Create Bash/Shell definition.
    pub fn bash() -> Self {
        Self {
            name: "bash".to_string(),
            keywords: vec![
                "if", "then", "else", "elif", "fi", "case", "esac", "for", "while",
                "do", "done", "in", "function", "select", "until", "return", "exit",
                "break", "continue", "local", "export", "readonly", "declare",
            ],
            types: vec![],
            builtins: vec![
                "echo", "printf", "read", "cd", "pwd", "ls", "cat", "grep", "sed",
                "awk", "cut", "sort", "uniq", "wc", "head", "tail", "find", "xargs",
                "test", "source", "alias", "set", "unset", "shift", "eval", "exec",
            ],
            constants: vec!["true", "false"],
            line_comment: Some("#"),
            block_comment: None,
            string_delimiters: vec!['"', '\''],
            char_delimiter: None,
            raw_string_prefix: None,
            attribute_prefix: None,
            macro_suffix: None,
            has_lifetimes: false,
        }
    }

    /// Get language definition by name.
    pub fn by_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "rust" | "rs" => Some(Self::rust()),
            "javascript" | "js" | "jsx" => Some(Self::javascript()),
            "typescript" | "ts" | "tsx" => Some(Self::javascript()),
            "python" | "py" => Some(Self::python()),
            "go" | "golang" => Some(Self::go()),
            "json" => Some(Self::json()),
            "bash" | "sh" | "shell" | "zsh" => Some(Self::bash()),
            _ => None,
        }
    }
}

// =============================================================================
// Highlighter
// =============================================================================

/// Syntax highlighter.
#[derive(Debug, Clone)]
pub struct Highlighter {
    theme: HighlightTheme,
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl Highlighter {
    /// Create a new highlighter with default theme.
    pub fn new() -> Self {
        Self {
            theme: HighlightTheme::default(),
        }
    }

    /// Create with a specific theme.
    pub fn with_theme(theme: HighlightTheme) -> Self {
        Self { theme }
    }

    /// Set theme.
    pub fn theme(mut self, theme: HighlightTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Highlight a single line of code.
    pub fn highlight_line(&self, line: &str, lang: &LanguageDefinition) -> Vec<Token> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let mut pos = 0;

        while pos < chars.len() {
            // Skip whitespace
            if chars[pos].is_whitespace() {
                let start = pos;
                while pos < chars.len() && chars[pos].is_whitespace() {
                    pos += 1;
                }
                tokens.push(Token::new(
                    chars[start..pos].iter().collect::<String>(),
                    TokenType::Text,
                    start,
                    pos,
                ));
                continue;
            }

            // Check for comments
            if let Some(prefix) = lang.line_comment {
                if line[pos..].starts_with(prefix) {
                    tokens.push(Token::new(
                        &line[pos..],
                        TokenType::Comment,
                        pos,
                        line.len(),
                    ));
                    break;
                }
            }

            // Check for strings
            if lang.string_delimiters.contains(&chars[pos]) {
                let delimiter = chars[pos];
                let start = pos;
                pos += 1;

                while pos < chars.len() {
                    if chars[pos] == '\\' && pos + 1 < chars.len() {
                        pos += 2; // Skip escaped character
                    } else if chars[pos] == delimiter {
                        pos += 1;
                        break;
                    } else {
                        pos += 1;
                    }
                }

                tokens.push(Token::new(
                    chars[start..pos].iter().collect::<String>(),
                    TokenType::String,
                    start,
                    pos,
                ));
                continue;
            }

            // Check for char literals
            if lang.char_delimiter == Some(chars[pos]) && lang.has_lifetimes {
                // Could be lifetime or char
                let start = pos;
                pos += 1;

                // Check for lifetime
                if pos < chars.len() && chars[pos].is_alphabetic() {
                    while pos < chars.len() && (chars[pos].is_alphanumeric() || chars[pos] == '_') {
                        pos += 1;
                    }

                    // If no closing quote, it's a lifetime
                    if pos >= chars.len() || chars[pos] != '\'' {
                        tokens.push(Token::new(
                            chars[start..pos].iter().collect::<String>(),
                            TokenType::Lifetime,
                            start,
                            pos,
                        ));
                        continue;
                    }
                }

                // It's a char literal
                while pos < chars.len() && chars[pos] != '\'' {
                    if chars[pos] == '\\' && pos + 1 < chars.len() {
                        pos += 2;
                    } else {
                        pos += 1;
                    }
                }
                if pos < chars.len() {
                    pos += 1;
                }

                tokens.push(Token::new(
                    chars[start..pos].iter().collect::<String>(),
                    TokenType::String,
                    start,
                    pos,
                ));
                continue;
            }

            // Check for numbers
            if chars[pos].is_ascii_digit() {
                let start = pos;
                while pos < chars.len() && (chars[pos].is_ascii_alphanumeric() || chars[pos] == '.' || chars[pos] == '_') {
                    pos += 1;
                }
                tokens.push(Token::new(
                    chars[start..pos].iter().collect::<String>(),
                    TokenType::Number,
                    start,
                    pos,
                ));
                continue;
            }

            // Check for identifiers/keywords
            if chars[pos].is_alphabetic() || chars[pos] == '_' {
                let start = pos;
                while pos < chars.len() && (chars[pos].is_alphanumeric() || chars[pos] == '_') {
                    pos += 1;
                }

                let word: String = chars[start..pos].iter().collect();

                // Check for macro
                if pos < chars.len() && lang.macro_suffix == Some(chars[pos]) {
                    pos += 1;
                    tokens.push(Token::new(
                        chars[start..pos].iter().collect::<String>(),
                        TokenType::Macro,
                        start,
                        pos,
                    ));
                    continue;
                }

                // Determine token type
                let token_type = if lang.keywords.contains(&word.as_str()) {
                    TokenType::Keyword
                } else if lang.types.contains(&word.as_str()) {
                    TokenType::Type
                } else if lang.constants.contains(&word.as_str()) {
                    TokenType::Constant
                } else if lang.builtins.contains(&word.as_str()) {
                    TokenType::Function
                } else if word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                    TokenType::Type
                } else if pos < chars.len() && chars[pos] == '(' {
                    TokenType::Function
                } else {
                    TokenType::Variable
                };

                tokens.push(Token::new(word, token_type, start, pos));
                continue;
            }

            // Check for attribute
            if let Some(prefix) = lang.attribute_prefix {
                if line[pos..].starts_with(prefix) {
                    let start = pos;
                    pos += prefix.len();

                    // Read until end of attribute
                    if pos < chars.len() && chars[pos] == '[' {
                        let mut depth = 1;
                        pos += 1;
                        while pos < chars.len() && depth > 0 {
                            if chars[pos] == '[' {
                                depth += 1;
                            } else if chars[pos] == ']' {
                                depth -= 1;
                            }
                            pos += 1;
                        }
                    } else {
                        while pos < chars.len() && (chars[pos].is_alphanumeric() || chars[pos] == '_') {
                            pos += 1;
                        }
                    }

                    tokens.push(Token::new(
                        chars[start..pos].iter().collect::<String>(),
                        TokenType::Attribute,
                        start,
                        pos,
                    ));
                    continue;
                }
            }

            // Operators and punctuation
            let start = pos;
            let ch = chars[pos];
            pos += 1;

            let token_type = match ch {
                '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '!' | '&' | '|' | '^' | '~' => {
                    // Multi-char operators
                    if pos < chars.len() {
                        match (ch, chars[pos]) {
                            ('=', '=') | ('!', '=') | ('<', '=') | ('>', '=') |
                            ('+', '=') | ('-', '=') | ('*', '=') | ('/', '=') |
                            ('&', '&') | ('|', '|') | ('-', '>') | ('=', '>') |
                            ('<', '<') | ('>', '>') | (':', ':') => {
                                pos += 1;
                            }
                            _ => {}
                        }
                    }
                    TokenType::Operator
                }
                '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';' | ':' | '.' | '?' | '@' => {
                    TokenType::Punctuation
                }
                _ => TokenType::Text,
            };

            tokens.push(Token::new(
                chars[start..pos].iter().collect::<String>(),
                token_type,
                start,
                pos,
            ));
        }

        tokens
    }

    /// Highlight code and return tokens.
    pub fn highlight(&self, code: &str, language: &str) -> Vec<Vec<Token>> {
        let lang = LanguageDefinition::by_name(language)
            .unwrap_or_else(|| LanguageDefinition {
                name: language.to_string(),
                keywords: vec![],
                types: vec![],
                builtins: vec![],
                constants: vec![],
                line_comment: Some("//"),
                block_comment: None,
                string_delimiters: vec!['"', '\''],
                char_delimiter: None,
                raw_string_prefix: None,
                attribute_prefix: None,
                macro_suffix: None,
                has_lifetimes: false,
            });

        code.lines()
            .map(|line| self.highlight_line(line, &lang))
            .collect()
    }

    /// Get color for a token type.
    pub fn color_for(&self, token_type: TokenType) -> Color {
        self.theme.color_for(token_type)
    }

    /// Get the theme.
    pub fn get_theme(&self) -> &HighlightTheme {
        &self.theme
    }
}

/// Create a highlighter with default settings.
pub fn create_highlighter() -> Highlighter {
    Highlighter::new()
}

/// Highlight code with a specific language.
pub fn highlight_code(code: &str, language: &str) -> Vec<Vec<Token>> {
    Highlighter::new().highlight(code, language)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_highlighting() {
        let highlighter = Highlighter::new();
        let code = "fn main() { let x = 42; }";
        let tokens = highlighter.highlight(code, "rust");

        assert!(!tokens.is_empty());
        let line = &tokens[0];

        // Find the 'fn' keyword
        let fn_token = line.iter().find(|t| t.text == "fn");
        assert!(fn_token.is_some());
        assert_eq!(fn_token.unwrap().token_type, TokenType::Keyword);
    }

    #[test]
    fn test_string_highlighting() {
        let highlighter = Highlighter::new();
        let code = r#"let s = "hello";"#;
        let tokens = highlighter.highlight(code, "rust");

        let line = &tokens[0];
        let string_token = line.iter().find(|t| t.text.contains("hello"));
        assert!(string_token.is_some());
        assert_eq!(string_token.unwrap().token_type, TokenType::String);
    }

    #[test]
    fn test_number_highlighting() {
        let highlighter = Highlighter::new();
        let code = "let x = 42;";
        let tokens = highlighter.highlight(code, "rust");

        let line = &tokens[0];
        let num_token = line.iter().find(|t| t.text == "42");
        assert!(num_token.is_some());
        assert_eq!(num_token.unwrap().token_type, TokenType::Number);
    }
}
