//! Timing Hooks
//!
//! Hooks for scheduling timed operations: intervals and timeouts.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

// =============================================================================
// Interval Hook
// =============================================================================

/// Interval handle for controlling the interval.
#[derive(Clone)]
pub struct IntervalHandle {
    running: Arc<AtomicBool>,
    interval_ms: Arc<AtomicU64>,
    last_tick: Arc<std::sync::Mutex<Instant>>,
    tick_count: Arc<AtomicU64>,
}

impl IntervalHandle {
    /// Check if interval is running.
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    /// Pause the interval.
    pub fn pause(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    /// Resume the interval.
    pub fn resume(&self) {
        self.running.store(true, Ordering::Relaxed);
    }

    /// Toggle running state.
    pub fn toggle(&self) {
        let current = self.running.load(Ordering::Relaxed);
        self.running.store(!current, Ordering::Relaxed);
    }

    /// Stop the interval permanently.
    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    /// Reset the interval.
    pub fn reset(&self) {
        let mut last = self.last_tick.lock().unwrap();
        *last = Instant::now();
        self.tick_count.store(0, Ordering::Relaxed);
    }

    /// Get the number of times the callback has been called.
    pub fn tick_count(&self) -> u64 {
        self.tick_count.load(Ordering::Relaxed)
    }

    /// Set a new interval duration.
    pub fn set_interval(&self, ms: u64) {
        self.interval_ms.store(ms, Ordering::Relaxed);
    }

    /// Get current interval duration in milliseconds.
    pub fn interval_ms(&self) -> u64 {
        self.interval_ms.load(Ordering::Relaxed)
    }

    /// Check if the interval should tick (based on elapsed time).
    /// Call this in your render loop.
    pub fn should_tick(&self) -> bool {
        if !self.is_running() {
            return false;
        }

        let mut last = self.last_tick.lock().unwrap();
        let interval = Duration::from_millis(self.interval_ms.load(Ordering::Relaxed));

        if last.elapsed() >= interval {
            *last = Instant::now();
            self.tick_count.fetch_add(1, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    /// Get time until next tick in milliseconds.
    pub fn time_until_next_tick(&self) -> u64 {
        let last = self.last_tick.lock().unwrap();
        let interval = Duration::from_millis(self.interval_ms.load(Ordering::Relaxed));
        let elapsed = last.elapsed();

        if elapsed >= interval {
            0
        } else {
            (interval - elapsed).as_millis() as u64
        }
    }
}

/// Options for configuring an interval.
#[derive(Debug, Clone)]
pub struct IntervalOptions {
    /// Initial delay before first tick (in ms).
    pub initial_delay: Option<u64>,
    /// Start immediately.
    pub immediate: bool,
    /// Start paused.
    pub paused: bool,
}

impl Default for IntervalOptions {
    fn default() -> Self {
        Self {
            initial_delay: None,
            immediate: false,
            paused: false,
        }
    }
}

/// Create an interval that calls a callback at regular intervals.
///
/// # Arguments
///
/// * `interval_ms` - Interval duration in milliseconds
///
/// # Returns
///
/// Returns an `IntervalHandle` for controlling the interval.
///
/// # Example
///
/// ```rust,ignore
/// let interval = use_interval(1000); // 1 second
///
/// // In your render loop:
/// if interval.should_tick() {
///     // Do something every second
/// }
///
/// // Control the interval
/// interval.pause();
/// interval.resume();
/// interval.stop();
/// ```
pub fn use_interval(interval_ms: u64) -> IntervalHandle {
    use_interval_with_options(interval_ms, IntervalOptions::default())
}

/// Create an interval with options.
pub fn use_interval_with_options(interval_ms: u64, options: IntervalOptions) -> IntervalHandle {
    let last_tick = if let Some(delay) = options.initial_delay {
        // Set last tick to past so first tick happens after delay
        Instant::now() - Duration::from_millis(interval_ms.saturating_sub(delay))
    } else if options.immediate {
        // Set last tick to far past so first tick happens immediately
        Instant::now() - Duration::from_millis(interval_ms)
    } else {
        Instant::now()
    };

    IntervalHandle {
        running: Arc::new(AtomicBool::new(!options.paused)),
        interval_ms: Arc::new(AtomicU64::new(interval_ms)),
        last_tick: Arc::new(std::sync::Mutex::new(last_tick)),
        tick_count: Arc::new(AtomicU64::new(0)),
    }
}

// =============================================================================
// Timeout Hook
// =============================================================================

/// Timeout state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeoutState {
    /// Waiting for timeout.
    Pending,
    /// Timeout has expired.
    Expired,
    /// Timeout was cancelled.
    Cancelled,
}

/// Timeout handle for controlling the timeout.
#[derive(Clone)]
pub struct TimeoutHandle {
    state: Arc<std::sync::Mutex<TimeoutState>>,
    start_time: Arc<std::sync::Mutex<Instant>>,
    duration_ms: Arc<AtomicU64>,
}

impl TimeoutHandle {
    /// Get current state.
    pub fn state(&self) -> TimeoutState {
        *self.state.lock().unwrap()
    }

    /// Check if timeout is pending.
    pub fn is_pending(&self) -> bool {
        self.state() == TimeoutState::Pending
    }

    /// Check if timeout has expired.
    pub fn is_expired(&self) -> bool {
        self.state() == TimeoutState::Expired
    }

    /// Check if timeout was cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.state() == TimeoutState::Cancelled
    }

    /// Cancel the timeout.
    pub fn cancel(&self) {
        let mut state = self.state.lock().unwrap();
        if *state == TimeoutState::Pending {
            *state = TimeoutState::Cancelled;
        }
    }

    /// Reset the timeout.
    pub fn reset(&self) {
        *self.state.lock().unwrap() = TimeoutState::Pending;
        *self.start_time.lock().unwrap() = Instant::now();
    }

    /// Reset with a new duration.
    pub fn reset_with_duration(&self, ms: u64) {
        self.duration_ms.store(ms, Ordering::Relaxed);
        self.reset();
    }

    /// Get remaining time in milliseconds.
    pub fn remaining_ms(&self) -> u64 {
        if self.state() != TimeoutState::Pending {
            return 0;
        }

        let start = *self.start_time.lock().unwrap();
        let duration = Duration::from_millis(self.duration_ms.load(Ordering::Relaxed));
        let elapsed = start.elapsed();

        if elapsed >= duration {
            0
        } else {
            (duration - elapsed).as_millis() as u64
        }
    }

    /// Get elapsed time in milliseconds.
    pub fn elapsed_ms(&self) -> u64 {
        let start = *self.start_time.lock().unwrap();
        start.elapsed().as_millis() as u64
    }

    /// Get progress as a float (0.0 to 1.0).
    pub fn progress(&self) -> f64 {
        let duration = self.duration_ms.load(Ordering::Relaxed) as f64;
        if duration == 0.0 {
            return 1.0;
        }
        (self.elapsed_ms() as f64 / duration).min(1.0)
    }

    /// Check if timeout should expire and update state.
    /// Call this in your render loop.
    pub fn check(&self) -> bool {
        let mut state = self.state.lock().unwrap();
        if *state != TimeoutState::Pending {
            return false;
        }

        let start = *self.start_time.lock().unwrap();
        let duration = Duration::from_millis(self.duration_ms.load(Ordering::Relaxed));

        if start.elapsed() >= duration {
            *state = TimeoutState::Expired;
            true
        } else {
            false
        }
    }
}

/// Create a timeout that expires after a duration.
///
/// # Arguments
///
/// * `ms` - Duration in milliseconds
///
/// # Returns
///
/// Returns a `TimeoutHandle` for controlling the timeout.
///
/// # Example
///
/// ```rust,ignore
/// let timeout = use_timeout(5000); // 5 seconds
///
/// // In your render loop:
/// if timeout.check() {
///     // Timeout expired!
/// }
///
/// // Control the timeout
/// timeout.cancel();
/// timeout.reset();
/// ```
pub fn use_timeout(ms: u64) -> TimeoutHandle {
    TimeoutHandle {
        state: Arc::new(std::sync::Mutex::new(TimeoutState::Pending)),
        start_time: Arc::new(std::sync::Mutex::new(Instant::now())),
        duration_ms: Arc::new(AtomicU64::new(ms)),
    }
}

/// Create a timeout that starts in cancelled state (won't expire until reset).
pub fn use_timeout_paused(ms: u64) -> TimeoutHandle {
    TimeoutHandle {
        state: Arc::new(std::sync::Mutex::new(TimeoutState::Cancelled)),
        start_time: Arc::new(std::sync::Mutex::new(Instant::now())),
        duration_ms: Arc::new(AtomicU64::new(ms)),
    }
}

// =============================================================================
// Debounce Hook
// =============================================================================

/// Debounce handle for debouncing function calls.
#[derive(Clone)]
pub struct DebounceHandle {
    last_call: Arc<std::sync::Mutex<Option<Instant>>>,
    delay_ms: Arc<AtomicU64>,
}

impl DebounceHandle {
    /// Call the debounced function.
    /// Returns true if the delay has passed and the function should be called.
    pub fn call(&self) -> bool {
        let mut last = self.last_call.lock().unwrap();
        let delay = Duration::from_millis(self.delay_ms.load(Ordering::Relaxed));

        if let Some(last_time) = *last {
            if last_time.elapsed() >= delay {
                *last = Some(Instant::now());
                true
            } else {
                *last = Some(Instant::now());
                false
            }
        } else {
            *last = Some(Instant::now());
            false
        }
    }

    /// Check if enough time has passed since the last call.
    pub fn is_ready(&self) -> bool {
        let last = self.last_call.lock().unwrap();
        let delay = Duration::from_millis(self.delay_ms.load(Ordering::Relaxed));

        match *last {
            Some(last_time) => last_time.elapsed() >= delay,
            None => true,
        }
    }

    /// Force immediate call on next check.
    pub fn flush(&self) {
        *self.last_call.lock().unwrap() = None;
    }

    /// Update delay.
    pub fn set_delay(&self, ms: u64) {
        self.delay_ms.store(ms, Ordering::Relaxed);
    }
}

/// Create a debounce hook.
///
/// Debouncing delays the execution until after a period of inactivity.
pub fn use_debounce(delay_ms: u64) -> DebounceHandle {
    DebounceHandle {
        last_call: Arc::new(std::sync::Mutex::new(None)),
        delay_ms: Arc::new(AtomicU64::new(delay_ms)),
    }
}

// =============================================================================
// Throttle Hook
// =============================================================================

/// Throttle handle for throttling function calls.
#[derive(Clone)]
pub struct ThrottleHandle {
    last_call: Arc<std::sync::Mutex<Option<Instant>>>,
    interval_ms: Arc<AtomicU64>,
}

impl ThrottleHandle {
    /// Call the throttled function.
    /// Returns true if enough time has passed since the last call.
    pub fn call(&self) -> bool {
        let mut last = self.last_call.lock().unwrap();
        let interval = Duration::from_millis(self.interval_ms.load(Ordering::Relaxed));

        match *last {
            Some(last_time) if last_time.elapsed() < interval => false,
            _ => {
                *last = Some(Instant::now());
                true
            }
        }
    }

    /// Check if a call would be allowed.
    pub fn would_call(&self) -> bool {
        let last = self.last_call.lock().unwrap();
        let interval = Duration::from_millis(self.interval_ms.load(Ordering::Relaxed));

        match *last {
            Some(last_time) => last_time.elapsed() >= interval,
            None => true,
        }
    }

    /// Get time until next call is allowed.
    pub fn time_until_next(&self) -> u64 {
        let last = self.last_call.lock().unwrap();
        let interval = Duration::from_millis(self.interval_ms.load(Ordering::Relaxed));

        match *last {
            Some(last_time) => {
                let elapsed = last_time.elapsed();
                if elapsed >= interval {
                    0
                } else {
                    (interval - elapsed).as_millis() as u64
                }
            }
            None => 0,
        }
    }

    /// Reset the throttle.
    pub fn reset(&self) {
        *self.last_call.lock().unwrap() = None;
    }

    /// Update interval.
    pub fn set_interval(&self, ms: u64) {
        self.interval_ms.store(ms, Ordering::Relaxed);
    }
}

/// Create a throttle hook.
///
/// Throttling limits the rate of function calls to at most once per interval.
pub fn use_throttle(interval_ms: u64) -> ThrottleHandle {
    ThrottleHandle {
        last_call: Arc::new(std::sync::Mutex::new(None)),
        interval_ms: Arc::new(AtomicU64::new(interval_ms)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_interval() {
        let interval = use_interval(10);

        // Should not tick immediately
        assert!(!interval.should_tick());

        // Wait and check
        thread::sleep(Duration::from_millis(15));
        assert!(interval.should_tick());
        assert_eq!(interval.tick_count(), 1);
    }

    #[test]
    fn test_timeout() {
        let timeout = use_timeout(10);

        // Should be pending
        assert!(timeout.is_pending());
        assert!(!timeout.check());

        // Wait and check
        thread::sleep(Duration::from_millis(15));
        assert!(timeout.check());
        assert!(timeout.is_expired());
    }

    #[test]
    fn test_debounce() {
        let debounce = use_debounce(10);

        // First call should not trigger
        assert!(!debounce.call());

        // Rapid calls should not trigger
        assert!(!debounce.call());
        assert!(!debounce.call());

        // Wait and check
        thread::sleep(Duration::from_millis(15));
        assert!(debounce.is_ready());
    }

    #[test]
    fn test_throttle() {
        let throttle = use_throttle(10);

        // First call should succeed
        assert!(throttle.call());

        // Immediate second call should fail
        assert!(!throttle.call());

        // Wait and check
        thread::sleep(Duration::from_millis(15));
        assert!(throttle.call());
    }
}
