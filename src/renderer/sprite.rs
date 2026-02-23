use super::{Renderer, SamplingMode, Texture};
use crate::image::Color;
use crate::math::{Mat3, Vec2};

/// Simple sprite description: axis-aligned quad with UVs.
#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub pos: Vec2,
    pub size: Vec2,
    pub uv_min: Vec2,
    pub uv_max: Vec2,
}

impl Sprite {
    pub fn new(pos: Vec2, size: Vec2, uv_min: Vec2, uv_max: Vec2) -> Self {
        Self {
            pos,
            size,
            uv_min,
            uv_max,
        }
    }
}

impl<'a> Renderer<'a> {
    /// Draw a single textured quad with alpha blending, using two textured triangles.
    pub fn draw_sprite(
        &mut self,
        sprite: Sprite,
        texture: &Texture,
        sampling: SamplingMode,
        model: Mat3,
    ) {
        if sprite.size.x <= 0.0 || sprite.size.y <= 0.0 {
            return;
        }

        let p0 = sprite.pos;
        let p1 = sprite.pos + Vec2::new(sprite.size.x, 0.0);
        let p2 = sprite.pos + Vec2::new(0.0, sprite.size.y);
        let p3 = sprite.pos + sprite.size;

        let p0 = model.transform_vec2(p0);
        let p1 = model.transform_vec2(p1);
        let p2 = model.transform_vec2(p2);
        let p3 = model.transform_vec2(p3);

        let uv0 = sprite.uv_min;
        let uv1 = Vec2::new(sprite.uv_max.x, sprite.uv_min.y);
        let uv2 = Vec2::new(sprite.uv_min.x, sprite.uv_max.y);
        let uv3 = sprite.uv_max;

        self.fill_triangle_textured_alpha(p0, p1, p2, uv0, uv1, uv2, texture, sampling);
        self.fill_triangle_textured_alpha(p2, p1, p3, uv2, uv1, uv3, texture, sampling);
    }

    /// Draw a batch of sprites with a shared texture/sampling.
    pub fn draw_sprite_batch(
        &mut self,
        sprites: &[Sprite],
        texture: &Texture,
        sampling: SamplingMode,
        model: Mat3,
    ) {
        for sprite in sprites {
            self.draw_sprite(*sprite, texture, sampling, model);
        }
    }

    fn fill_triangle_textured_alpha(
        &mut self,
        a: Vec2,
        b: Vec2,
        c: Vec2,
        uv_a: Vec2,
        uv_b: Vec2,
        uv_c: Vec2,
        texture: &Texture,
        sampling: SamplingMode,
    ) {
        // Reject non-finite inputs or degenerate triangles.
        if !a.x.is_finite()
            || !a.y.is_finite()
            || !b.x.is_finite()
            || !b.y.is_finite()
            || !c.x.is_finite()
            || !c.y.is_finite()
        {
            return;
        }

        let area2 = (b - a).cross(c - a);
        if !area2.is_finite() || area2.abs() < 1e-6 {
            return;
        }
        let inv_area = 1.0 / area2;
        let area_pos = area2 > 0.0;

        // Integer bounding box (half-open) clamped to framebuffer; scissor enforced per-pixel.
        let mut min_x = a.x.min(b.x).min(c.x).floor() as i32;
        let mut max_x = a.x.max(b.x).max(c.x).ceil() as i32;
        let mut min_y = a.y.min(b.y).min(c.y).floor() as i32;
        let mut max_y = a.y.max(b.y).max(c.y).ceil() as i32;

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

                let px = x as f32 + 0.5;
                let py = y as f32 + 0.5;

                let w0 = (b.x - a.x) * (py - a.y) - (b.y - a.y) * (px - a.x);
                let w1 = (c.x - b.x) * (py - b.y) - (c.y - b.y) * (px - b.x);
                let w2 = (a.x - c.x) * (py - c.y) - (a.y - c.y) * (px - c.x);

                let inside = if area_pos {
                    w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
                } else {
                    w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0
                };

                if !inside {
                    continue;
                }

                let b0 = w0 * inv_area;
                let b1 = w1 * inv_area;
                let b2 = w2 * inv_area;

                let u = uv_a.x * b0 + uv_b.x * b1 + uv_c.x * b2;
                let v = uv_a.y * b0 + uv_b.y * b1 + uv_c.y * b2;

                let src = super::triangle::sample_texture(texture, u, v, sampling);
                self.alpha_blend_pixel(x, y, src);
            }
        }
    }

    #[inline]
    fn alpha_blend_pixel(&mut self, x: i32, y: i32, src: Color) {
        if !self.in_clip(x, y) {
            return;
        }
        let dst = self
            .framebuffer
            .get_pixel(x as usize, y as usize)
            .map(Color::from_u32)
            .unwrap_or(Color::TRANSPARENT);

        let sa = src.a as f32 / 255.0;
        if sa <= 0.0 {
            return;
        }

        let da = dst.a as f32 / 255.0;
        let out_a = sa + da * (1.0 - sa);

        let blend_chan = |s: u8, d: u8| -> u8 {
            let sf = s as f32 / 255.0;
            let df = d as f32 / 255.0;
            ((sf * sa + df * (1.0 - sa)) * 255.0)
                .round()
                .clamp(0.0, 255.0) as u8
        };

        let out = Color {
            r: blend_chan(src.r, dst.r),
            g: blend_chan(src.g, dst.g),
            b: blend_chan(src.b, dst.b),
            a: (out_a * 255.0).round().clamp(0.0, 255.0) as u8,
        };

        self.framebuffer
            .set_pixel(x as usize, y as usize, out.to_u32());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::FrameBuffer;

    fn make_rgba_texture(w: usize, h: usize, pixels: Vec<u8>) -> Texture {
        let img = crate::image::Image::new(w, h, pixels, crate::image::PixelFormat::Rgba8);
        Texture::from(img)
    }

    #[test]
    fn sprite_alpha_blends_over_bg() {
        let mut fb = FrameBuffer::new(4, 4);
        let mut r = Renderer::new(&mut fb);
        r.clear(Color::BLUE);

        // 1x1 texture: semi-transparent red.
        let tex = make_rgba_texture(1, 1, vec![255, 0, 0, 128]);

        let sprite = Sprite::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(4.0, 4.0),
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 1.0),
        );

        r.draw_sprite(sprite, &tex, SamplingMode::Nearest, Mat3::IDENTITY);

        // Top-left pixel should be a blend of red over blue (half alpha): both channels non-zero.
        let px = Color::from_u32(fb.get_pixel(0, 0).unwrap());
        assert!(px.r > 0, "sprite red should contribute");
        assert!(px.b > 0, "background blue should contribute");
    }
}
