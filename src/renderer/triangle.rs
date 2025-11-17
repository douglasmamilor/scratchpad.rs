use super::Renderer;
use crate::color::Color;
use crate::math::{Mat3, barycentric, vec2::Vec2};

impl<'a> Renderer<'a> {
    /// Draw a triangle outline using three vertices.
    ///
    /// Skips degenerate (collinear) triangles.
    ///
    /// # Example
    /// ```
    /// # use scratchpad_rs::{color::Color, math::{vec2::Vec2, Mat3}};
    /// # use scratchpad_rs::framebuffer::FrameBuffer;
    /// # use scratchpad_rs::renderer::Renderer;
    /// # let mut fb = FrameBuffer::new(64, 64);
    /// # let mut r = Renderer::new(&mut fb);
    /// r.draw_triangle(
    ///     Vec2::new(100.0, 100.0),
    ///     Vec2::new(50.0, 200.0),
    ///     Vec2::new(150.0, 200.0),
    ///     Color::RED,
    ///     Mat3::IDENTITY,
    /// );
    ///
    #[inline]
    pub fn draw_triangle(&mut self, a: Vec2, b: Vec2, c: Vec2, color: Color, model: Mat3) {
        // Optional: skip degenerate (collinear/tiny) triangles
        // This calculates the cross product which gives twice the area of the triangle
        // So if twice the area is near zero, the points are collinear
        let area2 = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
        if area2.abs() < 1e-6 {
            return;
        }

        self.draw_line_aa(a, b, color, model);
        self.draw_line_aa(b, c, color, model);
        self.draw_line_aa(c, a, color, model);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn fill_triangle_colored(
        &mut self,
        a: Vec2,
        b: Vec2,
        c: Vec2,
        color_a: Color,
        color_b: Color,
        color_c: Color,
        model: Mat3,
    ) {
        // Transform vertices to screen space
        let a_s = model.transform_vec2(a);
        let b_s = model.transform_vec2(b);
        let c_s = model.transform_vec2(c);

        // Bounding box of the triangle
        let min_x = a_s.x.min(b_s.x).min(c_s.x).floor() as i32;
        let max_x = a_s.x.max(b_s.x).max(c_s.x).ceil() as i32;
        let min_y = a_s.y.min(b_s.y).min(c_s.y).floor() as i32;
        let max_y = a_s.y.max(b_s.y).max(c_s.y).ceil() as i32;

        // Precompute constants for incremental barycentric interpolation
        let v0 = b_s - a_s;
        let v1 = c_s - a_s;
        let denom = v0.cross(v1);
        if denom.abs() < f32::EPSILON {
            return; // degenerate triangle
        }
        let inv_denom = 1.0 / denom;

        // Loop over each scanline
        for y in min_y..=max_y {
            // Compute barycentric coordinates at the leftmost pixel of this scanline
            let y_f = y as f32 + 0.5; // center of pixel
            let v_start = ((Vec2::new(min_x as f32 + 0.5, y_f) - a_s).cross(v1)) * inv_denom;
            let w_start = (v0.cross(Vec2::new(min_x as f32 + 0.5, y_f) - a_s)) * inv_denom;

            // Compute delta for horizontal movement (incremental barycentric)
            // v(x+1) = v(x) + (v0.y * inv_denom)
            // w(x+1) = w(x) - (v1.y * inv_denom)
            let delta_v = v1.y * inv_denom;
            let delta_w = -v0.y * inv_denom;

            let mut v = v_start;
            let mut w = w_start;

            for x in min_x..=max_x {
                let u = 1.0 - v - w;

                // Check if pixel is inside triangle
                if u >= 0.0 && v >= 0.0 && w >= 0.0 {
                    let color = barycentric::interpolate_color(
                        &barycentric::BarycentricCoords { u, v, w },
                        color_a,
                        color_b,
                        color_c,
                    );
                    self.set_pixel((x, y), color);
                }

                // Increment barycentric coordinates for next pixel
                v += delta_v;
                w += delta_w;
            }
        }
    }
    /// Fill a triangle using a scanline algorithm.
    ///
    /// O(height) in the triangle’s vertical span. Degenerate (collinear) triangles are skipped.
    ///
    /// # Example
    /// ```
    /// # use scratchpad_rs::{color::Color, math::{vec2::Vec2, Mat3}};
    /// # use scratchpad_rs::framebuffer::FrameBuffer;
    /// # use scratchpad_rs::renderer::Renderer;
    /// # let mut fb = FrameBuffer::new(64, 64);
    /// # let mut r = Renderer::new(&mut fb);
    /// r.fill_triangle(
    ///     Vec2::new(100.0, 100.0),
    ///     Vec2::new(50.0, 200.0),
    ///     Vec2::new(150.0, 200.0),
    ///     Color::RED,
    ///     Mat3::IDENTITY,
    /// );
    /// ```
    ///
    /// # Degenerate
    /// ```
    /// # use scratchpad_rs::{color::Color, math::{vec2::Vec2, Mat3}};
    /// # use scratchpad_rs::framebuffer::FrameBuffer;
    /// # use scratchpad_rs::renderer::Renderer;
    /// # let mut fb = FrameBuffer::new(64, 64);
    /// # let mut r = Renderer::new(&mut fb);
    /// r.fill_triangle(
    ///     Vec2::new(10.0, 10.0),
    ///     Vec2::new(20.0, 10.0),
    ///     Vec2::new(30.0, 10.0),
    ///     Color::RED,
    ///     Mat3::IDENTITY,
    /// ); // skipped (collinear)
    /// ```
    pub fn fill_triangle(&mut self, a: Vec2, b: Vec2, c: Vec2, color: Color, model: Mat3) {
        let a_s = model.transform_vec2(a); // float, screen space
        let b_s = model.transform_vec2(b);
        let c_s = model.transform_vec2(c);

        let mut vertices = [a_s, b_s, c_s];
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
    pub fn fill_flat_top(&mut self, A: Vec2, B: Vec2, C: Vec2, color: Color) {
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
            let x_start = (xa + 0.5).floor() as i32;
            let x_end = (xb + 0.5).floor() as i32;
            self.hspan(y, x_start, x_end, color);
            x1 += inv_slope_1;
            x2 += inv_slope_2;
        }
    }

    #[allow(non_snake_case)]
    fn fill_flat_bottom(&mut self, A: Vec2, B: Vec2, C: Vec2, color: Color) {
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
            let x_start = (xa + 0.5).floor() as i32;
            let x_end = (xb + 0.5).floor() as i32;
            self.hspan(y, x_start, x_end, color);
            x1 += inv_slope_1;
            x2 += inv_slope_2;
        }
    }
}
