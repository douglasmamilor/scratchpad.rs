/// Linear interpolation - no easing applied
/// 
/// Returns `t` unchanged, providing constant velocity animation.
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::linear;
/// 
/// assert_eq!(linear(0.0), 0.0);
/// assert_eq!(linear(0.5), 0.5);
/// assert_eq!(linear(1.0), 1.0);
/// ```
pub fn linear(t: f32) -> f32 {
    t
}

/// Quadratic ease-in - starts slow, accelerates
/// 
/// Provides a gentle acceleration curve. Good for objects starting from rest.
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_quad;
/// 
/// assert_eq!(ease_in_quad(0.0), 0.0);
/// assert!(ease_in_quad(0.5) < 0.5); // Slower than linear
/// assert_eq!(ease_in_quad(1.0), 1.0);
/// ```
pub fn ease_in_quad(t: f32) -> f32 {
    t * t
}

/// Quadratic ease-out - starts fast, decelerates
/// 
/// Provides a gentle deceleration curve. Good for objects coming to rest.
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_out_quad;
/// 
/// assert_eq!(ease_out_quad(0.0), 0.0);
/// assert!(ease_out_quad(0.5) > 0.5); // Faster than linear
/// assert_eq!(ease_out_quad(1.0), 1.0);
/// ```
pub fn ease_out_quad(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

/// Quadratic ease-in-out - slow start, fast middle, slow end
/// 
/// Combines ease-in and ease-out for smooth acceleration and deceleration.
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_out_quad;
/// 
/// assert_eq!(ease_in_out_quad(0.0), 0.0);
/// assert_eq!(ease_in_out_quad(1.0), 1.0);
/// ```
pub fn ease_in_out_quad(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

/// Cubic ease-in - stronger acceleration than quadratic
/// 
/// More pronounced slow start than ease-in-quad.
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::{ease_in_quad, ease_in_cubic};
/// 
/// assert_eq!(ease_in_cubic(0.0), 0.0);
/// assert!(ease_in_cubic(0.5) < ease_in_quad(0.5)); // Even slower
/// assert_eq!(ease_in_cubic(1.0), 1.0);
/// ```
pub fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}

/// Cubic ease-out - stronger deceleration than quadratic
/// 
/// More pronounced slow end than ease-out-quad.
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::{ease_out_quad, ease_out_cubic};
/// 
/// assert_eq!(ease_out_cubic(0.0), 0.0);
/// assert!(ease_out_cubic(0.5) > ease_out_quad(0.5)); // Even faster mid-point
/// assert_eq!(ease_out_cubic(1.0), 1.0);
/// ```
pub fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

/// Cubic ease-in-out - smooth acceleration and deceleration
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_out_cubic;
/// 
/// assert_eq!(ease_in_out_cubic(0.0), 0.0);
/// assert_eq!(ease_in_out_cubic(1.0), 1.0);
/// ```
pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

/// Quartic ease-in - very strong acceleration
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_quart;
/// 
/// assert_eq!(ease_in_quart(0.0), 0.0);
/// assert_eq!(ease_in_quart(1.0), 1.0);
/// ```
pub fn ease_in_quart(t: f32) -> f32 {
    t * t * t * t
}

/// Quartic ease-out - very strong deceleration
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_out_quart;
/// 
/// assert_eq!(ease_out_quart(0.0), 0.0);
/// assert_eq!(ease_out_quart(1.0), 1.0);
/// ```
pub fn ease_out_quart(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(4)
}

/// Quartic ease-in-out
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_out_quart;
/// 
/// assert_eq!(ease_in_out_quart(0.0), 0.0);
/// assert_eq!(ease_in_out_quart(1.0), 1.0);
/// ```
pub fn ease_in_out_quart(t: f32) -> f32 {
    if t < 0.5 {
        8.0 * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
    }
}

/// Quintic ease-in - extremely strong acceleration
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_quint;
/// 
/// assert_eq!(ease_in_quint(0.0), 0.0);
/// assert_eq!(ease_in_quint(1.0), 1.0);
/// ```
pub fn ease_in_quint(t: f32) -> f32 {
    t * t * t * t * t
}

/// Quintic ease-out - extremely strong deceleration
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_out_quint;
/// 
/// assert_eq!(ease_out_quint(0.0), 0.0);
/// assert_eq!(ease_out_quint(1.0), 1.0);
/// ```
pub fn ease_out_quint(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(5)
}

/// Quintic ease-in-out
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_out_quint;
/// 
/// assert_eq!(ease_in_out_quint(0.0), 0.0);
/// assert_eq!(ease_in_out_quint(1.0), 1.0);
/// ```
pub fn ease_in_out_quint(t: f32) -> f32 {
    if t < 0.5 {
        16.0 * t * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
    }
}

/// Sine ease-in - smooth sinusoidal acceleration
/// 
/// Uses a cosine curve for natural-feeling motion.
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_sine;
/// 
/// assert_eq!(ease_in_sine(0.0), 0.0);
/// assert_eq!(ease_in_sine(1.0), 1.0);
/// ```
pub fn ease_in_sine(t: f32) -> f32 {
    1.0 - (t * std::f32::consts::PI / 2.0).cos()
}

/// Sine ease-out - smooth sinusoidal deceleration
/// 
/// Uses a sine curve for natural-feeling motion.
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_out_sine;
/// 
/// assert_eq!(ease_out_sine(0.0), 0.0);
/// assert_eq!(ease_out_sine(1.0), 1.0);
/// ```
pub fn ease_out_sine(t: f32) -> f32 {
    (t * std::f32::consts::PI / 2.0).sin()
}

/// Sine ease-in-out - smooth sinusoidal curve
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_out_sine;
/// 
/// assert_eq!(ease_in_out_sine(0.0), 0.0);
/// assert_eq!(ease_in_out_sine(1.0), 1.0);
/// ```
pub fn ease_in_out_sine(t: f32) -> f32 {
    -(std::f32::consts::PI * t).cos() / 2.0 + 0.5
}

/// Exponential ease-in - exponential acceleration
/// 
/// Creates a very dramatic slow start with rapid acceleration.
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_expo;
/// 
/// assert_eq!(ease_in_expo(0.0), 0.0);
/// assert_eq!(ease_in_expo(1.0), 1.0);
/// ```
pub fn ease_in_expo(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else {
        2.0_f32.powf(10.0 * (t - 1.0))
    }
}

/// Exponential ease-out - exponential deceleration
/// 
/// Creates rapid deceleration with a dramatic slow end.
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_out_expo;
/// 
/// assert_eq!(ease_out_expo(0.0), 0.0);
/// assert_eq!(ease_out_expo(1.0), 1.0);
/// ```
pub fn ease_out_expo(t: f32) -> f32 {
    if t == 1.0 {
        1.0
    } else {
        1.0 - 2.0_f32.powf(-10.0 * t)
    }
}

/// Exponential ease-in-out
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_out_expo;
/// 
/// assert_eq!(ease_in_out_expo(0.0), 0.0);
/// assert_eq!(ease_in_out_expo(1.0), 1.0);
/// ```
pub fn ease_in_out_expo(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else if t < 0.5 {
        2.0_f32.powf(20.0 * t - 10.0) / 2.0
    } else {
        (2.0 - 2.0_f32.powf(-20.0 * t + 10.0)) / 2.0
    }
}

/// Circular ease-in - circular curve acceleration
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_circ;
/// 
/// assert_eq!(ease_in_circ(0.0), 0.0);
/// assert_eq!(ease_in_circ(1.0), 1.0);
/// ```
pub fn ease_in_circ(t: f32) -> f32 {
    1.0 - (1.0 - t * t).sqrt()
}

/// Circular ease-out - circular curve deceleration
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_out_circ;
/// 
/// assert_eq!(ease_out_circ(0.0), 0.0);
/// assert_eq!(ease_out_circ(1.0), 1.0);
/// ```
pub fn ease_out_circ(t: f32) -> f32 {
    (1.0 - (t - 1.0) * (t - 1.0)).sqrt()
}

/// Circular ease-in-out
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_out_circ;
/// 
/// assert_eq!(ease_in_out_circ(0.0), 0.0);
/// assert_eq!(ease_in_out_circ(1.0), 1.0);
/// ```
pub fn ease_in_out_circ(t: f32) -> f32 {
    if t < 0.5 {
        (1.0 - (1.0 - (2.0 * t) * (2.0 * t)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * t + 2.0) * (-2.0 * t + 2.0)).sqrt() + 1.0) / 2.0
    }
}

