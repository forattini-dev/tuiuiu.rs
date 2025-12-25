//! Text Utilities

use super::ansi::strip_ansi;

/// Measure the visible width of a string (excluding ANSI codes).
pub fn measure_text(s: &str) -> usize {
    visible_width(s)
}

/// Get the visible width of a string.
pub fn visible_width(s: &str) -> usize {
    let stripped = strip_ansi(s);
    // Simple implementation - doesn't handle Unicode width properly
    stripped.chars().count()
}

/// Wrap text to a maximum width.
pub fn wrap_text(s: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![s.to_string()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for word in s.split_whitespace() {
        let word_width = visible_width(word);

        if current_width + word_width + (if current_width > 0 { 1 } else { 0 }) > max_width {
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
                current_width = 0;
            }
        }

        if !current_line.is_empty() {
            current_line.push(' ');
            current_width += 1;
        }
        current_line.push_str(word);
        current_width += word_width;
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

/// Truncate text to a maximum width.
pub fn truncate_text(s: &str, max_width: usize, ellipsis: &str) -> String {
    let width = visible_width(s);
    if width <= max_width {
        return s.to_string();
    }

    let ellipsis_width = visible_width(ellipsis);
    if max_width <= ellipsis_width {
        return ellipsis.chars().take(max_width).collect();
    }

    let target_width = max_width - ellipsis_width;
    let mut result = String::new();
    let mut current_width = 0;

    for c in strip_ansi(s).chars() {
        if current_width >= target_width {
            break;
        }
        result.push(c);
        current_width += 1;
    }

    result.push_str(ellipsis);
    result
}

/// Slice a string with ANSI codes preserved.
pub fn slice_ansi(s: &str, start: usize, end: usize) -> String {
    // Simple implementation - strips ANSI and slices
    let stripped = strip_ansi(s);
    stripped.chars().skip(start).take(end - start).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible_width() {
        assert_eq!(visible_width("Hello"), 5);
        assert_eq!(visible_width("\x1B[31mHello\x1B[0m"), 5);
    }

    #[test]
    fn test_wrap_text() {
        let wrapped = wrap_text("Hello World", 6);
        assert_eq!(wrapped, vec!["Hello", "World"]);
    }

    #[test]
    fn test_truncate() {
        let truncated = truncate_text("Hello World", 8, "...");
        assert_eq!(truncated, "Hello...");
    }
}
