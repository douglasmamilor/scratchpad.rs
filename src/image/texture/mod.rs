mod atlas;

use crate::image::{Image, PixelFormat, color::Color};

#[derive(Clone)]
pub struct Texture {
    width: usize,
    height: usize,
    data: Vec<u8>,
    format: PixelFormat,
}

impl From<Image> for Texture {
    fn from(img: Image) -> Self {
        let width = img.width();
        let height = img.height();
        let format = *img.format();
        let data = img.data().to_vec();
        Self {
            width,
            height,
            data,
            format,
        }
    }
}

impl Texture {
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn format(&self) -> &PixelFormat {
        &self.format
    }

    #[inline]
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        assert!(
            x < self.width && y < self.height,
            "Pixel coordinates out of bounds"
        );

        let bpp = self.format.bytes_per_pixel();
        let i = (x + y * self.width) * bpp;

        assert!(i + bpp <= self.data.len(), "Pixel index out of bounds");
        match self.format {
            PixelFormat::Rgb8 => Color {
                r: self.data[i],
                g: self.data[i + 1],
                b: self.data[i + 2],
                a: 255,
            },
            PixelFormat::Rgba8 => Color {
                r: self.data[i],
                g: self.data[i + 1],
                b: self.data[i + 2],
                a: self.data[i + 3],
            },
            PixelFormat::Bgra8 => Color {
                r: self.data[i + 2],
                g: self.data[i + 1],
                b: self.data[i],
                a: self.data[i + 3],
            },
            PixelFormat::Bgr8 => Color {
                r: self.data[i + 2],
                g: self.data[i + 1],
                b: self.data[i],
                a: 255,
            },
        }
    }
}
