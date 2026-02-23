//! Image filtering operations: convolution kernels, blur, sharpen, edge detection, and color adjustments.

use super::{Color, Image};

/// A convolution kernel for image filtering.
#[derive(Debug, Clone)]
pub struct Kernel {
    data: Vec<f32>,
    width: usize,
    height: usize,
}

impl Kernel {
    /// Create a kernel from a flat array of weights.
    /// Width and height must be odd numbers for proper centering.
    pub fn new(width: usize, height: usize, data: Vec<f32>) -> Self {
        assert!(width > 0 && height > 0, "Kernel dimensions must be positive");
        assert!(width % 2 == 1 && height % 2 == 1, "Kernel dimensions must be odd");
        assert_eq!(data.len(), width * height, "Data length must match dimensions");
        Self { data, width, height }
    }

    /// Create a kernel from a 2D slice (row-major).
    pub fn from_rows(rows: &[&[f32]]) -> Self {
        let height = rows.len();
        assert!(height > 0, "Kernel must have at least one row");
        let width = rows[0].len();
        assert!(width > 0, "Kernel must have at least one column");
        for row in rows {
            assert_eq!(row.len(), width, "All rows must have same length");
        }
        let data: Vec<f32> = rows.iter().flat_map(|r| r.iter().copied()).collect();
        Self::new(width, height, data)
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
    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.data[y * self.width + x]
    }

    /// Sum of all kernel weights (for normalization).
    pub fn sum(&self) -> f32 {
        self.data.iter().sum()
    }

    // --- Predefined kernels ---

    /// Identity kernel (no change).
    pub fn identity() -> Self {
        Self::new(3, 3, vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0])
    }

    /// Box blur 3x3 (uniform average).
    pub fn box_blur_3x3() -> Self {
        Self::new(3, 3, vec![1.0; 9])
    }

    /// Box blur 5x5.
    pub fn box_blur_5x5() -> Self {
        Self::new(5, 5, vec![1.0; 25])
    }

    /// Gaussian blur 3x3 (approximation).
    pub fn gaussian_3x3() -> Self {
        Self::new(
            3,
            3,
            vec![
                1.0, 2.0, 1.0,
                2.0, 4.0, 2.0,
                1.0, 2.0, 1.0,
            ],
        )
    }

    /// Gaussian blur 5x5 (approximation).
    pub fn gaussian_5x5() -> Self {
        Self::new(
            5,
            5,
            vec![
                1.0, 4.0, 7.0, 4.0, 1.0,
                4.0, 16.0, 26.0, 16.0, 4.0,
                7.0, 26.0, 41.0, 26.0, 7.0,
                4.0, 16.0, 26.0, 16.0, 4.0,
                1.0, 4.0, 7.0, 4.0, 1.0,
            ],
        )
    }

    /// Sharpen kernel.
    pub fn sharpen() -> Self {
        Self::new(
            3,
            3,
            vec![
                0.0, -1.0, 0.0,
                -1.0, 5.0, -1.0,
                0.0, -1.0, 0.0,
            ],
        )
    }

    /// Edge detection (Laplacian).
    pub fn edge_detect() -> Self {
        Self::new(
            3,
            3,
            vec![
                0.0, -1.0, 0.0,
                -1.0, 4.0, -1.0,
                0.0, -1.0, 0.0,
            ],
        )
    }

    /// Sobel operator for horizontal edges (detects vertical gradients).
    pub fn sobel_x() -> Self {
        Self::new(
            3,
            3,
            vec![
                -1.0, 0.0, 1.0,
                -2.0, 0.0, 2.0,
                -1.0, 0.0, 1.0,
            ],
        )
    }

    /// Sobel operator for vertical edges (detects horizontal gradients).
    pub fn sobel_y() -> Self {
        Self::new(
            3,
            3,
            vec![
                -1.0, -2.0, -1.0,
                0.0, 0.0, 0.0,
                1.0, 2.0, 1.0,
            ],
        )
    }

    /// Emboss kernel (3D relief effect).
    pub fn emboss() -> Self {
        Self::new(
            3,
            3,
            vec![
                -2.0, -1.0, 0.0,
                -1.0, 1.0, 1.0,
                0.0, 1.0, 2.0,
            ],
        )
    }
}

