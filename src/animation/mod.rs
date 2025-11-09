mod easing;
mod keyframe;
mod lerp;

pub use easing::*;
pub use keyframe::*;
pub use lerp::Lerp;

pub struct Animation<T> {
    start: T,
    end: T,
    duration: f32,
    elapsed: f32,
    easing: fn(f32) -> f32,
}

impl<T: Lerp> Animation<T> {
    pub fn new(start: T, end: T, duration: f32) -> Self {
        Self {
            start,
            end,
            duration: duration.clamp(0.0, f32::MAX), // Should be >= 0.0
            elapsed: 0.0,
            easing: |t| t, // Linear easing by default
        }
    }

    pub fn with_easing(start: T, end: T, duration: f32, easing: fn(f32) -> f32) -> Self {
        Self {
            start,
            end,
            duration: duration.clamp(0.0, f32::MAX), // Should be >= 0.0
            elapsed: 0.0,
            easing,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Advance time
        self.elapsed += delta_time;

        // Do not exceed animation duration
        self.elapsed = self.elapsed.min(self.duration)
    }

    pub fn progress(&self) -> f32 {
        if self.duration <= 0.0 {
            return 1.0;
        }
        (self.elapsed / self.duration).clamp(0.0, 1.0)
    }

    pub fn is_complete(&self) -> bool {
        self.elapsed >= self.duration
    }

    pub fn reset(&mut self) {
        self.elapsed = 0.0;
    }

    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    pub fn duration(&self) -> f32 {
        self.duration
    }

    pub fn value(&self) -> T {
        if self.duration == 0.0 {
            return self.end;
        }

        let t = self.progress();
        let t_eased = (self.easing)(t);

        Lerp::lerp(self.start, self.end, t_eased)
    }

    pub fn set_easing(&mut self, easing: fn(f32) -> f32) {
        self.easing = easing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec2;

    #[test]
    fn animation_creation() {
        let anim = Animation::new(0.0, 100.0, 2.0);
        assert_eq!(anim.duration(), 2.0);
        assert_eq!(anim.elapsed(), 0.0);
        assert!(!anim.is_complete());
        assert_eq!(anim.progress(), 0.0);
    }

    #[test]
    fn animation_initial_value() {
        let anim = Animation::new(0.0, 100.0, 2.0);
        assert_eq!(anim.value(), 0.0);
    }

    #[test]
    fn animation_progress() {
        let mut anim = Animation::new(0.0, 100.0, 2.0);
        assert_eq!(anim.progress(), 0.0);

        anim.update(1.0);
        assert_eq!(anim.progress(), 0.5);

        anim.update(1.0);
        assert_eq!(anim.progress(), 1.0);
        assert!(anim.is_complete());
    }

    #[test]
    fn animation_value_interpolation() {
        let mut anim = Animation::new(0.0, 100.0, 2.0);

        assert_eq!(anim.value(), 0.0);

        anim.update(1.0);
        assert!((anim.value() - 50.0).abs() < 1e-6);

        anim.update(1.0);
        assert_eq!(anim.value(), 100.0);
    }

    #[test]
    fn animation_completion() {
        let mut anim = Animation::new(0.0, 100.0, 2.0);
        assert!(!anim.is_complete());

        anim.update(1.0);
        assert!(!anim.is_complete());

        anim.update(1.0);
        assert!(anim.is_complete());

        // Updating past duration should not change completion
        anim.update(1.0);
        assert!(anim.is_complete());
        assert_eq!(anim.value(), 100.0);
    }

    #[test]
    fn animation_reset() {
        let mut anim = Animation::new(0.0, 100.0, 2.0);
        anim.update(2.0);
        assert!(anim.is_complete());
        assert_eq!(anim.value(), 100.0);

        anim.reset();
        assert!(!anim.is_complete());
        assert_eq!(anim.elapsed(), 0.0);
        assert_eq!(anim.value(), 0.0);
    }

    #[test]
    fn animation_vec2_interpolation() {
        let mut anim = Animation::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 200.0), 2.0);

        assert_eq!(anim.value(), Vec2::new(0.0, 0.0));

        anim.update(1.0);
        let value = anim.value();
        assert!((value.x - 50.0).abs() < 1e-6);
        assert!((value.y - 100.0).abs() < 1e-6);

        anim.update(1.0);
        assert_eq!(anim.value(), Vec2::new(100.0, 200.0));
    }

