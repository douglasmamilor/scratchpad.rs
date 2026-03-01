use crate::{BitmapFont, Color, Mat3, renderer::Renderer, text::GlyphInstance};

impl<'a> Renderer<'a> {
    // TODO: (doug) - support color tinting for text rendering
    pub fn render_text(
        &mut self,
        font: &BitmapFont,
        instances: impl AsRef<[GlyphInstance]>,
        model: Mat3,
    ) {
        let instances = instances.as_ref();

        for instance in instances {
            self.draw_sprite(
                instance.into(),
                &font.atlas().into(),
                super::SamplingMode::Nearest,
                model,
            );
        }
    }
}
