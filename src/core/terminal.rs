//! Terminal Driver - Raw terminal I/O handling
//!
//! This module provides low-level terminal control without external dependencies:
//! - Raw mode activation/deactivation
//! - Keyboard input parsing (including escape sequences)
//! - Mouse event handling
//! - Screen buffer management
//! - Cursor control

use std::io::{self, Read, Write, Stdout, stdin, stdout};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

#[cfg(unix)]
use std::os::unix::io::AsRawFd;

// =============================================================================
// Terminal Mode State
// =============================================================================

static RAW_MODE_ENABLED: AtomicBool = AtomicBool::new(false);
static MOUSE_ENABLED: AtomicBool = AtomicBool::new(false);
static ALTERNATE_SCREEN: AtomicBool = AtomicBool::new(false);

// Store original termios for restoration
#[cfg(unix)]
static mut ORIGINAL_TERMIOS: Option<libc::termios> = None;

// =============================================================================
// Terminal Struct
// =============================================================================

/// Terminal handle for I/O operations.
pub struct Terminal {
    stdout: Stdout,
    size: (u16, u16),
}

impl Terminal {
    /// Create a new terminal handle.
    pub fn new() -> io::Result<Self> {
        let size = get_terminal_size()?;
        Ok(Self {
            stdout: stdout(),
            size,
        })
    }

    /// Get terminal dimensions (columns, rows).
    pub fn size(&self) -> (u16, u16) {
        self.size
    }

    /// Refresh terminal size.
    pub fn refresh_size(&mut self) -> io::Result<()> {
        self.size = get_terminal_size()?;
        Ok(())
    }

    /// Write content to the terminal.
    pub fn write(&mut self, content: &str) -> io::Result<()> {
        write!(self.stdout, "{}", content)?;
        Ok(())
    }

    /// Flush the output buffer.
    pub fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }

    /// Clear the entire screen.
    pub fn clear(&mut self) -> io::Result<()> {
        write!(self.stdout, "\x1B[2J\x1B[H")?;
        self.flush()
    }

    /// Clear from cursor to end of screen.
    pub fn clear_from_cursor(&mut self) -> io::Result<()> {
        write!(self.stdout, "\x1B[J")?;
        Ok(())
    }

    /// Clear the current line.
    pub fn clear_line(&mut self) -> io::Result<()> {
        write!(self.stdout, "\x1B[2K")?;
        Ok(())
    }

    /// Move cursor to position (1-indexed).
    pub fn move_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        write!(self.stdout, "\x1B[{};{}H", y, x)?;
        Ok(())
    }

    /// Hide the cursor.
    pub fn hide_cursor(&mut self) -> io::Result<()> {
        write!(self.stdout, "\x1B[?25l")?;
        Ok(())
    }

    /// Show the cursor.
    pub fn show_cursor(&mut self) -> io::Result<()> {
        write!(self.stdout, "\x1B[?25h")?;
        Ok(())
    }

    /// Save cursor position.
    pub fn save_cursor(&mut self) -> io::Result<()> {
        write!(self.stdout, "\x1B7")?;
        Ok(())
    }

    /// Restore cursor position.
    pub fn restore_cursor(&mut self) -> io::Result<()> {
        write!(self.stdout, "\x1B8")?;
        Ok(())
    }

    /// Enable alternate screen buffer.
    pub fn enter_alternate_screen(&mut self) -> io::Result<()> {
        if !ALTERNATE_SCREEN.swap(true, Ordering::SeqCst) {
            write!(self.stdout, "\x1B[?1049h")?;
            self.flush()?;
        }
        Ok(())
    }

    /// Disable alternate screen buffer.
    pub fn leave_alternate_screen(&mut self) -> io::Result<()> {
        if ALTERNATE_SCREEN.swap(false, Ordering::SeqCst) {
            write!(self.stdout, "\x1B[?1049l")?;
            self.flush()?;
        }
        Ok(())
    }

    /// Enable mouse capture.
    pub fn enable_mouse(&mut self) -> io::Result<()> {
        if !MOUSE_ENABLED.swap(true, Ordering::SeqCst) {
            // Enable mouse tracking (SGR extended mode)
            write!(self.stdout, "\x1B[?1000h\x1B[?1002h\x1B[?1006h")?;
            self.flush()?;
        }
        Ok(())
    }

    /// Disable mouse capture.
    pub fn disable_mouse(&mut self) -> io::Result<()> {
        if MOUSE_ENABLED.swap(false, Ordering::SeqCst) {
            write!(self.stdout, "\x1B[?1006l\x1B[?1002l\x1B[?1000l")?;
            self.flush()?;
        }
        Ok(())
    }

    /// Enable raw mode.
    pub fn enable_raw_mode(&self) -> io::Result<()> {
        enable_raw_mode()
    }

    /// Disable raw mode.
    pub fn disable_raw_mode(&self) -> io::Result<()> {
        disable_raw_mode()
    }

    /// Check if raw mode is enabled.
    pub fn is_raw_mode(&self) -> bool {
        RAW_MODE_ENABLED.load(Ordering::SeqCst)
    }

    /// Read a single event with optional timeout.
    pub fn read_event(&self, timeout: Option<Duration>) -> io::Result<Option<TerminalEvent>> {
        read_event(timeout)
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new().expect("Failed to create terminal")
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        // Cleanup on drop
        let _ = self.disable_mouse();
        let _ = self.leave_alternate_screen();
        let _ = self.show_cursor();
        let _ = disable_raw_mode();
    }
}

