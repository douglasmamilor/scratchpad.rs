use crate::color::Color;
use crate::renderer::Renderer;
use crate::renderer::lines_aa::roundi;

impl<'a> Renderer<'a> {
    pub fn draw_circle(&mut self, ctr: (f32, f32), r: f32) {
        // TODO: should we allow floats? What would that look like? What would it involve?
        let ri = roundi(r);
        let d = ri * 2;
        let (cx, cy) = (roundi(ctr.0), roundi(ctr.1));
        let (x0, y0) = (cx - ri, cy - ri);
        let (x, _y) = (x0 + d - 1, y0 + d - 1);

        let mut yi;
        let mut myi; // mirror of yi
        for xi in x0..=x {
            yi =
                ctr.1 as i32 + roundi((r * r - ((xi as f32 - ctr.0) * (xi as f32 - ctr.0))).sqrt());
            myi = 2 * cy - yi;
            // myi = cy - (yi - cy); // Same thing

            self.set_pixel((xi, yi), &Color::WHITE);
            self.set_pixel((xi, myi), &Color::WHITE);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::FrameBuffer;
    use std::collections::HashSet;

    fn collect_circle_points(center: (f32, f32), radius: f32) -> Vec<(i32, i32)> {
        let mut fb = FrameBuffer::new(64, 64);
        {
            let mut renderer = Renderer::new(&mut fb);
            renderer.draw_circle(center, radius);
        }

        let mut points = Vec::new();
        for y in 0..fb.height() as i32 {
            for x in 0..fb.width() as i32 {
                if fb.get_pixel(x as usize, y as usize).unwrap_or(0) != 0 {
                    points.push((x, y));
                }
            }
        }
        points
    }

    #[test]
    fn circle_has_vertical_symmetry() {
        let center = (20.3_f32, 18.7_f32);
        let radius = 6.0_f32;
        let points = collect_circle_points(center, radius);

        assert!(!points.is_empty(), "circle should plot at least one pixel");

        let cy = roundi(center.1);
        let point_set: HashSet<_> = points.iter().copied().collect();

        for &(x, y) in &points {
            let mirror = (x, 2 * cy - y);
            assert!(point_set.contains(&mirror), "missing mirror of ({x},{y}) -> {mirror:?}");
        }
    }

    #[test]
    fn circle_points_stay_within_bounding_box() {
        let center = (12.0_f32, 10.0_f32);
        let radius = 5.0_f32;
        let points = collect_circle_points(center, radius);

        let cx = roundi(center.0);
        let cy = roundi(center.1);
        let ri = roundi(radius);

        for &(x, y) in &points {
            assert!(x >= cx - ri && x <= cx + ri, "x out of bounds: {x}");
            assert!(y >= cy - ri && y <= cy + ri, "y out of bounds: {y}");
        }
    }
}