/// Back ease-in - overshoots backward before moving forward
/// 
/// Creates a slight backward motion before accelerating forward, like a rubber band.
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_back;
/// 
/// assert_eq!(ease_in_back(0.0), 0.0);
/// assert!(ease_in_back(0.1) < 0.0); // Negative value (overshoot)
/// assert_eq!(ease_in_back(1.0), 1.0);
/// ```
pub fn ease_in_back(t: f32) -> f32 {
    const C1: f32 = 1.70158;
    const C3: f32 = C1 + 1.0;
    C3 * t * t * t - C1 * t * t
}

/// Back ease-out - overshoots forward before settling
/// 
/// Creates a slight forward overshoot before settling, like a rubber band.
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_out_back;
/// 
/// assert_eq!(ease_out_back(0.0), 0.0);
/// assert!(ease_out_back(0.9) > 1.0); // Overshoots past 1.0
/// assert_eq!(ease_out_back(1.0), 1.0);
/// ```
pub fn ease_out_back(t: f32) -> f32 {
    const C1: f32 = 1.70158;
    const C3: f32 = C1 + 1.0;
    1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
}

/// Back ease-in-out - overshoots at both ends
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_out_back;
/// 
/// assert_eq!(ease_in_out_back(0.0), 0.0);
/// assert_eq!(ease_in_out_back(1.0), 1.0);
/// ```
pub fn ease_in_out_back(t: f32) -> f32 {
    const C1: f32 = 1.70158;
    const C2: f32 = C1 * 1.525;
    if t < 0.5 {
        ((2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
    } else {
        ((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (t * 2.0 - 2.0) + C2) + 2.0) / 2.0
    }
}

/// Elastic ease-in - bouncy spring-like motion at start
/// 
/// Creates an elastic, bouncy effect with oscillations at the beginning.
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_elastic;
/// 
/// assert_eq!(ease_in_elastic(0.0), 0.0);
/// assert_eq!(ease_in_elastic(1.0), 1.0);
/// ```
pub fn ease_in_elastic(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        -(2.0_f32.powf(10.0 * (t - 1.0))) * ((t - 1.0 - 0.075) * (2.0 * std::f32::consts::PI) / 0.3).sin()
    }
}

/// Elastic ease-out - bouncy spring-like motion at end
/// 
/// Creates an elastic, bouncy effect with oscillations at the end.
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_out_elastic;
/// 
/// assert_eq!(ease_out_elastic(0.0), 0.0);
/// assert_eq!(ease_out_elastic(1.0), 1.0);
/// ```
pub fn ease_out_elastic(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        2.0_f32.powf(-10.0 * t) * ((t - 0.075) * (2.0 * std::f32::consts::PI) / 0.3).sin() + 1.0
    }
}

