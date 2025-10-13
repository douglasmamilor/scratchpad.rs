use crate::color::Color;
use crate::math::{IVec2, Mat3, Vec2};
use crate::renderer::{Renderer, quantize_point, snap_axis};

impl<'a> Renderer<'a> {
    /// Draws the outline of a rectangle using any two corners.
    ///
    /// `p0` and `p1` are any two opposite corners of the rectangle.
    /// The function normalizes them internally to determine the actual bounds.
    /// Uses anti-aliased lines for smooth edges.
    ///
    /// # Examples
    /// ```
    /// use scratchpad_rs::math::{Vec2, Mat3};
    /// use scratchpad_rs::color::Color;
    /// use scratchpad_rs::framebuffer::FrameBuffer;
    /// use scratchpad_rs::renderer::Renderer;
    ///
    /// let mut frame_buffer = FrameBuffer::new(100, 100);
    /// let mut renderer = Renderer::new(&mut frame_buffer);
    ///
    /// // Draw rectangle from (10.0, 10.0) to (50.0, 30.0)
    /// renderer.draw_rect(Vec2::new(10.0, 10.0), Vec2::new(50.0, 30.0), Color::RED, Mat3::IDENTITY);
    ///
    /// // Same rectangle, corners swapped
    /// renderer.draw_rect(Vec2::new(50.0, 30.0), Vec2::new(10.0, 10.0), Color::RED, Mat3::IDENTITY);
    /// ```
    pub fn draw_rect(&mut self, p0: Vec2, p1: Vec2, color: Color, model: Mat3) {
        let p0_s = model.transform_vec2(p0); // float, screen space
        let p1_s = model.transform_vec2(p1);

        let x0 = p0_s.x.min(p1_s.x);
        let x1 = p0_s.x.max(p1_s.x);
        let y0 = p0_s.y.min(p1_s.y);
        let y1 = p0_s.y.max(p1_s.y);
        let w = x1 - x0;
        let h = y1 - y0;

        if w <= 0.0 && h <= 0.0 {
            // Single pixel case - both corners are the same
            let (ix, iy) = quantize_point(Vec2::new(x0, y0));
            self.set_pixel((ix, iy), color);
            return;
        }

        if w <= 0.0 {
            // Single vertical line
            self.draw_line_aa(Vec2::new(x0, y0), Vec2::new(x0, y1), color, Mat3::IDENTITY);
            return;
        }

        if h <= 0.0 {
            // Single horizontal line
            self.draw_line_aa(Vec2::new(x0, y0), Vec2::new(x1, y0), color, Mat3::IDENTITY);
            return;
        }

        // Draw all four edges using anti-aliased lines
        self.draw_line_aa(Vec2::new(x0, y0), Vec2::new(x1, y0), color, Mat3::IDENTITY); // top edge
        self.draw_line_aa(Vec2::new(x0, y1), Vec2::new(x1, y1), color, Mat3::IDENTITY); // bottom edge
        self.draw_line_aa(Vec2::new(x0, y0), Vec2::new(x0, y1), color, Mat3::IDENTITY); // left edge
        self.draw_line_aa(Vec2::new(x1, y0), Vec2::new(x1, y1), color, Mat3::IDENTITY); // right edge
    }

    /// Fills a rectangle using any two corners.
    ///
    /// `p0` and `p1` are any two opposite corners of the rectangle.
    /// The function normalizes them internally and uses half-open intervals for correctness.
    ///
    /// # Examples
    /// ```
    /// use scratchpad_rs::math::{Vec2, Mat3};
    /// use scratchpad_rs::color::Color;
    /// use scratchpad_rs::framebuffer::FrameBuffer;
    /// use scratchpad_rs::renderer::Renderer;
    ///
    /// let mut frame_buffer = FrameBuffer::new(100, 100);
    /// let mut renderer = Renderer::new(&mut frame_buffer);
    ///
    /// // Fill rectangle from (10.0, 10.0) to (50.0, 30.0)
    /// renderer.fill_rect(Vec2::new(10.0, 10.0), Vec2::new(50.0, 30.0), Color::BLUE, Mat3::IDENTITY);
    /// ```
    pub fn fill_rect(&mut self, p0: Vec2, p1: Vec2, color: Color, model: Mat3) {
        let p0_s = model.transform_vec2(p0); // float, screen space
        let p1_s = model.transform_vec2(p1);

        let x0 = p0_s.x.min(p1_s.x);
        let x1 = p0_s.x.max(p1_s.x);
        let y0 = p0_s.y.min(p1_s.y);
        let y1 = p0_s.y.max(p1_s.y);

        if x1 <= x0 || y1 <= y0 {
            return;
        }

        // Convert to pixel bounds using half-open intervals [x0i, x1i), [y0i, y1i)
        let x0i = x0.floor() as i32;
        let x1i = x1.ceil() as i32;
        let y0i = y0.floor() as i32;
        let y1i = y1.ceil() as i32;

        for yi in y0i..y1i {
            for xi in x0i..x1i {
                self.set_pixel((xi, yi), color);
            }
        }
    }

