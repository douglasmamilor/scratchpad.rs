mod circle;
mod debug;
mod ellipse;
mod fill;
mod helpers;
mod polygon;
mod polyline;
mod raster_line;
mod raster_line_aa;
mod rectangle;
pub mod stroke;
mod triangle;
mod triangle_barycentric;

pub use helpers::{quantize_hspan, quantize_point, quantize_vspan, snap_axis};
pub use polygon::FillRule;
pub use polyline::PolyLine;
pub use stroke::pattern::apply_stroke_pattern;
pub use stroke::types::{LineCap, LineJoin, StrokePattern, StrokeSpace, StrokeStyle};

use crate::color::Color;
use crate::framebuffer::{DepthBuffer, DepthFunc, DepthState, FrameBuffer};

pub struct Renderer<'a> {
    framebuffer: &'a mut FrameBuffer,
    depth_buffer: DepthBuffer,
    depth_state: DepthState,
}

impl<'a> Renderer<'a> {
    pub fn new(framebuffer: &'a mut FrameBuffer) -> Self {
        let depth_state = DepthState::default();
        let depth_buffer = DepthBuffer::new(framebuffer.width(), framebuffer.height());

        Self {
            framebuffer,
            depth_buffer,
            depth_state,
        }
    }

    /// Resize the internal depth buffer to match the current framebuffer size.
    ///
    /// Useful if you hold onto a `Renderer` across framebuffer resizes instead
    /// of constructing a new one each frame.
    #[inline]
    pub fn resize_depth_to_framebuffer(&mut self) {
        self.depth_buffer
            .resize(self.framebuffer.width(), self.framebuffer.height());
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
    fn depth_test(&mut self, x: i32, y: i32, depth: f32) -> bool {
        // When depth is disabled, always pass without touching the buffer.
        if !self.depth_state.enabled {
            return true;
        }

        if !self.in_bounds(x, y) {
            return false;
        }

        let (ux, uy) = (x as usize, y as usize);
        let current = self.depth_buffer.get_depth(ux, uy);

        let pass = match self.depth_state.func {
            DepthFunc::Less => depth < current,
            DepthFunc::LessEq => depth <= current,
        };

        if pass && self.depth_state.write_enabled {
            self.depth_buffer.set_depth(ux, uy, depth);
        }

        pass
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
    pub fn set_pixel(&mut self, point: (i32, i32), color: Color) {
        if self.in_bounds(point.0, point.1) {
            self.framebuffer
                .set_pixel(point.0 as usize, point.1 as usize, color.to_u32());
        }
    }

    #[inline]
    pub fn get_depth(&mut self, point: (usize, usize)) -> f32 {
        self.depth_buffer.get_depth(point.0, point.1)
    }

    #[inline]
    pub fn set_depth(&mut self, point: (usize, usize), depth: f32) {
        self.depth_buffer.set_depth(point.0, point.1, depth);
    }

    #[inline]
    pub fn clear(&mut self, color: Color) {
        self.framebuffer.clear(color.to_u32());
        self.depth_buffer.clear(self.depth_state.clear_value);
    }

    #[inline]
    pub fn hspan(&mut self, y: i32, mut x0: i32, mut x: i32, color: Color) {
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

        // For now we use a default layer depth (0.0). When a proper 3D
        // pipeline is added, per-fragment depths will be passed instead.
        let depth = 0.0f32;
        for xi in x0..=x {
            if self.depth_test(xi, y, depth) {
                self.set_pixel((xi, y), color);
            }
        }
    }

    #[inline]
    pub fn vspan(&mut self, x: i32, mut y0: i32, mut y: i32, color: Color) {
        if y0 > y {
            std::mem::swap(&mut y, &mut y0);
        }

        for yi in y0..=y {
            self.set_pixel((x, yi), color);
        }
    }
}
