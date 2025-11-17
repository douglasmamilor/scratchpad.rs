use crate::animation::easing::linear as LinearEasing;
use crate::animation::lerp::Lerp;

/// A single keyframe in an animation timeline
///
/// Keyframes define a value at a specific time, with optional easing
/// for the segment leading out from this keyframe.
///
/// # Examples
///
/// ```
/// use scratchpad_rs::animation::Keyframe;
///
/// // Simple keyframe
/// let kf = Keyframe::new(1.0, 100.0);
/// assert_eq!(kf.time, 1.0);
/// assert_eq!(kf.value, 100.0);
///
/// // Keyframe with easing
/// use scratchpad_rs::animation::ease_in_quad;
/// let kf = Keyframe::with_easing(2.0, 200.0, ease_in_quad);
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Keyframe<T> {
    pub time: f32,
    pub value: T,
    pub easing_out: Option<fn(f32) -> f32>,
}

impl<T> Keyframe<T> {
    /// Create a new keyframe at the given time with the given value
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::Keyframe;
    ///
    /// let kf: Keyframe<f32> = Keyframe::new(1.5, 42.0);
    /// assert_eq!(kf.time, 1.5);
    /// assert_eq!(kf.value, 42.0);
    /// ```
    pub fn new(time: f32, value: T) -> Self {
        Self {
            time,
            value,
            easing_out: None,
        }
    }

    /// Create a new keyframe with custom easing for the segment after this keyframe
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::Keyframe;
    /// use scratchpad_rs::animation::ease_out_bounce;
    ///
    /// let kf = Keyframe::with_easing(2.0, 100.0, ease_out_bounce);
    /// ```
    pub fn with_easing(time: f32, value: T, easing: fn(f32) -> f32) -> Self {
        Self {
            time,
            value,
            easing_out: Some(easing),
        }
    }

    /// Set the easing function for the segment after this keyframe
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::Keyframe;
    /// use scratchpad_rs::animation::ease_in_cubic;
    ///
    /// let mut kf: Keyframe<f32> = Keyframe::new(1.0, 50.0);
    /// kf.set_easing_out(ease_in_cubic);
    /// ```
    pub fn set_easing_out(&mut self, easing: fn(f32) -> f32) {
        self.easing_out = Some(easing);
    }
}

/// A keyframe-based animation that interpolates between multiple keyframes
///
/// Supports multiple keyframes at different times, with optional per-segment
/// easing functions. Can loop or play once.
///
/// # Examples
///
/// ```
/// use scratchpad_rs::animation::KeyFrameAnimation;
///
/// let mut anim: KeyFrameAnimation<f32> = KeyFrameAnimation::new();
/// anim.add_keyframe(0.0, 0.0);
/// anim.add_keyframe(1.0, 100.0);
/// anim.add_keyframe(2.0, 50.0);
///
/// anim.play();
/// anim.update(0.5);
/// let value = anim.value(); // Interpolated value at time 0.5
/// ```
pub struct KeyFrameAnimation<T> {
    keyframes: Vec<Keyframe<T>>,
    current_time: f32,
    duration: f32,
    is_playing: bool,
    is_looping: bool,
    default_easing: fn(f32) -> f32,
}

impl<T: Lerp> KeyFrameAnimation<T> {
    /// Create a new keyframe animation with linear easing
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let _anim = KeyFrameAnimation::<f32>::new();
    /// ```
    pub fn new() -> Self {
        Self::with_easing(LinearEasing)
    }

    /// Create a new keyframe animation with a custom default easing function
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    /// use scratchpad_rs::animation::ease_in_out_cubic;
    ///
    /// let _anim = KeyFrameAnimation::<f32>::with_easing(ease_in_out_cubic);
    /// ```
    pub fn with_easing(easing: fn(f32) -> f32) -> Self {
        Self {
            keyframes: Vec::new(),
            current_time: 0.0,
            duration: 0.0,
            is_playing: false,
            is_looping: false,
            default_easing: easing,
        }
    }