/// Edge handling mode for convolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeMode {
    /// Clamp to edge pixels.
    Clamp,
    /// Wrap around (tile).
    Wrap,
}

/// Apply a convolution kernel to an image.
/// Returns a new image with the filter applied.
pub fn convolve(image: &Image, kernel: &Kernel, edge_mode: EdgeMode) -> Image {
    let width = image.width();
    let height = image.height();
    let format = *image.format();
    let bpp = format.bytes_per_pixel();

    let mut output = Image::new(width, height, vec![0u8; width * height * bpp], format);

    let kw = kernel.width() as i32;
    let kh = kernel.height() as i32;
    let kw_half = kw / 2;
    let kh_half = kh / 2;

    let kernel_sum = kernel.sum();
    let normalize = kernel_sum.abs() > 0.001;

    for y in 0..height {
        for x in 0..width {
            let mut sum_r = 0.0f32;
            let mut sum_g = 0.0f32;
            let mut sum_b = 0.0f32;
            let mut sum_a = 0.0f32;

            for ky in 0..kh {
                for kx in 0..kw {
                    let sample_x = x as i32 + kx - kw_half;
                    let sample_y = y as i32 + ky - kh_half;

                    let (sx, sy) = match edge_mode {
                        EdgeMode::Clamp => (
                            sample_x.clamp(0, width as i32 - 1) as usize,
                            sample_y.clamp(0, height as i32 - 1) as usize,
                        ),
                        EdgeMode::Wrap => (
                            sample_x.rem_euclid(width as i32) as usize,
                            sample_y.rem_euclid(height as i32) as usize,
                        ),
                    };

                    let pixel = image.get_pixel(sx, sy);
                    let weight = kernel.get(kx as usize, ky as usize);

                    sum_r += pixel.r as f32 * weight;
                    sum_g += pixel.g as f32 * weight;
                    sum_b += pixel.b as f32 * weight;
                    sum_a += pixel.a as f32 * weight;
                }
            }

            if normalize {
                sum_r /= kernel_sum;
                sum_g /= kernel_sum;
                sum_b /= kernel_sum;
                sum_a /= kernel_sum;
            }

            let out_color = Color {
                r: sum_r.round().clamp(0.0, 255.0) as u8,
                g: sum_g.round().clamp(0.0, 255.0) as u8,
                b: sum_b.round().clamp(0.0, 255.0) as u8,
                a: sum_a.round().clamp(0.0, 255.0) as u8,
            };

            output.set_pixel(x, y, out_color);
        }
    }

    output
}

/// Apply Sobel edge detection, returning edge magnitude as grayscale.
pub fn sobel_edge_detect(image: &Image, edge_mode: EdgeMode) -> Image {
    let width = image.width();
    let height = image.height();
    let format = *image.format();
    let bpp = format.bytes_per_pixel();

    let mut output = Image::new(width, height, vec![0u8; width * height * bpp], format);

    let sobel_x = Kernel::sobel_x();
    let sobel_y = Kernel::sobel_y();

    for y in 0..height {
        for x in 0..width {
            let mut gx_r = 0.0f32;
            let mut gx_g = 0.0f32;
            let mut gx_b = 0.0f32;
            let mut gy_r = 0.0f32;
            let mut gy_g = 0.0f32;
            let mut gy_b = 0.0f32;

            for ky in 0..3i32 {
                for kx in 0..3i32 {
                    let sample_x = x as i32 + kx - 1;
                    let sample_y = y as i32 + ky - 1;

                    let (sx, sy) = match edge_mode {
                        EdgeMode::Clamp => (
                            sample_x.clamp(0, width as i32 - 1) as usize,
                            sample_y.clamp(0, height as i32 - 1) as usize,
                        ),
                        EdgeMode::Wrap => (
                            sample_x.rem_euclid(width as i32) as usize,
                            sample_y.rem_euclid(height as i32) as usize,
                        ),
                    };

                    let pixel = image.get_pixel(sx, sy);
                    let wx = sobel_x.get(kx as usize, ky as usize);
                    let wy = sobel_y.get(kx as usize, ky as usize);

                    gx_r += pixel.r as f32 * wx;
                    gx_g += pixel.g as f32 * wx;
                    gx_b += pixel.b as f32 * wx;
                    gy_r += pixel.r as f32 * wy;
                    gy_g += pixel.g as f32 * wy;
                    gy_b += pixel.b as f32 * wy;
                }
            }

            // Compute magnitude per channel, then average to grayscale
            let mag_r = (gx_r * gx_r + gy_r * gy_r).sqrt();
            let mag_g = (gx_g * gx_g + gy_g * gy_g).sqrt();
            let mag_b = (gx_b * gx_b + gy_b * gy_b).sqrt();
            let magnitude = ((mag_r + mag_g + mag_b) / 3.0).clamp(0.0, 255.0) as u8;

            let original = image.get_pixel(x, y);
            output.set_pixel(x, y, Color::RGBA(magnitude, magnitude, magnitude, original.a));
        }
    }

    output
}

