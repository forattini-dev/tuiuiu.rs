//! Format Utilities
//!
//! Formatting functions for numbers, bytes, durations, and more.

/// Format a number with thousand separators.
pub fn format_number(n: f64) -> String {
    let is_negative = n < 0.0;
    let abs_n = n.abs();

    if abs_n.fract() == 0.0 {
        format_integer(abs_n as i64, is_negative)
    } else {
        let formatted = format!("{:.2}", abs_n);
        let parts: Vec<&str> = formatted.split('.').collect();
        let int_part = format_integer_str(parts[0]);
        let result = format!("{}.{}", int_part, parts[1]);
        if is_negative {
            format!("-{}", result)
        } else {
            result
        }
    }
}

/// Format an integer with thousand separators.
pub fn format_integer(n: i64, negative: bool) -> String {
    let s = n.abs().to_string();
    let result = format_integer_str(&s);
    if negative {
        format!("-{}", result)
    } else {
        result
    }
}

fn format_integer_str(s: &str) -> String {
    let chars: Vec<char> = s.chars().rev().collect();
    let mut result = String::new();
    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(*ch);
    }
    result.chars().rev().collect()
}

/// Format bytes in human-readable form.
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];

    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    if size.fract() < 0.01 {
        format!("{} {}", size as u64, UNITS[unit_idx])
    } else {
        format!("{:.1} {}", size, UNITS[unit_idx])
    }
}

/// Format bytes using SI units (1000-based).
pub fn format_bytes_si(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "kB", "MB", "GB", "TB", "PB"];

    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1000.0 && unit_idx < UNITS.len() - 1 {
        size /= 1000.0;
        unit_idx += 1;
    }

    if size.fract() < 0.01 {
        format!("{} {}", size as u64, UNITS[unit_idx])
    } else {
        format!("{:.1} {}", size, UNITS[unit_idx])
    }
}

/// Format duration in human-readable form.
pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        return format!("{}s", seconds);
    }

    if seconds < 3600 {
        let mins = seconds / 60;
        let secs = seconds % 60;
        return if secs == 0 {
            format!("{}m", mins)
        } else {
            format!("{}m {}s", mins, secs)
        };
    }

    if seconds < 86400 {
        let hours = seconds / 3600;
        let mins = (seconds % 3600) / 60;
        return if mins == 0 {
            format!("{}h", hours)
        } else {
            format!("{}h {}m", hours, mins)
        };
    }

    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    if hours == 0 {
        format!("{}d", days)
    } else {
        format!("{}d {}h", days, hours)
    }
}

/// Format duration in compact form (e.g., "1:23:45").
pub fn format_duration_compact(seconds: u64) -> String {
    let hours = seconds / 3600;
    let mins = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, mins, secs)
    } else {
        format!("{}:{:02}", mins, secs)
    }
}

/// Format a percentage.
pub fn format_percent(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}%", value as i64)
    } else {
        format!("{:.1}%", value)
    }
}

/// Format as currency.
pub fn format_currency(value: f64, symbol: &str) -> String {
    let is_negative = value < 0.0;
    let abs_value = value.abs();
    let formatted = format!("{:.2}", abs_value);
    let parts: Vec<&str> = formatted.split('.').collect();
    let int_part = format_integer_str(parts[0]);
    let result = format!("{}{}.{}", symbol, int_part, parts[1]);
    if is_negative {
        format!("-{}", result)
    } else {
        result
    }
}

/// Compact number formatting (e.g., 1.2K, 3.4M).
pub fn format_compact(n: f64) -> String {
    let is_negative = n < 0.0;
    let abs_n = n.abs();

    let (value, suffix) = if abs_n >= 1_000_000_000_000.0 {
        (abs_n / 1_000_000_000_000.0, "T")
    } else if abs_n >= 1_000_000_000.0 {
        (abs_n / 1_000_000_000.0, "B")
    } else if abs_n >= 1_000_000.0 {
        (abs_n / 1_000_000.0, "M")
    } else if abs_n >= 1_000.0 {
        (abs_n / 1_000.0, "K")
    } else {
        return if is_negative {
            format!("-{}", abs_n)
        } else {
            abs_n.to_string()
        };
    };

    let formatted = if value >= 10.0 || value.fract() < 0.05 {
        format!("{}{}", value as i64, suffix)
    } else {
        format!("{:.1}{}", value, suffix)
    };

    if is_negative {
        format!("-{}", formatted)
    } else {
        formatted
    }
}

/// Format relative time (e.g., "2 hours ago", "in 3 days").
pub fn format_relative_time(seconds_ago: i64) -> String {
    let is_future = seconds_ago < 0;
    let abs_seconds = seconds_ago.abs() as u64;

    let (value, unit) = if abs_seconds < 60 {
        (abs_seconds, "second")
    } else if abs_seconds < 3600 {
        (abs_seconds / 60, "minute")
    } else if abs_seconds < 86400 {
        (abs_seconds / 3600, "hour")
    } else if abs_seconds < 2592000 {
        (abs_seconds / 86400, "day")
    } else if abs_seconds < 31536000 {
        (abs_seconds / 2592000, "month")
    } else {
        (abs_seconds / 31536000, "year")
    };

    let plural = if value == 1 { "" } else { "s" };

    if is_future {
        format!("in {} {}{}", value, unit, plural)
    } else {
        format!("{} {}{} ago", value, unit, plural)
    }
}

/// Truncate a string with ellipsis.
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else if max_len <= 1 {
        "…".to_string()
    } else {
        let truncated: String = s.chars().take(max_len - 1).collect();
        format!("{}…", truncated)
    }
}

/// Pad a string to the left.
pub fn pad_left(s: &str, width: usize, ch: char) -> String {
    let len = s.chars().count();
    if len >= width {
        s.to_string()
    } else {
        let padding: String = std::iter::repeat(ch).take(width - len).collect();
        format!("{}{}", padding, s)
    }
}

/// Pad a string to the right.
pub fn pad_right(s: &str, width: usize, ch: char) -> String {
    let len = s.chars().count();
    if len >= width {
        s.to_string()
    } else {
        let padding: String = std::iter::repeat(ch).take(width - len).collect();
        format!("{}{}", s, padding)
    }
}

/// Center a string.
pub fn center(s: &str, width: usize, ch: char) -> String {
    let len = s.chars().count();
    if len >= width {
        s.to_string()
    } else {
        let total_padding = width - len;
        let left_padding = total_padding / 2;
        let right_padding = total_padding - left_padding;
        let left: String = std::iter::repeat(ch).take(left_padding).collect();
        let right: String = std::iter::repeat(ch).take(right_padding).collect();
        format!("{}{}{}", left, s, right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(1234567.89), "1,234,567.89");
        assert_eq!(format_number(1000.0), "1,000");
        assert_eq!(format_number(-1234.5), "-1,234.50");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1 MB");
        assert_eq!(format_bytes(1073741824), "1 GB");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(45), "45s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3600), "1h");
        assert_eq!(format_duration(3661), "1h 1m");
        assert_eq!(format_duration(86400), "1d");
        assert_eq!(format_duration(90000), "1d 1h");
    }

    #[test]
    fn test_format_compact() {
        assert_eq!(format_compact(999.0), "999");
        assert_eq!(format_compact(1500.0), "1.5K");
        assert_eq!(format_compact(1500000.0), "1.5M");
        assert_eq!(format_compact(1000000000.0), "1B");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello world", 5), "hell…");
        assert_eq!(truncate("hi", 5), "hi");
        assert_eq!(truncate("abc", 1), "…");
    }
}