    fn normalize_keyframes(&mut self) {
        // 1. Sort by time
        self.keyframes.sort_by(|a, b| a.time.total_cmp(&b.time));

        // 2. Dedup by time (keeps the *first* keyframe for each time)
        self.keyframes.dedup_by(|a, b| a.time == b.time);

        // 3. Recompute duration
        self.duration = self.keyframes.last().map_or(0.0, |k| k.time);
    }

    /// Add a keyframe at the given time with the given value
    ///
    /// Keyframes are automatically sorted by time. If a keyframe already exists
    /// at the same time, it will be replaced.
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// anim.add_keyframe(0.0, 0.0);
    /// anim.add_keyframe(1.0, 100.0);
    /// anim.add_keyframe(2.0, 50.0);
    /// ```
    pub fn add_keyframe(&mut self, time: f32, value: T) {
        self.keyframes.push(Keyframe::new(time, value));
        self.normalize_keyframes();
    }

    /// Add multiple keyframes at once
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::{KeyFrameAnimation, Keyframe};
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// anim.add_keyframes(vec![
    ///     Keyframe::new(0.0, 0.0),
    ///     Keyframe::new(1.0, 100.0),
    ///     Keyframe::new(2.0, 50.0),
    /// ]);
    /// ```
    pub fn add_keyframes<I>(&mut self, keyframes: I)
    where
        I: IntoIterator<Item = Keyframe<T>>,
    {
        self.keyframes.extend(keyframes);
        self.normalize_keyframes();
    }

