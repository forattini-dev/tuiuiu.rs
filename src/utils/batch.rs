//! Batch Utilities
//!
//! Utilities for batching operations and debouncing updates.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// A batcher that collects items and flushes them periodically or when full.
#[derive(Debug)]
pub struct Batcher<T> {
    /// Items waiting to be flushed
    items: VecDeque<T>,
    /// Maximum batch size
    max_size: usize,
    /// Time of last flush
    last_flush: Instant,
    /// Maximum time between flushes
    max_delay: Duration,
}

impl<T> Batcher<T> {
    /// Create a new batcher with default settings.
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
            max_size: 100,
            last_flush: Instant::now(),
            max_delay: Duration::from_millis(16), // ~60fps
        }
    }

    /// Create a batcher with custom settings.
    pub fn with_config(max_size: usize, max_delay_ms: u64) -> Self {
        Self {
            items: VecDeque::new(),
            max_size,
            last_flush: Instant::now(),
            max_delay: Duration::from_millis(max_delay_ms),
        }
    }

    /// Add an item to the batch.
    pub fn push(&mut self, item: T) {
        self.items.push_back(item);
    }

    /// Check if batch should be flushed.
    pub fn should_flush(&self) -> bool {
        self.items.len() >= self.max_size
            || (!self.items.is_empty() && self.last_flush.elapsed() >= self.max_delay)
    }

    /// Flush all items from the batch.
    pub fn flush(&mut self) -> Vec<T> {
        self.last_flush = Instant::now();
        self.items.drain(..).collect()
    }

    /// Get number of pending items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if batch is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Clear all pending items.
    pub fn clear(&mut self) {
        self.items.clear();
        self.last_flush = Instant::now();
    }
}

impl<T> Default for Batcher<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A rate limiter that allows a certain number of operations per time window.
#[derive(Debug)]
pub struct RateLimiter {
    /// Maximum operations allowed
    max_ops: usize,
    /// Time window
    window: Duration,
    /// Timestamps of recent operations
    timestamps: VecDeque<Instant>,
}

impl RateLimiter {
    /// Create a new rate limiter.
    pub fn new(max_ops: usize, window_ms: u64) -> Self {
        Self {
            max_ops,
            window: Duration::from_millis(window_ms),
            timestamps: VecDeque::with_capacity(max_ops),
        }
    }

    /// Check if an operation is allowed.
    pub fn check(&mut self) -> bool {
        let now = Instant::now();

        // Remove timestamps outside the window
        while let Some(front) = self.timestamps.front() {
            if now.duration_since(*front) >= self.window {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }

        // Check if we're under the limit
        if self.timestamps.len() < self.max_ops {
            self.timestamps.push_back(now);
            true
        } else {
            false
        }
    }

    /// Get remaining operations allowed.
    pub fn remaining(&self) -> usize {
        self.max_ops.saturating_sub(self.timestamps.len())
    }

    /// Reset the rate limiter.
    pub fn reset(&mut self) {
        self.timestamps.clear();
    }
}

/// Coalesces rapid updates, keeping only the latest value.
#[derive(Debug)]
pub struct UpdateCoalescer<T> {
    /// Latest value
    value: Option<T>,
    /// Last update time
    last_update: Instant,
    /// Minimum time between updates
    min_interval: Duration,
}

impl<T> UpdateCoalescer<T> {
    /// Create a new update coalescer.
    pub fn new(min_interval_ms: u64) -> Self {
        Self {
            value: None,
            last_update: Instant::now(),
            min_interval: Duration::from_millis(min_interval_ms),
        }
    }

    /// Set a new value.
    pub fn set(&mut self, value: T) {
        self.value = Some(value);
        self.last_update = Instant::now();
    }

    /// Take the value if enough time has passed.
    pub fn take(&mut self) -> Option<T> {
        if self.value.is_some() && self.last_update.elapsed() >= self.min_interval {
            self.value.take()
        } else {
            None
        }
    }

    /// Force take the value regardless of timing.
    pub fn force_take(&mut self) -> Option<T> {
        self.value.take()
    }

    /// Check if there's a pending value.
    pub fn has_pending(&self) -> bool {
        self.value.is_some()
    }

    /// Check if value is ready to be taken.
    pub fn is_ready(&self) -> bool {
        self.value.is_some() && self.last_update.elapsed() >= self.min_interval
    }
}

impl<T> Default for UpdateCoalescer<T> {
    fn default() -> Self {
        Self::new(16) // ~60fps default
    }
}

/// Accumulator for collecting values over time.
#[derive(Debug)]
pub struct Accumulator<T> {
    /// Accumulated items
    items: Vec<T>,
    /// Maximum capacity
    capacity: usize,
}

impl<T> Accumulator<T> {
    /// Create a new accumulator with unlimited capacity.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            capacity: usize::MAX,
        }
    }

    /// Create an accumulator with a maximum capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity.min(1024)),
            capacity,
        }
    }

    /// Add an item.
    pub fn add(&mut self, item: T) -> bool {
        if self.items.len() < self.capacity {
            self.items.push(item);
            true
        } else {
            false
        }
    }

    /// Take all accumulated items.
    pub fn drain(&mut self) -> Vec<T> {
        std::mem::take(&mut self.items)
    }

    /// Get count of items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Check if at capacity.
    pub fn is_full(&self) -> bool {
        self.items.len() >= self.capacity
    }

    /// Clear all items.
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl<T> Default for Accumulator<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_batcher() {
        let mut batcher = Batcher::with_config(3, 1000);

        batcher.push(1);
        batcher.push(2);
        assert!(!batcher.should_flush());

        batcher.push(3);
        assert!(batcher.should_flush());

        let items = batcher.flush();
        assert_eq!(items, vec![1, 2, 3]);
        assert!(batcher.is_empty());
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(2, 100);

        assert!(limiter.check());
        assert!(limiter.check());
        assert!(!limiter.check());

        assert_eq!(limiter.remaining(), 0);

        // After window expires
        sleep(Duration::from_millis(110));
        assert!(limiter.check());
    }

    #[test]
    fn test_accumulator() {
        let mut acc: Accumulator<i32> = Accumulator::with_capacity(3);

        assert!(acc.add(1));
        assert!(acc.add(2));
        assert!(acc.add(3));
        assert!(!acc.add(4)); // At capacity

        assert!(acc.is_full());

        let items = acc.drain();
        assert_eq!(items, vec![1, 2, 3]);
        assert!(acc.is_empty());
    }
}
