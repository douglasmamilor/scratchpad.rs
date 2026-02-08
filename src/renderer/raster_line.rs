use super::Renderer;
use crate::Color;
use crate::math::{IVec2, Mat3, vec2::Vec2};

impl<'a> Renderer<'a> {
    fn visit_line_points<F>(start: (i32, i32), end: (i32, i32), mut visit: F)
    where
        F: FnMut((i32, i32)) -> bool,
    {
        let (mut x0, mut y0) = (start.0, start.1);
        let (mut x1, mut y1) = (end.0, end.1);

        // Determine if the line is steep (> 45 degrees) and normalise coordinates
        let steep = (y1 - y0).abs() > (x1 - x0).abs();
        if steep {
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
        }

        // If moving right to left, swap start and end points
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        // separate step direction from change in y
        let dx = x1 - x0;
        let dy = (y1 - y0).abs();
        let y_step = if (y1 - y0) >= 0 { 1 } else { -1 };

        #[allow(non_snake_case)]
        let mut D = 2 * dy - dx;
        let mut y = y0;

        for x in x0..=x1 {
            let point = if steep { (y, x) } else { (x, y) };
            if !visit(point) {
                break;
            }

            if D >= 0 {
                y += y_step;
                D += 2 * (dy - dx);
            } else {
                D += 2 * dy;
            }
        }
    }

    // draw line using Bresenham's line drawing algorithm returning visited points
    pub fn plot_line(start: &(i32, i32), end: &(i32, i32)) -> Vec<(i32, i32)> {
        let mut points = Vec::new();
        // Bresenham generates the pixels we want to visit; add them to the result vec
        Renderer::visit_line_points(*start, *end, |point| {
            points.push(point);
            true
        });
        points
    }

    /// Draws a crisp 1-pixel line using Bresenham's algorithm.
    ///
    /// Uses integer coordinates for pixel-perfect drawing.
    /// This is the fast, crisp counterpart to `draw_line_aa`.
    ///
    /// # Examples
    /// ```
    /// use scratchpad_rs::math::{IVec2, Mat3};
    /// use scratchpad_rs::color::Color;
    /// use scratchpad_rs::framebuffer::FrameBuffer;
    /// use scratchpad_rs::renderer::Renderer;
    ///
    /// let mut frame_buffer = FrameBuffer::new(100, 100);
    /// let mut renderer = Renderer::new(&mut frame_buffer);
    ///
    /// // Draw crisp line from (10, 10) to (50, 30)
    /// renderer.draw_line_pixel(IVec2::new(10, 10), IVec2::new(50, 30), Color::RED, Mat3::IDENTITY);
    /// ```
    pub fn draw_line_pixel(&mut self, a: IVec2, b: IVec2, color: Color, model: Mat3) {
        let a_s = model.transform_vec2(Vec2::new(a.x as f32, a.y as f32)); // float, screen space
        let b_s = model.transform_vec2(Vec2::new(b.x as f32, b.y as f32));

        // Convert back to integer coordinates for pixel-perfect drawing
        let a_pixel = (a_s.x.round() as i32, a_s.y.round() as i32);
        let b_pixel = (b_s.x.round() as i32, b_s.y.round() as i32);

        // Note: the visitor lets us draw without allocating the intermediate Vec
        Renderer::visit_line_points(a_pixel, b_pixel, |point| {
            self.set_pixel(point, color);
            true
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_line() {
        let start = (0, 0);
        let end = (5, 0);
        let points = Renderer::plot_line(&start, &end);

        let expected = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)];
        assert_eq!(points, expected);
    }

    #[test]
    fn test_vertical_line() {
        let start = (0, 0);
        let end = (0, 5);
        let points = Renderer::plot_line(&start, &end);

        let expected = vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)];
        assert_eq!(points, expected);
    }

    #[test]
    fn test_diagonal_line() {
        let start = (0, 0);
        let end = (3, 3);
        let points = Renderer::plot_line(&start, &end);

        let expected = vec![(0, 0), (1, 1), (2, 2), (3, 3)];
        assert_eq!(points, expected);
    }

    #[test]
    fn test_reverse_direction() {
        let start = (5, 0);
        let end = (0, 0);
        let points = Renderer::plot_line(&start, &end);

        let expected = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)];
        assert_eq!(points, expected);
    }

    #[test]
    fn test_steep_line() {
        let start = (0, 0);
        let end = (2, 5);
        let points = Renderer::plot_line(&start, &end);

        // For steep lines, coordinates are swapped
        let expected = vec![(0, 0), (0, 1), (1, 2), (1, 3), (2, 4), (2, 5)];
        assert_eq!(points, expected);
    }

    #[test]
    fn test_single_point() {
        let start = (5, 5);
        let end = (5, 5);
        let points = Renderer::plot_line(&start, &end);

        assert_eq!(points, vec![(5, 5)]);
    }

    #[test]
    fn test_negative_coordinates() {
        let start = (-2, -2);
        let end = (2, 2);
        let points = Renderer::plot_line(&start, &end);

        let expected = vec![(-2, -2), (-1, -1), (0, 0), (1, 1), (2, 2)];
        assert_eq!(points, expected);
    }
}
