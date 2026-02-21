use crate::Color;
use crate::math::{Mat3, vec2::Vec2};
use crate::renderer::{Renderer, quantize_hspan, quantize_point, quantize_vspan};

impl<'a> Renderer<'a> {
    /// Draw outline for axis-aligned ellipse via midpoint (Bresenham-style) algorithm.
    ///
    /// `center` is specified as a Vec2 in floating-point coordinates.
    /// `rx` and `ry` are the horizontal and vertical radii, must be non-negative.
    ///
    /// # Examples
    /// ```
    /// use scratchpad_rs::math::{Vec2, Mat3};
/// use scratchpad_rs::image::Color;
    /// use scratchpad_rs::framebuffer::FrameBuffer;
    /// use scratchpad_rs::renderer::Renderer;
    ///
    /// let mut frame_buffer = FrameBuffer::new(100, 100);
    /// let mut renderer = Renderer::new(&mut frame_buffer);
    ///
    /// // Draw ellipse at (100.5, 50.0) with radii 30.0 and 20.0
    /// renderer.draw_ellipse(Vec2::new(100.5, 50.0), 30.0, 20.0, Color::RED, Mat3::IDENTITY);
    /// ```
    pub fn draw_ellipse(&mut self, center: Vec2, rx: f32, ry: f32, color: Color, model: Mat3) {
        if rx < 0.0 && ry < 0.0 {
            return;
        }

        let center_s = model.transform_vec2(center); // float, screen space

        // Degenerate case: single point
        if rx == 0.0 && ry == 0.0 {
            let (ix, iy) = quantize_point(center_s);
            self.set_pixel((ix, iy), color);
            return;
        }

        // Degenerate case: horizontal line
        if ry == 0.0 {
            let (iy, x0i, x1i) = quantize_hspan(center_s.y, center_s.x - rx, center_s.x + rx);
            self.hspan(iy, x0i, x1i, color);
            return;
        }

        // Degenerate case: vertical line
        if rx == 0.0 {
            let (ix, y0i, y1i) = quantize_vspan(center_s.x, center_s.y - ry, center_s.y + ry);
            self.vspan(ix, y0i, y1i, color);
            return;
        }

        let rx2 = rx * rx;
        let ry2 = ry * ry;
        let two_rx2 = 2.0 * rx2;
        let two_ry2 = 2.0 * ry2;

        let mut x = 0.0;
        let mut y = ry;

        let mut px = 0.0;
        let mut py = two_rx2 * y;

        // Region 1 decision parameter
        let mut p1 = ry2 - rx2 * y + (rx2 / 4.0);

        // Helper: plot 4 symmetric points
        let mut plot4 = |cx: f32, cy: f32, x: f32, y: f32| {
            let (ix1, iy1) = quantize_point(Vec2::new(cx + x, cy + y));
            let (ix2, iy2) = quantize_point(Vec2::new(cx - x, cy + y));
            let (ix3, iy3) = quantize_point(Vec2::new(cx + x, cy - y));
            let (ix4, iy4) = quantize_point(Vec2::new(cx - x, cy - y));

            self.set_pixel((ix1, iy1), color);
            self.set_pixel((ix2, iy2), color);
            self.set_pixel((ix3, iy3), color);
            self.set_pixel((ix4, iy4), color);
        };

        // ----------------
        // Region 1: |dy/dx| <= 1  (px < py) (advancing x)
        // ----------------
        while px < py {
            plot4(center_s.x, center_s.y, x, y);

            x += 1.0;
            px += two_ry2;

            if p1 < 0.0 {
                // E step
                p1 += px + ry2;
            } else {
                // SE step
                y -= 1.0;
                py -= two_rx2;
                p1 += px - py + ry2;
            }
        }

        // ----------------
        // Region 2 init
        // p2 = ry^2*(x+0.5)^2 + rx^2*(y-1)^2 - rx^2*ry^2
        // ----------------
        let mut p2 = ry2 * (x * x + x) + (ry2 / 4.0) + rx2 * (y * y - 2.0 * y + 1.0) - rx2 * ry2;

        // ----------------
        // Region 2: |dy/dx| > 1  (descending y)
        // ----------------
        while y >= 0.0 {
            plot4(center_s.x, center_s.y, x, y);

            y -= 1.0;
            py -= two_rx2;

            if p2 > 0.0 {
                // S step
                p2 += rx2 - py;
            } else {
                // SE step
                x += 1.0;
                px += two_ry2;
                p2 += px - py + rx2;
            }
        }
    }

