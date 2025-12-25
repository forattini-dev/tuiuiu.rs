//! Application Lifecycle Management
//!
//! Manages the main render loop, event handling, and application state.

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crate::core::terminal::{Terminal, TerminalEvent, Key};
use crate::core::renderer::OutputBuffer;
use crate::core::component::VNode;

// =============================================================================
// App State
// =============================================================================

static APP_RUNNING: AtomicBool = AtomicBool::new(false);
static APP_EXIT_REQUESTED: AtomicBool = AtomicBool::new(false);

/// Request the application to exit.
pub fn exit() {
    APP_EXIT_REQUESTED.store(true, Ordering::SeqCst);
}

/// Check if exit has been requested.
pub fn exit_requested() -> bool {
    APP_EXIT_REQUESTED.load(Ordering::SeqCst)
}

/// Check if an app is currently running.
pub fn is_running() -> bool {
    APP_RUNNING.load(Ordering::SeqCst)
}

// =============================================================================
// Render Options
// =============================================================================

/// Options for rendering.
#[derive(Debug, Clone)]
pub struct RenderOptions {
    /// Frames per second (0 for unlimited)
    pub fps: u32,
    /// Use alternate screen buffer
    pub alternate_screen: bool,
    /// Enable mouse capture
    pub mouse: bool,
    /// Exit on Escape key
    pub exit_on_escape: bool,
    /// Exit on Ctrl+C
    pub exit_on_ctrl_c: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            fps: 60,
            alternate_screen: true,
            mouse: true,
            exit_on_escape: true,
            exit_on_ctrl_c: true,
        }
    }
}

// =============================================================================
// App Instance
// =============================================================================

/// The main application instance.
pub struct App {
    terminal: Terminal,
    options: RenderOptions,
    #[allow(dead_code)]
    buffer: OutputBuffer,
    #[allow(dead_code)]
    prev_buffer: OutputBuffer,
    exit_code: i32,
}

impl App {
    /// Create a new app with options.
    pub fn new(options: RenderOptions) -> io::Result<Self> {
        let terminal = Terminal::new()?;
        let (width, height) = terminal.size();

        Ok(Self {
            terminal,
            options,
            buffer: OutputBuffer::new(width, height),
            prev_buffer: OutputBuffer::new(width, height),
            exit_code: 0,
        })
    }

    /// Get terminal size.
    pub fn size(&self) -> (u16, u16) {
        self.terminal.size()
    }

    /// Set the exit code.
    pub fn set_exit_code(&mut self, code: i32) {
        self.exit_code = code;
    }

    /// Initialize the terminal for the app.
    pub fn init(&mut self) -> io::Result<()> {
        self.terminal.enable_raw_mode()?;

        if self.options.alternate_screen {
            self.terminal.enter_alternate_screen()?;
        }

        if self.options.mouse {
            self.terminal.enable_mouse()?;
        }

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;

        APP_RUNNING.store(true, Ordering::SeqCst);
        APP_EXIT_REQUESTED.store(false, Ordering::SeqCst);

        Ok(())
    }

    /// Cleanup the terminal.
    pub fn cleanup(&mut self) -> io::Result<()> {
        APP_RUNNING.store(false, Ordering::SeqCst);

        self.terminal.show_cursor()?;

        if self.options.mouse {
            self.terminal.disable_mouse()?;
        }

        if self.options.alternate_screen {
            self.terminal.leave_alternate_screen()?;
        }

        self.terminal.disable_raw_mode()?;

        Ok(())
    }

    /// Read a terminal event.
    pub fn read_event(&self, timeout: Option<Duration>) -> io::Result<Option<TerminalEvent>> {
        self.terminal.read_event(timeout)
    }

    /// Check if we should exit based on an event.
    pub fn should_exit(&self, event: &TerminalEvent) -> bool {
        match event {
            TerminalEvent::Key(key_event) => {
                // Escape
                if self.options.exit_on_escape && key_event.key == Key::Escape {
                    return true;
                }

                // Ctrl+C
                if self.options.exit_on_ctrl_c
                    && key_event.key == Key::Char('c')
                    && key_event.modifiers.ctrl
                {
                    return true;
                }

                false
            }
            _ => false,
        }
    }

    /// Render content to the terminal.
    pub fn render(&mut self, content: &str) -> io::Result<()> {
        self.terminal.move_cursor(1, 1)?;
        self.terminal.write(content)?;
        self.terminal.flush()?;
        Ok(())
    }

    /// Clear and render.
    pub fn clear_and_render(&mut self, content: &str) -> io::Result<()> {
        self.terminal.clear()?;
        self.render(content)
    }

    /// Wait until exit is requested.
    pub fn wait_until_exit(&mut self) -> io::Result<i32> {
        loop {
            if exit_requested() {
                break;
            }

            if let Some(event) = self.read_event(Some(Duration::from_millis(16)))? {
                if self.should_exit(&event) {
                    break;
                }
            }
        }

        Ok(self.exit_code)
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

// =============================================================================
// Render Functions
// =============================================================================

/// Render a component function.
pub fn render<F, C>(component: F) -> io::Result<App>
where
    F: Fn() -> C,
    C: Into<VNode>,
{
    render_with_options(component, RenderOptions::default())
}

/// Render with custom options.
pub fn render_with_options<F, C>(component: F, options: RenderOptions) -> io::Result<App>
where
    F: Fn() -> C,
    C: Into<VNode>,
{
    let mut app = App::new(options)?;
    app.init()?;

    // Initial render
    let vnode = component().into();
    let (width, height) = app.size();
    let content = crate::core::renderer::render_to_string(&vnode, width, height);
    app.render(&content)?;

    Ok(app)
}

/// Render once and return immediately.
pub fn render_once<F, C>(component: F) -> io::Result<String>
where
    F: Fn() -> C,
    C: Into<VNode>,
{
    let vnode = component().into();
    Ok(crate::core::renderer::render_to_string(&vnode, 80, 24))
}

// =============================================================================
// App Context
// =============================================================================

/// Context available to components.
#[derive(Debug, Clone)]
pub struct AppContext {
    /// Terminal width
    pub width: u16,
    /// Terminal height
    pub height: u16,
}

impl AppContext {
    /// Exit the application.
    pub fn exit(&self) {
        exit();
    }

    /// Exit with a specific code.
    pub fn exit_with_code(&self, _code: i32) {
        // In a full implementation, this would set the exit code
        exit();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_options_default() {
        let opts = RenderOptions::default();
        assert_eq!(opts.fps, 60);
        assert!(opts.alternate_screen);
        assert!(opts.mouse);
    }

    #[test]
    fn test_exit_state() {
        APP_EXIT_REQUESTED.store(false, Ordering::SeqCst);
        assert!(!exit_requested());

        exit();
        assert!(exit_requested());

        APP_EXIT_REQUESTED.store(false, Ordering::SeqCst);
    }
}
