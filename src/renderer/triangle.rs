use super::{FillRule, Renderer, SamplingMode, Texture};
use crate::Color;
use crate::math::space::clip::clip_polygon;
use crate::math::{Mat3, Point2, vec2::Vec2};

impl<'a> Renderer<'a> {
    /// Draw a triangle outline using three vertices.
    ///
    /// Skips degenerate (collinear) triangles.
    ///
    /// # Example
    /// ```
    /// # use scratchpad_rs::{image::Color, math::{vec2::Vec2, Mat3}};
    /// # use scratchpad_rs::framebuffer::FrameBuffer;
    /// # use scratchpad_rs::renderer::Renderer;
    /// # let mut fb = FrameBuffer::new(64, 64);
    /// # let mut r = Renderer::new(&mut fb);
    /// r.draw_triangle(
    ///     Vec2::new(100.0, 100.0),
    ///     Vec2::new(50.0, 200.0),
    ///     Vec2::new(150.0, 200.0),
    ///     Color::RED,
    ///     Mat3::IDENTITY,
    /// );
    ///
    #[inline]
    pub fn draw_triangle(&mut self, a: Vec2, b: Vec2, c: Vec2, color: Color, model: Mat3) {
        // Skip degenerate (collinear or tiny) triangles to avoid NaNs and useless work.
        // cross = 2 * area. If it is tiny or non-finite, treat as empty.
        let area2 = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
        if !area2.is_finite() || area2.abs() < 1e-6 {
            return;
        }

        self.draw_line_aa(a, b, color, model);
        self.draw_line_aa(b, c, color, model);
        self.draw_line_aa(c, a, color, model);
    }

