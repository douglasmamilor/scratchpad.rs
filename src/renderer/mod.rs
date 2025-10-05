mod circle;
mod ellipse;
mod line;
mod line_aa;
mod polygon;
mod rectangle;

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
    pub fn set_pixel(&mut self, point: (i32, i32), color: &Color) {
        self.framebuffer
            .set_pixel(point.0 as usize, point.1 as usize, color.to_u32());
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
