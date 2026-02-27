use std::{error::Error, fs, path::Path};

use crate::{
    BitmapFont,
    image::{Image, ImageLoader, Texture},
    text::BitmapFontLoader,
};

/// Minimal asset loader façade. Expands as more formats are supported.
pub struct AssetLoader;

impl AssetLoader {
    /// Load a BMPFont file from disk into a `BitmapFont`
    pub fn load_bmp_font(
        font_path: impl AsRef<Path>,
        texture_path: impl AsRef<Path>,
    ) -> Result<BitmapFont, Box<dyn Error>> {
        let texture = Self::load_bmp_image(texture_path)?;
        let font_data = fs::read_to_string(font_path)?;

        Ok(BitmapFontLoader::from_font(&font_data, texture))
    }

    /// Load a BMP file from disk into an `Image`.
    pub fn load_bmp_image(path: impl AsRef<Path>) -> Result<Image, Box<dyn Error>> {
        let bytes = fs::read(path)?;
        Ok(ImageLoader::from_bmp_bytes(&bytes))
    }

    /// Load a BMP file from disk into a `Texture`.
    pub fn load_bmp_texture(path: impl AsRef<Path>) -> Result<Texture, Box<dyn Error>> {
        let image = Self::load_bmp_image(path)?;
        Ok(Texture::from(image))
    }
}