// =============================================================================
// Raw Mode
// =============================================================================

/// Enable raw mode for the terminal.
#[cfg(unix)]
pub fn enable_raw_mode() -> io::Result<()> {
    if RAW_MODE_ENABLED.swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    unsafe {
        let fd = stdin().as_raw_fd();
        let mut termios: libc::termios = std::mem::zeroed();

        if libc::tcgetattr(fd, &mut termios) != 0 {
            return Err(io::Error::last_os_error());
        }

        // Store original settings
        ORIGINAL_TERMIOS = Some(termios);

        // Modify for raw mode
        termios.c_lflag &= !(libc::ECHO | libc::ICANON | libc::ISIG | libc::IEXTEN);
        termios.c_iflag &= !(libc::IXON | libc::ICRNL | libc::BRKINT | libc::INPCK | libc::ISTRIP);
        termios.c_oflag &= !libc::OPOST;
        termios.c_cflag |= libc::CS8;
        termios.c_cc[libc::VMIN] = 0;
        termios.c_cc[libc::VTIME] = 1;

        if libc::tcsetattr(fd, libc::TCSAFLUSH, &termios) != 0 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}

/// Disable raw mode, restoring original settings.
#[cfg(unix)]
pub fn disable_raw_mode() -> io::Result<()> {
    if !RAW_MODE_ENABLED.swap(false, Ordering::SeqCst) {
        return Ok(());
    }

    unsafe {
        if let Some(termios) = ORIGINAL_TERMIOS {
            let fd = stdin().as_raw_fd();
            if libc::tcsetattr(fd, libc::TCSAFLUSH, &termios) != 0 {
                return Err(io::Error::last_os_error());
            }
        }
    }

    Ok(())
}

/// Check if raw mode is currently enabled.
pub fn is_raw_mode_enabled() -> bool {
    RAW_MODE_ENABLED.load(Ordering::SeqCst)
}

// Windows stubs (would need proper implementation)
#[cfg(windows)]
pub fn enable_raw_mode() -> io::Result<()> {
    RAW_MODE_ENABLED.store(true, Ordering::SeqCst);
    Ok(())
}

#[cfg(windows)]
pub fn disable_raw_mode() -> io::Result<()> {
    RAW_MODE_ENABLED.store(false, Ordering::SeqCst);
    Ok(())
}

// =============================================================================
// Terminal Size
// =============================================================================

/// Get the current terminal size.
#[cfg(unix)]
pub fn get_terminal_size() -> io::Result<(u16, u16)> {
    unsafe {
        let mut size: libc::winsize = std::mem::zeroed();
        if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut size) == 0 {
            Ok((size.ws_col, size.ws_row))
        } else {
            // Fallback
            Ok((80, 24))
        }
    }
}

