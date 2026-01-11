use crate::renderer::stroke::types::{LineCap, StrokeSpace, StrokeStyle};
use crate::{Mat3, Vec2, color::Color, renderer::Renderer};

impl<'a> Renderer<'a> {
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

    pub fn stroke_line(&mut self, start: Vec2, end: Vec2, style: &StrokeStyle, model: Mat3) {
        match *style.space() {
            StrokeSpace::ScreenSpace { thickness } => {
                let start_s = model.transform_vec2(start);
                let end_s = model.transform_vec2(end);

                self.stroke_segment_core(
                    start_s,
                    end_s,
                    thickness as f32,
                    style.cap(),
                    style.color(),
                    Mat3::IDENTITY,
                    RoundCapMode::ScreenFullDisk,
                );
            }

            StrokeSpace::WorldSpace { thickness } => {
                self.stroke_segment_core(
                    start,
                    end,
                    thickness as f32,
                    style.cap(),
                    style.color(),
                    model,
                    RoundCapMode::WorldHalfDisk,
                );
            }
        }
    }

    /// Core segment stroker in "working space".
    ///
    /// - For ScreenSpace: start/end are already in screen coords, and model_for_tris = IDENTITY.
    /// - For WorldSpace: start/end are in world coords, and model_for_tris = model.
    ///
    /// This avoids duplicating the quad math.
    fn stroke_segment_core(
        &mut self,
        mut start_w: Vec2,
        mut end_w: Vec2,
        thickness: f32,
        cap: LineCap,
        color: Color,
        model_for_tris: Mat3,
        round_caps: RoundCapMode,
    ) {
        if !thickness.is_finite() {
            return;
        }
        let thickness = thickness.max(0.0);
        if thickness <= 0.0 {
            return;
        }

        let half = 0.5 * thickness;

        let u = end_w - start_w;
        let u_len = u.len();
        if !u_len.is_finite() {
            return;
        }

        // Degenerate: treat as a dot (cap decides shape)
        if u_len <= 1e-6 {
            match cap {
                LineCap::Round => {
                    self.draw_round_dot(start_w, half, color, model_for_tris, round_caps);
                }
                LineCap::Butt | LineCap::Square => {
                    // Square dot in working space, transformed by model_for_tris.
                    let p0 = Vec2::new(start_w.x - half, start_w.y - half);
                    let p1 = Vec2::new(start_w.x + half, start_w.y - half);
                    let p2 = Vec2::new(start_w.x + half, start_w.y + half);
                    let p3 = Vec2::new(start_w.x - half, start_w.y + half);

                    self.fill_triangle(p0, p1, p2, color, model_for_tris);
                    self.fill_triangle(p0, p2, p3, color, model_for_tris);
                }
            }
            return;
        }

        let u_hat = u / u_len;

        // Square caps extend endpoints by half thickness along the segment direction in working space.
        if cap == LineCap::Square {
            start_w -= u_hat * half;
            end_w += u_hat * half;
        }

        // Expand into a quad around the segment in working space.
        let n = Vec2::new(-u_hat.y, u_hat.x);
        let offset = n * half;

        let p0 = start_w + offset;
        let p1 = start_w - offset;
        let p2 = end_w - offset;
        let p3 = end_w + offset;

        self.fill_triangle(p0, p1, p2, color, model_for_tris);
        self.fill_triangle(p0, p2, p3, color, model_for_tris);

        // Round caps:
        if cap == LineCap::Round {
            match round_caps {
                RoundCapMode::ScreenFullDisk => {
                    // working space is already screen; model_for_tris is IDENTITY
                    self.fill_circle(start_w, half, color, Mat3::IDENTITY);
                    self.fill_circle(end_w, half, color, Mat3::IDENTITY);
                }
                RoundCapMode::WorldHalfDisk => {
                    // working space is world; model_for_tris is the real model
                    self.fill_transformed_half_disc(start_w, -u_hat, half, color, model_for_tris);
                    self.fill_transformed_half_disc(end_w, u_hat, half, color, model_for_tris);
                }
            }
        }
    }