    /// Filled, axis-aligned ellipse via midpoint (Bresenham-style) algorithm.
    ///
    /// `center` is specified as a Vec2 in floating-point coordinates.
    /// `rx` and `ry` are the horizontal and vertical radii, must be non-negative.
    ///
    /// # Examples
    /// ```
    /// use scratchpad_rs::math::{Vec2, Mat3};
/// use scratchpad_rs::image::Color;
    /// use scratchpad_rs::framebuffer::FrameBuffer;
    /// use scratchpad_rs::renderer::Renderer;
    ///
    /// let mut frame_buffer = FrameBuffer::new(100, 100);
    /// let mut renderer = Renderer::new(&mut frame_buffer);
    ///
    /// // Fill ellipse at (100.5, 50.0) with radii 30.0 and 20.0
    /// renderer.fill_ellipse(Vec2::new(100.5, 50.0), 30.0, 20.0, Color::BLUE, Mat3::IDENTITY);
    /// ```
    pub fn fill_ellipse(&mut self, center: Vec2, rx: f32, ry: f32, color: Color, model: Mat3) {
        if rx < 0.0 || ry < 0.0 {
            return;
        }

        let center_s = model.transform_vec2(center); // float, screen space

        // Degenerates
        if rx == 0.0 && ry == 0.0 {
            let (ix, iy) = quantize_point(center_s);
            self.set_pixel((ix, iy), color);
            return;
        }
        if ry == 0.0 {
            // single horizontal span on y = cy
            let (iy, x0i, x1i) = quantize_hspan(center_s.y, center_s.x - rx, center_s.x + rx);
            self.hspan(iy, x0i, x1i, color);
            return;
        }
        if rx == 0.0 {
            // vertical line on x = cx
            let (ix, y0i, y1i) = quantize_vspan(center_s.x, center_s.y - ry, center_s.y + ry);
            self.vspan(ix, y0i, y1i, color);
            return;
        }

        let rx2 = rx * rx;
        let ry2 = ry * ry;
        let two_rx2 = 2.0 * rx2;
        let two_ry2 = 2.0 * ry2;

        // Start at (x, y) = (0, ry) in first quadrant
        let mut x = 0.0;
        let mut y = ry;

        // Accumulators for region switch
        let mut px = 0.0; // = 2*ry^2*x
        let mut py = two_rx2 * y; // = 2*rx^2*y

        // Region 1 decision parameter (midpoint)
        let mut p1 = ry2 - rx2 * y + (rx2 / 4.0);

        // Helper that draws the two horizontal spans for current (x, y)
        let draw_span_pair = |cx: f32, cy: f32, x: f32, y: f32, this: &mut Renderer<'a>| {
            if y == 0.0 {
                // Both rows coincide at y = cy
                let (iy, x0i, x1i) = quantize_hspan(cy, cx - x, cx + x);
                this.hspan(iy, x0i, x1i, color);
            } else {
                let (iy1, x0i1, x1i1) = quantize_hspan(cy + y, cx - x, cx + x);
                let (iy2, x0i2, x1i2) = quantize_hspan(cy - y, cx - x, cx + x);
                this.hspan(iy1, x0i1, x1i1, color);
                this.hspan(iy2, x0i2, x1i2, color);
            }
        };

        // -------------
        // Region 1: |dy/dx| <= 1 (advance x)
        // -------------
        while px < py {
            draw_span_pair(center_s.x, center_s.y, x, y, self);

            x += 1.0;
            px += two_ry2;

            if p1 < 0.0 {
                // E
                p1 += px + ry2;
            } else {
                // SE
                y -= 1.0;
                py -= two_rx2;
                p1 += px - py + ry2;
            }
        }

        // -------------
        // Region 2 init:
        // p2 = ry^2*(x+0.5)^2 + rx^2*(y-1)^2 - rx^2*ry^2
        // -------------
        let mut p2 = ry2 * (x * x + x) + (ry2 / 4.0) + rx2 * (y * y - 2.0 * y + 1.0) - rx2 * ry2;

