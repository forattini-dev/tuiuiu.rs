//! Utilities
//!
//! Helper functions and types for terminal manipulation.

pub mod ansi;
pub mod batch;
pub mod border;
pub mod cursor;
pub mod format;
pub mod highlighter;
pub mod system;
pub mod text;

pub use ansi::{colorize, strip_ansi, style, Color, Style};
pub use batch::{Accumulator, Batcher, RateLimiter, UpdateCoalescer};
pub use border::{BorderChars, BorderStyle, BORDER_STYLES};
pub use cursor::{hide_cursor, move_cursor, restore_cursor, save_cursor, show_cursor};
pub use format::{
    center, format_bytes, format_bytes_si, format_compact, format_currency, format_duration,
    format_duration_compact, format_number, format_percent, format_relative_time, pad_left,
    pad_right, truncate,
};
pub use highlighter::{
    create_highlighter, highlight_code, HighlightTheme, Highlighter, LanguageDefinition, Token,
    TokenType,
};
pub use system::{
    format_bytes as format_bytes_system, format_uptime, get_cpu_usage, get_memory_info,
    get_state_description, get_system_info, CpuStats, CpuUsage, MemoryInfo, ProcessInfo,
    SystemInfo, TaskCounts,
};
pub use text::{measure_text, slice_ansi, truncate_text, visible_width, wrap_text};
