//! Reactive Format Hooks
//!
//! Reactive wrappers for format utilities. These hooks return memoized
//! values that update when inputs change.
//!
//! # Example
//!
//! ```rust,ignore
//! use tuiuiu::hooks::format::{use_format_bytes, use_format_duration};
//! use tuiuiu::core::signals::create_signal;
//!
//! let (file_size, set_file_size) = create_signal(1024u64);
//! let formatted_size = use_format_bytes(file_size.clone());
//! // formatted_size.get() == "1.0K"
//! ```

use crate::core::signals::{create_memo, Memo, ReadSignal};
use crate::utils::format::{
    format_bytes, format_bytes_si, format_compact, format_currency, format_duration,
    format_duration_compact, format_number, format_percent, format_relative_time,
};

// =============================================================================
// Reactive Format Hooks
// =============================================================================

/// Reactive bytes formatter (binary units: KiB, MiB, GiB).
///
/// # Arguments
///
/// * `bytes` - Signal returning byte count
///
/// # Returns
///
/// Memo returning formatted string
///
/// # Example
///
/// ```rust,ignore
/// let (size, set_size) = create_signal(1024u64);
/// let formatted = use_format_bytes(size.clone());
/// // formatted.get() == "1.0K"
/// ```
pub fn use_format_bytes(bytes: ReadSignal<u64>) -> Memo<String> {
    create_memo(move || format_bytes(bytes.get()))
}

/// Reactive bytes formatter (SI units: KB, MB, GB).
///
/// Uses SI/decimal units (1000-based) instead of binary (1024-based).
pub fn use_format_bytes_si(bytes: ReadSignal<u64>) -> Memo<String> {
    create_memo(move || format_bytes_si(bytes.get()))
}

/// Reactive duration formatter.
///
/// # Arguments
///
/// * `seconds` - Signal returning seconds
///
/// # Returns
///
/// Memo returning formatted string like "1h 30m 45s"
pub fn use_format_duration(seconds: ReadSignal<u64>) -> Memo<String> {
    create_memo(move || format_duration(seconds.get()))
}

/// Reactive compact duration formatter.
///
/// Returns shorter format like "1:30:45".
pub fn use_format_duration_compact(seconds: ReadSignal<u64>) -> Memo<String> {
    create_memo(move || format_duration_compact(seconds.get()))
}

/// Reactive number formatter with thousand separators.
///
/// # Arguments
///
/// * `n` - Signal returning number
///
/// # Returns
///
/// Memo returning formatted string like "1,234,567"
pub fn use_format_number(n: ReadSignal<f64>) -> Memo<String> {
    create_memo(move || format_number(n.get()))
}

/// Reactive compact number formatter.
///
/// # Arguments
///
/// * `n` - Signal returning number
///
/// # Returns
///
/// Memo returning formatted string like "1.5M"
pub fn use_format_compact(n: ReadSignal<f64>) -> Memo<String> {
    create_memo(move || format_compact(n.get()))
}

/// Reactive percentage formatter.
///
/// # Arguments
///
/// * `n` - Signal returning decimal number (0.156 = 15.6%)
///
/// # Returns
///
/// Memo returning percentage string like "15.6%"
pub fn use_format_percent(n: ReadSignal<f64>) -> Memo<String> {
    create_memo(move || format_percent(n.get()))
}

/// Reactive currency formatter.
///
/// # Arguments
///
/// * `amount` - Signal returning amount
/// * `symbol` - Currency symbol
///
/// # Returns
///
/// Memo returning formatted currency like "$1,234.56"
pub fn use_format_currency(amount: ReadSignal<f64>, symbol: &'static str) -> Memo<String> {
    create_memo(move || format_currency(amount.get(), symbol))
}

/// Reactive relative time formatter.
///
/// Formats a timestamp relative to now. The timestamp should be in seconds
/// since epoch. Returns strings like "5 minutes ago" or "2 hours ago".
///
/// # Arguments
///
/// * `seconds_ago` - Signal returning seconds since the event occurred
///
/// # Returns
///
/// Memo returning formatted relative time like "5 minutes ago"
///
/// # Example
///
/// ```rust,ignore
/// let (seconds_ago, _) = create_signal(120i64); // 2 minutes ago
/// let relative = use_format_relative(seconds_ago.clone());
/// // relative.get() == "2 minutes ago"
/// ```
pub fn use_format_relative(seconds_ago: ReadSignal<i64>) -> Memo<String> {
    create_memo(move || format_relative_time(seconds_ago.get()))
}

/// Reactive delta/change formatter.
///
/// # Arguments
///
/// * `delta` - Signal returning change value
///
/// # Returns
///
/// Memo returning delta string like "+12.5%" or "-3.2%"
pub fn use_format_delta(delta: ReadSignal<f64>) -> Memo<String> {
    create_memo(move || {
        let d = delta.get();
        let sign = if d >= 0.0 { "+" } else { "" };
        format!("{}{}%", sign, format_compact(d))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::signals::create_signal;

    #[test]
    fn test_use_format_bytes() {
        let (size, _) = create_signal(1024u64);
        let formatted = use_format_bytes(size);
        assert_eq!(formatted.get(), "1 KB");
    }

    #[test]
    fn test_use_format_number() {
        let (n, _) = create_signal(1234567.0f64);
        let formatted = use_format_number(n);
        assert_eq!(formatted.get(), "1,234,567");
    }

    #[test]
    fn test_use_format_percent() {
        // format_percent expects the percentage value directly (45.6 -> "45.6%")
        let (p, _) = create_signal(45.6f64);
        let formatted = use_format_percent(p);
        assert_eq!(formatted.get(), "45.6%");
    }

    #[test]
    fn test_use_format_duration() {
        // format_duration shows only the two largest units
        let (secs, _) = create_signal(3661u64); // 1h 1m (seconds omitted)
        let formatted = use_format_duration(secs);
        assert_eq!(formatted.get(), "1h 1m");
    }
}
