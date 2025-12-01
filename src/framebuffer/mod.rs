pub mod depth_buffer;
pub mod depth_state;

pub use depth_buffer::DepthBuffer;
pub use depth_state::{DepthFunc, DepthState};

const BYTES_PER_PIXEL: usize = 4;

/// A framebuffer for storing ARGB8888 pixels
pub struct FrameBuffer {
    width: usize,
    height: usize,
    pitch: usize,
    pub pixels: Vec<u32>, // ARGB8888
}

impl FrameBuffer {
    /// Create a new framebuffer with the given dimensions
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pitch: width * BYTES_PER_PIXEL,
            pixels: vec![0u32; width * height],
        }
    }

    /// Resize the framebuffer to new dimensions
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.pitch = width * BYTES_PER_PIXEL;
        self.pixels.resize(width * height, 0u32);
    }

    /// Get the width of the framebuffer
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the height of the framebuffer
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get the pitch (bytes per row) of the framebuffer
    pub fn pitch(&self) -> usize {
        self.pitch
    }

    /// Clear the framebuffer to a specific color
    pub fn clear(&mut self, color: u32) {
        self.pixels.fill(color);
    }

    /// Set a pixel at the given coordinates
    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        // TODO: For now all clipping is done here. When we get to the clipping module we need to
        // clip at higher level to unnecessary allocations.
        // Also at a minimum clipping should be done in the renderer in order to keep this struct
        // dumb
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    /// Get a pixel at the given coordinates
    #[inline]
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u32> {
        if x < self.width && y < self.height {
            Some(self.pixels[y * self.width + x])
        } else {
            None
        }
    }

    /// Get the framebuffer as a byte slice for texture upload
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.pixels.as_ptr() as *const u8,
                self.pixels.len() * std::mem::size_of::<u32>(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_framebuffer() {
        let fb = FrameBuffer::new(800, 600);
        assert_eq!(fb.width(), 800);
        assert_eq!(fb.height(), 600);
        assert_eq!(fb.pitch(), 800 * 4);
        assert_eq!(fb.pixels.len(), 800 * 600);
        // Should be initialized to black (0)
        assert!(fb.pixels.iter().all(|&p| p == 0));
    }

    #[test]
    fn test_resize() {
        let mut fb = FrameBuffer::new(100, 100);
        assert_eq!(fb.pixels.len(), 10000);

        fb.resize(200, 150);
        assert_eq!(fb.width(), 200);
        assert_eq!(fb.height(), 150);
        assert_eq!(fb.pitch(), 200 * 4);
        assert_eq!(fb.pixels.len(), 30000);
    }

    #[test]
    fn test_clear() {
        let mut fb = FrameBuffer::new(10, 10);
        let red = 0xFFFF0000u32;

        fb.clear(red);
        assert!(fb.pixels.iter().all(|&p| p == red));
    }

    #[test]
    fn test_set_pixel_valid() {
        let mut fb = FrameBuffer::new(100, 100);
        let white = 0xFFFFFFFF;

        fb.set_pixel(50, 50, white);
        assert_eq!(fb.pixels[50 * 100 + 50], white);

        // Test corners
        fb.set_pixel(0, 0, white);
        assert_eq!(fb.pixels[0], white);

        fb.set_pixel(99, 99, white);
        assert_eq!(fb.pixels[99 * 100 + 99], white);
    }

    #[test]
    fn test_set_pixel_out_of_bounds() {
        let mut fb = FrameBuffer::new(100, 100);
        let white = 0xFFFFFFFF;

        // These should not panic, just do nothing
        fb.set_pixel(100, 50, white); // x out of bounds
        fb.set_pixel(50, 100, white); // y out of bounds
        fb.set_pixel(200, 200, white); // Both out of bounds

        // Verify nothing was written
        assert!(fb.pixels.iter().all(|&p| p == 0));
    }

    #[test]
    fn test_get_pixel() {
        let mut fb = FrameBuffer::new(100, 100);
        let blue = 0xFF0000FF;

        fb.set_pixel(25, 25, blue);

        // Valid coordinates
        assert_eq!(fb.get_pixel(25, 25), Some(blue));
        assert_eq!(fb.get_pixel(0, 0), Some(0));

        // Out of bounds
        assert_eq!(fb.get_pixel(100, 50), None);
        assert_eq!(fb.get_pixel(50, 100), None);
        assert_eq!(fb.get_pixel(200, 200), None);
    }

    #[test]
    fn test_as_bytes() {
        let fb = FrameBuffer::new(2, 2);
        let bytes = fb.as_bytes();

        // 2x2 pixels * 4 bytes per pixel = 16 bytes
        assert_eq!(bytes.len(), 16);
    }

    #[test]
    fn test_pixel_indexing() {
        let mut fb = FrameBuffer::new(10, 10);

        // Test that pixel indexing is row-major (y * width + x)
        for y in 0..10 {
            for x in 0..10 {
                let color = (y * 10 + x) as u32;
                fb.set_pixel(x, y, color);
            }
        }

        for y in 0..10 {
            for x in 0..10 {
                let expected = (y * 10 + x) as u32;
                assert_eq!(fb.get_pixel(x, y), Some(expected));
            }
        }
    }
}
