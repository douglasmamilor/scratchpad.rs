/// DepthBuffer module provides a simple depth buffer implementation for depth testing
pub struct DepthBuffer {
    width: usize,
    height: usize,
    pixels: Vec<f32>,
}

impl DepthBuffer {
    /// Create a new depth buffer with the given dimensions
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![f32::INFINITY; width * height],
        }
    }

    /// Clear the depth buffer to a specific depth value
    pub fn clear(&mut self, depth: f32) {
        self.pixels.fill(depth);
    }

    /// Resize the depth buffer to new dimensions
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.pixels.resize(width * height, f32::INFINITY);
    }

    /// Get the width of the depth buffer
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the height of the depth buffer
    pub fn height(&self) -> usize {
        self.height
    }

    /// Set a depth value at the given coordinates
    #[inline]
    pub fn set_depth(&mut self, x: usize, y: usize, depth: f32) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = depth;
        }
    }

    /// Get a depth value at the given coordinates
    #[inline]
    pub fn get_depth(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x]
        } else {
            f32::INFINITY
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_depth_buffer_initializes_to_infinity() {
        let db = DepthBuffer::new(800, 600);
        assert_eq!(db.width(), 800);
        assert_eq!(db.height(), 600);
        assert_eq!(db.pixels.len(), 800 * 600);
        assert!(db.pixels.iter().all(|&d| d.is_infinite()));

        // Spot-check a few coordinates through the public API
        assert!(db.get_depth(0, 0).is_infinite());
        assert!(db.get_depth(799, 599).is_infinite());
    }

    #[test]
    fn clear_sets_all_depths_to_given_value() {
        let mut db = DepthBuffer::new(10, 10);
        db.clear(1.0);

        assert!(db.pixels.iter().all(|&d| d == 1.0));
        // Public API check
        assert_eq!(db.get_depth(0, 0), 1.0);
        assert_eq!(db.get_depth(9, 9), 1.0);
    }

    #[test]
    fn resize_updates_dimensions_and_capacity() {
        let mut db = DepthBuffer::new(2, 2);
        db.set_depth(0, 0, 0.5);
        db.set_depth(1, 1, 0.25);

        db.resize(3, 4);
        assert_eq!(db.width(), 3);
        assert_eq!(db.height(), 4);
        assert_eq!(db.pixels.len(), 3 * 4);

        // Existing entries are preserved in the underlying vector,
        // new entries are initialized to INFINITY.
        assert_eq!(db.pixels[0], 0.5);
        assert_eq!(db.pixels[3], 0.25);
        assert!(db.pixels[4..].iter().all(|&d| d.is_infinite()));
    }

    #[test]
    fn set_and_get_depth_in_bounds() {
        let mut db = DepthBuffer::new(5, 5);
        db.clear(1.0);

        db.set_depth(2, 3, 0.7);
        assert_eq!(db.get_depth(2, 3), 0.7);

        // Other cells remain unchanged
        assert_eq!(db.get_depth(0, 0), 1.0);
        assert_eq!(db.get_depth(4, 4), 1.0);
    }

    #[test]
    fn out_of_bounds_get_returns_infinity() {
        let db = DepthBuffer::new(4, 4);

        assert!(db.get_depth(4, 0).is_infinite());
        assert!(db.get_depth(0, 4).is_infinite());
        assert!(db.get_depth(10, 10).is_infinite());
    }

    #[test]
    fn out_of_bounds_set_does_not_modify_buffer() {
        let mut db = DepthBuffer::new(4, 4);
        db.clear(1.0);
        let before = db.pixels.clone();

        db.set_depth(10, 10, 0.0);
        db.set_depth(4, 0, 0.0);
        db.set_depth(0, 4, 0.0);

        assert_eq!(db.pixels, before);
    }
}
