use crate::{BitmapFont, Mat3, renderer::Renderer, text::GlyphInstance};

impl<'a> Renderer<'a> {
    // TODO: (doug) - support color tinting for text rendering
    pub fn render_text(
        &mut self,
        font: &BitmapFont,
        instances: impl AsRef<[GlyphInstance]>,
        model: Mat3,
    ) {
        let instances = instances.as_ref();
        let atlas_texture = super::Texture::from(font.atlas());

        for instance in instances {
            self.draw_sprite(
                instance.into(),
                &atlas_texture,
                super::SamplingMode::Nearest,
                model,
            );
        }
    }
}
