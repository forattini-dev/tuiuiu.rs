//! Animation System
//!
//! Easing functions, transitions, and spring physics.

/// Easing function type.
pub type EasingFunction = fn(f64) -> f64;

/// Linear easing.
pub fn linear(t: f64) -> f64 { t }

/// Ease in quad.
pub fn ease_in_quad(t: f64) -> f64 { t * t }

/// Ease out quad.
pub fn ease_out_quad(t: f64) -> f64 { t * (2.0 - t) }

/// Ease in out quad.
pub fn ease_in_out_quad(t: f64) -> f64 {
    if t < 0.5 { 2.0 * t * t } else { -1.0 + (4.0 - 2.0 * t) * t }
}

/// Ease in cubic.
pub fn ease_in_cubic(t: f64) -> f64 { t * t * t }

/// Ease out cubic.
pub fn ease_out_cubic(t: f64) -> f64 { 
    let t = t - 1.0;
    t * t * t + 1.0
}

/// Linear interpolation.
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Interpolate between two colors.
pub fn lerp_color(a: (u8, u8, u8), b: (u8, u8, u8), t: f64) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    (
        lerp(a.0 as f64, b.0 as f64, t) as u8,
        lerp(a.1 as f64, b.1 as f64, t) as u8,
        lerp(a.2 as f64, b.2 as f64, t) as u8,
    )
}

/// Animation options.
#[derive(Debug, Clone)]
pub struct AnimationOptions {
    pub duration_ms: u64,
    pub easing: EasingFunction,
    pub delay_ms: u64,
}

impl Default for AnimationOptions {
    fn default() -> Self {
        Self {
            duration_ms: 300,
            easing: ease_out_quad,
            delay_ms: 0,
        }
    }
}

/// Spring options.
#[derive(Debug, Clone)]
pub struct SpringOptions {
    pub stiffness: f64,
    pub damping: f64,
    pub mass: f64,
}

impl Default for SpringOptions {
    fn default() -> Self {
        Self {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
        }
    }
}

/// Transition state.
#[derive(Debug, Clone)]
pub struct TransitionState {
    pub from: f64,
    pub to: f64,
    pub current: f64,
    pub progress: f64,
}

/// Named easing functions.
pub fn get_easing(name: &str) -> EasingFunction {
    match name {
        "linear" => linear,
        "easeInQuad" => ease_in_quad,
        "easeOutQuad" => ease_out_quad,
        "easeInOutQuad" => ease_in_out_quad,
        "easeInCubic" => ease_in_cubic,
        "easeOutCubic" => ease_out_cubic,
        _ => linear,
    }
}
