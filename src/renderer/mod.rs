mod lines;
mod lines_aa;

use crate::color::Color;
use crate::framebuffer::FrameBuffer;

pub struct Renderer<'a> {
    framebuffer: &'a mut FrameBuffer,
}

impl<'a> Renderer<'a> {
    pub fn new(framebuffer: &'a mut FrameBuffer) -> Self {
        Self { framebuffer }
    }

    pub fn width(&self) -> usize {
        self.framebuffer.width()
    }

    pub fn height(&self) -> usize {
        self.framebuffer.height()
    }

    pub fn set_pixel(&mut self, point: (i32, i32), color: &Color) {
        self.framebuffer
            .set_pixel(point.0 as usize, point.1 as usize, color.to_u32());
    }

    pub fn clear(&mut self, color: &Color) {
        self.framebuffer.clear(color.to_u32());
    }
}
