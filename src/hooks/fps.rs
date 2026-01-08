//! FPS Tracking Hook
//!
//! Hook for tracking frames per second and performance metrics.
//!
//! # Example
//!
//! ```rust,ignore
//! use tuiuiu::hooks::fps::use_fps;
//!
//! let fps_info = use_fps();
//! println!("FPS: {}", fps_info.fps);
//! println!("Status: {}", fps_info.status_color);
//! ```

use std::cell::RefCell;
use std::collections::VecDeque;
use std::time::Instant;

// =============================================================================
// Types
// =============================================================================

/// FPS metrics result.
#[derive(Debug, Clone)]
pub struct FpsMetrics {
    /// Current FPS (frames in last second).
    pub fps: u32,
    /// Average FPS over the sample window.
    pub avg_fps: f64,
    /// Minimum FPS in the sample window.
    pub min_fps: u32,
    /// Maximum FPS in the sample window.
    pub max_fps: u32,
    /// Total frames tracked.
    pub total_frames: u64,
    /// Performance color indicator.
    pub status_color: FpsColor,
}

impl Default for FpsMetrics {
    fn default() -> Self {
        Self {
            fps: 0,
            avg_fps: 0.0,
            min_fps: 0,
            max_fps: 0,
            total_frames: 0,
            status_color: FpsColor::Green,
        }
    }
}

/// FPS status color for visual indication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FpsColor {
    /// Good performance (>= 30 FPS).
    Green,
    /// Moderate performance (15-29 FPS).
    Yellow,
    /// Poor performance (< 15 FPS).
    Red,
}

impl FpsColor {
    /// Get the color name as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            FpsColor::Green => "green",
            FpsColor::Yellow => "yellow",
            FpsColor::Red => "red",
        }
    }
}

// =============================================================================
// FPS Tracker State
// =============================================================================

thread_local! {
    static FPS_STATE: RefCell<FpsState> = RefCell::new(FpsState::new());
}

struct FpsState {
    /// Timestamps of recent frames.
    frame_times: VecDeque<Instant>,
    /// FPS samples for averaging.
    fps_samples: VecDeque<u32>,
    /// Total frames ever tracked.
    total_frames: u64,
    /// Last calculated FPS.
    current_fps: u32,
    /// Sample window size for averaging.
    sample_window: usize,
}

impl FpsState {
    fn new() -> Self {
        Self {
            frame_times: VecDeque::with_capacity(120),
            fps_samples: VecDeque::with_capacity(60),
            total_frames: 0,
            current_fps: 0,
            sample_window: 60,
        }
    }

    fn track_frame(&mut self) {
        let now = Instant::now();
        self.frame_times.push_back(now);
        self.total_frames += 1;

        // Remove frames older than 1 second
        let one_second_ago = now - std::time::Duration::from_secs(1);
        while self.frame_times.front().map_or(false, |&t| t < one_second_ago) {
            self.frame_times.pop_front();
        }

        // Calculate current FPS
        self.current_fps = self.frame_times.len() as u32;

        // Add to samples
        self.fps_samples.push_back(self.current_fps);
        if self.fps_samples.len() > self.sample_window {
            self.fps_samples.pop_front();
        }
    }

    fn get_metrics(&self) -> FpsMetrics {
        let fps = self.current_fps;

        let (avg_fps, min_fps, max_fps) = if self.fps_samples.is_empty() {
            (0.0, 0, 0)
        } else {
            let sum: u32 = self.fps_samples.iter().sum();
            let avg = sum as f64 / self.fps_samples.len() as f64;
            let min = *self.fps_samples.iter().min().unwrap_or(&0);
            let max = *self.fps_samples.iter().max().unwrap_or(&0);
            (avg, min, max)
        };

        let status_color = if fps >= 30 {
            FpsColor::Green
        } else if fps >= 15 {
            FpsColor::Yellow
        } else {
            FpsColor::Red
        };

        FpsMetrics {
            fps,
            avg_fps,
            min_fps,
            max_fps,
            total_frames: self.total_frames,
            status_color,
        }
    }
}

// =============================================================================
// Public API
// =============================================================================

/// Track a frame render.
///
/// Call this function once per frame/render to track FPS.
pub fn track_frame() {
    FPS_STATE.with(|state| {
        state.borrow_mut().track_frame();
    });
}

/// Get the current FPS.
pub fn get_fps() -> u32 {
    FPS_STATE.with(|state| state.borrow().current_fps)
}

/// Get detailed FPS metrics.
pub fn get_fps_metrics() -> FpsMetrics {
    FPS_STATE.with(|state| state.borrow().get_metrics())
}

/// Reset the FPS tracker.
pub fn reset_fps() {
    FPS_STATE.with(|state| {
        *state.borrow_mut() = FpsState::new();
    });
}

/// Hook for FPS tracking.
///
/// Automatically calls `track_frame()` and returns current metrics.
///
/// # Returns
///
/// Current FPS metrics including fps, average, min, max, and status color.
///
/// # Example
///
/// ```rust,ignore
/// use tuiuiu::hooks::fps::use_fps;
///
/// let metrics = use_fps();
/// if metrics.fps < 30 {
///     println!("Performance warning!");
/// }
/// ```
pub fn use_fps() -> FpsMetrics {
    // Track this render as a frame
    track_frame();
    get_fps_metrics()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fps_tracking() {
        reset_fps();

        // Track some frames
        for _ in 0..10 {
            track_frame();
        }

        let metrics = get_fps_metrics();
        assert!(metrics.total_frames >= 10);
    }

    #[test]
    fn test_fps_color() {
        assert_eq!(FpsColor::Green.as_str(), "green");
        assert_eq!(FpsColor::Yellow.as_str(), "yellow");
        assert_eq!(FpsColor::Red.as_str(), "red");
    }

    #[test]
    fn test_use_fps() {
        reset_fps();
        let metrics = use_fps();
        assert!(metrics.total_frames >= 1);
    }
}
