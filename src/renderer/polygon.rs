use super::Renderer;
use crate::color::Color;
use crate::math::vec2::Vec2;

impl<'a> Renderer<'a> {
    /// Draw a polygon outline by connecting vertices with lines
    ///
    /// # Arguments
    /// * `vertices` - Array of Vec2 points defining the polygon
    /// * `color` - Color for the outline
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::color::Color;
    /// use scratchpad_rs::math::vec2::Vec2;
    /// use scratchpad_rs::renderer::Renderer;
    /// use scratchpad_rs::framebuffer::FrameBuffer;
    /// 
    /// let mut framebuffer = FrameBuffer::new(800, 600);
    /// let mut renderer = Renderer::new(&mut framebuffer);
    /// 
    /// let vertices = vec![
    ///     Vec2::new(100.0, 100.0),  // Top
    ///     Vec2::new(150.0, 150.0),  // Right
    ///     Vec2::new(100.0, 200.0),  // Bottom
    ///     Vec2::new(50.0, 150.0),   // Left
    /// ];
    /// renderer.draw_polygon(&vertices, Color::RED);
    /// ```
    pub fn draw_polygon(&mut self, vertices: &[Vec2], color: Color) {
        if vertices.len() < 2 {
            return;
        }

        // walk each consecutive pair
        for w in vertices.windows(2) {
            let start: (f32, f32) = w[0].into();
            let end: (f32, f32) = w[1].into();
            self.draw_line_aa(start, end, color);
        }

        // close polygon
        let last = vertices.last().unwrap();
        let first = vertices.first().unwrap();
        self.draw_line_aa((*last).into(), (*first).into(), color);
    }

    pub fn draw_regular_polygon(
        &mut self,
        ctr: Vec2,
        r: f32,
        rot: f32,
        sides: usize,
        color: Color,
    ) {
        let step = std::f32::consts::TAU / sides as f32;
        let mut vertices: Vec<Vec2> = Vec::with_capacity(sides);

        for i in 0..sides {
            let angle = rot + i as f32 * step;
            let x = ctr.x + r * angle.cos();
            let y = ctr.y + r * angle.sin();

            vertices.push(Vec2::from((x, y)))
        }

        self.draw_polygon(&vertices, color);
    }
}
