use super::Renderer;
use crate::color::Color;
use crate::math::vec2::Vec2;

impl<'a> Renderer<'a> {
    /// Draw a triangle outline using three vertices.
    ///
    /// Skips degenerate (collinear) triangles.
    ///
    /// # Example
    /// ```
    /// # use scratchpad_rs::{color::Color, math::vec2::Vec2};
    /// # use scratchpad_rs::framebuffer::FrameBuffer;
    /// # use scratchpad_rs::renderer::Renderer;
    /// # let mut fb = FrameBuffer::new(64, 64);
    /// # let mut r = Renderer::new(&mut fb);
    /// r.draw_triangle(
    ///     Vec2::new(100.0, 100.0),
    ///     Vec2::new(50.0, 200.0),
    ///     Vec2::new(150.0, 200.0),
    ///     &Color::RED,
    /// );
    ///
    #[inline]
    pub fn draw_triangle(&mut self, a: Vec2, b: Vec2, c: Vec2, color: &Color) {
        // Optional: skip degenerate (collinear/tiny) triangles
        // This calculates the cross product which gives twice the area of the triangle
        // So if twice the area is near zero, the points are collinear
        let area2 = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
        if area2.abs() < 1e-6 {
            return;
        }

        self.draw_line_aa(a.into(), b.into(), color);
        self.draw_line_aa(b.into(), c.into(), color);
        self.draw_line_aa(c.into(), a.into(), color);
    }

    /// Fill a triangle using a scanline algorithm.
    ///
    /// O(height) in the triangle’s vertical span. Degenerate (collinear) triangles are skipped.
    ///
    /// # Example
    /// ```
    /// # use scratchpad_rs::{color::Color, math::vec2::Vec2};
    /// # use scratchpad_rs::framebuffer::FrameBuffer;
    /// # use scratchpad_rs::renderer::Renderer;
    /// # let mut fb = FrameBuffer::new(64, 64);
    /// # let mut r = Renderer::new(&mut fb);
    /// r.fill_triangle(
    ///     Vec2::new(100.0, 100.0),
    ///     Vec2::new(50.0, 200.0),
    ///     Vec2::new(150.0, 200.0),
    ///     &Color::RED,
    /// );
    /// ```
    ///
    /// # Degenerate
    /// ```
    /// # use scratchpad_rs::{color::Color, math::vec2::Vec2};
    /// # use scratchpad_rs::framebuffer::FrameBuffer;
    /// # use scratchpad_rs::renderer::Renderer;
    /// # let mut fb = FrameBuffer::new(64, 64);
    /// # let mut r = Renderer::new(&mut fb);
    /// r.fill_triangle(
    ///     Vec2::new(10.0, 10.0),
    ///     Vec2::new(20.0, 10.0),
    ///     Vec2::new(30.0, 10.0),
    ///     &Color::RED,
    /// ); // skipped (collinear)
    /// ```
    pub fn fill_triangle(&mut self, a: Vec2, b: Vec2, c: Vec2, color: &Color) {
        let mut vertices = [a, b, c];
        vertices.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

        #[allow(non_snake_case)]
        let A = vertices[0];

        #[allow(non_snake_case)]
        let B = vertices[1];

        #[allow(non_snake_case)]
        let C = vertices[2];

        if A.y == B.y {
            self.fill_flat_top(A, B, C, color);
            return;
        } else if B.y == C.y {
            self.fill_flat_bottom(A, B, C, color);
            return;
        }

        // The point on AC (that is the vector C - A) that has the same y as B
        let ty = (B.y - A.y) / (C.y - A.y);
        // We plug it into the line equation to get the x coordinate
        #[allow(non_snake_case)]
        let Dx = A.x + ty * (C.x - A.x);
        // The y coordinate is simply B.y
        #[allow(non_snake_case)]
        let Dy = B.y;

        #[allow(non_snake_case)]
        let D = Vec2::new(Dx, Dy);

        self.fill_flat_bottom(A, B, D, color);
        self.fill_flat_top(B, D, C, color);
    }

    #[allow(non_snake_case)]
    pub fn fill_flat_top(&mut self, A: Vec2, B: Vec2, C: Vec2, color: &Color) {
        let eps = 1e-6;
        if (C.y - A.y).abs() < eps || (C.y - B.y).abs() < eps {
            return; // Degenerate flat top
        }

        let inv_slope_1 = (C.x - A.x) / (C.y - A.y);
        let inv_slope_2 = (C.x - B.x) / (C.y - B.y);

        let y_start = A.y.ceil() as i32;
        let y_end = C.y.ceil() as i32;

        let mut x1 = A.x + (y_start as f32 - A.y) * inv_slope_1;
        let mut x2 = B.x + (y_start as f32 - B.y) * inv_slope_2;

        for y in y_start..y_end {
            let (xa, xb) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            self.draw_line_aa((xa, y as f32), (xb, y as f32), color);
            x1 += inv_slope_1;
            x2 += inv_slope_2;
        }
    }

    #[allow(non_snake_case)]
    fn fill_flat_bottom(&mut self, A: Vec2, B: Vec2, C: Vec2, color: &Color) {
        let eps = 1e-6; // small tolerance
        if (B.y - A.y).abs() < eps || (C.y - A.y).abs() < eps {
            return; // basically zero height
        }

        let inv_slope_1 = (B.x - A.x) / (B.y - A.y);
        let inv_slope_2 = (C.x - A.x) / (C.y - A.y);

        let y_start = A.y.ceil() as i32;
        let y_end = C.y.ceil() as i32;

        let mut x1 = A.x + (y_start as f32 - A.y) * inv_slope_1;
        let mut x2 = A.x + (y_start as f32 - A.y) * inv_slope_2;

        for y in y_start..y_end {
            let (xa, xb) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            self.draw_line_aa((xa, y as f32), (xb, y as f32), color);
            x1 += inv_slope_1;
            x2 += inv_slope_2;
        }
    }
}