    /// Fill a triangle using a scanline algorithm.
    ///
    /// O(height) in the triangle’s vertical span. Degenerate (collinear) triangles are skipped.
    ///
    /// # Example
    /// ```
    /// # use scratchpad_rs::{image::Color, math::{vec2::Vec2, Mat3}};
    /// # use scratchpad_rs::framebuffer::FrameBuffer;
    /// # use scratchpad_rs::renderer::Renderer;
    /// # let mut fb = FrameBuffer::new(64, 64);
    /// # let mut r = Renderer::new(&mut fb);
    /// r.fill_triangle(
    ///     Vec2::new(100.0, 100.0),
    ///     Vec2::new(50.0, 200.0),
    ///     Vec2::new(150.0, 200.0),
    ///     Color::RED,
    ///     Mat3::IDENTITY,
    /// );
    /// ```
    ///
    /// # Degenerate
    /// ```
    /// # use scratchpad_rs::{image::Color, math::{vec2::Vec2, Mat3}};
    /// # use scratchpad_rs::framebuffer::FrameBuffer;
    /// # use scratchpad_rs::renderer::Renderer;
    /// # let mut fb = FrameBuffer::new(64, 64);
    /// # let mut r = Renderer::new(&mut fb);
    /// r.fill_triangle(
    ///     Vec2::new(10.0, 10.0),
    ///     Vec2::new(20.0, 10.0),
    ///     Vec2::new(30.0, 10.0),
    ///     Color::RED,
    ///     Mat3::IDENTITY,
    /// ); // skipped (collinear)
    /// ```
    pub fn fill_triangle(&mut self, a: Vec2, b: Vec2, c: Vec2, color: Color, model: Mat3) {
        // Early out to AA path if enabled (flat color).
        if self.aa_triangles {
            self.fill_triangle_aa(a, b, c, color, model);
            return;
        }

        // Transform first; all raster decisions are in screen space.
        let a_s = model.transform_vec2(a);
        let b_s = model.transform_vec2(b);
        let c_s = model.transform_vec2(c);

        // Reject non-finite inputs.
        if !a_s.x.is_finite()
            || !a_s.y.is_finite()
            || !b_s.x.is_finite()
            || !b_s.y.is_finite()
            || !c_s.x.is_finite()
            || !c_s.y.is_finite()
        {
            return;
        }

        // Reject degenerate/zero-area triangles up front (prevents div-by-zero below).
        let area2 = (b_s - a_s).cross(c_s - a_s);

        if !area2.is_finite() || area2.abs() < 1e-6 {
            return;
        }

        // Clip triangle to the active clip rect (viewport/scissor). If clipping
        // produces a polygon with >3 verts, fall back to polygon fill.
        let mut vertices = [a_s, b_s, c_s];
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
                vertices = [
                    Vec2::new(clipped[0].x, clipped[0].y),
                    Vec2::new(clipped[1].x, clipped[1].y),
                    Vec2::new(clipped[2].x, clipped[2].y),
                ];
            } else {
                self.fill_polygon(clipped, color, FillRule::NonZeroWinding);
                return;
            }
        }

        vertices.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

        #[allow(non_snake_case)]
        let A = vertices[0];

        #[allow(non_snake_case)]
        let B = vertices[1];

        #[allow(non_snake_case)]
        let C = vertices[2];

        if (A.y - B.y).abs() < 1e-6 {
            self.fill_flat_top(A, B, C, color);
            return;
        } else if (B.y - C.y).abs() < 1e-6 {
            self.fill_flat_bottom(A, B, C, color);
            return;
        }

        // The point on AC (that is the vector C - A) that has the same y as B
        let ty = (B.y - A.y) / (C.y - A.y);
        // We plug it into the line equation to get the x coordinate
        #[allow(non_snake_case)]
        let Dx = A.x + ty * (C.x - A.x);
        // The y coordinate is simply B.y
        #[allow(non_snake_case)]
        let Dy = B.y;

        #[allow(non_snake_case)]
        let D = Vec2::new(Dx, Dy);

        self.fill_flat_bottom(A, B, D, color);
        self.fill_flat_top(B, D, C, color);
    }

    /// Fill a triangle with a texture using barycentric UVs.
    ///
    /// Applies an optional sampling mode (nearest or bilinear) and respects the current scissor/viewport.
    /// Degenerate triangles are skipped.
    /// Intentionally avoids using utility functions since this is a hot inner loop and we want to
    /// minimize temporary allocations and redundant checks.
    #[allow(clippy::too_many_arguments)]
    pub fn fill_triangle_textured(
        &mut self,
        a: Vec2,
        b: Vec2,
        c: Vec2,
        uv_a: Vec2,
        uv_b: Vec2,
        uv_c: Vec2,
        texture: &Texture,
        sampling: SamplingMode,
        model: Mat3,
    ) {
        let a_s = model.transform_vec2(a);
        let b_s = model.transform_vec2(b);
        let c_s = model.transform_vec2(c);

        // Reject non-finite inputs or degenerate triangles.
        if !a_s.x.is_finite()
            || !a_s.y.is_finite()
            || !b_s.x.is_finite()
            || !b_s.y.is_finite()
            || !c_s.x.is_finite()
            || !c_s.y.is_finite()
        {
            return;
        }

        let area2 = (b_s - a_s).cross(c_s - a_s);
        if !area2.is_finite() || area2.abs() < 1e-6 {
            return;
        }
        // Barycentric weights via edge functions (same math as `math::barycentric`,
        // kept inline to avoid per-pixel Vec2 temporaries/Option checks). Precompute
        // the reciprocal area once for reuse in the inner loop.
        let inv_area = 1.0 / area2;
        let area_pos = area2 > 0.0;

        // Integer bounding box (half-open) clamped to the framebuffer; scissor is enforced per-pixel.
        let mut min_x = a_s.x.min(b_s.x).min(c_s.x).floor() as i32;
        let mut max_x = a_s.x.max(b_s.x).max(c_s.x).ceil() as i32;
        let mut min_y = a_s.y.min(b_s.y).min(c_s.y).floor() as i32;
        let mut max_y = a_s.y.max(b_s.y).max(c_s.y).ceil() as i32;

        let fb_w = self.width() as i32;
        let fb_h = self.height() as i32;
        min_x = min_x.clamp(0, fb_w);
        max_x = max_x.clamp(0, fb_w);
        min_y = min_y.clamp(0, fb_h);
        max_y = max_y.clamp(0, fb_h);

        for y in min_y..max_y {
            for x in min_x..max_x {
                if !self.in_scissor(x, y) {
                    continue;
                }

                // Bias to pixel centre for renderer
                let px = x as f32 + 0.5;
                let py = y as f32 + 0.5;

                // Point-in-triangle test via edge functions (same math as `math::barycentric`, but
                // we keep the raw w0/w1/w2
                //
                // Computes the signed area of the parallelogram formed by:
                // •Edge vector
                // •Vector to the test point
                //
                // If result > 0  → P is on left side of edge
                // If result < 0  → P is on right side
                // If result = 0  → P is on the edge
                //
                // w0 → edge AB -> (B - A) cross (P - A)
                // w1 → edge BC -> (C - B) cross (P - B)
                // w2 → edge CA -> (A - C) cross (P - C)
                let w0 = (b_s.x - a_s.x) * (py - a_s.y) - (b_s.y - a_s.y) * (px - a_s.x);
                let w1 = (c_s.x - b_s.x) * (py - b_s.y) - (c_s.y - b_s.y) * (px - b_s.x);
                let w2 = (a_s.x - c_s.x) * (py - c_s.y) - (a_s.y - c_s.y) * (px - c_s.x);

                let inside = if area_pos {
                    w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
                } else {
                    w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0
                };

                if !inside {
                    continue;
                }

                // Barycentric weights (sum to 1) using the precomputed inverse area.
                let b0 = w0 * inv_area;
                let b1 = w1 * inv_area;
                let b2 = w2 * inv_area;

                // Barycentric interpolation of UVs: uv = u0 * b0 + u1 * b1 + u2 * b2
                let u = uv_a.x * b0 + uv_b.x * b1 + uv_c.x * b2;
                let v = uv_a.y * b0 + uv_b.y * b1 + uv_c.y * b2;

                let texel = sample_texture(texture, u, v, sampling);
                self.set_pixel((x, y), texel);
            }
        }
    }

    #[allow(non_snake_case)]
    pub fn fill_flat_top(&mut self, A: Vec2, B: Vec2, C: Vec2, color: Color) {
        let eps = 1e-6;
        if (C.y - A.y).abs() < eps || (C.y - B.y).abs() < eps {
            return; // Degenerate flat top
        }

        let inv_slope_1 = (C.x - A.x) / (C.y - A.y);
        let inv_slope_2 = (C.x - B.x) / (C.y - B.y);

        // Half-open vertical range [y_start, y_end), inclusive top, exclusive bottom (top-left rule).
        let y_start = A.y.ceil() as i32;
        let y_end = C.y.ceil() as i32;

        let mut x1 = A.x + (y_start as f32 - A.y) * inv_slope_1;
        let mut x2 = B.x + (y_start as f32 - B.y) * inv_slope_2;

        for y in y_start..y_end {
            let (xa, xb) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            // Pixel centers are at integer coords; adding 0.5 then floor picks the covered pixels.
            let x_start = (xa + 0.5).floor() as i32;
            let x_end = (xb + 0.5).floor() as i32;
            self.hspan(y, x_start, x_end, color);
            x1 += inv_slope_1;
            x2 += inv_slope_2;
        }
    }

    #[allow(non_snake_case)]
    fn fill_flat_bottom(&mut self, A: Vec2, B: Vec2, C: Vec2, color: Color) {
        let eps = 1e-6; // small tolerance
        if (B.y - A.y).abs() < eps || (C.y - A.y).abs() < eps {
            return; // basically zero height
        }

        let inv_slope_1 = (B.x - A.x) / (B.y - A.y);
        let inv_slope_2 = (C.x - A.x) / (C.y - A.y);

        // Half-open vertical range [y_start, y_end), inclusive top, exclusive bottom (top-left rule).
        let y_start = A.y.ceil() as i32;
        let y_end = C.y.ceil() as i32;

        let mut x1 = A.x + (y_start as f32 - A.y) * inv_slope_1;
        let mut x2 = A.x + (y_start as f32 - A.y) * inv_slope_2;

        for y in y_start..y_end {
            let (xa, xb) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            let x_start = (xa + 0.5).floor() as i32;
            let x_end = (xb + 0.5).floor() as i32;
            self.hspan(y, x_start, x_end, color);
            x1 += inv_slope_1;
            x2 += inv_slope_2;
        }
    }

    /// Simple triangle edge AA via 2x2 supersampling at pixel corners and blending by coverage.
    /// This is a single-color variant; depth not handled here.
    pub fn fill_triangle_aa(&mut self, a: Vec2, b: Vec2, c: Vec2, color: Color, model: Mat3) {
        // Transform to screen space
        let a_s = model.transform_vec2(a);
        let b_s = model.transform_vec2(b);
        let c_s = model.transform_vec2(c);

        // Degenerate check
        let area2 = (b_s - a_s).cross(c_s - a_s);
        if !area2.is_finite() || area2.abs() < 1e-6 {
            return;
        }
        let area_pos = area2 > 0.0;

        // Bounding box
        let min_x = a_s.x.min(b_s.x).min(c_s.x).floor() as i32;
        let max_x = a_s.x.max(b_s.x).max(c_s.x).ceil() as i32;
        let min_y = a_s.y.min(b_s.y).min(c_s.y).floor() as i32;
        let max_y = a_s.y.max(b_s.y).max(c_s.y).ceil() as i32;

        // Subpixel sample offsets
        let samples: &[(f32, f32)] = if self.aa_supersample {
            &[(0.25f32, 0.25f32), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)]
        } else {
            &[(0.5, 0.5)]
        };

        for y in min_y..max_y {
            for x in min_x..max_x {
                let mut hit = 0;
                for (ox, oy) in samples {
                    let px = x as f32 + ox;
                    let py = y as f32 + oy;

                    let w0 = (b_s.x - a_s.x) * (py - a_s.y) - (b_s.y - a_s.y) * (px - a_s.x);
                    let w1 = (c_s.x - b_s.x) * (py - b_s.y) - (c_s.y - b_s.y) * (px - b_s.x);
                    let w2 = (a_s.x - c_s.x) * (py - c_s.y) - (a_s.y - c_s.y) * (px - c_s.x);

                    if area_pos {
                        if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                            hit += 1;
                        }
                    } else if w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0 {
                        hit += 1;
                    }
                }

                if hit > 0 {
                    let cov = hit as f32 / samples.len() as f32;
                    self.blend_coverage(x, y, color, cov);
                }
            }
        }
    }
}