/// Elastic ease-in-out - bouncy spring-like motion
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_out_elastic;
/// 
/// assert_eq!(ease_in_out_elastic(0.0), 0.0);
/// assert_eq!(ease_in_out_elastic(1.0), 1.0);
/// ```
pub fn ease_in_out_elastic(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else if t < 0.5 {
        -(2.0_f32.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * (2.0 * std::f32::consts::PI) / 4.5).sin()) / 2.0
    } else {
        (2.0_f32.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * (2.0 * std::f32::consts::PI) / 4.5).sin()) / 2.0 + 1.0
    }
}

/// Bounce ease-in - bounces backward at start
/// 
/// Creates a bouncing effect that starts with backward motion.
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_bounce;
/// 
/// assert_eq!(ease_in_bounce(0.0), 0.0);
/// assert_eq!(ease_in_bounce(1.0), 1.0);
/// ```
pub fn ease_in_bounce(t: f32) -> f32 {
    1.0 - ease_out_bounce(1.0 - t)
}

/// Bounce ease-out - bounces at end
/// 
/// Creates a bouncing effect with multiple bounces as it settles.
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_out_bounce;
/// 
/// assert_eq!(ease_out_bounce(0.0), 0.0);
/// assert_eq!(ease_out_bounce(1.0), 1.0);
/// ```
pub fn ease_out_bounce(t: f32) -> f32 {
    const N1: f32 = 7.5625;
    const D1: f32 = 2.75;
    
    if t < 1.0 / D1 {
        N1 * t * t
    } else if t < 2.0 / D1 {
        let t = t - 1.5 / D1;
        N1 * t * t + 0.75
    } else if t < 2.5 / D1 {
        let t = t - 2.25 / D1;
        N1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / D1;
        N1 * t * t + 0.984375
    }
}

