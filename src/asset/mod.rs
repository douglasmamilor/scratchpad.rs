use std::{error::Error, fs, path::Path};

use crate::image::{Image, ImageLoader, Texture};

/// Minimal asset loader façade. Expands as more formats are supported.
pub struct AssetLoader;

impl AssetLoader {
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