    /// Clear all keyframes
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// anim.add_keyframe(0.0, 0.0);
    /// anim.clear();
    /// assert_eq!(anim.keyframe_count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.keyframes.clear();
        self.duration = 0.0;
    }

    /// Get the number of keyframes
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// anim.add_keyframe(0.0, 0.0);
    /// anim.add_keyframe(1.0, 100.0);
    /// assert_eq!(anim.keyframe_count(), 2);
    /// ```
    pub fn keyframe_count(&self) -> usize {
        self.keyframes.len()
    }

    /// Start playing the animation
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// anim.add_keyframe(0.0, 0.0);
    /// anim.add_keyframe(1.0, 100.0);
    /// anim.play();
    /// assert!(anim.is_playing());
    /// ```
    pub fn play(&mut self) {
        self.is_playing = true;
    }

    /// Pause the animation (keeps current time)
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// anim.play();
    /// anim.pause();
    /// assert!(!anim.is_playing());
    /// ```
    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    /// Stop the animation and reset to the beginning
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// anim.add_keyframe(0.0, 0.0);
    /// anim.add_keyframe(1.0, 100.0);
    /// anim.play();
    /// anim.update(0.5);
    /// anim.stop();
    /// assert_eq!(anim.current_time(), 0.0);
    /// assert!(!anim.is_playing());
    /// ```
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.current_time = 0.0;
    }

    /// Set whether the animation should loop
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// anim.set_looping(true);
    /// ```
    pub fn set_looping(&mut self, looping: bool) {
        self.is_looping = looping;
    }

    /// Update the animation by advancing time
    ///
    /// Only advances time if the animation is playing. Respects looping behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// anim.add_keyframe(0.0, 0.0);
    /// anim.add_keyframe(1.0, 100.0);
    /// anim.play();
    /// anim.update(0.5); // Advance by 0.5 seconds
    /// assert_eq!(anim.current_time(), 0.5);
    /// ```
    pub fn update(&mut self, delta_time: f32) {
        if self.is_playing {
            self.current_time += delta_time;

            if self.is_looping {
                if self.current_time >= self.duration && self.duration > 0.0 {
                    self.current_time %= self.duration;
                }
            } else {
                self.current_time = self.current_time.min(self.duration);
            }
        }
    }

    /// Seek to a specific time in the animation
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// anim.add_keyframe(0.0, 0.0);
    /// anim.add_keyframe(2.0, 100.0);
    /// anim.seek(1.0);
    /// assert_eq!(anim.current_time(), 1.0);
    /// ```
    pub fn seek(&mut self, time: f32) {
        if self.duration <= 0.0 {
            self.current_time = 0.0;
            return;
        }

        if self.is_looping {
            self.current_time = time.rem_euclid(self.duration);
        } else {
            self.current_time = time.clamp(0.0, self.duration);
        }
    }

    /// Get the current time in the animation
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// anim.add_keyframe(0.0, 0.0);
    /// anim.add_keyframe(1.0, 100.0);
    /// anim.seek(0.5);
    /// assert_eq!(anim.current_time(), 0.5);
    /// ```
    pub fn current_time(&self) -> f32 {
        self.current_time
    }

    /// Get the total duration of the animation
    ///
    /// This is the time of the last keyframe.
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// anim.add_keyframe(0.0, 0.0);
    /// anim.add_keyframe(2.5, 100.0);
    /// assert_eq!(anim.duration(), 2.5);
    /// ```
    pub fn duration(&self) -> f32 {
        self.duration
    }

    /// Check if the animation is currently playing
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// assert!(!anim.is_playing());
    /// anim.play();
    /// assert!(anim.is_playing());
    /// ```
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    /// Check if the animation is set to loop
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// assert!(!anim.is_looping());
    /// anim.set_looping(true);
    /// assert!(anim.is_looping());
    /// ```
    pub fn is_looping(&self) -> bool {
        self.is_looping
    }

    /// Check if the animation has completed (only relevant when not looping)
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// anim.add_keyframe(0.0, 0.0);
    /// anim.add_keyframe(1.0, 100.0);
    /// anim.play();
    /// anim.update(2.0); // Past the end
    /// assert!(anim.is_complete());
    /// ```
    pub fn is_complete(&self) -> bool {
        self.keyframe_count() > 0 && self.current_time >= self.duration && !self.is_looping
    }

    /// Get the progress from 0.0 to 1.0
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// anim.add_keyframe(0.0, 0.0);
    /// anim.add_keyframe(2.0, 100.0);
    /// anim.seek(1.0);
    /// assert_eq!(anim.progress(), 0.5);
    /// ```
    pub fn progress(&self) -> f32 {
        if self.duration == 0.0 {
            0.0
        } else {
            (self.current_time / self.duration).clamp(0.0, 1.0)
        }
    }

    fn find_segment_index(&self, time: f32) -> usize {
        self.keyframes
            .binary_search_by(|kf| kf.time.total_cmp(&time))
            .unwrap_or_else(|i| if i == 0 { 0 } else { i - 1 })
    }

    /// Get the current interpolated value
    ///
    /// # Panics
    ///
    /// Panics if no keyframes have been added.
    ///
    /// # Examples
    ///
    /// ```
    /// use scratchpad_rs::animation::KeyFrameAnimation;
    ///
    /// let mut anim = KeyFrameAnimation::<f32>::new();
    /// anim.add_keyframe(0.0, 0.0);
    /// anim.add_keyframe(1.0, 100.0);
    /// anim.seek(0.5);
    /// let value = anim.value(); // Approximately 50.0 with linear easing
    /// ```
    pub fn value(&self) -> T {
        match self.keyframes.len() {
            0 => panic!(
                "KeyframeAnimation::value() called with no keyframes. \
                 Did you forget to call add_keyframe()?"
            ),
            1 => self.keyframes[0].value,
            _ => {
                let t = self.current_time;

                if t <= self.keyframes[0].time {
                    return self.keyframes[0].value;
                }

                let last = self.keyframes.len() - 1;
                if t >= self.keyframes[last].time {
                    return self.keyframes[last].value;
                }

                let i = self.find_segment_index(t);
                let k0 = &self.keyframes[i];
                let k1 = &self.keyframes[i + 1];

                let segment_len = k1.time - k0.time;
                let t_local = (t - k0.time) / segment_len;

                let easing = self.segment_easing_or_default(i);
                let t_eased = easing(t_local);

                Lerp::lerp(k0.value, k1.value, t_eased)
            }
        }
    }

    fn segment_easing_or_default(&self, i: usize) -> fn(f32) -> f32 {
        self.keyframes[i].easing_out.unwrap_or(self.default_easing)
    }
}

