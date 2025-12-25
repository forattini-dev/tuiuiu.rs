//! Global Tick System
//!
//! Provides synchronized animation timing across all components.

use std::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};

// =============================================================================
// Tick State
// =============================================================================

static TICK_VALUE: AtomicU64 = AtomicU64::new(0);
static TICK_RUNNING: AtomicBool = AtomicBool::new(false);
static TICK_PAUSED: AtomicBool = AtomicBool::new(false);

thread_local! {
    static TICK_RATE: Cell<u32> = const { Cell::new(60) }; // FPS
    static TICK_START: RefCell<Option<Instant>> = const { RefCell::new(None) };
    static TICK_SUBSCRIBERS: RefCell<Vec<Box<dyn Fn(u64)>>> = const { RefCell::new(Vec::new()) };
}

// =============================================================================
// Tick Control
// =============================================================================

/// Start the global tick.
pub fn start_tick() {
    TICK_RUNNING.store(true, Ordering::SeqCst);
    TICK_PAUSED.store(false, Ordering::SeqCst);
    TICK_START.with(|s| *s.borrow_mut() = Some(Instant::now()));
}

/// Stop the global tick.
pub fn stop_tick() {
    TICK_RUNNING.store(false, Ordering::SeqCst);
}

/// Pause the tick.
pub fn pause_tick() {
    TICK_PAUSED.store(true, Ordering::SeqCst);
}

/// Resume the tick.
pub fn resume_tick() {
    TICK_PAUSED.store(false, Ordering::SeqCst);
}

/// Reset the tick counter.
pub fn reset_tick() {
    TICK_VALUE.store(0, Ordering::SeqCst);
    TICK_START.with(|s| *s.borrow_mut() = Some(Instant::now()));
}

/// Set the tick rate (FPS).
pub fn set_tick_rate(fps: u32) {
    TICK_RATE.with(|r| r.set(fps));
}

/// Get the tick rate.
pub fn get_tick_rate() -> u32 {
    TICK_RATE.with(|r| r.get())
}

/// Check if the tick is running.
pub fn is_tick_running() -> bool {
    TICK_RUNNING.load(Ordering::SeqCst) && !TICK_PAUSED.load(Ordering::SeqCst)
}

// =============================================================================
// Tick Value
// =============================================================================

/// Get the current tick value.
pub fn get_tick() -> u64 {
    TICK_VALUE.load(Ordering::SeqCst)
}

/// Alias for get_tick.
pub fn tick() -> u64 {
    get_tick()
}

/// Advance the tick by one (for testing/manual control).
pub fn advance_tick() {
    let new = TICK_VALUE.fetch_add(1, Ordering::SeqCst) + 1;
    notify_subscribers(new);
}

/// Set the tick value directly (for testing).
pub fn set_tick_value(value: u64) {
    TICK_VALUE.store(value, Ordering::SeqCst);
    notify_subscribers(value);
}

fn notify_subscribers(tick: u64) {
    TICK_SUBSCRIBERS.with(|subs| {
        for sub in subs.borrow().iter() {
            sub(tick);
        }
    });
}

// =============================================================================
// Subscriptions
// =============================================================================

/// Subscribe to tick updates.
pub fn on_tick<F: Fn(u64) + 'static>(callback: F) {
    TICK_SUBSCRIBERS.with(|subs| {
        subs.borrow_mut().push(Box::new(callback));
    });
}

// =============================================================================
// Animation Utilities
// =============================================================================

/// Get the current frame for an animation.
pub fn get_frame(frames: usize) -> usize {
    (tick() as usize) % frames
}

/// Get an item from a slice based on current tick.
pub fn get_frame_item<T: Clone>(items: &[T]) -> Option<T> {
    if items.is_empty() {
        None
    } else {
        Some(items[get_frame(items.len())].clone())
    }
}

/// Oscillate between 0.0 and 1.0.
pub fn oscillate(speed: f64) -> f64 {
    let t = (tick() as f64) * speed;
    (t.sin() + 1.0) / 2.0
}

/// Get elapsed seconds since tick start.
pub fn get_elapsed_seconds() -> f64 {
    TICK_START.with(|s| {
        s.borrow()
            .map(|start| start.elapsed().as_secs_f64())
            .unwrap_or(0.0)
    })
}

/// Check if we're on every Nth tick.
pub fn every_n_ticks(n: u64) -> bool {
    tick() % n == 0
}

// =============================================================================
// FPS Tracking
// =============================================================================

thread_local! {
    static FPS_FRAMES: RefCell<Vec<Instant>> = const { RefCell::new(Vec::new()) };
    static FPS_VALUE: Cell<f64> = const { Cell::new(0.0) };
}

/// FPS metrics.
#[derive(Debug, Clone, Default)]
pub struct FpsMetrics {
    /// Current FPS
    pub current: f64,
    /// Average FPS
    pub average: f64,
    /// Minimum FPS
    pub min: f64,
    /// Maximum FPS
    pub max: f64,
    /// Frame time in ms
    pub frame_time_ms: f64,
}

/// Track a frame for FPS calculation.
pub fn track_frame() {
    let now = Instant::now();

    FPS_FRAMES.with(|frames| {
        let mut frames = frames.borrow_mut();
        frames.push(now);

        // Keep last second of frames
        let cutoff = now - Duration::from_secs(1);
        frames.retain(|&t| t > cutoff);

        // Calculate FPS
        let fps = frames.len() as f64;
        FPS_VALUE.set(fps);
    });
}

/// Get current FPS.
pub fn get_fps() -> f64 {
    FPS_VALUE.get()
}

/// Get FPS metrics.
pub fn get_fps_metrics() -> FpsMetrics {
    let current = get_fps();
    FpsMetrics {
        current,
        average: current,
        min: current,
        max: current,
        frame_time_ms: if current > 0.0 { 1000.0 / current } else { 0.0 },
    }
}

/// Reset FPS tracking.
pub fn reset_fps() {
    FPS_FRAMES.with(|f| f.borrow_mut().clear());
    FPS_VALUE.set(0.0);
}

/// Get a color based on FPS (green = good, red = bad).
pub fn get_fps_color(fps: f64) -> &'static str {
    if fps >= 55.0 {
        "green"
    } else if fps >= 30.0 {
        "yellow"
    } else {
        "red"
    }
}

// =============================================================================
// Tick Signal
// =============================================================================

/// A signal-like interface for tick.
pub struct Tick;

impl Tick {
    /// Get current tick value.
    pub fn get(&self) -> u64 {
        tick()
    }

    /// Get current frame for animation.
    pub fn frame(&self, count: usize) -> usize {
        get_frame(count)
    }

    /// Get oscillating value.
    pub fn oscillate(&self, speed: f64) -> f64 {
        oscillate(speed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_basic() {
        reset_tick();
        assert_eq!(tick(), 0);

        advance_tick();
        assert_eq!(tick(), 1);

        advance_tick();
        assert_eq!(tick(), 2);
    }

    #[test]
    fn test_get_frame() {
        set_tick_value(0);
        assert_eq!(get_frame(4), 0);

        set_tick_value(3);
        assert_eq!(get_frame(4), 3);

        set_tick_value(4);
        assert_eq!(get_frame(4), 0);
    }

    #[test]
    fn test_every_n_ticks() {
        set_tick_value(0);
        assert!(every_n_ticks(5));

        set_tick_value(5);
        assert!(every_n_ticks(5));

        set_tick_value(3);
        assert!(!every_n_ticks(5));
    }
}
