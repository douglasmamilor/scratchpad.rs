use crate::{BitmapFont, Color, Mat3, Vec2, renderer::Renderer, text::GlyphInstance};

impl<'a> Renderer<'a> {
    pub fn render_text(
        &mut self,
        font: &BitmapFont,
        instances: impl AsRef<[GlyphInstance]>,
        model: Mat3,
    ) {
        self.render_text_tinted(font, instances, Color::WHITE, model);
    }

    pub fn render_text_tinted(
        &mut self,
        font: &BitmapFont,
        instances: impl AsRef<[GlyphInstance]>,
        tint: Color,
        model: Mat3,
    ) {
        let instances = instances.as_ref();
        let atlas_texture = super::Texture::from(font.atlas());

        for instance in instances {
            self.draw_sprite_tinted(
                instance.into(),
                &atlas_texture,
                super::SamplingMode::Nearest,
                tint,
                model,
            );
        }
    }

    /// Render text with a simple drop shadow.
    ///
    /// This draws the glyphs twice:
    /// 1) shadow pass: `shadow_offset` + `shadow_tint`
    /// 2) main pass: no offset + `text_tint`
    pub fn render_text_with_shadow(
        &mut self,
        font: &BitmapFont,
        instances: impl AsRef<[GlyphInstance]>,
        text_tint: Color,
        shadow_offset: Vec2,
        shadow_tint: Color,
        model: Mat3,
    ) {
        let instances = instances.as_ref();
        let atlas_texture = super::Texture::from(font.atlas());

        for instance in instances {
            let mut sprite: super::Sprite = instance.into();
            sprite.pos = sprite.pos + shadow_offset;
            self.draw_sprite_tinted(
                sprite,
                &atlas_texture,
                super::SamplingMode::Nearest,
                shadow_tint,
                model,
            );
        }

        for instance in instances {
            self.draw_sprite_tinted(
                instance.into(),
                &atlas_texture,
                super::SamplingMode::Nearest,
                text_tint,
                model,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::FrameBuffer;
    use crate::image::{Image, PixelFormat};
    use crate::renderer::Renderer;
    use crate::text::BitmapFont;

    fn make_single_glyph_font() -> BitmapFont {
        let fnt = r#"
common lineHeight=1 base=1 scaleW=1 scaleH=1 pages=1 packed=0
chars count=2
char id=65 x=0 y=0 width=1 height=1 xoffset=0 yoffset=0 xadvance=1 page=0 chnl=0
char id=63 x=0 y=0 width=1 height=1 xoffset=0 yoffset=0 xadvance=1 page=0 chnl=0
"#;
        let atlas = Image::new(1, 1, vec![255, 255, 255, 255], PixelFormat::Rgba8);
        BitmapFont::new(fnt, atlas)
    }

    #[test]
    fn render_text_with_shadow_draws_two_positions() {
        let font = make_single_glyph_font();
        let mut fb = FrameBuffer::new(4, 4);
        let mut r = Renderer::new(&mut fb);
        r.set_triangle_aa_gamma(false);
        r.clear(Color::TRANSPARENT);

        let instances = vec![GlyphInstance::new(
            'A',
            1.0,
            1.0,
            (0.0, 0.0, 1.0, 1.0),
            (1, 1),
        )];

        r.render_text_with_shadow(
            &font,
            &instances,
            Color::WHITE,
            Vec2::new(1.0, 0.0),
            Color::BLACK,
            Mat3::IDENTITY,
        );

        assert_eq!(r.get_pixel((1, 1)).unwrap(), Color::WHITE);
        assert_eq!(r.get_pixel((2, 1)).unwrap(), Color::BLACK);
    }
}
