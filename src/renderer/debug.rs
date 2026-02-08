use crate::Color;
use crate::renderer::Renderer;

impl<'a> Renderer<'a> {
    /// Debug function to visualize the depth buffer.
    /// It maps depth values to grayscale colors between the specified near and far planes.
    pub fn debug_draw_depth(&mut self, near: f32, far: f32) {
        let (w, h) = (self.width(), self.height());

        for y in 0..h {
            for x in 0..w {
                let z = self.get_depth((x, y));

                let mut t = ((z - near) / (far - near)).clamp(0.0, 1.0); // get normalized depth
                t = 1.0 - t; // So that nearer depths are brighter

                let c = (t * 255.0).round() as u8; // Grayscale value
                let color = Color::RGB(c, c, c);

                self.set_pixel((x as i32, y as i32), color);
            }
        }
    }
}
