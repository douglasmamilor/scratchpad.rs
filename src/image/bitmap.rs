use crate::{Image, mem::align_up_pow2};

#[derive(Debug, Clone)]
pub struct BitmapDecoder<'a> {
    data_offset: usize,

    width: usize,
    height: usize,
    bits_per_pixel: usize,

    is_top_down: bool,
    file_bytes: &'a [u8],
}

impl<'a> BitmapDecoder<'a> {
    pub(crate) fn new(file_bytes: &'a [u8]) -> Self {
        Self::make_bitmap(file_bytes)
    }

    #[inline]
    pub fn pixel_count(&self) -> usize {
        self.width * self.height
    }

    #[inline]
    pub fn byte_count(&self) -> usize {
        self.pixel_count() * self.bytes_per_pixel()
    }

    #[inline]
    pub fn row_stride(&self) -> usize {
        align_up_pow2(self.width * self.bytes_per_pixel(), 4)
    }

    #[inline]
    pub fn data_size(&self) -> usize {
        self.row_stride() * self.height
    }

    #[inline]
    pub fn data_offset(&self) -> usize {
        self.data_offset
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
    pub fn has_alpha(&self) -> bool {
        self.bytes_per_pixel() == 4
    }

    #[inline]
    pub fn is_top_down(&self) -> bool {
        self.is_top_down
    }

    #[inline]
    pub fn bits_per_pixel(&self) -> usize {
        self.bits_per_pixel
    }

    #[inline]
    pub fn bytes_per_pixel(&self) -> usize {
        self.bits_per_pixel / 8
    }

    fn make_bitmap(b: &'a [u8]) -> BitmapDecoder<'a> {
        let header = &b[0..14];
        let info = &b[14..54];

        let bi_file_type = [header[0], header[1]];
        assert_eq!(bi_file_type, [0x42, 0x4D]); // 'BM'

        let header_size = u32::from_le_bytes([info[0], info[1], info[2], info[3]]);
        assert_eq!(header_size, 40);

        let bi_planes = u16::from_le_bytes([info[12], info[13]]);
        assert_eq!(bi_planes, 1); // must be 1

        let compression = u32::from_le_bytes([info[16], info[17], info[18], info[19]]);
        assert_eq!(compression, 0); // 0 == 'BI_RGB' (no compression) according to BMP spec

        let bits_per_pixel = u16::from_le_bytes([info[14], info[15]]);
        assert!(bits_per_pixel == 24); // only support 24-bit at the moment

        let data_offset = u32::from_le_bytes([header[10], header[11], header[12], header[13]]);

        let mut height = i32::from_le_bytes([info[8], info[9], info[10], info[11]]);
        assert_ne!(height, 0); // height cannot be 0

        let is_top_down = height < 0;
        height = height.unsigned_abs() as i32; // convert to positive if negative

        let width = u32::from_le_bytes([info[4], info[5], info[6], info[7]]);
        assert_ne!(width, 0); // width cannot be 0

        BitmapDecoder {
            data_offset: data_offset as usize,
            width: width as usize,
            height: height as usize,
            bits_per_pixel: bits_per_pixel as usize,
            is_top_down,
            file_bytes: b,
        }
    }
}

impl<'a> From<BitmapDecoder<'a>> for Image {
    fn from(bitmap: BitmapDecoder<'a>) -> Self {
        let row_stride = bitmap.row_stride();
        let mut img_buffer = vec![0u8; bitmap.byte_count()];

        for y in 0..bitmap.height {
            let read_y = if bitmap.is_top_down() {
                y
            } else {
                bitmap.height - 1 - y
            };

            let row_start = bitmap.data_offset + read_y * row_stride;
            let row_end = bitmap.data_offset + ((read_y + 1) * row_stride);

            // TODO: (doug) This indexing will panic on malformed bitmaps, but that's probably fine since we
            // want to fail fast on bad input.
            // When we do a second pass after go-live, we can improve this
            let mut row =
                bitmap.file_bytes[row_start..row_end].chunks_exact(bitmap.bytes_per_pixel());

            for x in 0..bitmap.width {
                let pixel = row.next().unwrap();
                let b = pixel[0];
                let g = pixel[1];
                let r = pixel[2];

                let img_i = (y * bitmap.width + x) * bitmap.bytes_per_pixel();
                img_buffer[img_i] = r;
                img_buffer[img_i + 1] = g;
                img_buffer[img_i + 2] = b;

                if bitmap.has_alpha() {
                    img_buffer[img_i + 3] = pixel[3];
                }
            }
        }

        Image::new(
            bitmap.width,
            bitmap.height,
            img_buffer,
            if bitmap.has_alpha() {
                crate::image::PixelFormat::Rgba8
            } else {
                crate::image::PixelFormat::Rgb8
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_bitmap() {
        // File header
        let mut bmp: Vec<u8> = vec![
            0x42, 0x4D, // 'BM'
            70, 0, 0, 0, // file size = 70 bytes
            0, 0, 0, 0, // reserved
            54, 0, 0, 0, // data offset = 54
            // DIB header (BITMAPINFOHEADER)
            40, 0, 0, 0, // header size
            2, 0, 0, 0, // width = 2
            2, 0, 0, 0, // height = 2 (positive => bottom-up)
            1, 0, // planes
            24, 0, // bpp = 24
            0, 0, 0, 0, // compression = 0 (BI_RGB)
            0, 0, 0, 0, // image size (can be 0 for BI_RGB)
            0, 0, 0, 0, // x ppm (unused)
            0, 0, 0, 0, // y ppm (unused)
            0, 0, 0, 0, // colors used
            0, 0, 0, 0, // important colors
        ];
        // Pixel data (bottom row first), each row padded to 8 bytes.
        // Row 0 (bottom): B,G,R pixels: (blue), (green)
        bmp.extend_from_slice(&[
            255, 0, 0, 0, 255, 0, 0, 0, // pad to 8 bytes
        ]);
        // Row 1 (top): (red), (white)
        bmp.extend_from_slice(&[
            0, 0, 255, 255, 255, 255, 0, 0, // pad
        ]);

        let dec = BitmapDecoder::new(&bmp);

        assert_eq!(dec.width(), 2);
        assert_eq!(dec.height(), 2);
        assert_eq!(dec.bits_per_pixel(), 24);
        assert_eq!(dec.bytes_per_pixel(), 3);
        assert_eq!(dec.data_offset(), 54);
        assert_eq!(dec.data_size(), 16); // 2 rows * 8 bytes per row
        assert!(!dec.is_top_down());
        assert_eq!(dec.row_stride(), 8); // align_up_pow2 will have rounded 6 bytes up to 8
        assert_eq!(dec.pixel_count(), 4);
        assert_eq!(dec.byte_count(), 12); // 4 pixels * 3 bytes per pixel
        assert!(!dec.has_alpha());

        let img: Image = dec.into();
        assert_eq!(img.format(), &crate::image::PixelFormat::Rgb8);
        assert_eq!(img.width(), 2);
        assert_eq!(img.height(), 2);
    }
}
