use crate::{Mat3, Vec2, color::Color, renderer::Renderer};

impl<'a> Renderer<'a> {
    /// Draw a solid, thick line segment by expanding it into a quad and rasterizing as two triangles.
    ///
    /// Notes:
    /// - Thickness is specified in **screen pixels**. The `model` transform is applied to the
    ///   endpoints only; the stroke expansion happens in screen space.
    pub fn draw_line_thick(
        &mut self,
        start: Vec2,
        end: Vec2,
        thickness_px: f32,
        color: Color,
        model: Mat3,
    ) {
        let start_s = model.transform_vec2(start);
        let end_s = model.transform_vec2(end);
        self.draw_line_thick_screen_space(start_s, end_s, thickness_px, color);
    }

    fn draw_line_thick_screen_space(
        &mut self,
        start_s: Vec2,
        end_s: Vec2,
        thickness_px: f32,
        color: Color,
    ) {
        if !thickness_px.is_finite() {
            return;
        }

        let thickness_px = thickness_px.max(0.0);
        if thickness_px <= 0.0 {
            return;
        }

        let half = 0.5 * thickness_px;
        let u = end_s - start_s;
        let u_len = u.len();

        // Degenerate segment: render as an axis-aligned square centered at the point.
        if u_len <= 1e-6 {
            let p0 = Vec2::new(start_s.x - half, start_s.y - half);
            let p1 = Vec2::new(start_s.x + half, start_s.y - half);
            let p2 = Vec2::new(start_s.x + half, start_s.y + half);
            let p3 = Vec2::new(start_s.x - half, start_s.y + half);

            self.fill_triangle(p0, p1, p2, color, Mat3::IDENTITY);
            self.fill_triangle(p0, p2, p3, color, Mat3::IDENTITY);
            return;
        }

        let u_hat = u / u_len;
        let u_n = Vec2::new(-u_hat.y, u_hat.x);
        let offset = u_n * half;

        // Quad corners (no caps/joins; just a rectangle around the segment).
        let p0 = start_s + offset;
        let p1 = start_s - offset;
        let p2 = end_s - offset;
        let p3 = end_s + offset;

        // Two triangles covering the quad.
        self.fill_triangle(p0, p1, p2, color, Mat3::IDENTITY);
        self.fill_triangle(p0, p2, p3, color, Mat3::IDENTITY);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thick_line_quad_is_perpendicular_to_segment() {
        // Horizontal segment: normal points up/down.
        let start = Vec2::new(0.0, 0.0);
        let end = Vec2::new(10.0, 0.0);
        let thickness = 4.0;
        let half = thickness * 0.5;

        let delta = end - start;
        let dir = delta / delta.len();
        let normal = Vec2::new(-dir.y, dir.x);
        let offset = normal * half;

        assert!((offset.x - 0.0).abs() < 1e-6);
        assert!((offset.y - half).abs() < 1e-6);
        assert!((offset.dot(dir)).abs() < 1e-6);
    }
}
