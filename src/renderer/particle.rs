use crate::{Mat3, Vec2, particle::Emitter};

use super::Renderer;

impl<'a> Renderer<'a> {
    pub fn draw_particles(&mut self, emitter: &Emitter) {
        for particle in &emitter.particles {
            let color = particle.current_color();
            let size = particle.current_size();

            let half_size = size / 2.0;
            let top_left = particle.position - Vec2::new(half_size, half_size);
            let bottom_right = particle.position + Vec2::new(half_size, half_size);

            self.fill_rect(top_left, bottom_right, color, Mat3::IDENTITY);
        }
    }
}
