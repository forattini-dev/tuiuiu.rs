//! Cursor Control Utilities

use std::io::{self, Write, stdout};

/// Show the cursor.
pub fn show_cursor() -> io::Result<()> {
    print!("\x1B[?25h");
    stdout().flush()
}

/// Hide the cursor.
pub fn hide_cursor() -> io::Result<()> {
    print!("\x1B[?25l");
    stdout().flush()
}

/// Move cursor to position (1-indexed).
pub fn move_cursor(x: u16, y: u16) -> io::Result<()> {
    print!("\x1B[{};{}H", y, x);
    stdout().flush()
}

/// Save cursor position.
pub fn save_cursor() -> io::Result<()> {
    print!("\x1B7");
    stdout().flush()
}

/// Restore cursor position.
pub fn restore_cursor() -> io::Result<()> {
    print!("\x1B8");
    stdout().flush()
}

/// Move cursor up.
pub fn cursor_up(n: u16) -> io::Result<()> {
    print!("\x1B[{}A", n);
    stdout().flush()
}

/// Move cursor down.
pub fn cursor_down(n: u16) -> io::Result<()> {
    print!("\x1B[{}B", n);
    stdout().flush()
}

/// Move cursor right.
pub fn cursor_right(n: u16) -> io::Result<()> {
    print!("\x1B[{}C", n);
    stdout().flush()
}

/// Move cursor left.
pub fn cursor_left(n: u16) -> io::Result<()> {
    print!("\x1B[{}D", n);
    stdout().flush()
}

/// Move cursor to beginning of line.
pub fn cursor_to_column(n: u16) -> io::Result<()> {
    print!("\x1B[{}G", n);
    stdout().flush()
}