    fn draw_round_dot(
        &mut self,
        center_w: Vec2,
        radius: f32,
        color: Color,
        model_for_tris: Mat3,
        round_caps: RoundCapMode,
    ) {
        match round_caps {
            RoundCapMode::ScreenFullDisk => {
                self.fill_circle(center_w, radius, color, Mat3::IDENTITY);
            }
            RoundCapMode::WorldHalfDisk => {
                // For a degenerate "line", a full transformed disc is a reasonable dot.
                self.fill_transformed_disc(center_w, radius, color, model_for_tris);
            }
        }
    }

    /// Full disc in world space, affinely transformed to screen by `model`.
    /// (Used only for the degenerate round-dot case.)
    fn fill_transformed_disc(
        &mut self,
        center: Vec2,
        radius_world: f32,
        color: Color,
        model: Mat3,
    ) {
        if !radius_world.is_finite() || radius_world <= 0.0 {
            return;
        }

        let segments = 48usize;
        let center_s = model.transform_vec2(center);

        let mut prev: Option<Vec2> = None;
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let a = t * std::f32::consts::TAU;
            let (sa, ca) = a.sin_cos();

            let p_world = Vec2::new(center.x + ca * radius_world, center.y + sa * radius_world);
            let p_s = model.transform_vec2(p_world);

            if let Some(prev_s) = prev {
                self.fill_triangle(center_s, prev_s, p_s, color, Mat3::IDENTITY);
            }
            prev = Some(p_s);
        }
    }

    /// Half disc (semicircle) in world space, affinely transformed to screen by `model`.
    /// This is the correct shape for a round line cap in world space.
    fn fill_transformed_half_disc(
        &mut self,
        center: Vec2,
        outward_dir: Vec2,
        radius_world: f32,
        color: Color,
        model: Mat3,
    ) {
        if !radius_world.is_finite() || radius_world <= 0.0 {
            return;
        }

        let d_len = outward_dir.len();
        if !d_len.is_finite() || d_len <= 1e-6 {
            // Fallback: if direction is invalid, just draw a full disc.
            self.fill_transformed_disc(center, radius_world, color, model);
            return;
        }

        let d = outward_dir / d_len;
        let n = Vec2::new(-d.y, d.x);

        let center_s = model.transform_vec2(center);

        // Semicircle sweep: -π/2 .. +π/2 around outward direction.
        let segments = 32usize;
        let mut prev_s: Option<Vec2> = None;

        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let theta = (-0.5 + t) * std::f32::consts::PI; // -π/2..+π/2
            let (s, c) = theta.sin_cos();

            // This is center + (r.costheta.d + r.sintheta.n)
            // Where d and n are effectively orthonormal basis vectors
            let p_world = center + (d * c + n * s) * radius_world;
            let p_s = model.transform_vec2(p_world);

            if let Some(prev) = prev_s {
                self.fill_triangle(center_s, prev, p_s, color, Mat3::IDENTITY);
            }
            prev_s = Some(p_s);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RoundCapMode {
    /// Screen-space stroking: a round cap is a filled disk in screen pixels.
    ScreenFullDisk,
    /// World-space stroking: a round cap is the affine image of a semicircle.
    WorldHalfDisk,
}

// ------------------------------
// Tests
// ------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_space_thickness_scales_with_model_direction() {
        let thickness_world = 10.0;
        let half = 0.5 * thickness_world;

        let start = Vec2::new(0.0, 0.0);
        let end = Vec2::new(10.0, 0.0);
        let u = end - start;
        let u_hat = u / u.len();
        let n = Vec2::new(-u_hat.y, u_hat.x);
        let offset_world = n * half;

        let model = Mat3::scale(2.0, 3.0);
        let offset_screen = model.transform_vec2_direction(offset_world);

        // Normal points "up"; with scale(2,3) we expect its magnitude to scale by ~3.
        assert!((offset_screen.x - 0.0).abs() < 1e-6);
        assert!((offset_screen.y - (half * 3.0)).abs() < 1e-6);
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
