use crate::renderer::{Color, Renderer};

impl<'a> Renderer<'a> {
    pub fn draw_ellipse(&mut self, ctr: (i32, i32), rx: i32, ry: i32, color: &Color) {
        if rx < 0 && ry < 0 {
            return;
        }

        let (cx, cy) = ctr;

        // Degenerate case: single point
        if rx == 0 && ry == 0 {
            self.set_pixel((cx, cy), color);
        }

        // Degenerate case: horizontal line
        if ry == 0 {
            self.hspan(cy, cx - rx, cx + rx, color);
        }

        // Degenerate case: vertical line
        if rx == 0 {
            self.vspan(cx, cy - ry, cy + ry, color);
        }

        let rx2 = (rx as i64) * (rx as i64);
        let ry2 = (ry as i64) * (ry as i64);
        let two_rx2: i64 = 2 * rx2;
        let two_ry2: i64 = 2 * ry2;

        let mut x: i64 = 0;
        let mut y: i64 = ry as i64;

        let mut px: i64 = 0;
        let mut py: i64 = two_rx2 * y;

        // Region 1 decision parameter
        let mut p1: i64 = ry2 - rx2 * y + (rx2 / 4);

        // Helper: plot 4 symmetric points
        let mut plot4 = |cx: i32, cy: i32, x: i64, y: i64| {
            let xi = x as i32;
            let yi = y as i32;
            self.set_pixel((cx + xi, cy + yi), color);
            self.set_pixel((cx - xi, cy + yi), color);
            self.set_pixel((cx + xi, cy - yi), color);
            self.set_pixel((cx - xi, cy - yi), color);
        };

        // ----------------
        // Region 1: |dy/dx| <= 1  (px < py)
        // ----------------
        while px < py {
            plot4(cx, cy, x, y);

            x += 1;
            px += two_ry2;

            if p1 < 0 {
                // E step
                p1 += px + ry2;
            } else {
                // SE step
                y -= 1;
                py -= two_rx2;
                p1 += px - py + ry2;
            }
        }

        // ----------------
        // Region 2 init
        // p2 = ry^2*(x+0.5)^2 + rx^2*(y-1)^2 - rx^2*ry^2
        // Integerized: ry2*(x*x + x) + (ry2/4) + rx2*(y*y - 2*y + 1) - rx2*ry2
        // ----------------
        let mut p2: i64 = ry2 * (x * x + x) + (ry2 / 4) + rx2 * (y * y - 2 * y + 1) - rx2 * ry2;

        // ----------------
        // Region 2: |dy/dx| > 1  (descending y)
        // ----------------
        while y >= 0 {
            plot4(cx, cy, x, y);

            y -= 1;
            py -= two_rx2;

            if p2 > 0 {
                // S step
                p2 += rx2 - py;
            } else {
                // SE step
                x += 1;
                px += two_ry2;
                p2 += px - py + rx2;
            }
        }
    }
}