/// Bounce ease-in-out - bounces at both ends
/// 
/// # Examples
/// 
/// ```
/// use scratchpad_rs::animation::ease_in_out_bounce;
/// 
/// assert_eq!(ease_in_out_bounce(0.0), 0.0);
/// assert_eq!(ease_in_out_bounce(1.0), 1.0);
/// ```
pub fn ease_in_out_bounce(t: f32) -> f32 {
    if t < 0.5 {
        (1.0 - ease_out_bounce(1.0 - 2.0 * t)) / 2.0
    } else {
        (1.0 + ease_out_bounce(2.0 * t - 1.0)) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-5;

    // Helper to test boundary conditions
    fn test_boundaries<F>(func: F, name: &str)
    where
        F: Fn(f32) -> f32,
    {
        assert_eq!(func(0.0), 0.0, "{} should return 0.0 at t=0.0", name);
        assert_eq!(func(1.0), 1.0, "{} should return 1.0 at t=1.0", name);
    }

    // Helper to test monotonicity (should be non-decreasing)
    fn test_monotonic<F>(func: F, name: &str)
    where
        F: Fn(f32) -> f32,
    {
        let mut prev = func(0.0);
        for i in 1..=100 {
            let t = i as f32 / 100.0;
            let curr = func(t);
            assert!(
                curr >= prev - EPSILON,
                "{} should be monotonic: failed at t={} (prev={}, curr={})",
                name,
                t,
                prev,
                curr
            );
            prev = curr;
        }
    }

    // Helper to test ease-in property (slower than linear at midpoint)
    fn test_ease_in<F>(func: F, name: &str)
    where
        F: Fn(f32) -> f32,
    {
        let mid = func(0.5);
        assert!(
            mid < 0.5,
            "{} should be ease-in (slower than linear at midpoint): got {}",
            name,
            mid
        );
    }

    // Helper to test ease-out property (faster than linear at midpoint)
    fn test_ease_out<F>(func: F, name: &str)
    where
        F: Fn(f32) -> f32,
    {
        let mid = func(0.5);
        assert!(
            mid > 0.5,
            "{} should be ease-out (faster than linear at midpoint): got {}",
            name,
            mid
        );
    }

    // Helper to test ease-in-out property (symmetric around 0.5)
    fn test_ease_in_out<F>(func: F, name: &str)
    where
        F: Fn(f32) -> f32,
    {
        // Should be symmetric: f(0.5 - t) + f(0.5 + t) ≈ 1.0
        for i in 1..=10 {
            let offset = i as f32 / 20.0;
            let left = func(0.5 - offset);
            let right = func(0.5 + offset);
            assert!(
                (left + right - 1.0).abs() < EPSILON,
                "{} should be symmetric around 0.5: f({}) + f({}) should be 1.0, got {}",
                name,
                0.5 - offset,
                0.5 + offset,
                left + right
            );
        }
    }

    #[test]
    fn test_linear() {
        test_boundaries(linear, "linear");
        assert_eq!(linear(0.5), 0.5);
        test_monotonic(linear, "linear");
    }

    #[test]
    fn test_ease_in_quad() {
        test_boundaries(ease_in_quad, "ease_in_quad");
        test_ease_in(ease_in_quad, "ease_in_quad");
        test_monotonic(ease_in_quad, "ease_in_quad");
        assert!((ease_in_quad(0.5) - 0.25).abs() < EPSILON);
    }

    #[test]
    fn test_ease_out_quad() {
        test_boundaries(ease_out_quad, "ease_out_quad");
        test_ease_out(ease_out_quad, "ease_out_quad");
        test_monotonic(ease_out_quad, "ease_out_quad");
    }

    #[test]
    fn test_ease_in_out_quad() {
        test_boundaries(ease_in_out_quad, "ease_in_out_quad");
        test_ease_in_out(ease_in_out_quad, "ease_in_out_quad");
        test_monotonic(ease_in_out_quad, "ease_in_out_quad");
    }

    #[test]
    fn test_ease_in_cubic() {
        test_boundaries(ease_in_cubic, "ease_in_cubic");
        test_ease_in(ease_in_cubic, "ease_in_cubic");
        test_monotonic(ease_in_cubic, "ease_in_cubic");
        assert!(ease_in_cubic(0.5) < ease_in_quad(0.5)); // Stronger than quad
    }

    #[test]
    fn test_ease_out_cubic() {
        test_boundaries(ease_out_cubic, "ease_out_cubic");
        test_ease_out(ease_out_cubic, "ease_out_cubic");
        test_monotonic(ease_out_cubic, "ease_out_cubic");
        assert!(ease_out_cubic(0.5) > ease_out_quad(0.5)); // Stronger than quad
    }

    #[test]
    fn test_ease_in_out_cubic() {
        test_boundaries(ease_in_out_cubic, "ease_in_out_cubic");
        test_ease_in_out(ease_in_out_cubic, "ease_in_out_cubic");
        test_monotonic(ease_in_out_cubic, "ease_in_out_cubic");
    }

    #[test]
    fn test_ease_in_quart() {
        test_boundaries(ease_in_quart, "ease_in_quart");
        test_ease_in(ease_in_quart, "ease_in_quart");
        test_monotonic(ease_in_quart, "ease_in_quart");
    }

    #[test]
    fn test_ease_out_quart() {
        test_boundaries(ease_out_quart, "ease_out_quart");
        test_ease_out(ease_out_quart, "ease_out_quart");
        test_monotonic(ease_out_quart, "ease_out_quart");
    }

    #[test]
    fn test_ease_in_out_quart() {
        test_boundaries(ease_in_out_quart, "ease_in_out_quart");
        test_ease_in_out(ease_in_out_quart, "ease_in_out_quart");
        test_monotonic(ease_in_out_quart, "ease_in_out_quart");
    }

    #[test]
    fn test_ease_in_quint() {
        test_boundaries(ease_in_quint, "ease_in_quint");
        test_ease_in(ease_in_quint, "ease_in_quint");
        test_monotonic(ease_in_quint, "ease_in_quint");
    }

    #[test]
    fn test_ease_out_quint() {
        test_boundaries(ease_out_quint, "ease_out_quint");
        test_ease_out(ease_out_quint, "ease_out_quint");
        test_monotonic(ease_out_quint, "ease_out_quint");
    }

    #[test]
    fn test_ease_in_out_quint() {
        test_boundaries(ease_in_out_quint, "ease_in_out_quint");
        test_ease_in_out(ease_in_out_quint, "ease_in_out_quint");
        test_monotonic(ease_in_out_quint, "ease_in_out_quint");
    }

    #[test]
    fn test_ease_in_sine() {
        test_boundaries(ease_in_sine, "ease_in_sine");
        test_ease_in(ease_in_sine, "ease_in_sine");
        test_monotonic(ease_in_sine, "ease_in_sine");
    }

    #[test]
    fn test_ease_out_sine() {
        test_boundaries(ease_out_sine, "ease_out_sine");
        test_ease_out(ease_out_sine, "ease_out_sine");
        test_monotonic(ease_out_sine, "ease_out_sine");
    }

    #[test]
    fn test_ease_in_out_sine() {
        test_boundaries(ease_in_out_sine, "ease_in_out_sine");
        test_ease_in_out(ease_in_out_sine, "ease_in_out_sine");
        test_monotonic(ease_in_out_sine, "ease_in_out_sine");
    }

    #[test]
    fn test_ease_in_expo() {
        test_boundaries(ease_in_expo, "ease_in_expo");
        test_ease_in(ease_in_expo, "ease_in_expo");
        test_monotonic(ease_in_expo, "ease_in_expo");
    }

    #[test]
    fn test_ease_out_expo() {
        test_boundaries(ease_out_expo, "ease_out_expo");
        test_ease_out(ease_out_expo, "ease_out_expo");
        test_monotonic(ease_out_expo, "ease_out_expo");
    }

    #[test]
    fn test_ease_in_out_expo() {
        test_boundaries(ease_in_out_expo, "ease_in_out_expo");
        test_ease_in_out(ease_in_out_expo, "ease_in_out_expo");
        test_monotonic(ease_in_out_expo, "ease_in_out_expo");
    }

    #[test]
    fn test_ease_in_circ() {
        test_boundaries(ease_in_circ, "ease_in_circ");
        test_ease_in(ease_in_circ, "ease_in_circ");
        test_monotonic(ease_in_circ, "ease_in_circ");
    }

    #[test]
    fn test_ease_out_circ() {
        test_boundaries(ease_out_circ, "ease_out_circ");
        test_ease_out(ease_out_circ, "ease_out_circ");
        test_monotonic(ease_out_circ, "ease_out_circ");
    }

    #[test]
    fn test_ease_in_out_circ() {
        test_boundaries(ease_in_out_circ, "ease_in_out_circ");
        test_ease_in_out(ease_in_out_circ, "ease_in_out_circ");
        test_monotonic(ease_in_out_circ, "ease_in_out_circ");
    }

    #[test]
    fn test_ease_in_back() {
        test_boundaries(ease_in_back, "ease_in_back");
        // Back functions can go negative, so we don't test monotonicity
        assert!(ease_in_back(0.1) < 0.0, "ease_in_back should overshoot backward");
    }

    #[test]
    fn test_ease_out_back() {
        test_boundaries(ease_out_back, "ease_out_back");
        assert!(ease_out_back(0.9) > 1.0, "ease_out_back should overshoot forward");
    }

    #[test]
    fn test_ease_in_out_back() {
        test_boundaries(ease_in_out_back, "ease_in_out_back");
        test_ease_in_out(ease_in_out_back, "ease_in_out_back");
    }

    #[test]
    fn test_ease_in_elastic() {
        test_boundaries(ease_in_elastic, "ease_in_elastic");
        // Elastic functions oscillate, so we don't test monotonicity
    }

    #[test]
    fn test_ease_out_elastic() {
        test_boundaries(ease_out_elastic, "ease_out_elastic");
        // Elastic functions oscillate, so we don't test monotonicity
    }

    #[test]
    fn test_ease_in_out_elastic() {
        test_boundaries(ease_in_out_elastic, "ease_in_out_elastic");
        // Elastic functions oscillate, so we don't test monotonicity
    }

    #[test]
    fn test_ease_in_bounce() {
        test_boundaries(ease_in_bounce, "ease_in_bounce");
        // Bounce functions can go backward, so we don't test monotonicity
    }

    #[test]
    fn test_ease_out_bounce() {
        test_boundaries(ease_out_bounce, "ease_out_bounce");
        // Bounce functions can overshoot, so we don't test monotonicity
        // But should be monotonic in the general sense
        let mut prev = ease_out_bounce(0.0);
        for i in 1..=100 {
            let t = i as f32 / 100.0;
            let curr = ease_out_bounce(t);
            // Allow some tolerance for bounce effects
            assert!(
                curr >= prev - 0.1,
                "ease_out_bounce should be generally increasing"
            );
            prev = curr;
        }
    }

    #[test]
    fn test_ease_in_out_bounce() {
        test_boundaries(ease_in_out_bounce, "ease_in_out_bounce");
        test_ease_in_out(ease_in_out_bounce, "ease_in_out_bounce");
    }

    #[test]
    fn test_all_functions_at_boundaries() {
        // Test all functions return correct values at 0.0 and 1.0
        let functions: Vec<(&str, fn(f32) -> f32)> = vec![
            ("linear", linear),
            ("ease_in_quad", ease_in_quad),
            ("ease_out_quad", ease_out_quad),
            ("ease_in_out_quad", ease_in_out_quad),
            ("ease_in_cubic", ease_in_cubic),
            ("ease_out_cubic", ease_out_cubic),
            ("ease_in_out_cubic", ease_in_out_cubic),
            ("ease_in_quart", ease_in_quart),
            ("ease_out_quart", ease_out_quart),
            ("ease_in_out_quart", ease_in_out_quart),
            ("ease_in_quint", ease_in_quint),
            ("ease_out_quint", ease_out_quint),
            ("ease_in_out_quint", ease_in_out_quint),
            ("ease_in_sine", ease_in_sine),
            ("ease_out_sine", ease_out_sine),
            ("ease_in_out_sine", ease_in_out_sine),
            ("ease_in_expo", ease_in_expo),
            ("ease_out_expo", ease_out_expo),
            ("ease_in_out_expo", ease_in_out_expo),
            ("ease_in_circ", ease_in_circ),
            ("ease_out_circ", ease_out_circ),
            ("ease_in_out_circ", ease_in_out_circ),
            ("ease_in_back", ease_in_back),
            ("ease_out_back", ease_out_back),
            ("ease_in_out_back", ease_in_out_back),
            ("ease_in_elastic", ease_in_elastic),
            ("ease_out_elastic", ease_out_elastic),
            ("ease_in_out_elastic", ease_in_out_elastic),
            ("ease_in_bounce", ease_in_bounce),
            ("ease_out_bounce", ease_out_bounce),
            ("ease_in_out_bounce", ease_in_out_bounce),
        ];

        for (name, func) in functions {
            assert_eq!(func(0.0), 0.0, "{} should return 0.0 at t=0.0", name);
            assert_eq!(func(1.0), 1.0, "{} should return 1.0 at t=1.0", name);
        }
    }

    #[test]
    fn test_polynomial_ordering() {
        // Test that higher-order polynomials are "stronger" (more extreme)
        let t = 0.5;
        assert!(ease_in_quint(t) < ease_in_quart(t));
        assert!(ease_in_quart(t) < ease_in_cubic(t));
        assert!(ease_in_cubic(t) < ease_in_quad(t));
        assert!(ease_in_quad(t) < linear(t));

        assert!(ease_out_quint(t) > ease_out_quart(t));
        assert!(ease_out_quart(t) > ease_out_cubic(t));
        assert!(ease_out_cubic(t) > ease_out_quad(t));
        assert!(ease_out_quad(t) > linear(t));
    }
}
