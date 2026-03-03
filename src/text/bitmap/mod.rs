mod font;
mod layout;
mod loader;

pub use font::{BitmapFont, GlyphInstance};
pub use layout::{
    TextAlign,
    layout_text,
    layout_text_aligned,
    layout_text_anchored,
    layout_text_block_aligned,
    layout_text_wrapped_aligned,
    measure_text,
    measure_text_block,
    measure_text_multiline,
    word_wrap,
    word_wrap_preserve_whitespace,
};
pub(crate) use loader::BitmapFontLoader;
