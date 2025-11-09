use crate::animation::easing::linear as LinearEasing;
use crate::animation::lerp::Lerp;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Keyframe<T> {
    pub time: f32,
    pub value: T,
    pub easing_out: Option<fn(f32) -> f32>,
}

impl<T> Keyframe<T> {
    pub fn new(time: f32, value: T) -> Self {
        Self {
            time,
            value,
            easing_out: None,
        }
    }

    pub fn with_easing(time: f32, value: T, easing: fn(f32) -> f32) -> Self {
        Self {
            time,
            value,
            easing_out: Some(easing),
        }
    }

    pub fn set_easing_out(&mut self, easing: fn(f32) -> f32) {
        self.easing_out = Some(easing);
    }
}

pub struct KeyFrameAnimation<T> {
    keyframes: Vec<Keyframe<T>>,
    current_time: f32,
    duration: f32,
    is_playing: bool,
    is_looping: bool,
    default_easing: fn(f32) -> f32,
}

impl<T: Lerp> KeyFrameAnimation<T> {
    pub fn new() -> Self {
        Self::with_easing(LinearEasing)
    }

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

    pub fn add_keyframe(&mut self, time: f32, value: T) {
        self.keyframes.push(Keyframe::new(time, value));
        self.normalize_keyframes();
    }

    pub fn add_keyframes<I>(&mut self, keyframes: I)
    where
        I: IntoIterator<Item = Keyframe<T>>,
    {
        self.keyframes.extend(keyframes);
        self.normalize_keyframes();
    }

    pub fn clear(&mut self) {
        self.keyframes.clear();
        self.duration = 0.0;
    }

    pub fn keyframe_count(&self) -> usize {
        self.keyframes.len()
    }

    pub fn play(&mut self) {
        self.is_playing = true;
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.current_time = 0.0;
    }

    pub fn set_looping(&mut self, looping: bool) {
        self.is_looping = looping;
    }

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

    pub fn current_time(&self) -> f32 {
        self.current_time
    }

    pub fn duration(&self) -> f32 {
        self.duration
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn is_complete(&self) -> bool {
        self.keyframe_count() > 0 && self.current_time >= self.duration && !self.is_looping
    }

    pub fn progress(&self) -> f32 {
        if self.duration == 0.0 {
            0.0
        } else {
            (self.current_time / self.duration).clamp(0.0, 1.0)
        }
    }

    pub fn find_segment_index(&self, time: f32) -> usize {
        self.keyframes
            .binary_search_by(|kf| kf.time.total_cmp(&time))
            .unwrap_or_else(|i| if i == 0 { 0 } else { i - 1 })
    }

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

    pub fn segment_easing_or_default(&self, i: usize) -> fn(f32) -> f32 {
        self.keyframes[i].easing_out.unwrap_or(self.default_easing)
    }
}

impl<T: Lerp> Default for KeyFrameAnimation<T> {
    fn default() -> Self {
        Self::new()
    }
}
