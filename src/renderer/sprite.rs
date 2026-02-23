use super::{Renderer, SamplingMode, Texture};
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
    /// Draw a single textured quad using two textured triangles.
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

        // Split along the p0-p3 diagonal for consistent mapping.
        self.fill_triangle_textured(p0, p1, p3, uv0, uv1, uv3, texture, sampling, Mat3::IDENTITY);
        self.fill_triangle_textured(p0, p3, p2, uv0, uv3, uv2, texture, sampling, Mat3::IDENTITY);
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

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprite_is_rectangular() {
        let mut fb = crate::framebuffer::FrameBuffer::new(8, 8);
        let mut r = Renderer::new(&mut fb);
        let tex = Texture::from(&crate::image::Image::new(
            2,
            2,
            vec![
                255, 0, 0, 255, 0, 255, // row 0
                0, 0, 255, 255, 255, 0, // row 1
            ],
            crate::image::PixelFormat::Rgb8,
        ));
        let sprite = Sprite::new(
            Vec2::new(2.0, 2.0),
            Vec2::new(4.0, 4.0),
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 1.0),
        );
        r.draw_sprite(sprite, &tex, SamplingMode::Nearest, Mat3::IDENTITY);
        // Check corners are covered.
        assert!(fb.get_pixel(2, 2).unwrap() != 0);
        assert!(fb.get_pixel(5, 2).unwrap() != 0);
        assert!(fb.get_pixel(2, 5).unwrap() != 0);
        assert!(fb.get_pixel(5, 5).unwrap() != 0);
    }
}
