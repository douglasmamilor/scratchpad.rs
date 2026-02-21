mod bitmap;
pub mod color;
mod texture;

pub use bitmap::BitmapDecoder;
pub use color::Color;
pub use texture::Texture;

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>,
    format: PixelFormat,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PixelFormat {
    Rgba8,
    Rgb8,
    Bgra8,
    Bgr8,
}

impl PixelFormat {
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            PixelFormat::Rgba8 | PixelFormat::Bgra8 => 4,
            PixelFormat::Rgb8 | PixelFormat::Bgr8 => 3,
        }
    }

    pub fn has_alpha(&self) -> bool {
        matches!(self, PixelFormat::Rgba8 | PixelFormat::Bgra8)
    }
}

impl Image {
    pub fn new(width: usize, height: usize, data: Vec<u8>, format: PixelFormat) -> Self {
        let bpp = format.bytes_per_pixel();

        assert!(width > 0 && height > 0, "Image dimensions must be positive");
        assert!(
            !data.is_empty() && data.len() == width * height * bpp,
            "Data length does not match dimensions and format"
        );

        Self {
            width,
            height,
            data,
            format,
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    #[inline]
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    #[inline]
    pub fn format(&self) -> &PixelFormat {
        &self.format
    }

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