    /// Draws a crisp pixel-perfect rectangle outline.
    ///
    /// Uses `snap_axis` to ensure 1-pixel lines appear sharp at any position.
    /// `p0` and `p1` are any two opposite corners of the rectangle.
    ///
    /// # Examples
    /// ```
    /// use scratchpad_rs::math::IVec2;
    /// use scratchpad_rs::color::Color;
    /// use scratchpad_rs::framebuffer::FrameBuffer;
    /// use scratchpad_rs::renderer::Renderer;
    ///
    /// let mut frame_buffer = FrameBuffer::new(100, 100);
    /// let mut renderer = Renderer::new(&mut frame_buffer);
    ///
    /// // Draw crisp rectangle from (10, 10) to (50, 30)
    /// renderer.draw_rect_pixel(IVec2::new(10, 10), IVec2::new(50, 30), Color::WHITE);
    /// ```
    pub fn draw_rect_pixel(&mut self, p0: IVec2, p1: IVec2, color: Color) {
        let (x0, x1) = (p0.x.min(p1.x), p0.x.max(p1.x));
        let (y0, y1) = (p0.y.min(p1.y), p0.y.max(p1.y));

        let x0s = snap_axis(x0 as f32, 1.0) as i32;
        let x1s = snap_axis(x1 as f32, 1.0) as i32;
        let y0s = snap_axis(y0 as f32, 1.0) as i32;
        let y1s = snap_axis(y1 as f32, 1.0) as i32;

        self.draw_line_pixel(
            IVec2::new(x0s, y0s),
            IVec2::new(x1s, y0s),
            color,
            Mat3::IDENTITY,
        );
        self.draw_line_pixel(
            IVec2::new(x0s, y1s),
            IVec2::new(x1s, y1s),
            color,
            Mat3::IDENTITY,
        );
        self.draw_line_pixel(
            IVec2::new(x0s, y0s),
            IVec2::new(x0s, y1s),
            color,
            Mat3::IDENTITY,
        );
        self.draw_line_pixel(
            IVec2::new(x1s, y0s),
            IVec2::new(x1s, y1s),
            color,
            Mat3::IDENTITY,
        );
    }

