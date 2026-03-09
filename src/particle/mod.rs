mod emitter;

pub use emitter::{Emitter, EmitterConfig, EmitterShape};

use crate::{
    Color, Vec2,
    animation::{Lerp, ease_out_quad},
};

pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,

    pub age: f32,
    pub lifetime: f32,

    pub color_start: Color,
    pub color_end: Color,

    pub size_start: f32,
    pub size_end: f32,

    pub rotation: f32,
    pub angular_velocity: f32,
}

impl Particle {
    #[inline]
    pub fn is_alive(&self) -> bool {
        self.age < self.lifetime
    }

    #[inline]
    pub fn normalized_age(&self) -> f32 {
        self.age / self.lifetime
    }

    #[inline]
    pub fn current_color(&self) -> Color {
        let t = self.normalized_age();
        let mut color = self.color_start.lerp(&self.color_end, t);

        // fade out alpha in last 25%
        if t > 0.75 {
            // normalised t over last 25%
            let alpha_t = (t - 0.75) / 0.25;
            color.a = ((1.0 - alpha_t) * color.a as f32) as u8;
        }

        color
    }

    #[inline]
    pub fn current_size(&self) -> f32 {
        let mut t = self.normalized_age();
        t = ease_out_quad(t);

        f32::lerp(self.size_start, self.size_end, t)
    }
}
