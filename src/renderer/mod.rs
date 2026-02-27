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
mod sprite;
mod stroke;
mod texture;
mod triangle;
mod triangle_barycentric;

pub(crate) use helpers::{quantize_hspan, quantize_point, quantize_vspan, snap_axis};
pub use polygon::FillRule;
pub use polyline::PolyLine;
pub use sprite::Sprite;
pub(crate) use stroke::pattern::apply_stroke_pattern;
pub use stroke::types::{LineCap, LineJoin, PatternSpace, StrokePattern, StrokeSpace, StrokeStyle};
pub use texture::{SamplingMode, Texture};

use crate::Color;
use crate::framebuffer::{DepthBuffer, DepthFunc, DepthState, FrameBuffer};
use crate::math::Rect;
#[derive(Clone, Copy, Debug)]
struct Scissor {
    /// Half-open integer bounds in screen space: [x0, x1), [y0, y1)
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

pub struct Renderer<'a> {
    framebuffer: &'a mut FrameBuffer,
    depth_buffer: DepthBuffer,
    depth_state: DepthState,
    scissor: Option<Scissor>,
    aa_triangles: bool,
    aa_supersample: bool,
    aa_gamma: bool,
}

impl<'a> Renderer<'a> {
    pub fn new(framebuffer: &'a mut FrameBuffer) -> Self {
        let depth_state = DepthState::default();
        let depth_buffer = DepthBuffer::new(framebuffer.width(), framebuffer.height());

        Self {
            framebuffer,
            depth_buffer,
            depth_state,
            scissor: None,
            aa_triangles: false,
            aa_supersample: true, // when AA enabled, default to 2x2 SSAA
            aa_gamma: true,
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

    /// Enable a rectangular scissor in screen space (pixels). Empty or out-of-bounds
    /// rect disables scissoring. Bounds are clamped to the framebuffer.
    pub fn set_scissor(&mut self, rect: Rect) {
        let w = self.framebuffer.width() as i32;
        let h = self.framebuffer.height() as i32;

        // Convert to half-open integer bounds.
        let mut x0 = rect.x.floor() as i32;
        let mut y0 = rect.y.floor() as i32;
        let mut x1 = (rect.x + rect.width).ceil() as i32;
        let mut y1 = (rect.y + rect.height).ceil() as i32;

        // Clamp to framebuffer.
        x0 = x0.clamp(0, w);
        x1 = x1.clamp(0, w);
        y0 = y0.clamp(0, h);
        y1 = y1.clamp(0, h);

        if x0 >= x1 || y0 >= y1 {
            self.scissor = None;
        } else {
            self.scissor = Some(Scissor { x0, y0, x1, y1 });
        }
    }

    /// Disable scissoring.
    pub fn clear_scissor(&mut self) {
        self.scissor = None;
    }

    /// Toggle simple triangle anti-aliasing (off by default).
    pub fn set_triangle_aa_enabled(&mut self, enabled: bool) {
        self.aa_triangles = enabled;
    }

    /// Choose whether triangle AA uses 2x2 supersampling (default true when AA is enabled).
    pub fn set_triangle_aa_supersample(&mut self, enabled: bool) {
        self.aa_supersample = enabled;
    }

    /// Enable/disable gamma-aware blending for AA edges (default true).
    pub fn set_triangle_aa_gamma(&mut self, enabled: bool) {
        self.aa_gamma = enabled;
    }

    /// Returns the active clip rect (scissor if set, otherwise full viewport),
    /// expressed as an inclusive rectangle suitable for polygon clipping.
    fn active_clip_rect(&self) -> Option<Rect> {
        let w = self.framebuffer.width() as i32;
        let h = self.framebuffer.height() as i32;
        if w <= 0 || h <= 0 {
            return None;
        }

        let (x0, y0, x1, y1) = if let Some(sc) = self.scissor {
            (sc.x0, sc.y0, sc.x1, sc.y1)
        } else {
            (0, 0, w, h)
        };

        if x0 >= x1 || y0 >= y1 {
            return None;
        }

        // Inclusive max: width/height account for pixel centers from x0..=x1_inclusive.
        let width = (x1 - 1 - x0) as f32;
        let height = (y1 - 1 - y0) as f32;
        if width < 0.0 || height < 0.0 {
            return None;
        }

        Some(Rect {
            x: x0 as f32,
            y: y0 as f32,
            width,
            height,
        })
    }

    #[inline]
    fn depth_test(&mut self, x: i32, y: i32, depth: f32) -> bool {
        // When depth is disabled, always pass without touching the buffer.
        if !self.depth_state.enabled {
            return true;
        }

        if !self.in_clip(x, y) {
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
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0
            && y >= 0
            && (x as i64) < self.framebuffer.width() as i64
            && (y as i64) < self.framebuffer.height() as i64
    }

    #[inline]
    fn in_scissor(&self, x: i32, y: i32) -> bool {
        if let Some(sc) = self.scissor {
            x >= sc.x0 && x < sc.x1 && y >= sc.y0 && y < sc.y1
        } else {
            true
        }
    }

    #[inline]
    fn in_clip(&self, x: i32, y: i32) -> bool {
        self.in_bounds(x, y) && self.in_scissor(x, y)
    }

    #[inline]
    fn get_pixel(&self, point: (i32, i32)) -> Option<Color> {
        let (x, y) = (point.0, point.1);

        if self.in_clip(x, y) {
            self.framebuffer
                .get_pixel(x as usize, y as usize)
                .map(Color::from_u32)
        } else {
            None
        }
    }

    #[inline]
    pub fn set_pixel(&mut self, point: (i32, i32), color: Color) {
        if self.in_clip(point.0, point.1) {
            self.framebuffer
                .set_pixel(point.0 as usize, point.1 as usize, color.to_u32());
        }
    }

    /// Alpha blend `src` over the destination at (x, y) with the given coverage in [0, 1].
    #[inline]
    fn blend_coverage(&mut self, x: i32, y: i32, src: Color, coverage: f32) {
        if coverage <= 0.0 {
            return;
        }
        if !self.in_clip(x, y) {
            return;
        }

        let cov = coverage.clamp(0.0, 1.0);
        let dst = self
            .framebuffer
            .get_pixel(x as usize, y as usize)
            .map(Color::from_u32)
            .unwrap_or(Color::TRANSPARENT);

        let blend_chan = |s: u8, d: u8| -> u8 {
            if self.aa_gamma {
                let sf = (s as f32 / 255.0).powf(2.2);
                let df = (d as f32 / 255.0).powf(2.2);
                let lin = sf * cov + df * (1.0 - cov);
                (lin.powf(1.0 / 2.2) * 255.0).round().clamp(0.0, 255.0) as u8
            } else {
                let sf = s as f32;
                let df = d as f32;
                ((sf * cov + df * (1.0 - cov)).round()).clamp(0.0, 255.0) as u8
            }
        };

        let out = Color {
            r: blend_chan(src.r, dst.r),
            g: blend_chan(src.g, dst.g),
            b: blend_chan(src.b, dst.b),
            a: 255,
        };

        self.framebuffer
            .set_pixel(x as usize, y as usize, out.to_u32());
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

        // Apply scissor horizontally/vertically.
        if let Some(sc) = self.scissor {
            if y < sc.y0 || y >= sc.y1 {
                return;
            }
            x0 = x0.max(sc.x0);
            x = x.min(sc.x1 - 1);
            if x0 > x {
                return;
            }
        }
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

// ------------------------------
// Tests
// ------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::FrameBuffer;
    use crate::{Mat3, Vec2};

    fn solid_rect_pixels(fb: &FrameBuffer) -> usize {
        fb.pixels.iter().filter(|&&p| p != 0).count()
    }

    #[test]
    fn scissor_clamps_and_blocks_outside_pixels() {
        let mut fb = FrameBuffer::new(10, 10);
        let mut r = Renderer::new(&mut fb);
        r.clear(Color::TRANSPARENT);

        // Set scissor to a small region.
        r.set_scissor(Rect::new(2.0, 2.0, 4.0, 4.0)); // covers x=2..5, y=2..5

        // Draw a big rect covering the whole buffer.
        r.fill_rect(
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 10.0),
            Color::WHITE,
            Mat3::IDENTITY,
        );

        // Only scissored area should be filled: 4x4 = 16 pixels.
        assert_eq!(solid_rect_pixels(&fb), 16);
    }

    #[test]
    fn triangle_is_clipped_to_scissor() {
        let mut fb = FrameBuffer::new(20, 20);
        let mut r = Renderer::new(&mut fb);
        r.clear(Color::TRANSPARENT);

        // Scissor a small box in the center.
        r.set_scissor(Rect::new(8.0, 8.0, 4.0, 4.0)); // x=8..11, y=8..11

        // Large triangle covering most of the framebuffer.
        r.fill_triangle(
            Vec2::new(0.0, 0.0),
            Vec2::new(19.0, 0.0),
            Vec2::new(0.0, 19.0),
            Color::WHITE,
            Mat3::IDENTITY,
        );

        // No pixels outside the scissor; some pixels inside.
        let mut inside_count = 0;
        for y in 0..20 {
            for x in 0..20 {
                let filled = fb.get_pixel(x, y).unwrap_or(0) != 0;
                let inside = x >= 8 && x <= 11 && y >= 8 && y <= 11;
                if filled && !inside {
                    panic!("Pixel outside scissor filled at ({x},{y})");
                }
                if filled && inside {
                    inside_count += 1;
                }
            }
        }
        assert!(
            inside_count > 0,
            "Triangle produced no pixels inside scissor"
        );
    }

    #[test]
    fn aa_triangle_hits_subpixel_corner() {
        let mut fb = FrameBuffer::new(2, 2);
        let mut r = Renderer::new(&mut fb);
        r.clear(Color::TRANSPARENT);

        // Tiny triangle in the bottom-left pixel corner.
        r.fill_triangle_aa(
            Vec2::new(0.1, 0.1),
            Vec2::new(0.9, 0.1),
            Vec2::new(0.1, 0.9),
            Color::WHITE,
            Mat3::IDENTITY,
        );

        // Expect pixel (0,0) to have some coverage.
        assert!(fb.get_pixel(0, 0).unwrap_or(0) != 0);
    }
}
