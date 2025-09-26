use crate::renderer::{Color, Renderer};

impl<'a> Renderer<'a> {
    /// Draw outline for axis-aligned ellipse via midpoint (Bresenham-style) algorithm.
    /// Center `ctr` in pixels; radii `rx`, `ry` must be non-negative.
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
        // Region 1: |dy/dx| <= 1  (px < py) (advancing x)
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

    /// Filled, axis-aligned ellipse via midpoint (Bresenham-style) algorithm.
    /// Center `ctr` in pixels; radii `rx`, `ry` must be non-negative.
    pub fn fill_ellipse(&mut self, ctr: (i32, i32), rx: i32, ry: i32, color: &Color) {
        if rx < 0 || ry < 0 {
            return;
        }

        let (cx, cy) = ctr;

        // Degenerates
        if rx == 0 && ry == 0 {
            self.set_pixel((cx, cy), color);
            return;
        }
        if ry == 0 {
            // single horizontal span on y = cy
            self.hspan(cy, cx - rx, cx + rx, color);
            return;
        }
        if rx == 0 {
            // vertical line on x = cx
            self.vspan(cx, cy - ry, cy + ry, color);
            return;
        }

        // Use i64 internally to avoid overflow for large radii.
        let rx2: i64 = (rx as i64) * (rx as i64);
        let ry2: i64 = (ry as i64) * (ry as i64);
        let two_rx2: i64 = 2 * rx2;
        let two_ry2: i64 = 2 * ry2;

        // Start at (x, y) = (0, ry) in first quadrant
        let mut x: i64 = 0;
        let mut y: i64 = ry as i64;

        // Accumulators for region switch
        let mut px: i64 = 0; // = 2*ry^2*x
        let mut py: i64 = two_rx2 * y; // = 2*rx^2*y

        // Region 1 decision parameter (integerized midpoint)
        // p1 = ry^2 - rx^2*ry + 0.25*rx^2  ≈  ry2 - rx2*y + (rx2 / 4)
        let mut p1: i64 = ry2 - rx2 * y + (rx2 / 4);

        // Helper that draws the two horizontal spans for current (x, y)
        let draw_span_pair = |cx: i32, cy: i32, x: i64, y: i64, this: &mut Renderer<'a>| {
            let xi = x as i32;
            let yi = y as i32;
            let x_left = cx - xi;
            let x_right = cx + xi;

            if yi == 0 {
                // Both rows coincide at y = cy
                this.hspan(cy, x_left, x_right, color);
            } else {
                this.hspan(cy + yi, x_left, x_right, color);
                this.hspan(cy - yi, x_left, x_right, color);
            }
        };

        // -------------
        // Region 1: |dy/dx| <= 1 (advance x)
        // -------------
        while px < py {
            draw_span_pair(cx, cy, x, y, self);

            x += 1;
            px += two_ry2;

            if p1 < 0 {
                // E
                p1 += px + ry2;
            } else {
                // SE
                y -= 1;
                py -= two_rx2;
                p1 += px - py + ry2;
            }
        }

        // -------------
        // Region 2 init:
        // p2 = ry^2*(x+0.5)^2 + rx^2*(y-1)^2 - rx^2*ry^2
        // integerized: ry2*(x*x + x) + (ry2/4) + rx2*(y*y - 2*y + 1) - rx2*ry2
        // -------------
        let mut p2: i64 = ry2 * (x * x + x) + (ry2 / 4) + rx2 * (y * y - 2 * y + 1) - rx2 * ry2;

        // -------------
        // Region 2: |dy/dx| > 1 (advance y)
        // -------------
        while y >= 0 {
            draw_span_pair(cx, cy, x, y, self);

            y -= 1;
            py -= two_rx2;

            if p2 > 0 {
                // S
                p2 += rx2 - py;
            } else {
                // SE
                x += 1;
                px += two_ry2;
                p2 += px - py + rx2;
            }
        }
    }
}
