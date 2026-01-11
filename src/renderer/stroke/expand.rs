use crate::math::{EPS, angle_delta, perp_left};
use crate::renderer::stroke::path::Path;
use crate::renderer::stroke::types::{LineCap, StrokeSpace, StrokeStyle};
use crate::renderer::{LineJoin, PolyLine, apply_stroke_pattern};
use crate::{Mat3, Vec2, color::Color, renderer::Renderer};

#[derive(Clone, Copy)]
enum JoinRoundMode {
    Screen,
    World { model: Mat3 },
}

impl<'a> Renderer<'a> {
    pub fn stroke_line(&mut self, start: Vec2, end: Vec2, style: &StrokeStyle, model: Mat3) {
        self.stroke_segment(start, end, style, model);
    }

    /// Stroke a continuous polyline (polyline layer).
    ///
    /// Pipeline expectation:
    /// - Flatten path -> polylines
    /// - Apply dash/dot pattern -> polylines
    /// - Then stroke each polyline here (joins happen here)
    pub fn stroke_polyline(&mut self, poly: &PolyLine, style: &StrokeStyle, model: Mat3) {
        let pts = poly.points();
        if pts.len() < 2 {
            return;
        }

        // 1) Stroke all segments (your existing segment stroker)
        //
        // NOTE: Ideally interior segments should be stroked with Butt caps to avoid “double caps”.
        // For now, keep it simple and rely on joins to visually connect.
        if !poly.is_closed() {
            for i in 0..(pts.len() - 1) {
                let a = pts[i]; // Point2 is Vec2
                let b = pts[i + 1]; // Point2 is Vec2
                self.stroke_segment(a, b, style, model);
            }
        } else {
            for i in 0..pts.len() {
                let a = pts[i];
                let b = pts[(i + 1) % pts.len()];
                self.stroke_segment(a, b, style, model);
            }
        }

        // 2) Emit joins (needs A-B-C)
        if pts.len() < 3 {
            return;
        }

        if !poly.is_closed() {
            for i in 1..(pts.len() - 1) {
                let a = pts[i - 1];
                let b = pts[i];
                let c = pts[i + 1];
                self.emit_join_abc(a, b, c, style, model);
            }
        } else {
            for i in 0..pts.len() {
                let a = pts[(i + pts.len() - 1) % pts.len()];
                let b = pts[i];
                let c = pts[(i + 1) % pts.len()];
                self.emit_join_abc(a, b, c, style, model);
            }
        }
    }

    pub fn stroke_path(&mut self, path: &Path, style: &StrokeStyle, model: Mat3) {
        let Some(polylines) = self.flatten_path_to_polylines(path) else {
            return;
        };

        let patterned = apply_stroke_pattern(&polylines, style.pattern());

        for pl in &patterned {
            self.stroke_polyline(pl, style, model);
        }
    }