// --- Color Adjustment Functions (Point Operations) ---

/// Adjust brightness by adding a value to each channel.
/// Positive values brighten, negative values darken.
pub fn adjust_brightness(image: &Image, amount: i32) -> Image {
    map_pixels(image, |c| Color {
        r: (c.r as i32 + amount).clamp(0, 255) as u8,
        g: (c.g as i32 + amount).clamp(0, 255) as u8,
        b: (c.b as i32 + amount).clamp(0, 255) as u8,
        a: c.a,
    })
}

/// Adjust contrast by scaling distance from middle gray.
/// Values > 1.0 increase contrast, < 1.0 decrease contrast.
pub fn adjust_contrast(image: &Image, factor: f32) -> Image {
    map_pixels(image, |c| {
        let adjust = |v: u8| -> u8 {
            let centered = v as f32 - 128.0;
            let scaled = centered * factor + 128.0;
            scaled.round().clamp(0.0, 255.0) as u8
        };
        Color {
            r: adjust(c.r),
            g: adjust(c.g),
            b: adjust(c.b),
            a: c.a,
        }
    })
}

/// Adjust saturation.
/// 0.0 = grayscale, 1.0 = unchanged, > 1.0 = oversaturated.
pub fn adjust_saturation(image: &Image, factor: f32) -> Image {
    map_pixels(image, |c| {
        // Luminance using standard coefficients
        let gray = 0.299 * c.r as f32 + 0.587 * c.g as f32 + 0.114 * c.b as f32;
        Color {
            r: (gray + (c.r as f32 - gray) * factor).round().clamp(0.0, 255.0) as u8,
            g: (gray + (c.g as f32 - gray) * factor).round().clamp(0.0, 255.0) as u8,
            b: (gray + (c.b as f32 - gray) * factor).round().clamp(0.0, 255.0) as u8,
            a: c.a,
        }
    })
}

/// Apply gamma correction.
/// Values > 1.0 lighten midtones, < 1.0 darken midtones.
pub fn adjust_gamma(image: &Image, gamma: f32) -> Image {
    let inv_gamma = 1.0 / gamma;
    map_pixels(image, |c| {
        let adjust = |v: u8| -> u8 {
            let normalized = v as f32 / 255.0;
            let corrected = normalized.powf(inv_gamma);
            (corrected * 255.0).round().clamp(0.0, 255.0) as u8
        };
        Color {
            r: adjust(c.r),
            g: adjust(c.g),
            b: adjust(c.b),
            a: c.a,
        }
    })
}

/// Invert colors (negative).
pub fn invert(image: &Image) -> Image {
    map_pixels(image, |c| Color {
        r: 255 - c.r,
        g: 255 - c.g,
        b: 255 - c.b,
        a: c.a,
    })
}

/// Convert to grayscale using luminance weights.
pub fn grayscale(image: &Image) -> Image {
    map_pixels(image, |c| {
        let gray = (0.299 * c.r as f32 + 0.587 * c.g as f32 + 0.114 * c.b as f32).round() as u8;
        Color {
            r: gray,
            g: gray,
            b: gray,
            a: c.a,
        }
    })
}

/// Threshold to black and white.
pub fn threshold(image: &Image, level: u8) -> Image {
    map_pixels(image, |c| {
        let gray = (0.299 * c.r as f32 + 0.587 * c.g as f32 + 0.114 * c.b as f32).round() as u8;
        let out = if gray >= level { 255 } else { 0 };
        Color {
            r: out,
            g: out,
            b: out,
            a: c.a,
        }
    })
}

