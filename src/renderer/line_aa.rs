use super::Renderer;
use crate::color::Color;
use crate::math::{Mat3, vec2::Vec2};

#[inline]
fn ipart(x: f32) -> i32 {
    x.floor() as i32
}
#[inline]
pub fn roundi(x: f32) -> i32 {
    (x + 0.5).floor() as i32
}
#[inline]
fn fpart(x: f32) -> f32 {
    x - x.floor()
}
#[inline]
fn rfpart(x: f32) -> f32 {
    let frac = fpart(x);
    if frac == 0.0 { 1.0 } else { 1.0 - frac }
}

impl<'a> Renderer<'a> {
    fn visit_line_points_aa<F>(start: (f32, f32), end: (f32, f32), mut visit: F)
    where
        F: FnMut((i32, i32, f32)) -> bool,
    {
        let (mut x0, mut y0) = start;
        let (mut x1, mut y1) = end;

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

        // Early return if the segment collapses to a single point
        if (x0 == x1) && (y0 == y1) {
            let point = if steep {
                (y0 as i32, x0 as i32, 1.0)
            } else {
                (x0 as i32, y0 as i32, 1.0)
            };
            let _ = visit(point);
            return;
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let m = dy / dx;

        // Each column can emit up to two pixels with different coverage values
        let mut emit = |x: i32, y: i32, coverage: f32| -> bool {
            let point = if steep {
                (y, x, coverage)
            } else {
                (x, y, coverage)
            };
            visit(point)
        };

        // Step 2: Process head endpoint (i.e endpoint at x0)
        let mut x_endpt = roundi(x0);
        let mut y_endpt = y0 + m * (x_endpt as f32 - x0);
        let mut x_endpt_gap = rfpart(x0 + 0.5);
        let x_start = x_endpt;
        let y_start = ipart(y_endpt);

        if !emit(x_start, y_start, rfpart(y_endpt) * x_endpt_gap) {
            return;
        }
        if !emit(x_start, y_start + 1, fpart(y_endpt) * x_endpt_gap) {
            return;
        }

        let mut y_nxt = y_endpt + m; // unit step in x direction

        // Step 3: Process tail endpoint (i.e endpoint at x1)
        x_endpt = roundi(x1);
        y_endpt = y1 + m * (x_endpt as f32 - x1);
        x_endpt_gap = fpart(x1 + 0.5);
        let x_end = x_endpt;
        let y_end = ipart(y_endpt);

        // Step 4: Process intermediate pixels
        for x in (x_start + 1)..x_end {
            if !emit(x, ipart(y_nxt), rfpart(y_nxt)) {
                return;
            }
            if !emit(x, ipart(y_nxt) + 1, fpart(y_nxt)) {
                return;
            }

            y_nxt += m;
        }

        // Step 5: Process tail endpoint (i.e endpoint at x1)
        if !emit(x_end, y_end, rfpart(y_endpt) * x_endpt_gap) {
            return;
        }
        if !emit(x_end, y_end + 1, fpart(y_endpt) * x_endpt_gap) {}
    }

    // draw antialiased line using Wu's line drawing algorithm returning visited points
    pub fn plot_line_aa(start: &(f32, f32), end: &(f32, f32)) -> Vec<(i32, i32, f32)> {
        let start_floor = start.0.floor() as i32;
        let end_ceil = end.0.ceil() as i32;
        let capacity = ((end_ceil - start_floor + 1).max(0) * 2) as usize;
        let mut points: Vec<(i32, i32, f32)> = Vec::with_capacity(capacity);
        Renderer::visit_line_points_aa(*start, *end, |point| {
            points.push(point);
            true
        });
        points
    }

    pub fn draw_line_aa(&mut self, start: Vec2, end: Vec2, color: Color, model: Mat3) {
        let a_s = model.transform_vec2(start); // float, screen space
        let b_s = model.transform_vec2(end);

        let start_tuple = (a_s.x, a_s.y);
        let end_tuple = (b_s.x, b_s.y);

        Renderer::visit_line_points_aa(start_tuple, end_tuple, |(x, y, coverage)| {
            let t = coverage.clamp(0.0, 1.0);
            let scaled = Color::RGBA(
                (color.r as f32 * t).round() as u8,
                (color.g as f32 * t).round() as u8,
                (color.b as f32 * t).round() as u8,
                (color.a as f32 * t).round() as u8,
            );

            self.set_pixel((x, y), scaled);
            true
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_points(actual: &[(i32, i32, f32)], expected: &[(i32, i32, f32)]) {
        assert_eq!(actual.len(), expected.len());
        for (a, e) in actual.iter().zip(expected.iter()) {
            assert_eq!(a.0, e.0);
            assert_eq!(a.1, e.1);
            assert!(
                (a.2 - e.2).abs() < 1e-6,
                "coverage mismatch: {} != {}",
                a.2,
                e.2
            );
        }
    }

    #[test]
    fn test_horizontal_line_aa() {
        let points = Renderer::plot_line_aa(&(0.0, 0.0), &(5.0, 0.0));
        let expected = vec![
            (0, 0, 0.5),
            (0, 1, 0.0),
            (1, 0, 1.0),
            (1, 1, 0.0),
            (2, 0, 1.0),
            (2, 1, 0.0),
            (3, 0, 1.0),
            (3, 1, 0.0),
            (4, 0, 1.0),
            (4, 1, 0.0),
            (5, 0, 0.5),
            (5, 1, 0.0),
        ];
        assert_points(&points, &expected);
    }

    #[test]
    fn test_vertical_line_aa() {
        let points = Renderer::plot_line_aa(&(0.0, 0.0), &(0.0, 5.0));
        let expected = vec![
            (0, 0, 0.5),
            (1, 0, 0.0),
            (0, 1, 1.0),
            (1, 1, 0.0),
            (0, 2, 1.0),
            (1, 2, 0.0),
            (0, 3, 1.0),
            (1, 3, 0.0),
            (0, 4, 1.0),
            (1, 4, 0.0),
            (0, 5, 0.5),
            (1, 5, 0.0),
        ];
        assert_points(&points, &expected);
    }

    #[test]
    fn test_diagonal_line_aa() {
        let points = Renderer::plot_line_aa(&(0.0, 0.0), &(3.0, 3.0));
        let expected = vec![
            (0, 0, 0.5),
            (0, 1, 0.0),
            (1, 1, 1.0),
            (1, 2, 0.0),
            (2, 2, 1.0),
            (2, 3, 0.0),
            (3, 3, 0.5),
            (3, 4, 0.0),
        ];
        assert_points(&points, &expected);
    }

    #[test]
    fn test_reverse_direction_aa() {
        let forward = Renderer::plot_line_aa(&(0.0, 0.0), &(4.0, 2.0));
        let reverse = Renderer::plot_line_aa(&(4.0, 2.0), &(0.0, 0.0));
        assert_points(&forward, &reverse);
    }

    #[test]
    fn test_single_point_aa() {
        let points = Renderer::plot_line_aa(&(2.0, 3.0), &(2.0, 3.0));
        assert_eq!(points, vec![(2, 3, 1.0)]);
    }

    #[test]
    fn test_negative_coordinates_aa() {
        let points = Renderer::plot_line_aa(&(-2.0, -1.0), &(2.0, 1.0));
        let expected = vec![
            (-2, -1, 0.5),
            (-2, 0, 0.0),
            (-1, -1, 0.5),
            (-1, 0, 0.5),
            (0, 0, 1.0),
            (0, 1, 0.0),
            (1, 0, 0.5),
            (1, 1, 0.5),
            (2, 1, 0.5),
            (2, 2, 0.0),
        ];
        assert_points(&points, &expected);
    }
}
