//! Utilities
//!
//! Helper functions and types for terminal manipulation.

pub mod ansi;
pub mod text;
pub mod cursor;
pub mod border;
pub mod highlighter;
pub mod format;
pub mod batch;
pub mod system;

pub use ansi::{strip_ansi, colorize, style, Color, Style};
pub use text::{measure_text, visible_width, wrap_text, truncate_text, slice_ansi};
pub use cursor::{show_cursor, hide_cursor, move_cursor, save_cursor, restore_cursor};
pub use border::{BorderStyle, BorderChars, BORDER_STYLES};
pub use highlighter::{
    Highlighter, HighlightTheme, Token, TokenType,
    LanguageDefinition, create_highlighter, highlight_code,
};
pub use format::{
    format_number, format_bytes, format_bytes_si, format_duration, format_duration_compact,
    format_percent, format_currency, format_compact, format_relative_time,
    truncate, pad_left, pad_right, center,
};
pub use batch::{
    Batcher, RateLimiter, UpdateCoalescer, Accumulator,
};
pub use system::{
    CpuUsage, CpuStats, MemoryInfo, ProcessInfo, SystemInfo, TaskCounts,
    get_cpu_usage, get_memory_info, get_system_info,
    format_bytes as format_bytes_system, format_uptime, get_state_description,
};