#[cfg(windows)]
pub fn get_terminal_size() -> io::Result<(u16, u16)> {
    Ok((80, 24)) // Stub
}

// =============================================================================
// Key Events
// =============================================================================

/// Keyboard key representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    /// Regular character
    Char(char),
    /// Function keys F1-F12
    F(u8),
    /// Backspace
    Backspace,
    /// Enter/Return
    Enter,
    /// Left arrow
    Left,
    /// Right arrow
    Right,
    /// Up arrow
    Up,
    /// Down arrow
    Down,
    /// Home
    Home,
    /// End
    End,
    /// Page Up
    PageUp,
    /// Page Down
    PageDown,
    /// Tab
    Tab,
    /// Backtab (Shift+Tab)
    BackTab,
    /// Delete
    Delete,
    /// Insert
    Insert,
    /// Escape
    Escape,
    /// Null/Unknown
    Null,
}

/// Key modifiers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct KeyModifiers {
    /// Shift key
    pub shift: bool,
    /// Control key
    pub ctrl: bool,
    /// Alt/Option key
    pub alt: bool,
    /// Meta/Super/Windows key
    pub meta: bool,
}

impl KeyModifiers {
    /// No modifiers.
    pub const NONE: Self = Self {
        shift: false,
        ctrl: false,
        alt: false,
        meta: false,
    };

    /// Check if any modifier is pressed.
    pub fn any(&self) -> bool {
        self.shift || self.ctrl || self.alt || self.meta
    }
}

/// Keyboard event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    /// The key pressed
    pub key: Key,
    /// Active modifiers
    pub modifiers: KeyModifiers,
}

impl KeyEvent {
    /// Create a new key event.
    pub fn new(key: Key, modifiers: KeyModifiers) -> Self {
        Self { key, modifiers }
    }

    /// Create a simple key event with no modifiers.
    pub fn simple(key: Key) -> Self {
        Self {
            key,
            modifiers: KeyModifiers::NONE,
        }
    }
}

// =============================================================================
// Mouse Events
// =============================================================================

/// Mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// Left button
    Left,
    /// Right button
    Right,
    /// Middle button (scroll wheel click)
    Middle,
    /// No button (for move events)
    None,
}

/// Mouse event kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseEventKind {
    /// Button pressed down
    Down(MouseButton),
    /// Button released
    Up(MouseButton),
    /// Mouse dragged with button held
    Drag(MouseButton),
    /// Mouse moved (no button)
    Move,
    /// Scroll up
    ScrollUp,
    /// Scroll down
    ScrollDown,
    /// Scroll left
    ScrollLeft,
    /// Scroll right
    ScrollRight,
}

/// Mouse event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseEvent {
    /// X coordinate (1-indexed)
    pub x: u16,
    /// Y coordinate (1-indexed)
    pub y: u16,
    /// Event kind
    pub kind: MouseEventKind,
    /// Active modifiers
    pub modifiers: KeyModifiers,
}

// =============================================================================
// Terminal Events
// =============================================================================

/// Terminal event types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalEvent {
    /// Keyboard event
    Key(KeyEvent),
    /// Mouse event
    Mouse(MouseEvent),
    /// Terminal resized
    Resize(u16, u16),
    /// Focus gained
    FocusGained,
    /// Focus lost
    FocusLost,
    /// Paste event (bracketed paste mode)
    Paste(String),
}

// =============================================================================
// Input Parsing
// =============================================================================