impl<T: Lerp> Default for KeyFrameAnimation<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::easing::*;
    use crate::math::Vec2;

    #[test]
    fn test_keyframe_creation() {
        let kf = Keyframe::new(1.0, 100.0);
        assert_eq!(kf.time, 1.0);
        assert_eq!(kf.value, 100.0);
        assert_eq!(kf.easing_out, None);
    }

    #[test]
    fn test_keyframe_with_easing() {
        let kf = Keyframe::with_easing(2.0, 200.0, ease_in_quad);
        assert_eq!(kf.time, 2.0);
        assert_eq!(kf.value, 200.0);
        assert!(kf.easing_out.is_some());
    }

    #[test]
    fn test_keyframe_set_easing() {
        let mut kf = Keyframe::new(1.0, 100.0);
        kf.set_easing_out(ease_out_quad);
        assert!(kf.easing_out.is_some());
    }

    #[test]
    fn test_animation_creation() {
        let anim: KeyFrameAnimation<f32> = KeyFrameAnimation::new();
        assert_eq!(anim.keyframe_count(), 0);
        assert_eq!(anim.current_time(), 0.0);
        assert_eq!(anim.duration(), 0.0);
        assert!(!anim.is_playing());
        assert!(!anim.is_looping());
    }

    #[test]
    fn test_animation_with_easing() {
        let anim = KeyFrameAnimation::<f32>::with_easing(ease_in_cubic);
        assert_eq!(anim.keyframe_count(), 0);
    }

    #[test]
    fn test_add_keyframe() {
        let mut anim: KeyFrameAnimation<f32> = KeyFrameAnimation::new();
        anim.add_keyframe(1.0, 100.0);
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(2.0, 200.0);

        // Should be sorted
        assert_eq!(anim.keyframe_count(), 3);
        assert_eq!(anim.duration(), 2.0);
        // Verify sorting by checking values at different times
        anim.seek(0.0);
        assert_eq!(anim.value(), 0.0);
        anim.seek(1.0);
        assert_eq!(anim.value(), 100.0);
        anim.seek(2.0);
        assert_eq!(anim.value(), 200.0);
    }

    #[test]
    fn test_add_keyframes_dedup() {
        let mut anim = KeyFrameAnimation::new();
        anim.add_keyframe(1.0, 100.0);
        anim.add_keyframe(1.0, 200.0); // Duplicate time
        anim.add_keyframe(1.0, 300.0); // Another duplicate

        // Should keep first and dedup
        assert_eq!(anim.keyframe_count(), 1);
        anim.seek(1.0);
        assert_eq!(anim.value(), 100.0);
    }

    #[test]
    fn test_add_keyframes_batch() {
        let mut anim = KeyFrameAnimation::new();
        anim.add_keyframes(vec![
            Keyframe::new(2.0, 200.0),
            Keyframe::new(0.0, 0.0),
            Keyframe::new(1.0, 100.0),
        ]);

        assert_eq!(anim.keyframe_count(), 3);
        assert_eq!(anim.duration(), 2.0);
    }

    #[test]
    fn test_clear() {
        let mut anim: KeyFrameAnimation<f32> = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(1.0, 100.0);
        anim.clear();

        assert_eq!(anim.keyframe_count(), 0);
        assert_eq!(anim.duration(), 0.0);
    }

    #[test]
    fn test_play_pause() {
        let mut anim: KeyFrameAnimation<f32> = KeyFrameAnimation::new();
        assert!(!anim.is_playing());

        anim.play();
        assert!(anim.is_playing());

        anim.pause();
        assert!(!anim.is_playing());
    }

    #[test]
    fn test_stop() {
        let mut anim = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(1.0, 100.0);
        anim.play();
        anim.update(0.5);

        anim.stop();
        assert!(!anim.is_playing());
        assert_eq!(anim.current_time(), 0.0);
    }

    #[test]
    fn test_update_when_playing() {
        let mut anim = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(1.0, 100.0);
        anim.play();
        anim.update(0.3);

        assert_eq!(anim.current_time(), 0.3);
    }

    #[test]
    fn test_update_when_paused() {
        let mut anim: KeyFrameAnimation<f32> = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(1.0, 100.0);
        anim.pause();
        anim.update(0.3);

        assert_eq!(anim.current_time(), 0.0); // Should not advance
    }

    #[test]
    fn test_update_clamps_to_duration() {
        let mut anim = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(1.0, 100.0);
        anim.play();
        anim.update(2.0); // Past duration

        assert_eq!(anim.current_time(), 1.0); // Clamped
        assert!(anim.is_complete());
    }

    #[test]
    fn test_update_looping() {
        let mut anim = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(1.0, 100.0);
        anim.set_looping(true);
        anim.play();
        anim.update(1.5); // Past duration

        assert!((anim.current_time() - 0.5).abs() < 1e-6); // Wrapped
        assert!(!anim.is_complete());
    }

    #[test]
    fn test_seek() {
        let mut anim = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(2.0, 100.0);
        anim.seek(1.0);

        assert_eq!(anim.current_time(), 1.0);
    }

    #[test]
    fn test_seek_clamps() {
        let mut anim: KeyFrameAnimation<f32> = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(1.0, 100.0);
        anim.seek(2.0); // Past duration

        assert_eq!(anim.current_time(), 1.0); // Clamped
    }

    #[test]
    fn test_seek_looping() {
        let mut anim = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(1.0, 100.0);
        anim.set_looping(true);
        anim.seek(1.5);

        assert!((anim.current_time() - 0.5).abs() < 1e-6); // Wrapped
    }

    #[test]
    fn test_progress() {
        let mut anim = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(2.0, 100.0);
        anim.seek(1.0);

        assert_eq!(anim.progress(), 0.5);
    }

    #[test]
    fn test_progress_empty() {
        let anim = KeyFrameAnimation::<f32>::new();
        assert_eq!(anim.progress(), 0.0);
    }

    #[test]
    fn test_value_single_keyframe() {
        let mut anim = KeyFrameAnimation::new();
        anim.add_keyframe(1.0, 100.0);
        anim.seek(0.5);

        assert_eq!(anim.value(), 100.0); // Always returns the single value
    }

    #[test]
    fn test_value_before_first() {
        let mut anim: KeyFrameAnimation<f32> = KeyFrameAnimation::new();
        anim.add_keyframe(1.0, 100.0);
        anim.add_keyframe(2.0, 200.0);
        anim.seek(0.5);

        assert_eq!(anim.value(), 100.0); // Returns first keyframe value
    }

    #[test]
    fn test_value_after_last() {
        let mut anim = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(1.0, 100.0);
        anim.seek(2.0);

        assert_eq!(anim.value(), 100.0); // Returns last keyframe value
    }

    #[test]
    fn test_value_interpolation_linear() {
        let mut anim = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(1.0, 100.0);
        anim.seek(0.5);

        let value = anim.value();
        assert!((value - 50.0).abs() < 1e-6); // Linear interpolation
    }

    #[test]
    fn test_value_interpolation_multiple_segments() {
        let mut anim: KeyFrameAnimation<f32> = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(1.0, 100.0);
        anim.add_keyframe(2.0, 50.0);
        anim.seek(1.5);

        let value = anim.value();
        assert!((value - 75.0).abs() < 1e-6); // Between 100 and 50
    }

    #[test]
    fn test_value_with_easing() {
        let mut anim: KeyFrameAnimation<f32> = KeyFrameAnimation::new();
        // First keyframe with ease-in-quad easing for the segment from 0.0 to 1.0
        let mut kf0 = Keyframe::new(0.0, 0.0);
        kf0.set_easing_out(ease_in_quad);
        anim.add_keyframes(vec![kf0]);
        anim.add_keyframe(1.0, 100.0);
        anim.seek(0.5);

        let value = anim.value();
        // With ease-in-quad, should be less than 50.0 (linear would be 50.0)
        assert!(value < 50.0);
        assert!(value > 0.0);
    }

    #[test]
    fn test_value_vec2() {
        let mut anim = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, Vec2::new(0.0, 0.0));
        anim.add_keyframe(1.0, Vec2::new(100.0, 200.0));
        anim.seek(0.5);

        let value = anim.value();
        assert!((value.x - 50.0).abs() < 1e-6);
        assert!((value.y - 100.0).abs() < 1e-6);
    }

    #[test]
    fn test_is_complete() {
        let mut anim = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(1.0, 100.0);
        anim.play();
        anim.update(1.0);

        assert!(anim.is_complete());
    }

    #[test]
    fn test_is_complete_not_when_looping() {
        let mut anim: KeyFrameAnimation<f32> = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(1.0, 100.0);
        anim.set_looping(true);
        anim.play();
        anim.update(2.0);

        assert!(!anim.is_complete()); // Looping animations never complete
    }

    #[test]
    fn test_is_complete_empty() {
        let anim = KeyFrameAnimation::<f32>::new();
        assert!(!anim.is_complete());
    }

    #[test]
    fn test_default_trait() {
        let anim: KeyFrameAnimation<f32> = KeyFrameAnimation::default();
        assert_eq!(anim.keyframe_count(), 0);
    }

    #[test]
    fn test_unsorted_keyframes() {
        let mut anim: KeyFrameAnimation<f32> = KeyFrameAnimation::new();
        // Add out of order
        anim.add_keyframe(2.0, 200.0);
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(1.0, 100.0);

        // Should be sorted - verify by checking values at keyframe times
        anim.seek(0.0);
        assert_eq!(anim.value(), 0.0);
        anim.seek(1.0);
        assert_eq!(anim.value(), 100.0);
        anim.seek(2.0);
        assert_eq!(anim.value(), 200.0);
    }

    #[test]
    fn test_multiple_updates() {
        let mut anim: KeyFrameAnimation<f32> = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(1.0, 100.0);
        anim.play();

        anim.update(0.1);
        assert_eq!(anim.current_time(), 0.1);
        anim.update(0.2);
        assert_eq!(anim.current_time(), 0.3);
        anim.update(0.3);
        assert_eq!(anim.current_time(), 0.6);
    }

    #[test]
    fn test_looping_wraps_correctly() {
        let mut anim: KeyFrameAnimation<f32> = KeyFrameAnimation::new();
        anim.add_keyframe(0.0, 0.0);
        anim.add_keyframe(1.0, 100.0);
        anim.set_looping(true);
        anim.play();

        anim.update(0.5);
        assert_eq!(anim.current_time(), 0.5);
        anim.update(0.6); // Should wrap to 0.1
        assert!((anim.current_time() - 0.1).abs() < 1e-6);
    }

    #[test]
    #[should_panic(expected = "no keyframes")]
    fn test_value_panics_with_no_keyframes() {
        let anim = KeyFrameAnimation::<f32>::new();
        let _ = anim.value(); // Should panic
    }

    #[test]
    fn test_segment_easing_per_keyframe() {
        let mut anim: KeyFrameAnimation<f32> = KeyFrameAnimation::new();
        // First keyframe with ease-in-quad for segment 0.0 -> 1.0
        let mut kf0 = Keyframe::new(0.0, 0.0);
        kf0.set_easing_out(ease_in_quad);
        anim.add_keyframes(vec![kf0]);

        // Second keyframe uses default linear for segment 1.0 -> 2.0
        anim.add_keyframe(1.0, 100.0);
        anim.add_keyframe(2.0, 200.0);

        // Test first segment (with ease-in-quad)
        anim.seek(0.5);
        let value1 = anim.value();
        assert!(value1 < 50.0); // Slower than linear

        // Test second segment (with linear)
        anim.seek(1.5);
        let value2 = anim.value();
        assert!((value2 - 150.0).abs() < 1e-6); // Linear interpolation
    }
}
