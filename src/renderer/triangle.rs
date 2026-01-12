use super::Renderer;
use crate::color::Color;
use crate::math::{Mat3, vec2::Vec2};

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
        // Skip degenerate (collinear or tiny) triangles to avoid NaNs and useless work.
        // cross = 2 * area. If it is tiny or non-finite, treat as empty.
        let area2 = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
        if !area2.is_finite() || area2.abs() < 1e-6 {
            return;
        }

        self.draw_line_aa(a, b, color, model);
        self.draw_line_aa(b, c, color, model);
        self.draw_line_aa(c, a, color, model);
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
        // Transform first; all raster decisions are in screen space.
        let a_s = model.transform_vec2(a);
        let b_s = model.transform_vec2(b);
        let c_s = model.transform_vec2(c);

        // Reject non-finite inputs.
        if !a_s.x.is_finite()
            || !a_s.y.is_finite()
            || !b_s.x.is_finite()
            || !b_s.y.is_finite()
            || !c_s.x.is_finite()
            || !c_s.y.is_finite()
        {
            return;
        }

        // Reject degenerate/zero-area triangles up front (prevents div-by-zero below).
        let area2 = (b_s.x - a_s.x) * (c_s.y - a_s.y) - (b_s.y - a_s.y) * (c_s.x - a_s.x);
        if !area2.is_finite() || area2.abs() < 1e-6 {
            return;
        }

        let mut vertices = [a_s, b_s, c_s];
        vertices.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

        #[allow(non_snake_case)]
        let A = vertices[0];

        #[allow(non_snake_case)]
        let B = vertices[1];

        #[allow(non_snake_case)]
        let C = vertices[2];

        if (A.y - B.y).abs() < 1e-6 {
            self.fill_flat_top(A, B, C, color);
            return;
        } else if (B.y - C.y).abs() < 1e-6 {
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

        // Half-open vertical range [y_start, y_end), inclusive top, exclusive bottom (top-left rule).
        let y_start = A.y.ceil() as i32;
        let y_end = C.y.ceil() as i32;

        let mut x1 = A.x + (y_start as f32 - A.y) * inv_slope_1;
        let mut x2 = B.x + (y_start as f32 - B.y) * inv_slope_2;

        for y in y_start..y_end {
            let (xa, xb) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            // Pixel centers are at integer coords; adding 0.5 then floor picks the covered pixels.
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

        // Half-open vertical range [y_start, y_end), inclusive top, exclusive bottom (top-left rule).
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

// ------------------------------
// Tests
// ------------------------------
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::framebuffer::FrameBuffer;
    use crate::renderer::Renderer;

    fn collect_pixels(fb: &FrameBuffer) -> HashSet<(usize, usize)> {
        fb.pixels
            .iter()
            .enumerate()
            .filter_map(|(i, &p)| {
                if p != 0 {
                    let w = fb.width();
                    let y = i / w;
                    let x = i % w;
                    Some((x, y))
                } else {
                    None
                }
            })
            .collect()
    }

    #[test]
    fn degenerate_triangle_renders_nothing() {
        let mut fb = FrameBuffer::new(64, 64);
        let mut r = Renderer::new(&mut fb);
        r.clear(Color::TRANSPARENT);

        // Collinear: zero area.
        r.fill_triangle(
            Vec2::new(10.0, 10.0),
            Vec2::new(20.0, 10.0),
            Vec2::new(30.0, 10.0),
            Color::WHITE,
            Mat3::IDENTITY,
        );

        assert!(collect_pixels(&fb).is_empty());
    }

    #[test]
    fn two_triangles_cover_rectangle_interior() {
        let mut fb = FrameBuffer::new(64, 64);
        let mut r = Renderer::new(&mut fb);
        r.clear(Color::TRANSPARENT);

        // Rectangle corners
        let p0 = Vec2::new(10.0, 10.0);
        let p1 = Vec2::new(30.0, 10.0);
        let p2 = Vec2::new(30.0, 30.0);
        let p3 = Vec2::new(10.0, 30.0);

        // Fill rectangle with two triangles.
        r.fill_triangle(p0, p1, p2, Color::WHITE, Mat3::IDENTITY);
        r.fill_triangle(p0, p2, p3, Color::WHITE, Mat3::IDENTITY);

        // Check interior (exclude the outermost edge pixels) has no gaps.
        for y in 11..30 {
            for x in 11..30 {
                assert!(
                    fb.get_pixel(x, y).unwrap_or(0) != 0,
                    "Missing coverage at interior pixel ({x},{y})"
                );
            }
        }
    }
}
