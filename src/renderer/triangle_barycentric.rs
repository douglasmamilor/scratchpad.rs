use super::Renderer;
use crate::Color;
use crate::math::space::clip::clip_polygon;
use crate::math::{Mat3, Point2, barycentric, vec2::Vec2};

impl<'a> Renderer<'a> {
    /// Fill a triangle with per-vertex colors using barycentric interpolation.
    ///
    /// This function fills a triangle with smooth color gradients by interpolating
    /// colors assigned to each vertex across the triangle surface using barycentric
    /// coordinates. This is known as Gouraud shading.
    ///
    /// # Arguments
    /// * `a`, `b`, `c` - Triangle vertices in model space
    /// * `color_a`, `color_b`, `color_c` - Colors assigned to each vertex
    /// * `model` - Transformation matrix to apply to vertices
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::image::Color;
    /// use scratchpad_rs::math::{Vec2, Mat3};
    /// use scratchpad_rs::framebuffer::FrameBuffer;
    /// use scratchpad_rs::renderer::Renderer;
    ///
    /// let mut fb = FrameBuffer::new(800, 600);
    /// let mut r = Renderer::new(&mut fb);
    ///
    /// // Triangle with red, green, blue at vertices
    /// r.fill_triangle_colored(
    ///     Vec2::new(100.0, 100.0),
    ///     Vec2::new(50.0, 200.0),
    ///     Vec2::new(150.0, 200.0),
    ///     Color::RED,
    ///     Color::GREEN,
    ///     Color::BLUE,
    ///     Mat3::IDENTITY,
    /// );
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn fill_triangle_colored(
        &mut self,
        a: Vec2,
        b: Vec2,
        c: Vec2,
        color_a: Color,
        color_b: Color,
        color_c: Color,
        model: Mat3,
    ) {
        self.fill_triangle_colored_with_depth(
            a, b, c, color_a, color_b, color_c, 0.0, 0.0, 0.0, model,
        );
    }

