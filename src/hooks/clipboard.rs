//! Clipboard Hook
//!
//! Hook for clipboard operations in TUI applications.
//!
//! # Example
//!
//! ```rust,ignore
//! use tuiuiu::hooks::clipboard::{use_clipboard, copy_to_clipboard};
//!
//! let clipboard = use_clipboard();
//! clipboard.copy("Hello, World!");
//! println!("Copied: {}", clipboard.read());
//! ```

use crate::core::signals::{create_signal, ReadSignal, WriteSignal};

// =============================================================================
// Types
// =============================================================================

/// Clipboard handle for reading and writing clipboard content.
#[derive(Clone)]
pub struct ClipboardHandle {
    content: ReadSignal<String>,
    set_content: WriteSignal<String>,
}

impl ClipboardHandle {
    /// Get the current clipboard content.
    pub fn read(&self) -> String {
        self.content.get()
    }

    /// Copy text to the clipboard.
    pub fn copy(&self, text: &str) {
        self.set_content.set(text.to_string());
    }

    /// Clear the clipboard.
    pub fn clear(&self) {
        self.set_content.set(String::new());
    }

    /// Check if clipboard is empty.
    pub fn is_empty(&self) -> bool {
        self.content.get().is_empty()
    }

    /// Get the clipboard content signal for reactive access.
    pub fn signal(&self) -> ReadSignal<String> {
        self.content.clone()
    }
}

// =============================================================================
// Internal Clipboard State
// =============================================================================

use std::cell::RefCell;

thread_local! {
    static CLIPBOARD: RefCell<String> = const { RefCell::new(String::new()) };
}

/// Copy text to the internal clipboard.
pub fn copy_to_clipboard(text: &str) {
    CLIPBOARD.with(|c| {
        *c.borrow_mut() = text.to_string();
    });
}

/// Read from the internal clipboard.
pub fn read_clipboard() -> String {
    CLIPBOARD.with(|c| c.borrow().clone())
}

/// Clear the internal clipboard.
pub fn clear_clipboard() {
    CLIPBOARD.with(|c| {
        c.borrow_mut().clear();
    });
}

// =============================================================================
// Hook
// =============================================================================

/// Create a clipboard handle for copy/paste operations.
///
/// This provides a simple in-memory clipboard for TUI applications.
/// For system clipboard integration, use platform-specific crates.
///
/// # Returns
///
/// A `ClipboardHandle` with reactive clipboard content.
///
/// # Example
///
/// ```rust,ignore
/// use tuiuiu::hooks::clipboard::use_clipboard;
///
/// let clipboard = use_clipboard();
///
/// // Copy some text
/// clipboard.copy("Hello!");
///
/// // Read it back
/// let content = clipboard.read();
/// ```
pub fn use_clipboard() -> ClipboardHandle {
    let initial = read_clipboard();
    let (content, set_content) = create_signal(initial);

    ClipboardHandle {
        content,
        set_content,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_copy_read() {
        clear_clipboard();
        let clipboard = use_clipboard();

        clipboard.copy("test content");
        assert_eq!(clipboard.read(), "test content");
    }

    #[test]
    fn test_clipboard_clear() {
        let clipboard = use_clipboard();

        clipboard.copy("something");
        assert!(!clipboard.is_empty());

        clipboard.clear();
        assert!(clipboard.is_empty());
    }

    #[test]
    fn test_global_clipboard() {
        clear_clipboard();
        copy_to_clipboard("global test");
        assert_eq!(read_clipboard(), "global test");
    }
}
