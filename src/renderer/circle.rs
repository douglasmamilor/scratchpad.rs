use crate::Color;
use crate::math::{Mat3, vec2::Vec2};
use crate::renderer::{Renderer, quantize_hspan, quantize_point};

impl<'a> Renderer<'a> {
    /// Draws the outline of a circle using the midpoint algorithm.
    ///
    /// `center` is specified as a Vec2 in floating-point coordinates.
    /// `radius` must be non-negative.
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
    /// // Draw circle at (100.5, 50.0) with radius 25.0
    /// renderer.draw_circle(Vec2::new(100.5, 50.0), 25.0, Color::RED, Mat3::IDENTITY);
    /// ```
    pub fn draw_circle(&mut self, center: Vec2, radius: f32, color: Color, model: Mat3) {
        if radius < 0.0 {
            return;
        }

        let center_s = model.transform_vec2(center); // float, screen space

        if radius == 0.0 {
            let (ix, iy) = quantize_point(center_s);
            self.set_pixel((ix, iy), color);
            return;
        }

        let mut x = radius;
        let mut y = 0.0;
        #[allow(non_snake_case)]
        let mut D = 1.0 - radius;

        let mut plot = |cx: f32, cy: f32, x: f32, y: f32| {
            let (ix1, iy1) = quantize_point(Vec2::new(cx + x, cy + y));
            let (ix2, iy2) = quantize_point(Vec2::new(cx - x, cy + y));
            let (ix3, iy3) = quantize_point(Vec2::new(cx + x, cy - y));
            let (ix4, iy4) = quantize_point(Vec2::new(cx - x, cy - y));
            let (ix5, iy5) = quantize_point(Vec2::new(cx + y, cy + x));
            let (ix6, iy6) = quantize_point(Vec2::new(cx - y, cy + x));
            let (ix7, iy7) = quantize_point(Vec2::new(cx + y, cy - x));
            let (ix8, iy8) = quantize_point(Vec2::new(cx - y, cy - x));

            self.set_pixel((ix1, iy1), color); // 1st octant (+x direction)
            self.set_pixel((ix2, iy2), color); // reflect across y-axis
            self.set_pixel((ix3, iy3), color); // reflect across x-axis
            self.set_pixel((ix4, iy4), color); // reflect across both axes
            self.set_pixel((ix5, iy5), color); // reflect across line y=x
            self.set_pixel((ix6, iy6), color); // reflect across line y=x then across y-axis
            self.set_pixel((ix7, iy7), color); // reflect across y=x, then across x-axis
            self.set_pixel((ix8, iy8), color); // reflect across line y=-x
        };

        while x >= y {
            plot(center_s.x, center_s.y, x, y);

            y += 1.0;
            if D < 0.0 {
                // Go North
                D += 2.0 * y + 1.0;
            } else {
                // Go North-West
                x -= 1.0;
                D += 2.0 * (y - x) + 1.0;
            }
        }
    }