/// Posterize (reduce color levels).
pub fn posterize(image: &Image, levels: u8) -> Image {
    assert!(levels >= 2, "Posterize needs at least 2 levels");
    let step = 256.0 / levels as f32;
    map_pixels(image, |c| {
        let quantize = |v: u8| -> u8 {
            let bucket = (v as f32 / step).floor();
            let center = bucket * step + step / 2.0;
            center.clamp(0.0, 255.0) as u8
        };
        Color {
            r: quantize(c.r),
            g: quantize(c.g),
            b: quantize(c.b),
            a: c.a,
        }
    })
}

/// Apply a sepia tone.
pub fn sepia(image: &Image) -> Image {
    map_pixels(image, |c| {
        let r = c.r as f32;
        let g = c.g as f32;
        let b = c.b as f32;
        Color {
            r: (0.393 * r + 0.769 * g + 0.189 * b).round().clamp(0.0, 255.0) as u8,
            g: (0.349 * r + 0.686 * g + 0.168 * b).round().clamp(0.0, 255.0) as u8,
            b: (0.272 * r + 0.534 * g + 0.131 * b).round().clamp(0.0, 255.0) as u8,
            a: c.a,
        }
    })
}

/// Helper to apply a function to each pixel.
fn map_pixels<F>(image: &Image, f: F) -> Image
where
    F: Fn(Color) -> Color,
{
    let width = image.width();
    let height = image.height();
    let format = *image.format();
    let bpp = format.bytes_per_pixel();

    let mut output = Image::new(width, height, vec![0u8; width * height * bpp], format);

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            output.set_pixel(x, y, f(pixel));
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image::PixelFormat;

    fn make_test_image() -> Image {
        // 3x3 RGB image with known values
        let data = vec![
            100, 100, 100, 150, 150, 150, 200, 200, 200,
            100, 100, 100, 150, 150, 150, 200, 200, 200,
            100, 100, 100, 150, 150, 150, 200, 200, 200,
        ];
        Image::new(3, 3, data, PixelFormat::Rgb8)
    }

    #[test]
    fn kernel_sum() {
        let box_blur = Kernel::box_blur_3x3();
        assert_eq!(box_blur.sum(), 9.0);

        let gaussian = Kernel::gaussian_3x3();
        assert_eq!(gaussian.sum(), 16.0);

        let edge = Kernel::edge_detect();
        assert_eq!(edge.sum(), 0.0);
    }

    #[test]
    fn identity_kernel_unchanged() {
        let image = make_test_image();
        let result = convolve(&image, &Kernel::identity(), EdgeMode::Clamp);

        for y in 0..3 {
            for x in 0..3 {
                assert_eq!(image.get_pixel(x, y), result.get_pixel(x, y));
            }
        }
    }

    #[test]
    fn brightness_adjustment() {
        let image = make_test_image();
        let brighter = adjust_brightness(&image, 50);
        let center = brighter.get_pixel(1, 1);
        assert_eq!(center.r, 200);

        let darker = adjust_brightness(&image, -50);
        let center = darker.get_pixel(1, 1);
        assert_eq!(center.r, 100);
    }

    #[test]
    fn grayscale_conversion() {
        let data = vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255];
        let image = Image::new(2, 2, data, PixelFormat::Rgb8);
        let gray = grayscale(&image);

        let red_gray = gray.get_pixel(0, 0);
        assert!(red_gray.r == red_gray.g && red_gray.g == red_gray.b);
        assert_eq!(red_gray.r, 76); // 0.299 * 255 ≈ 76
    }

    #[test]
    fn invert_colors() {
        let data = vec![0, 128, 255];
        let image = Image::new(1, 1, data, PixelFormat::Rgb8);
        let inverted = invert(&image);
        let pixel = inverted.get_pixel(0, 0);
        assert_eq!(pixel.r, 255);
        assert_eq!(pixel.g, 127);
        assert_eq!(pixel.b, 0);
    }

    #[test]
    fn threshold_binary() {
        let data = vec![50, 50, 50, 200, 200, 200];
        let image = Image::new(2, 1, data, PixelFormat::Rgb8);
        let binary = threshold(&image, 128);

        assert_eq!(binary.get_pixel(0, 0).r, 0);
        assert_eq!(binary.get_pixel(1, 0).r, 255);
    }
}
