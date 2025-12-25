//! Terminal Hooks
//!
//! Terminal-related hooks.

use crate::core::terminal::get_terminal_size;
use crate::core::signals::{create_signal, create_effect, ReadSignal};
use crate::core::tick::{get_fps, get_fps_metrics, FpsMetrics};

/// Get terminal size as reactive signals.
pub fn use_terminal_size() -> (ReadSignal<u16>, ReadSignal<u16>) {
    let (width, height) = get_terminal_size().unwrap_or((80, 24));
    let (w_signal, _) = create_signal(width);
    let (h_signal, _) = create_signal(height);

    // In a real implementation, this would listen for SIGWINCH

    (w_signal, h_signal)
}

/// FPS tracking result.
pub struct UseFpsResult {
    /// Current FPS signal
    pub fps: ReadSignal<f64>,
    /// Get full metrics
    pub get_metrics: Box<dyn Fn() -> FpsMetrics>,
}

impl std::fmt::Debug for UseFpsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseFpsResult")
            .field("fps", &self.fps)
            .field("get_metrics", &"<fn>")
            .finish()
    }
}

/// Track frames per second.
pub fn use_fps() -> UseFpsResult {
    let (fps_signal, set_fps) = create_signal(0.0);

    // Update FPS periodically
    create_effect(move || {
        let fps = get_fps();
        set_fps.set(fps);
    });

    UseFpsResult {
        fps: fps_signal,
        get_metrics: Box::new(get_fps_metrics),
    }
}

/// Get terminal dimensions.
pub fn use_dimensions() -> (u16, u16) {
    get_terminal_size().unwrap_or((80, 24))
}