    /// Fills a circle using the midpoint algorithm (draws horizontal spans).
    ///
    /// `center` is specified as a Vec2 in floating-point coordinates.
    /// `radius` must be non-negative.
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
    /// // Fill circle at (100.5, 50.0) with radius 25.0
    /// renderer.fill_circle(Vec2::new(100.5, 50.0), 25.0, Color::BLUE, Mat3::IDENTITY);
    /// ```
    pub fn fill_circle(&mut self, center: Vec2, radius: f32, color: Color, model: Mat3) {
        if radius < 0.0 {
            return;
        }

        let center_s = model.transform_vec2(center); // float, screen space

        if radius == 0.0 {
            let (ix, iy) = quantize_point(center_s);
            self.set_pixel((ix, iy), color);
            return;
        }

        let mut x = radius;
        let mut y = 0.0;
        #[allow(non_snake_case)]
        let mut D = 1.0 - radius;

        while x >= y {
            // 4 symmetric spans (covering all 8 octants)
            let (iy1, x0i1, x1i1) = quantize_hspan(center_s.y + y, center_s.x - x, center_s.x + x);
            self.hspan(iy1, x0i1, x1i1, color);

            let (iy2, x0i2, x1i2) = quantize_hspan(center_s.y - y, center_s.x - x, center_s.x + x);
            self.hspan(iy2, x0i2, x1i2, color);

            let (iy3, x0i3, x1i3) = quantize_hspan(center_s.y + x, center_s.x - y, center_s.x + y);
            self.hspan(iy3, x0i3, x1i3, color);

            let (iy4, x0i4, x1i4) = quantize_hspan(center_s.y - x, center_s.x - y, center_s.x + y);
            self.hspan(iy4, x0i4, x1i4, color);

            // Midpoint step: N vs NW
            y += 1.0;
            if D < 0.0 {
                // Go North
                D += 2.0 * y + 1.0;
            } else {
                // Go North-West
                x -= 1.0;
                D += 2.0 * (y - x) + 1.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::FrameBuffer;
    use std::collections::HashSet;

    fn collect_circle(center: Vec2, radius: f32) -> HashSet<(i32, i32)> {
        let mut fb = FrameBuffer::new(96, 96);
        {
            let mut renderer = Renderer::new(&mut fb);
            renderer.draw_circle(center, radius, Color::WHITE, Mat3::IDENTITY);
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
    fn circle_draws_symmetrically() {
        let center = Vec2::new(32.0, 24.0);
        let radius = 10.0;
        let samples = collect_circle(center, radius);

        let center_i = (center.x.round() as i32, center.y.round() as i32);
        assert!(samples.contains(&(center_i.0 + radius as i32, center_i.1)));
        assert!(samples.contains(&(center_i.0 - radius as i32, center_i.1)));
        assert!(samples.contains(&(center_i.0, center_i.1 + radius as i32)));
        assert!(samples.contains(&(center_i.0, center_i.1 - radius as i32)));

        for &(x, y) in &samples {
            let dx = x - center_i.0;
            let dy = y - center_i.1;
            let mirrored = [
                (center_i.0 + dx, center_i.1 - dy),
                (center_i.0 - dx, center_i.1 + dy),
                (center_i.0 - dx, center_i.1 - dy),
                (center_i.0 + dy, center_i.1 + dx),
                (center_i.0 - dy, center_i.1 + dx),
                (center_i.0 + dy, center_i.1 - dx),
                (center_i.0 - dy, center_i.1 - dx),
            ];

            assert!(
                mirrored.iter().any(|m| samples.contains(m)),
                "missing symmetry for ({x},{y})"
            );
        }
    }

    #[test]
    fn circle_respects_bounds() {
        let center = Vec2::new(30.0, 28.0);
        let radius = 12.0;
        let samples = collect_circle(center, radius);

        let center_i = (center.x.round() as i32, center.y.round() as i32);
        for &(x, y) in &samples {
            let dx = x - center_i.0;
            let dy = y - center_i.1;
            assert!(
                dx.abs() <= radius as i32 && dy.abs() <= radius as i32,
                "point outside bounding square"
            );
        }
    }

    #[test]
    fn circle_with_zero_radius_draws_single_pixel() {
        let center = Vec2::new(10.0, 15.0);
        let radius = 0.0;
        let samples = collect_circle(center, radius);

        assert_eq!(samples.len(), 1);
        assert!(samples.contains(&(center.x.round() as i32, center.y.round() as i32)));
    }

    #[test]
    fn circle_with_fractional_center() {
        let center = Vec2::new(10.5, 15.7);
        let radius = 5.0;
        let samples = collect_circle(center, radius);

        // Should still draw a valid circle
        assert!(!samples.is_empty());

        // Center should be rounded to (11, 16)
        let center_i = (center.x.round() as i32, center.y.round() as i32);
        assert_eq!(center_i, (11, 16));
    }

    #[test]
    fn circle_with_fractional_radius() {
        let center = Vec2::new(20.0, 20.0);
        let radius = 5.5;
        let samples = collect_circle(center, radius);

        // Should draw a circle with radius 5.5
        assert!(!samples.is_empty());

        // Should have points at approximately the right distance
        let center_i = (center.x.round() as i32, center.y.round() as i32);
        let has_radius_points = samples.iter().any(|&(x, y)| {
            let dx = x - center_i.0;
            let dy = y - center_i.1;
            let dist = ((dx * dx + dy * dy) as f32).sqrt();
            dist >= 5.0 && dist <= 6.0
        });
        assert!(
            has_radius_points,
            "Should have points at the expected radius"
        );
    }
}
