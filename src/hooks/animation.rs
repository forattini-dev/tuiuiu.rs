//! Animation Hook
//!
//! Hooks for creating smooth animations and transitions.
//!
//! # Example
//!
//! ```rust,ignore
//! use tuiuiu::hooks::animation::{use_animation, Easing};
//!
//! let anim = use_animation(0.0, 100.0, 1000, Easing::EaseInOut);
//! anim.start();
//!
//! // During animation, anim.value() smoothly transitions from 0 to 100
//! ```

use crate::core::signals::{create_signal, ReadSignal, WriteSignal};

// =============================================================================
// Easing Functions
// =============================================================================

/// Easing function types for animations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
    /// Linear interpolation (no easing).
    Linear,
    /// Ease in (slow start).
    EaseIn,
    /// Ease out (slow end).
    EaseOut,
    /// Ease in and out (slow start and end).
    EaseInOut,
    /// Quadratic ease in.
    EaseInQuad,
    /// Quadratic ease out.
    EaseOutQuad,
    /// Quadratic ease in/out.
    EaseInOutQuad,
    /// Cubic ease in.
    EaseInCubic,
    /// Cubic ease out.
    EaseOutCubic,
    /// Cubic ease in/out.
    EaseInOutCubic,
    /// Elastic bounce (overshoot).
    Elastic,
    /// Bounce at the end.
    Bounce,
}

impl Easing {
    /// Apply the easing function to a progress value (0.0 to 1.0).
    pub fn apply(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => t * (2.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Easing::EaseInQuad => t * t,
            Easing::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Easing::EaseInCubic => t * t * t,
            Easing::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            Easing::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Easing::Elastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let p = 0.3;
                    let s = p / 4.0;
                    (2.0_f64).powf(-10.0 * t) * ((t - s) * std::f64::consts::TAU / p).sin() + 1.0
                }
            }
            Easing::Bounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    let t = t - 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    let t = t - 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
        }
    }
}

impl Default for Easing {
    fn default() -> Self {
        Easing::EaseInOut
    }
}

// =============================================================================
// Animation State
// =============================================================================

/// Animation state enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    /// Animation hasn't started.
    Idle,
    /// Animation is running.
    Running,
    /// Animation is paused.
    Paused,
    /// Animation completed.
    Completed,
}

/// Animation handle for controlling and reading animation state.
#[derive(Clone)]
pub struct AnimationHandle {
    /// Current animated value.
    value: ReadSignal<f64>,
    set_value: WriteSignal<f64>,
    /// Animation state.
    state: ReadSignal<AnimationState>,
    set_state: WriteSignal<AnimationState>,
    /// Animation progress (0.0 to 1.0).
    progress: ReadSignal<f64>,
    set_progress: WriteSignal<f64>,
    /// Configuration.
    from: f64,
    to: f64,
    duration_ms: u64,
    easing: Easing,
}

impl AnimationHandle {
    /// Get the current animated value.
    pub fn value(&self) -> f64 {
        self.value.get()
    }

    /// Get the current state.
    pub fn state(&self) -> AnimationState {
        self.state.get()
    }

    /// Get the progress (0.0 to 1.0).
    pub fn progress(&self) -> f64 {
        self.progress.get()
    }

    /// Check if animation is running.
    pub fn is_running(&self) -> bool {
        self.state.get() == AnimationState::Running
    }

    /// Check if animation is completed.
    pub fn is_completed(&self) -> bool {
        self.state.get() == AnimationState::Completed
    }

    /// Start the animation.
    pub fn start(&self) {
        self.set_state.set(AnimationState::Running);
        self.set_progress.set(0.0);
        self.set_value.set(self.from);
    }

    /// Pause the animation.
    pub fn pause(&self) {
        if self.state.get() == AnimationState::Running {
            self.set_state.set(AnimationState::Paused);
        }
    }

    /// Resume the animation.
    pub fn resume(&self) {
        if self.state.get() == AnimationState::Paused {
            self.set_state.set(AnimationState::Running);
        }
    }

    /// Reset the animation to the beginning.
    pub fn reset(&self) {
        self.set_state.set(AnimationState::Idle);
        self.set_progress.set(0.0);
        self.set_value.set(self.from);
    }

    /// Jump to the end of the animation.
    pub fn finish(&self) {
        self.set_state.set(AnimationState::Completed);
        self.set_progress.set(1.0);
        self.set_value.set(self.to);
    }

    /// Reverse the animation direction.
    pub fn reverse(&self) -> AnimationHandle {
        AnimationHandle {
            value: self.value.clone(),
            set_value: self.set_value.clone(),
            state: self.state.clone(),
            set_state: self.set_state.clone(),
            progress: self.progress.clone(),
            set_progress: self.set_progress.clone(),
            from: self.to,
            to: self.from,
            duration_ms: self.duration_ms,
            easing: self.easing,
        }
    }

