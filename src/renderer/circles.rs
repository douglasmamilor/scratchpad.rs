use crate::color::Color;
use crate::renderer::Renderer;

impl<'a> Renderer<'a> {
    /// Draws the outline of a circle using the midpoint algorithm.
    ///
    /// `ctr` (the centre) is specified in screen space (integer pixels). `r` the radius must be non-negative.
    pub fn draw_circle(&mut self, ctr: (i32, i32), r: i32, color: &Color) {
        if r < 0 {
            return;
        }

        let (cx, cy) = ctr;
        if r == 0 {
            self.set_pixel((cx, cy), color);
            return;
        }

        let mut x = r;
        let mut y = 0;
        #[allow(non_snake_case)]
        let mut D = 1 - r;

        let mut plot = |cx: i32, cy: i32, x: i32, y: i32| {
            self.set_pixel((cx + x, cy + y), color); // 1st octant (+x direction)
            self.set_pixel((cx - x, cy + y), color); // reflect across y-axis
            self.set_pixel((cx + x, cy - y), color); // reflect across x-axis
            self.set_pixel((cx - x, cy - y), color); // reflect across both axes

            self.set_pixel((cx + y, cy + x), color); // reflect across line y=x
            self.set_pixel((cx - y, cy + x), color); // reflect across line y=x then across y-axis
            self.set_pixel((cx + y, cy - x), color); // reflect across y=x, then across x-axis
            self.set_pixel((cx - y, cy - x), color); // reflect across line y=-x
        };

        while x >= y {
            plot(cx, cy, x, y);

            y += 1;
            if D < 0 {
                // Go North
                D += 2 * y + 1;
            } else {
                // Go North-West
                x -= 1;
                D += 2 * (y - x) + 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::FrameBuffer;
    use std::collections::HashSet;

    fn collect_circle(center: (i32, i32), radius: i32) -> HashSet<(i32, i32)> {
        let mut fb = FrameBuffer::new(96, 96);
        {
            let mut renderer = Renderer::new(&mut fb);
            renderer.draw_circle(center, radius, &Color::WHITE);
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
        let center = (32, 24);
        let radius = 10;
        let samples = collect_circle(center, radius);

        assert!(samples.contains(&(center.0 + radius, center.1)));
        assert!(samples.contains(&(center.0 - radius, center.1)));
        assert!(samples.contains(&(center.0, center.1 + radius)));
        assert!(samples.contains(&(center.0, center.1 - radius)));

        for &(x, y) in &samples {
            let dx = x - center.0;
            let dy = y - center.1;
            let mirrored = [
                (center.0 + dx, center.1 - dy),
                (center.0 - dx, center.1 + dy),
                (center.0 - dx, center.1 - dy),
                (center.0 + dy, center.1 + dx),
                (center.0 - dy, center.1 + dx),
                (center.0 + dy, center.1 - dx),
                (center.0 - dy, center.1 - dx),
            ];

            assert!(
                mirrored.iter().any(|m| samples.contains(m)),
                "missing symmetry for ({x},{y})"
            );
        }
    }

    #[test]
    fn circle_respects_bounds() {
        let center = (30, 28);
        let radius = 12;
        let samples = collect_circle(center, radius);

        for &(x, y) in &samples {
            let dx = x - center.0;
            let dy = y - center.1;
            assert!(
                dx.abs() <= radius && dy.abs() <= radius,
                "point outside bounding square"
            );
        }
    }

    #[test]
    fn circle_with_zero_radius_draws_single_pixel() {
        let center = (10, 15);
        let radius = 0;
        let samples = collect_circle(center, radius);

        assert_eq!(samples.len(), 1);
        assert!(samples.contains(&center));
    }
}