    #[test]
    fn animation_zero_duration() {
        let anim = Animation::new(0.0, 100.0, 0.0);
        assert!(anim.is_complete());
        assert_eq!(anim.value(), 100.0); // Should return end value immediately
        assert_eq!(anim.progress(), 1.0);
    }

    #[test]
    fn animation_negative_duration_clamped() {
        let anim = Animation::new(0.0, 100.0, -5.0);
        assert_eq!(anim.duration(), 0.0); // Should be clamped to 0.0
        assert!(anim.is_complete());
    }

    #[test]
    fn animation_with_easing() {
        // Custom easing: quadratic (ease-in)
        fn ease_in_quad(t: f32) -> f32 {
            t * t
        }

        let mut anim = Animation::with_easing(0.0, 100.0, 2.0, ease_in_quad);

        // At 50% time, with ease-in, should be less than 50% value
        anim.update(1.0);
        let value = anim.value();
        assert!(value < 50.0); // Ease-in means slower start
        assert!(value > 0.0);
    }

    #[test]
    fn animation_set_easing() {
        fn ease_in(t: f32) -> f32 {
            t * t
        }

        fn ease_out(t: f32) -> f32 {
            1.0 - (1.0 - t) * (1.0 - t)
        }

        let mut anim = Animation::new(0.0, 100.0, 2.0);
        anim.update(1.0);
        let linear_value = anim.value(); // Should be 50.0 with linear

        anim.reset();
        anim.set_easing(ease_in);
        anim.update(1.0);
        let ease_in_value = anim.value(); // Should be less than 50.0

        assert!(ease_in_value < linear_value);

        anim.reset();
        anim.set_easing(ease_out);
        anim.update(1.0);
        let ease_out_value = anim.value(); // Should be more than 50.0

        assert!(ease_out_value > linear_value);
    }

    #[test]
    fn animation_progress_clamping() {
        let mut anim = Animation::new(0.0, 100.0, 2.0);

        // Update way past duration
        anim.update(10.0);

        assert_eq!(anim.progress(), 1.0);
        assert_eq!(anim.value(), 100.0);
        assert!(anim.is_complete());
    }

    #[test]
    fn animation_small_delta_times() {
        let mut anim = Animation::new(0.0, 100.0, 2.0);

        // Update with many small steps (200 * 0.01 = 2.0)
        for _ in 0..200 {
            anim.update(0.01);
        }

        // Due to floating point precision, might be slightly less than 2.0
        // But should be very close to complete
        assert!(anim.elapsed() >= 1.99);
        assert!(anim.progress() >= 0.995);
    }

    #[test]
    fn animation_f32_negative_values() {
        let mut anim = Animation::new(-100.0, 100.0, 2.0);

        assert_eq!(anim.value(), -100.0);

        anim.update(1.0);
        assert!((anim.value() - 0.0).abs() < 1e-6);

        anim.update(1.0);
        assert_eq!(anim.value(), 100.0);
    }

    #[test]
    fn animation_vec2_negative_values() {
        let mut anim = Animation::new(Vec2::new(-50.0, -100.0), Vec2::new(50.0, 100.0), 2.0);

        assert_eq!(anim.value(), Vec2::new(-50.0, -100.0));

        anim.update(1.0);
        assert_eq!(anim.value(), Vec2::new(0.0, 0.0));

        anim.update(1.0);
        assert_eq!(anim.value(), Vec2::new(50.0, 100.0));
    }

    #[test]
    fn animation_multiple_resets() {
        let mut anim = Animation::new(0.0, 100.0, 2.0);

        for _ in 0..5 {
            anim.update(2.0);
            assert!(anim.is_complete());
            anim.reset();
            assert!(!anim.is_complete());
            assert_eq!(anim.value(), 0.0);
        }
    }

    #[test]
    fn animation_very_short_duration() {
        let mut anim = Animation::new(0.0, 100.0, 0.001);
        anim.update(0.001);
        assert!(anim.is_complete());
        assert_eq!(anim.value(), 100.0);
    }

    #[test]
    fn animation_very_long_duration() {
        let mut anim = Animation::new(0.0, 100.0, 1000.0);
        anim.update(500.0);
        assert!((anim.progress() - 0.5).abs() < 1e-6);
        assert!((anim.value() - 50.0).abs() < 1e-6);
    }
}