    fn emit_join_abc(&mut self, a: Vec2, b: Vec2, c: Vec2, style: &StrokeStyle, model: Mat3) {
        // Determine working space + half thickness.
        let (a_w, b_w, c_w, half, model_for_tris, join_round_mode) = match *style.space() {
            StrokeSpace::ScreenSpace { thickness } => {
                // transform points to screen, render in screen with identity model
                let a_s = model.transform_vec2(a);
                let b_s = model.transform_vec2(b);
                let c_s = model.transform_vec2(c);
                let half = 0.5 * (thickness as f32);
                (a_s, b_s, c_s, half, Mat3::IDENTITY, JoinRoundMode::Screen)
            }
            StrokeSpace::WorldSpace { thickness } => {
                let half = 0.5 * (thickness as f32);
                (a, b, c, half, model, JoinRoundMode::World { model })
            }
        };

        if !half.is_finite() || half <= 0.0 {
            return;
        }

        // If either segment is degenerate, skip join.
        let u_in = b_w - a_w;
        let u_out = c_w - b_w;

        let u_in_len = u_in.len();
        let u_out_len = u_out.len();
        if !u_in_len.is_finite() || !u_out_len.is_finite() {
            return;
        }
        if u_in_len <= EPS || u_out_len <= EPS {
            return;
        }

        let u_in_hat = u_in / u_in_len;
        let u_out_hat = u_out / u_out_len;

        // Collinear? Then no join geometry needed.
        //
        // NOTE: Screen space in this project is Y-down, which makes it a left-handed
        // coordinate system. Our turn/orientation tests (cross product sign) assume
        // the usual Y-up right-handed convention, so we flip the sign in screen space
        // to keep “outer side” selection consistent.
        let mut cross = u_in_hat.cross(u_out_hat);
        if matches!(join_round_mode, JoinRoundMode::Screen) {
            cross = -cross;
        }
        if cross.abs() <= 1e-6 {
            return;
        }

        // Left normals for each segment in working space.
        let n_in = perp_left(u_in_hat);
        let n_out = perp_left(u_out_hat);

        // Determine outer side.
        // cross > 0 => left turn => outer is LEFT
        // cross < 0 => right turn => outer is RIGHT (flip normals)
        let (n_outer_in, n_outer_out, sweep_ccw) = if cross > 0.0 {
            (n_in, n_out, true)
        } else {
            (-n_in, -n_out, false)
        };

        // The two outer offset points at the join.
        let p_in_outer = b_w + n_outer_in * half;
        let p_out_outer = b_w + n_outer_out * half;

        match style.join() {
            LineJoin::Bevel => {
                // Fill the outer wedge with one triangle.
                self.fill_triangle(b_w, p_in_outer, p_out_outer, style.color(), model_for_tris);
            }

            LineJoin::Miter { limit } => {
                self.emit_join_miter(
                    b_w,
                    u_in_hat,
                    u_out_hat,
                    p_in_outer,
                    p_out_outer,
                    half,
                    limit,
                    style.color(),
                    model_for_tris,
                );
            }

            LineJoin::Round => {
                self.emit_join_round(
                    b_w,
                    p_in_outer,
                    p_out_outer,
                    half,
                    sweep_ccw,
                    style.color(),
                    model_for_tris,
                    join_round_mode,
                );
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn emit_join_miter(
        &mut self,
        b: Vec2,
        u_in_hat: Vec2,
        u_out_hat: Vec2,
        p_in_outer: Vec2,
        p_out_outer: Vec2,
        half: f32,
        miter_limit: f32,
        color: Color,
        model_for_tris: Mat3,
    ) {
        if !miter_limit.is_finite() || miter_limit <= 0.0 {
            // Treat bad limit as bevel.
            self.fill_triangle(b, p_in_outer, p_out_outer, color, model_for_tris);
            return;
        }

        // Intersect the two outer offset lines:
        // L1: p = p_in_outer  + t * u_in_hat
        // L2: p = p_out_outer + s * u_out_hat
        let denom = u_in_hat.cross(u_out_hat);
        if denom.abs() <= 1e-6 {
            // Parallel-ish, bevel fallback.
            self.fill_triangle(b, p_in_outer, p_out_outer, color, model_for_tris);
            return;
        }

        let t = (p_out_outer - p_in_outer).cross(u_out_hat) / denom;
        let miter_pt = p_in_outer + u_in_hat * t;

        let miter_len = (miter_pt - b).len();
        if !miter_len.is_finite() {
            self.fill_triangle(b, p_in_outer, p_out_outer, color, model_for_tris);
            return;
        }

        // SVG-style miter limit check: compare (miter_len / half) to limit.
        let ratio = miter_len / half;
        if ratio > miter_limit {
            // Too pointy => bevel.
            self.fill_triangle(b, p_in_outer, p_out_outer, color, model_for_tris);
            return;
        }

        // Valid miter: fill outer wedge as triangle to miter point.
        self.fill_triangle(p_in_outer, miter_pt, p_out_outer, color, model_for_tris);
    }

    #[allow(clippy::too_many_arguments)]
    fn emit_join_round(
        &mut self,
        b: Vec2,
        p0: Vec2, // b + n_outer_in  * half
        p1: Vec2, // b + n_outer_out * half
        half: f32,
        sweep_ccw: bool,
        color: Color,
        model_for_tris: Mat3,
        mode: JoinRoundMode,
    ) {
        // Build angles around b in working space.
        let v0 = p0 - b;
        let v1 = p1 - b;

        let a0 = v0.y.atan2(v0.x);
        let a1 = v1.y.atan2(v1.x);

        // Compute a signed sweep delta consistent with desired direction.
        let delta = angle_delta(a0, a1, sweep_ccw);

        // How many fan segments? (tuneable)
        // ~ every 10 degrees => PI/18
        let steps = ((delta.abs() / (std::f32::consts::PI / 18.0)).ceil() as i32).max(6) as usize;

        match mode {
            JoinRoundMode::Screen => {
                // Points are already in screen, triangles draw with identity.
                let mut prev = p0;
                for i in 1..=steps {
                    let t = i as f32 / steps as f32;
                    let ang = a0 + delta * t;
                    let (s, c) = ang.sin_cos();
                    let p = b + Vec2::new(c, s) * half;

                    self.fill_triangle(b, prev, p, color, model_for_tris);
                    prev = p;
                }
            }

            JoinRoundMode::World { model } => {
                // Generate arc points in world space, transform each to screen,
                // and draw fan in screen (identity).
                let b_s = model.transform_vec2(b);
                let mut prev_s = model.transform_vec2(p0);

                for i in 1..=steps {
                    let t = i as f32 / steps as f32;
                    let ang = a0 + delta * t;
                    let (s, c) = ang.sin_cos();
                    let p_world = b + Vec2::new(c, s) * half;
                    let p_s = model.transform_vec2(p_world);

                    self.fill_triangle(b_s, prev_s, p_s, color, Mat3::IDENTITY);
                    prev_s = p_s;
                }
            }
        }
    }

    pub fn stroke_segment(&mut self, start: Vec2, end: Vec2, style: &StrokeStyle, model: Mat3) {
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
    #[allow(clippy::too_many_arguments)]
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
