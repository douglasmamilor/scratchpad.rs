use crate::{
    color::Color,
    math::{Mat3, Vec2},
    renderer::Renderer,
    transform::TransformStack,
};

pub struct Canvas<'a, 'r> {
    r: &'r mut Renderer<'a>,
    ts: TransformStack,
}

impl<'a, 'r> Canvas<'a, 'r> {
    #[inline]
    pub fn push(&mut self, m: Mat3) {
        self.ts.push(m);
    }

    #[inline]
    pub fn pop(&mut self) {
        self.ts.pop();
    }

    #[inline]
    pub fn translate(&mut self, x: f32, y: f32) {
        self.ts.translate(x, y);
    }

    #[inline]
    pub fn rotate(&mut self, a: f32) {
        self.ts.rotate(a);
    }

    #[inline]
    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.ts.scale(sx, sy);
    }

    #[inline]
    pub fn draw_line(&mut self, a: Vec2, b: Vec2, color: Color) {
        self.r.draw_line_aa(a, b, color, self.ts.current());
    }

    pub fn with(&mut self, m: Mat3, f: impl FnOnce(&mut Self)) {
        self.push(m);
        f(self);
        self.pop();
    }
}