    /// Fill a triangle with per-vertex colors and per-vertex depth using
    /// barycentric interpolation.
    ///
    /// This is the depth-aware variant of `fill_triangle_colored`. Depth
    /// values are interpolated per pixel using the same barycentric weights
    /// as the colors and passed through the renderer's depth test.
    #[allow(clippy::too_many_arguments)]
    pub fn fill_triangle_colored_with_depth(
        &mut self,
        a: Vec2,
        b: Vec2,
        c: Vec2,
        color_a: Color,
        color_b: Color,
        color_c: Color,
        depth_a: f32,
        depth_b: f32,
        depth_c: f32,
        model: Mat3,
    ) {
        // Transform vertices to screen space
        let mut a_s = model.transform_vec2(a);
        let mut b_s = model.transform_vec2(b);
        let mut c_s = model.transform_vec2(c);

        // Optional clipping to active viewport/scissor. If clipping produces exactly
        // one triangle, use it; if it produces more verts, fall back to scissor-only.
        if let Some(clip_rect) = self.active_clip_rect() {
            let pts = [
                Point2::new(a_s.x, a_s.y),
                Point2::new(b_s.x, b_s.y),
                Point2::new(c_s.x, c_s.y),
            ];
            let clipped = clip_polygon(&pts, clip_rect);
            if clipped.len() < 3 {
                return;
            }
            if clipped.len() == 3 {
                a_s = Vec2::new(clipped[0].x, clipped[0].y);
                b_s = Vec2::new(clipped[1].x, clipped[1].y);
                c_s = Vec2::new(clipped[2].x, clipped[2].y);
            }
        }

        // Sort vertices by Y, keeping colors aligned
        let mut vertices = [(a_s, color_a), (b_s, color_b), (c_s, color_c)];
        vertices.sort_by(|(v1, _), (v2, _)| v1.y.partial_cmp(&v2.y).unwrap());

        #[allow(non_snake_case)]
        let (A, color_A) = vertices[0];
        #[allow(non_snake_case)]
        let (B, color_B) = vertices[1];
        #[allow(non_snake_case)]
        let (C, color_C) = vertices[2];

        // Precompute barycentric constants (using original a_s, b_s, c_s)
        let v0 = b_s - a_s;
        let v1 = c_s - a_s;
        let denom = v0.cross(v1);
        if denom.abs() < f32::EPSILON {
            return; // degenerate triangle
        }
        let inv_denom = 1.0 / denom;

        if A.y == B.y {
            self.fill_flat_top_colored(
                A, B, C, color_A, color_B, color_C, a_s, b_s, c_s, inv_denom, depth_a, depth_b,
                depth_c,
            );
            return;
        } else if B.y == C.y {
            self.fill_flat_bottom_colored(
                A, B, C, color_A, color_B, color_C, a_s, b_s, c_s, inv_denom, depth_a, depth_b,
                depth_c,
            );
            return;
        }

        // Split triangle at middle vertex
        let ty = (B.y - A.y) / (C.y - A.y);
        #[allow(non_snake_case)]
        let D = Vec2::new(A.x + ty * (C.x - A.x), B.y);

        // Interpolate color at D (on edge AC)
        if let Some(coords_d) = barycentric::barycentric(D, a_s, b_s, c_s) {
            #[allow(non_snake_case)]
            let color_D = barycentric::interpolate_color(&coords_d, color_a, color_b, color_c);

            #[allow(non_snake_case)]
            let depth_D = barycentric::interpolate_f32(&coords_d, depth_a, depth_b, depth_c);

            self.fill_flat_bottom_colored(
                A, B, D, color_A, color_B, color_D, a_s, b_s, c_s, inv_denom, depth_a, depth_b,
                depth_D,
            );
            self.fill_flat_top_colored(
                B, D, C, color_B, color_D, color_C, a_s, b_s, c_s, inv_denom, depth_b, depth_D,
                depth_c,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(non_snake_case)]
    fn fill_flat_top_colored(
        &mut self,
        A: Vec2,
        B: Vec2,
        C: Vec2,
        color_A: Color,
        color_B: Color,
        color_C: Color,
        a_s: Vec2, // Original vertices for barycentric calculation
        b_s: Vec2,
        c_s: Vec2,
        inv_denom: f32,
        depth_a: f32,
        depth_b: f32,
        depth_c: f32,
    ) {
        let eps = 1e-6;
        if (C.y - A.y).abs() < eps || (C.y - B.y).abs() < eps {
            return; // Degenerate flat top
        }

        let inv_slope_1 = (C.x - A.x) / (C.y - A.y);
        let inv_slope_2 = (C.x - B.x) / (C.y - B.y);

        let y_start = A.y.ceil() as i32;
        let y_end = C.y.ceil() as i32;

        let mut x1 = A.x + (y_start as f32 - A.y) * inv_slope_1;
        let mut x2 = B.x + (y_start as f32 - B.y) * inv_slope_2;

        // Precompute barycentric deltas for horizontal movement
        let v0 = b_s - a_s;
        let v1 = c_s - a_s;
        let delta_v = v1.y * inv_denom;
        let delta_w = -v0.y * inv_denom;

        for y in y_start..y_end {
            let (xa, xb) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            let x_start = (xa + 0.5).floor() as i32;
            let x_end = (xb + 0.5).floor() as i32;

            // Compute barycentric coords for leftmost pixel of this scanline
            let y_f = y as f32 + 0.5;
            let p_left = Vec2::new(x_start as f32 + 0.5, y_f);
            let v2_left = p_left - a_s;
            let mut v = v2_left.cross(v1) * inv_denom;
            let mut w = v0.cross(v2_left) * inv_denom;

            // Fill scanline with interpolated colors
            for x in x_start..=x_end {
                let u = 1.0 - v - w;

                // Interpolate color and depth using barycentric coordinates
                let coords = barycentric::BarycentricCoords { u, v, w };
                let color = barycentric::interpolate_color(
                    &barycentric::BarycentricCoords { u, v, w },
                    color_A,
                    color_B,
                    color_C,
                );
                let depth = barycentric::interpolate_f32(&coords, depth_a, depth_b, depth_c);

                if self.depth_test(x, y, depth) {
                    self.set_pixel((x, y), color);
                }

                // Increment barycentric coordinates for next pixel
                v += delta_v;
                w += delta_w;
            }

            // Update edge positions for next scanline
            x1 += inv_slope_1;
            x2 += inv_slope_2;
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(non_snake_case)]
    fn fill_flat_bottom_colored(
        &mut self,
        A: Vec2,
        B: Vec2,
        C: Vec2,
        color_A: Color,
        color_B: Color,
        color_C: Color,
        a_s: Vec2,
        b_s: Vec2,
        c_s: Vec2,
        inv_denom: f32,
        depth_a: f32,
        depth_b: f32,
        depth_c: f32,
    ) {
        let eps = 1e-6;
        if (B.y - A.y).abs() < eps || (C.y - A.y).abs() < eps {
            return; // basically zero height
        }

        let inv_slope_1 = (B.x - A.x) / (B.y - A.y);
        let inv_slope_2 = (C.x - A.x) / (C.y - A.y);

        let y_start = A.y.ceil() as i32;
        let y_end = C.y.ceil() as i32;

        let mut x1 = A.x + (y_start as f32 - A.y) * inv_slope_1;
        let mut x2 = A.x + (y_start as f32 - A.y) * inv_slope_2;

        // Precompute barycentric deltas
        let v0 = b_s - a_s;
        let v1 = c_s - a_s;
        let delta_v = v1.y * inv_denom;
        let delta_w = -v0.y * inv_denom;

        for y in y_start..y_end {
            let (xa, xb) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            let x_start = (xa + 0.5).floor() as i32;
            let x_end = (xb + 0.5).floor() as i32;

            // Compute barycentric coords for leftmost pixel
            let y_f = y as f32 + 0.5;
            let p_left = Vec2::new(x_start as f32 + 0.5, y_f);
            let v2_left = p_left - a_s;
            let mut v = v2_left.cross(v1) * inv_denom;
            let mut w = v0.cross(v2_left) * inv_denom;

            // Fill scanline
            for x in x_start..=x_end {
                let u = 1.0 - v - w;

                let coords = barycentric::BarycentricCoords { u, v, w };
                let color = barycentric::interpolate_color(&coords, color_A, color_B, color_C);

                let depth = barycentric::interpolate_f32(&coords, depth_a, depth_b, depth_c);

                if self.depth_test(x, y, depth) {
                    self.set_pixel((x, y), color);
                }

                v += delta_v;
                w += delta_w;
            }

            x1 += inv_slope_1;
            x2 += inv_slope_2;
        }
    }
}