    /// Update the animation. Call this each frame.
    pub fn tick(&self, delta_ms: u64) {
        if self.state.get() != AnimationState::Running {
            return;
        }

        let progress = self.progress.get();
        let new_progress = progress + (delta_ms as f64 / self.duration_ms as f64);

        if new_progress >= 1.0 {
            self.set_progress.set(1.0);
            self.set_value.set(self.to);
            self.set_state.set(AnimationState::Completed);
        } else {
            self.set_progress.set(new_progress);
            let eased = self.easing.apply(new_progress);
            let value = self.from + (self.to - self.from) * eased;
            self.set_value.set(value);
        }
    }

    /// Get the value signal for reactive access.
    pub fn signal(&self) -> ReadSignal<f64> {
        self.value.clone()
    }
}

// =============================================================================
// Hooks
// =============================================================================

/// Create an animation from one value to another.
///
/// # Arguments
///
/// * `from` - Starting value
/// * `to` - Ending value
/// * `duration_ms` - Animation duration in milliseconds
/// * `easing` - Easing function to use
///
/// # Returns
///
/// An `AnimationHandle` for controlling the animation.
///
/// # Example
///
/// ```rust,ignore
/// use tuiuiu::hooks::animation::{use_animation, Easing};
///
/// let fade = use_animation(0.0, 1.0, 500, Easing::EaseOut);
/// fade.start();
///
/// // In render loop:
/// let opacity = fade.value();
/// ```
pub fn use_animation(from: f64, to: f64, duration_ms: u64, easing: Easing) -> AnimationHandle {
    let (value, set_value) = create_signal(from);
    let (state, set_state) = create_signal(AnimationState::Idle);
    let (progress, set_progress) = create_signal(0.0);

    AnimationHandle {
        value,
        set_value,
        state,
        set_state,
        progress,
        set_progress,
        from,
        to,
        duration_ms,
        easing,
    }
}

/// Create a simple fade in animation.
pub fn use_fade_in(duration_ms: u64) -> AnimationHandle {
    use_animation(0.0, 1.0, duration_ms, Easing::EaseOut)
}

/// Create a simple fade out animation.
pub fn use_fade_out(duration_ms: u64) -> AnimationHandle {
    use_animation(1.0, 0.0, duration_ms, Easing::EaseIn)
}

/// Create a slide animation.
pub fn use_slide(from: f64, to: f64, duration_ms: u64) -> AnimationHandle {
    use_animation(from, to, duration_ms, Easing::EaseInOut)
}

/// Linear interpolation between two values.
pub fn lerp(from: f64, to: f64, t: f64) -> f64 {
    from + (to - from) * t.clamp(0.0, 1.0)
}

/// Map a value from one range to another.
pub fn map_range(value: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    let t = (value - in_min) / (in_max - in_min);
    lerp(out_min, out_max, t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_linear() {
        assert_eq!(Easing::Linear.apply(0.0), 0.0);
        assert_eq!(Easing::Linear.apply(0.5), 0.5);
        assert_eq!(Easing::Linear.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_bounds() {
        for easing in [
            Easing::Linear,
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::EaseInOut,
        ] {
            assert_eq!(easing.apply(0.0), 0.0);
            let end = easing.apply(1.0);
            assert!(
                (end - 1.0).abs() < 0.001,
                "Easing {:?} end: {}",
                easing,
                end
            );
        }
    }

    #[test]
    fn test_animation_handle() {
        let anim = use_animation(0.0, 100.0, 1000, Easing::Linear);

        assert_eq!(anim.state(), AnimationState::Idle);
        assert_eq!(anim.value(), 0.0);

        anim.start();
        assert_eq!(anim.state(), AnimationState::Running);

        anim.tick(500); // Half way
        assert!((anim.value() - 50.0).abs() < 0.1);

        anim.tick(600); // Past end
        assert_eq!(anim.state(), AnimationState::Completed);
        assert_eq!(anim.value(), 100.0);
    }

    #[test]
    fn test_animation_pause_resume() {
        let anim = use_animation(0.0, 100.0, 1000, Easing::Linear);

        anim.start();
        anim.tick(250);

        anim.pause();
        assert_eq!(anim.state(), AnimationState::Paused);

        let value_when_paused = anim.value();
        anim.tick(250); // Should not advance
        assert_eq!(anim.value(), value_when_paused);

        anim.resume();
        assert_eq!(anim.state(), AnimationState::Running);
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 100.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 100.0, 0.5), 50.0);
        assert_eq!(lerp(0.0, 100.0, 1.0), 100.0);
    }

    #[test]
    fn test_map_range() {
        assert_eq!(map_range(0.5, 0.0, 1.0, 0.0, 100.0), 50.0);
        assert_eq!(map_range(50.0, 0.0, 100.0, 0.0, 1.0), 0.5);
    }
}