/// Read a terminal event with optional timeout.
pub fn read_event(timeout: Option<Duration>) -> io::Result<Option<TerminalEvent>> {
    let mut buf = [0u8; 32];
    let stdin = stdin();
    let mut handle = stdin.lock();

    // Set non-blocking if timeout specified
    #[cfg(unix)]
    if let Some(t) = timeout {
        // Simple polling approach
        let start = std::time::Instant::now();
        loop {
            match handle.read(&mut buf[..1]) {
                Ok(0) => {
                    if start.elapsed() >= t {
                        return Ok(None);
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
                Ok(_) => break,
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    if start.elapsed() >= t {
                        return Ok(None);
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
                Err(e) => return Err(e),
            }
        }
    } else {
        if handle.read(&mut buf[..1])? == 0 {
            return Ok(None);
        }
    }

    // Windows stub
    #[cfg(windows)]
    {
        if handle.read(&mut buf[..1])? == 0 {
            return Ok(None);
        }
    }

    // Parse the input
    parse_input(&buf[0..1], &mut handle)
}

fn parse_input<R: Read>(initial: &[u8], reader: &mut R) -> io::Result<Option<TerminalEvent>> {
    if initial.is_empty() {
        return Ok(None);
    }

    let first = initial[0];

    // Escape sequence
    if first == 0x1B {
        return parse_escape_sequence(reader);
    }

    // Control characters
    if first < 32 {
        let key = match first {
            0 => Key::Null,
            1..=26 => {
                // Ctrl+A through Ctrl+Z
                let c = (first + b'a' - 1) as char;
                return Ok(Some(TerminalEvent::Key(KeyEvent {
                    key: Key::Char(c),
                    modifiers: KeyModifiers {
                        ctrl: true,
                        ..Default::default()
                    },
                })));
            }
            9 => Key::Tab,
            10 | 13 => Key::Enter,
            27 => Key::Escape,
            127 => Key::Backspace,
            _ => Key::Null,
        };
        return Ok(Some(TerminalEvent::Key(KeyEvent::simple(key))));
    }

    // Regular character (including UTF-8)
    let c = if first < 128 {
        first as char
    } else {
        // UTF-8 multi-byte
        let mut bytes = vec![first];
        let needed = if first & 0xE0 == 0xC0 {
            1
        } else if first & 0xF0 == 0xE0 {
            2
        } else if first & 0xF8 == 0xF0 {
            3
        } else {
            0
        };

        let mut buf = [0u8; 3];
        if needed > 0 {
            reader.read_exact(&mut buf[..needed])?;
            bytes.extend_from_slice(&buf[..needed]);
        }

        std::str::from_utf8(&bytes)
            .ok()
            .and_then(|s| s.chars().next())
            .unwrap_or('?')
    };

    Ok(Some(TerminalEvent::Key(KeyEvent::simple(Key::Char(c)))))
}

fn parse_escape_sequence<R: Read>(reader: &mut R) -> io::Result<Option<TerminalEvent>> {
    let mut buf = [0u8; 1];

    // Try to read next byte
    match reader.read(&mut buf) {
        Ok(0) => return Ok(Some(TerminalEvent::Key(KeyEvent::simple(Key::Escape)))),
        Ok(_) => {}
        Err(_) => return Ok(Some(TerminalEvent::Key(KeyEvent::simple(Key::Escape)))),
    }

    match buf[0] {
        b'[' => parse_csi_sequence(reader),
        b'O' => parse_ss3_sequence(reader),
        _ => {
            // Alt + key
            let key = if buf[0] < 32 {
                Key::Null
            } else {
                Key::Char(buf[0] as char)
            };
            Ok(Some(TerminalEvent::Key(KeyEvent {
                key,
                modifiers: KeyModifiers {
                    alt: true,
                    ..Default::default()
                },
            })))
        }
    }
}

fn parse_csi_sequence<R: Read>(reader: &mut R) -> io::Result<Option<TerminalEvent>> {
    let mut params = Vec::new();
    let mut buf = [0u8; 1];

    // Read until we get a letter
    loop {
        if reader.read(&mut buf)? == 0 {
            break;
        }

        match buf[0] {
            b'0'..=b'9' | b';' | b':' => params.push(buf[0]),
            b'<' => params.push(buf[0]), // SGR mouse
            b'A' => return Ok(Some(TerminalEvent::Key(KeyEvent::simple(Key::Up)))),
            b'B' => return Ok(Some(TerminalEvent::Key(KeyEvent::simple(Key::Down)))),
            b'C' => return Ok(Some(TerminalEvent::Key(KeyEvent::simple(Key::Right)))),
            b'D' => return Ok(Some(TerminalEvent::Key(KeyEvent::simple(Key::Left)))),
            b'H' => return Ok(Some(TerminalEvent::Key(KeyEvent::simple(Key::Home)))),
            b'F' => return Ok(Some(TerminalEvent::Key(KeyEvent::simple(Key::End)))),
            b'Z' => return Ok(Some(TerminalEvent::Key(KeyEvent::simple(Key::BackTab)))),
            b'~' => {
                // Parse param to determine key
                let param: u8 = std::str::from_utf8(&params)
                    .ok()
                    .and_then(|s| s.split(';').next())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);

                let key = match param {
                    1 => Key::Home,
                    2 => Key::Insert,
                    3 => Key::Delete,
                    4 => Key::End,
                    5 => Key::PageUp,
                    6 => Key::PageDown,
                    15 => Key::F(5),
                    17 => Key::F(6),
                    18 => Key::F(7),
                    19 => Key::F(8),
                    20 => Key::F(9),
                    21 => Key::F(10),
                    23 => Key::F(11),
                    24 => Key::F(12),
                    _ => Key::Null,
                };
                return Ok(Some(TerminalEvent::Key(KeyEvent::simple(key))));
            }
            b'M' | b'm' => {
                // SGR mouse event
                return parse_sgr_mouse(&params, buf[0] == b'm');
            }
            b'I' => return Ok(Some(TerminalEvent::FocusGained)),
            b'O' => return Ok(Some(TerminalEvent::FocusLost)),
            _ => break,
        }
    }

    Ok(Some(TerminalEvent::Key(KeyEvent::simple(Key::Null))))
}

fn parse_ss3_sequence<R: Read>(reader: &mut R) -> io::Result<Option<TerminalEvent>> {
    let mut buf = [0u8; 1];
    if reader.read(&mut buf)? == 0 {
        return Ok(None);
    }

    let key = match buf[0] {
        b'A' => Key::Up,
        b'B' => Key::Down,
        b'C' => Key::Right,
        b'D' => Key::Left,
        b'H' => Key::Home,
        b'F' => Key::End,
        b'P' => Key::F(1),
        b'Q' => Key::F(2),
        b'R' => Key::F(3),
        b'S' => Key::F(4),
        _ => Key::Null,
    };

    Ok(Some(TerminalEvent::Key(KeyEvent::simple(key))))
}

fn parse_sgr_mouse(params: &[u8], released: bool) -> io::Result<Option<TerminalEvent>> {
    let s = std::str::from_utf8(params).unwrap_or("");
    let s = s.trim_start_matches('<');
    let parts: Vec<&str> = s.split(';').collect();

    if parts.len() < 3 {
        return Ok(None);
    }

    let button: u8 = parts[0].parse().unwrap_or(0);
    let x: u16 = parts[1].parse().unwrap_or(1);
    let y: u16 = parts[2].parse().unwrap_or(1);

    let modifiers = KeyModifiers {
        shift: button & 4 != 0,
        alt: button & 8 != 0,
        ctrl: button & 16 != 0,
        ..Default::default()
    };

    let button_num = button & 0b11;
    let motion = button & 32 != 0;

    let kind = if button & 64 != 0 {
        // Scroll
        match button_num {
            0 => MouseEventKind::ScrollUp,
            1 => MouseEventKind::ScrollDown,
            2 => MouseEventKind::ScrollLeft,
            3 => MouseEventKind::ScrollRight,
            _ => MouseEventKind::ScrollUp,
        }
    } else {
        let btn = match button_num {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            3 => MouseButton::None,
            _ => MouseButton::None,
        };

        if released {
            MouseEventKind::Up(btn)
        } else if motion {
            MouseEventKind::Drag(btn)
        } else {
            MouseEventKind::Down(btn)
        }
    };

    Ok(Some(TerminalEvent::Mouse(MouseEvent {
        x,
        y,
        kind,
        modifiers,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_modifiers() {
        let mods = KeyModifiers::NONE;
        assert!(!mods.any());

        let mods = KeyModifiers {
            ctrl: true,
            ..Default::default()
        };
        assert!(mods.any());
    }

    #[test]
    fn test_key_event() {
        let event = KeyEvent::simple(Key::Enter);
        assert_eq!(event.key, Key::Enter);
        assert!(!event.modifiers.any());
    }
}