        // -------------
        // Region 2: |dy/dx| > 1 (advance y)
        // -------------
        while y >= 0.0 {
            draw_span_pair(center_s.x, center_s.y, x, y, self);

            y -= 1.0;
            py -= two_rx2;

            if p2 > 0.0 {
                // S
                p2 += rx2 - py;
            } else {
                // SE
                x += 1.0;
                px += two_ry2;
                p2 += px - py + rx2;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::FrameBuffer;
    use std::collections::HashSet;

    fn collect_ellipse(center: Vec2, rx: f32, ry: f32) -> HashSet<(i32, i32)> {
        let mut fb = FrameBuffer::new(96, 96);
        {
            let mut renderer = Renderer::new(&mut fb);
            renderer.draw_ellipse(center, rx, ry, Color::WHITE, Mat3::IDENTITY);
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
    fn ellipse_draws_symmetrically() {
        let center = Vec2::new(32.0, 24.0);
        let rx = 15.0;
        let ry = 10.0;
        let samples = collect_ellipse(center, rx, ry);

        let center_i = (center.x.round() as i32, center.y.round() as i32);
        assert!(samples.contains(&(center_i.0 + rx as i32, center_i.1)));
        assert!(samples.contains(&(center_i.0 - rx as i32, center_i.1)));
        assert!(samples.contains(&(center_i.0, center_i.1 + ry as i32)));
        assert!(samples.contains(&(center_i.0, center_i.1 - ry as i32)));

        for &(x, y) in &samples {
            let dx = x - center_i.0;
            let dy = y - center_i.1;
            let mirrored = [
                (center_i.0 + dx, center_i.1 - dy),
                (center_i.0 - dx, center_i.1 + dy),
                (center_i.0 - dx, center_i.1 - dy),
            ];

            assert!(
                mirrored.iter().any(|m| samples.contains(m)),
                "missing symmetry for ({x},{y})"
            );
        }
    }

    #[test]
    fn ellipse_respects_bounds() {
        let center = Vec2::new(30.0, 28.0);
        let rx = 12.0;
        let ry = 8.0;
        let samples = collect_ellipse(center, rx, ry);

        let center_i = (center.x.round() as i32, center.y.round() as i32);
        for &(x, y) in &samples {
            let dx = x - center_i.0;
            let dy = y - center_i.1;
            assert!(
                dx.abs() <= rx as i32 && dy.abs() <= ry as i32,
                "point outside bounding rectangle"
            );
        }
    }

    #[test]
    fn ellipse_with_zero_radii_draws_single_pixel() {
        let center = Vec2::new(10.0, 15.0);
        let samples = collect_ellipse(center, 0.0, 0.0);

        assert_eq!(samples.len(), 1);
        assert!(samples.contains(&(center.x.round() as i32, center.y.round() as i32)));
    }

    #[test]
    fn ellipse_with_fractional_center() {
        let center = Vec2::new(10.5, 15.7);
        let rx = 5.0;
        let ry = 3.0;
        let samples = collect_ellipse(center, rx, ry);

        // Should still draw a valid ellipse
        assert!(!samples.is_empty());

        // Center should be rounded to (11, 16)
        let center_i = (center.x.round() as i32, center.y.round() as i32);
        assert_eq!(center_i, (11, 16));
    }

    #[test]
    fn ellipse_with_fractional_radii() {
        let center = Vec2::new(20.0, 20.0);
        let rx = 5.5;
        let ry = 3.2;
        let samples = collect_ellipse(center, rx, ry);

        // Should draw an ellipse with fractional radii
        assert!(!samples.is_empty());

        // Should have points at approximately the right distance
        let center_i = (center.x.round() as i32, center.y.round() as i32);
        let has_radius_points = samples.iter().any(|&(x, y)| {
            let dx = x - center_i.0;
            let dy = y - center_i.1;
            let dist_x = dx.abs() as f32;
            let dist_y = dy.abs() as f32;
            (dist_x >= 5.0 && dist_x <= 6.0) || (dist_y >= 3.0 && dist_y <= 4.0)
        });
        assert!(
            has_radius_points,
            "Should have points at the expected radii"
        );
    }

    #[test]
    fn ellipse_degenerate_cases() {
        let center = Vec2::new(20.0, 20.0);

        // Horizontal line
        let samples_h = collect_ellipse(center, 5.0, 0.0);
        assert!(!samples_h.is_empty());

        // Vertical line
        let samples_v = collect_ellipse(center, 0.0, 5.0);
        assert!(!samples_v.is_empty());
    }
}
