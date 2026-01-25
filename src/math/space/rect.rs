#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width / self.height
    }

    pub fn is_landscape(&self) -> bool {
        self.width > self.height
    }

    pub fn is_portrait(&self) -> bool {
        self.height > self.width
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;

    #[test]
    fn aspect_ratio() {
        let r = Rect::new(0.0, 0.0, 16.0, 9.0);

        assert_eq!(r.aspect_ratio(), 16.0 / 9.0);
    }

    #[test]
    fn is_landscape() {
        let r = Rect::new(0.0, 0.0, 16.0, 9.0);

        assert!(r.is_landscape());
    }

    #[test]
    fn is_portrait() {
        let r = Rect::new(0.0, 0.0, 9.0, 19.0);

        assert!(r.is_portrait());
    }
}
