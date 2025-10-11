mod circle;
mod ellipse;
mod fill;
mod line;
mod line_aa;
mod polygon;
mod rectangle;
mod triangle;

use crate::color::Color;
use crate::framebuffer::FrameBuffer;

pub struct Renderer<'a> {
    framebuffer: &'a mut FrameBuffer,
}

impl<'a> Renderer<'a> {
    pub fn new(framebuffer: &'a mut FrameBuffer) -> Self {
        Self { framebuffer }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.framebuffer.width()
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.framebuffer.height()
    }

    #[inline]
    fn in_bounds_y(&self, y: i32) -> bool {
        y >= 0 && (y as i64) < self.framebuffer.height() as i64
    }

    #[inline]
    fn in_bounds_x(&self, y: i32) -> bool {
        y >= 0 && (y as i64) < self.framebuffer.height() as i64
    }

    #[inline]
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0
            && y >= 0
            && (x as i64) < self.framebuffer.width() as i64
            && (y as i64) < self.framebuffer.height() as i64
    }

    #[inline]
    fn get_pixel(&self, point: (i32, i32)) -> Option<Color> {
        let (x, y) = (point.0, point.1);

        if self.in_bounds(x, y) {
            self.framebuffer
                .get_pixel(x as usize, y as usize)
                .map(Color::from_u32)
        } else {
            None
        }
    }

    #[inline]
    pub fn set_pixel(&mut self, point: (i32, i32), color: &Color) {
        if self.in_bounds(point.0, point.1) {
            self.framebuffer
                .set_pixel(point.0 as usize, point.1 as usize, color.to_u32());
        }
    }

    #[inline]
    pub fn clear(&mut self, color: &Color) {
        self.framebuffer.clear(color.to_u32());
    }

    #[inline]
    pub fn hspan(&mut self, y: i32, mut x0: i32, mut x: i32, color: &Color) {
        if x0 > x {
            std::mem::swap(&mut x, &mut x0);
        }

        // --- boundary check ---
        if y < 0 || y >= self.framebuffer.height() as i32 {
            return;
        }
        let w = self.framebuffer.width() as i32;
        // if both ends are out of bounds after the swap, return
        if x < 0 || x0 >= w {
            return;
        }
        x0 = x0.clamp(0, w - 1);
        x = x.clamp(0, w - 1);
        // -----------------------

        for xi in x0..=x {
            self.set_pixel((xi, y), color);
        }
    }

    #[inline]
    pub fn vspan(&mut self, x: i32, mut y0: i32, mut y: i32, color: &Color) {
        if y0 > y {
            std::mem::swap(&mut y, &mut y0);
        }

        for yi in y0..=y {
            self.set_pixel((x, yi), color);
        }
    }
}
