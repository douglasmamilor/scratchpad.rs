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