    /// Legacy function for backward compatibility.
    ///
    /// Converts the old API to the new Vec2-based API.
    #[deprecated(note = "Use draw_rect with Vec2 parameters instead")]
    pub fn draw_rect_pts(&mut self, top_left: (i32, i32), top_right: (i32, i32), color: Color) {
        let (x0, y0) = top_left;
        let (x, y) = top_right;
        let (w, h) = (x - x0 + 1, y - y0 + 1);

        self.draw_rect(
            Vec2::new(x0 as f32, y0 as f32),
            Vec2::new((x0 + w - 1) as f32, (y0 + h - 1) as f32),
            color,
            Mat3::IDENTITY,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::FrameBuffer;
    use std::collections::HashSet;

    fn collect_rect(p0: Vec2, p1: Vec2) -> HashSet<(i32, i32)> {
        let mut fb = FrameBuffer::new(96, 96);
        {
            let mut renderer = Renderer::new(&mut fb);
            renderer.draw_rect(p0, p1, Color::WHITE, Mat3::IDENTITY);
        }

        let mut points = HashSet::new();
        for y in 0..fb.height() as i32 {
            for x in 0..fb.width() as i32 {
                if fb.get_pixel(x as usize, y as usize).unwrap_or(0) != 0 {
                    points.insert((x, y));
                }
            }
        }
        points
    }

    fn collect_fill_rect(p0: Vec2, p1: Vec2) -> HashSet<(i32, i32)> {
        let mut fb = FrameBuffer::new(96, 96);
        {
            let mut renderer = Renderer::new(&mut fb);
            renderer.fill_rect(p0, p1, Color::WHITE, Mat3::IDENTITY);
        }

        let mut points = HashSet::new();
        for y in 0..fb.height() as i32 {
            for x in 0..fb.width() as i32 {
                if fb.get_pixel(x as usize, y as usize).unwrap_or(0) != 0 {
                    points.insert((x, y));
                }
            }
        }
        points
    }

    #[test]
    fn draw_rect_with_any_corners() {
        let p0 = Vec2::new(10.0, 10.0);
        let p1 = Vec2::new(20.0, 20.0);
        let points = collect_rect(p0, p1);

        // Should have outline pixels
        assert!(!points.is_empty());

        // Should have corners
        assert!(points.contains(&(10, 10)));
        assert!(points.contains(&(20, 20)));
    }

    #[test]
    fn draw_rect_corners_swapped() {
        let p0 = Vec2::new(20.0, 20.0);
        let p1 = Vec2::new(10.0, 10.0);
        let points = collect_rect(p0, p1);

        // Should work the same as normal order
        assert!(!points.is_empty());
        assert!(points.contains(&(10, 10)));
        assert!(points.contains(&(20, 20)));
    }

    #[test]
    fn draw_rect_single_pixel() {
        let p0 = Vec2::new(10.0, 10.0);
        let p1 = Vec2::new(10.0, 10.0);
        let points = collect_rect(p0, p1);

        // Should draw a single pixel
        assert_eq!(points.len(), 1);
        assert!(points.contains(&(10, 10)));
    }

    #[test]
    fn draw_rect_horizontal_line() {
        let p0 = Vec2::new(10.0, 10.0);
        let p1 = Vec2::new(20.0, 10.0);
        let points = collect_rect(p0, p1);

        // Should draw a horizontal line
        assert!(!points.is_empty());
        assert!(points.contains(&(10, 10)));
        assert!(points.contains(&(20, 10)));
    }

    #[test]
    fn draw_rect_vertical_line() {
        let p0 = Vec2::new(10.0, 10.0);
        let p1 = Vec2::new(10.0, 20.0);
        let points = collect_rect(p0, p1);

        // Should draw a vertical line
        assert!(!points.is_empty());
        assert!(points.contains(&(10, 10)));
        assert!(points.contains(&(10, 20)));
    }

    #[test]
    fn fill_rect_basic() {
        let p0 = Vec2::new(10.0, 10.0);
        let p1 = Vec2::new(15.0, 15.0);
        let points = collect_fill_rect(p0, p1);

        // Should fill the rectangle
        assert!(!points.is_empty());

        // Should have interior pixels
        assert!(points.contains(&(12, 12)));
    }

    #[test]
    fn fill_rect_half_open_interval() {
        let p0 = Vec2::new(10.5, 10.5);
        let p1 = Vec2::new(12.5, 12.5);
        let points = collect_fill_rect(p0, p1);

        // Should use half-open intervals correctly
        // [10.5, 12.5) should include pixels 10, 11, 12
        assert!(points.contains(&(10, 10)));
        assert!(points.contains(&(11, 10)));
        assert!(points.contains(&(12, 10)));
        assert!(points.contains(&(10, 11)));
        assert!(points.contains(&(11, 11)));
        assert!(points.contains(&(12, 11)));
        assert!(points.contains(&(10, 12)));
        assert!(points.contains(&(11, 12)));
        assert!(points.contains(&(12, 12)));

        // Should not include pixel 13
        assert!(!points.contains(&(13, 13)));
    }

    #[test]
    fn fill_rect_empty() {
        let p0 = Vec2::new(10.0, 10.0);
        let p1 = Vec2::new(10.0, 10.0);
        let points = collect_fill_rect(p0, p1);

        // Empty rectangle should not draw anything
        assert!(points.is_empty());
    }

    #[test]
    fn fill_rect_negative_size() {
        let p0 = Vec2::new(15.0, 15.0);
        let p1 = Vec2::new(10.0, 10.0);
        let points = collect_fill_rect(p0, p1);

        // Should work the same as normal order
        assert!(!points.is_empty());
        assert!(points.contains(&(12, 12)));
    }

    #[test]
    fn draw_rect_pixel_crisp() {
        let p0 = IVec2::new(10, 10);
        let p1 = IVec2::new(15, 15);
        let mut fb = FrameBuffer::new(96, 96);
        {
            let mut renderer = Renderer::new(&mut fb);
            renderer.draw_rect_pixel(p0, p1, Color::WHITE);
        }
        let mut points = HashSet::new();
        for y in 0..fb.height() as i32 {
            for x in 0..fb.width() as i32 {
                if fb.get_pixel(x as usize, y as usize).unwrap_or(0) != 0 {
                    points.insert((x, y));
                }
            }
        }

        // Should draw crisp lines
        assert!(!points.is_empty());

        // Should have crisp pixel coordinates
        assert!(points.contains(&(10, 10))); // Top-left corner
        assert!(points.contains(&(15, 15))); // Bottom-right corner
    }

    #[test]
    fn draw_rect_fractional_coordinates() {
        let p0 = Vec2::new(10.7, 10.3);
        let p1 = Vec2::new(15.2, 15.8);
        let points = collect_rect(p0, p1);

        // Should handle fractional coordinates correctly
        assert!(!points.is_empty());

        // Should round to nearest pixels
        assert!(points.contains(&(11, 10))); // 10.7 -> 11, 10.3 -> 10
        assert!(points.contains(&(15, 16))); // 15.2 -> 15, 15.8 -> 16
    }
}
