use crate::image::{BitmapDecoder, Image};

/// Image loading helpers.
pub struct ImageLoader;

impl ImageLoader {
    /// Decode a BMP from a byte slice into an `Image`.
    ///
    /// Currently supports uncompressed 24/32-bit BMPs.
    pub fn from_bmp_bytes(bytes: &[u8]) -> Image {
        let decoder = BitmapDecoder::new(bytes);
        decoder.into()
    }
}