pub(crate) fn sample_texture(texture: &Texture, u: f32, v: f32, sampling: SamplingMode) -> Color {
    let u = u.clamp(0.0, 1.0);
    let v = v.clamp(0.0, 1.0);
    match sampling {
        SamplingMode::Nearest => sample_nearest(texture, u, v),
        SamplingMode::Bilinear => sample_bilinear(texture, u, v),
    }
}

fn sample_nearest(texture: &Texture, u: f32, v: f32) -> Color {
    let tx = u * (texture.width() as f32 - 1.0);
    let ty = v * (texture.height() as f32 - 1.0);

    let ix = tx.round().clamp(0.0, texture.width() as f32 - 1.0) as usize;
    let iy = ty.round().clamp(0.0, texture.height() as f32 - 1.0) as usize;

    texture.get_pixel(ix, iy)
}

fn sample_bilinear(texture: &Texture, u: f32, v: f32) -> Color {
    let tx = u * (texture.width() as f32 - 1.0);
    let ty = v * (texture.height() as f32 - 1.0);

    let x0 = tx.floor().clamp(0.0, texture.width() as f32 - 1.0) as usize;
    let y0 = ty.floor().clamp(0.0, texture.height() as f32 - 1.0) as usize;
    let x1 = (x0 + 1).min(texture.width() - 1);
    let y1 = (y0 + 1).min(texture.height() - 1);

    let fx = (tx - x0 as f32).clamp(0.0, 1.0);
    let fy = (ty - y0 as f32).clamp(0.0, 1.0);

    let c00 = texture.get_pixel(x0, y0);
    let c10 = texture.get_pixel(x1, y0);
    let c01 = texture.get_pixel(x0, y1);
    let c11 = texture.get_pixel(x1, y1);

    let top = c00.lerp(&c10, fx);
    let bottom = c01.lerp(&c11, fx);

    top.lerp(&bottom, fy)
}

