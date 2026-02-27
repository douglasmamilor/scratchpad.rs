use crate::Image;
use crate::text::BitmapFont;

pub(crate) struct BitmapFontLoader;

impl BitmapFontLoader {
    /// Create a bitmap font from an in-memory `.fnt` string and decoded atlas image.
    pub(crate) fn from_font(fnt: &str, atlas: Image) -> BitmapFont {
        BitmapFont::new(fnt, atlas)
    }
}
