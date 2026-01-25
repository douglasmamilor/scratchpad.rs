use crate::{Point2, Rect};

type OutCode = u8;

const LEFT: OutCode = 1 << 0;
const RIGHT: OutCode = 1 << 1;
const BOTTOM: OutCode = 1 << 2;
const TOP: OutCode = 1 << 3;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    a: Point2, // start point
    b: Point2, // end point
}

impl Line {
    /// Clip to a rectangle using the Cohen-Sutherland algorithm
    pub fn clip_to_rect(&self, rect: Rect) -> Option<Line> {
        let (mut a, mut b) = (self.a, self.b);
        let mut code_a = self.out_code(a, rect);
        let mut code_b = self.out_code(b, rect);
        let x_min = rect.x;
        let x_max = rect.x + rect.width;
        let y_min = rect.y;
        let y_max = rect.y + rect.height;
        let mut t: f32;

        loop {
            if code_a == 0 && code_b == 0 {
                return Some(Line { a, b });
            }

            if (code_a & code_b) != 0 {
                return None;
            }

            let code_out = if code_a != 0 { code_a } else { code_b };
            let x: f32;
            let y: f32;

            if code_out & LEFT != 0 {
                x = x_min;
                t = (x - a.x) / (b.x - a.x);
                y = a.y + t * (b.y - a.y);
            } else if code_out & RIGHT != 0 {
                x = x_max;
                t = (x - a.x) / (b.x - a.x);
                y = a.y + t * (b.y - a.y);
            } else if code_out & BOTTOM != 0 {
                y = y_min;
                t = (y - a.y) / (b.y - a.y);
                x = a.x + t * (b.x - a.x);
            } else if code_out & TOP != 0 {
                y = y_max;
                t = (y - a.y) / (b.y - a.y);
                x = a.x + t * (b.x - a.x);
            } else {
                return None;
            }

            if code_out == code_a {
                a = Point2 { x, y };
                code_a = self.out_code(a, rect);
            } else {
                b = Point2 { x, y };
                code_b = self.out_code(b, rect);
            }
        }
    }

    fn out_code(&self, p: Point2, rect: Rect) -> OutCode {
        let mut code = 0;
        let x_min = rect.x;
        let x_max = rect.x + rect.width;
        let y_min = rect.y;
        let y_max = rect.y + rect.height;

        if p.x < x_min {
            code |= LEFT;
        }
        if p.x > x_max {
            code |= RIGHT;
        }
        if p.y < y_min {
            code |= BOTTOM;
        }
        if p.y > y_max {
            code |= TOP;
        }

        code
    }
}