// ------------------------------
// Tests
// ------------------------------
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::framebuffer::FrameBuffer;
    use crate::renderer::Renderer;

    fn collect_pixels(fb: &FrameBuffer) -> HashSet<(usize, usize)> {
        fb.pixels
            .iter()
            .enumerate()
            .filter_map(|(i, &p)| {
                if p != 0 {
                    let w = fb.width();
                    let y = i / w;
                    let x = i % w;
                    Some((x, y))
                } else {
                    None
                }
            })
            .collect()
    }

    #[test]
    fn degenerate_triangle_renders_nothing() {
        let mut fb = FrameBuffer::new(64, 64);
        let mut r = Renderer::new(&mut fb);
        r.clear(Color::TRANSPARENT);

        // Collinear: zero area.
        r.fill_triangle(
            Vec2::new(10.0, 10.0),
            Vec2::new(20.0, 10.0),
            Vec2::new(30.0, 10.0),
            Color::WHITE,
            Mat3::IDENTITY,
        );

        assert!(collect_pixels(&fb).is_empty());
    }

    #[test]
    fn two_triangles_cover_rectangle_interior() {
        let mut fb = FrameBuffer::new(64, 64);
        let mut r = Renderer::new(&mut fb);
        r.clear(Color::TRANSPARENT);

        // Rectangle corners
        let p0 = Vec2::new(10.0, 10.0);
        let p1 = Vec2::new(30.0, 10.0);
        let p2 = Vec2::new(30.0, 30.0);
        let p3 = Vec2::new(10.0, 30.0);

        // Fill rectangle with two triangles.
        r.fill_triangle(p0, p1, p2, Color::WHITE, Mat3::IDENTITY);
        r.fill_triangle(p0, p2, p3, Color::WHITE, Mat3::IDENTITY);

        // Check interior (exclude the outermost edge pixels) has no gaps.
        for y in 11..30 {
            for x in 11..30 {
                assert!(
                    fb.get_pixel(x, y).unwrap_or(0) != 0,
                    "Missing coverage at interior pixel ({x},{y})"
                );
            }
        }
    }

    fn make_test_texture() -> Texture {
        // 2x2 RGBA texture:
        // [ (255,0,0) , (0,255,0) ]
        // [ (0,0,255) , (255,255,255) ]
        let data = vec![
            255, 0, 0, 255, // top-left
            0, 255, 0, 255, // top-right
            0, 0, 255, 255, // bottom-left
            255, 255, 255, 255, // bottom-right
        ];
        let img = crate::image::Image::new(2, 2, data, crate::image::PixelFormat::Rgba8);
        img.into()
    }

    #[test]
    fn textured_triangle_nearest_constant_uv_is_uniform() {
        let mut fb = FrameBuffer::new(4, 4);
        let mut r = Renderer::new(&mut fb);
        r.clear(Color::TRANSPARENT);

        let tex = make_test_texture();
        let uv = Vec2::new(0.0, 0.0); // always sample top-left texel (red)

        r.fill_triangle_textured(
            Vec2::new(0.0, 0.0),
            Vec2::new(3.0, 0.0),
            Vec2::new(0.0, 3.0),
            uv,
            uv,
            uv,
            &tex,
            SamplingMode::Nearest,
            Mat3::IDENTITY,
        );

        let mut covered = 0;
        for y in 0..fb.height() {
            for x in 0..fb.width() {
                let p = fb.get_pixel(x, y).unwrap_or(0);
                if p != 0 {
                    covered += 1;
                    assert_eq!(Color::from_u32(p), Color::RED);
                }
            }
        }
        assert!(covered > 0, "Triangle should cover at least one pixel");
    }

    #[test]
    fn textured_triangle_bilinear_constant_uv_is_average() {
        let mut fb = FrameBuffer::new(4, 4);
        let mut r = Renderer::new(&mut fb);
        r.clear(Color::TRANSPARENT);

        let tex = make_test_texture();
        let uv_center = Vec2::new(0.5, 0.5); // center of the texture

        r.fill_triangle_textured(
            Vec2::new(0.0, 0.0),
            Vec2::new(3.0, 0.0),
            Vec2::new(0.0, 3.0),
            uv_center,
            uv_center,
            uv_center,
            &tex,
            SamplingMode::Bilinear,
            Mat3::IDENTITY,
        );

        let mut covered = 0;
        let expected = Color {
            r: 128,
            g: 128,
            b: 128,
            a: 255,
        };

        for y in 0..fb.height() {
            for x in 0..fb.width() {
                let p = fb.get_pixel(x, y).unwrap_or(0);
                if p != 0 {
                    covered += 1;
                    assert_eq!(Color::from_u32(p), expected);
                }
            }
        }
        assert!(covered > 0, "Triangle should cover at least one pixel");
    }
}
