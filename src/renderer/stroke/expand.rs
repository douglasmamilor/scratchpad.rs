use crate::renderer::stroke::types::{LineCap, StrokeSpace, StrokeStyle};
use crate::{Mat3, Vec2, color::Color, renderer::Renderer};

impl<'a> Renderer<'a> {
    /// Convenience wrapper that matches your old signature.
    /// Internally uses the canonical style-based API.
    pub fn draw_line_thick(
        &mut self,
        start: Vec2,
        end: Vec2,
        thickness_px: f32,
        color: Color,
        model: Mat3,
    ) {
        let style = StrokeStyle::solid_screen_px(thickness_px, color);
        self.stroke_line(start, end, &style, model);
    }

    /// Canonical API for a line segment: styling is controlled by StrokeStyle.
    ///
    /// Currently implemented:
    /// - ScreenSpace thickness
    /// - Caps: Butt, Square, Round
    ///
    /// Not yet implemented (but your style already carries them):
    /// - WorldSpace thickness (you can implement later by transforming normals)
    /// - Joins / miters (only relevant for polylines)
    /// - Patterns (apply_stroke_pattern works on polylines; lines can be treated as a 2-point polyline later)
    pub fn stroke_line(&mut self, start: Vec2, end: Vec2, style: &StrokeStyle, model: Mat3) {
        let thickness_px = match *style.space() {
            StrokeSpace::ScreenSpace { thickness } => thickness as f32,
            StrokeSpace::WorldSpace { thickness } => thickness as f32, // TODO: proper world-space stroking
        };

        let start_s = model.transform_vec2(start);
        let end_s = model.transform_vec2(end);

        self.stroke_line_screen_space(start_s, end_s, thickness_px, style.cap(), style.color());
    }

    fn stroke_line_screen_space(
        &mut self,
        mut start_s: Vec2,
        mut end_s: Vec2,
        thickness_px: f32,
        cap: LineCap,
        color: Color,
    ) {
        // Defensive guard: avoid NaN/∞ poisoning.
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
        if !u_len.is_finite() {
            return;
        }

        // Degenerate segment: render as a dot (cap decides dot shape).
        if u_len <= 1e-6 {
            match cap {
                LineCap::Round => {
                    self.fill_circle(start_s, half, color, Mat3::IDENTITY);
                }
                LineCap::Butt | LineCap::Square => {
                    // Square “dot” centered at the point.
                    let p0 = Vec2::new(start_s.x - half, start_s.y - half);
                    let p1 = Vec2::new(start_s.x + half, start_s.y - half);
                    let p2 = Vec2::new(start_s.x + half, start_s.y + half);
                    let p3 = Vec2::new(start_s.x - half, start_s.y + half);

                    self.fill_triangle(p0, p1, p2, color, Mat3::IDENTITY);
                    self.fill_triangle(p0, p2, p3, color, Mat3::IDENTITY);
                }
            }
            return;
        }

        let u_hat = u / u_len;

        // Square caps extend endpoints by half-thickness along the segment direction.
        if cap == LineCap::Square {
            start_s -= u_hat * half;
            end_s += u_hat * half;
        }

        // Expand into a quad around the segment.
        let n = Vec2::new(-u_hat.y, u_hat.x);
        let offset = n * half;

        let p0 = start_s + offset;
        let p1 = start_s - offset;
        let p2 = end_s - offset;
        let p3 = end_s + offset;

        self.fill_triangle(p0, p1, p2, color, Mat3::IDENTITY);
        self.fill_triangle(p0, p2, p3, color, Mat3::IDENTITY);

        // Round caps: your fill_circle is span-based, so this is fine (no tessellation).
        if cap == LineCap::Round {
            self.fill_circle(start_s, half, color, Mat3::IDENTITY);
            self.fill_circle(end_s, half, color, Mat3::IDENTITY);
        }
    }
}

// ------------------------------
// Tests (keep yours, optional additions)
// ------------------------------

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

    #[test]
    fn thick_line_offset_is_perpendicular() {
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
