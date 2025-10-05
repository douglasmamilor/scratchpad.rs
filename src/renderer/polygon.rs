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
    /// let vertices = vec![
    ///     Vec2::new(100.0, 100.0),  // Top
    ///     Vec2::new(150.0, 150.0),  // Right
    ///     Vec2::new(100.0, 200.0),  // Bottom
    ///     Vec2::new(50.0, 150.0),   // Left
    /// ];
    /// renderer.draw_polygon_outline(&vertices, &Color::RED);
    /// ```
    pub fn draw_polygon(&mut self, vertices: &[Vec2], color: &Color) {
        if vertices.is_empty() {
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
}
