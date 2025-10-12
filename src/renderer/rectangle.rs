use crate::renderer::{Color, Renderer};

impl<'a> Renderer<'a> {
    // useful when you have two opposite corners of the rectangle
    // e.g when drawing a selection box
    pub fn draw_rect_pts(&mut self, top_left: (i32, i32), top_right: (i32, i32), color: Color) {
        let (x0, y0) = top_left;
        let (x, y) = top_right;
        let (w, h) = (x - x0 + 1, y - y0 + 1);

        self.draw_rect((x0, y0), (w as usize, h as usize), color);
    }

    // normal rectangle drawing function using top-left corner and size
    pub fn draw_rect(&mut self, top_left: (i32, i32), size: (usize, usize), color: Color) {
        let (w, h) = (size.0 as i32, size.1 as i32);
        if w <= 0 || h <= 0 {
            return;
        }

        let (x0, y0) = top_left;
        let (x, y) = (x0 + w - 1, y0 + h - 1);

        // Degenerate case: single line
        if h == 1 {
            self.draw_line((x0, y0), (x, y0), color); // single row
            return;
        }
        // Degenerate case: single column
        if w == 1 {
            // Single column
            self.draw_line((x0, y0), (x0, y), color);
            return;
        }

        self.draw_line((x0, y0), (x, y0), color); // top line
        self.draw_line((x0, y), (x, y), color); // bottom line

        self.draw_line((x0, y0 + 1), (x0, y - 1), color); // left line without corners
        self.draw_line((x, y0 + 1), (x, y - 1), color); // right line without corners
    }

    pub fn fill_rect(&mut self, top_left: (i32, i32), size: (usize, usize), color: Color) {
        let (w, h) = (size.0 as i32, size.1 as i32);
        let (x0, y0) = top_left;
        let (x, y) = (x0 + w - 1, y0 + h - 1);

        if w <= 0 || h <= 0 {
            return;
        }

        for yi in y0..=y {
            for xi in x0..=x {
                self.set_pixel((xi, yi), color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect_rect<F>(top_left: (i32, i32), size: (usize, usize), draw_fn: F) -> Vec<(i32, i32)>
    where
        F: FnOnce(&mut Renderer, (i32, i32), (usize, usize), Color),
    {
        let mut fb = crate::framebuffer::FrameBuffer::new(16, 16);
        {
            let mut renderer = Renderer::new(&mut fb);
            draw_fn(&mut renderer, top_left, size, Color::WHITE);
        }

        let mut points = Vec::new();
        for y in 0..fb.height() as i32 {
            for x in 0..fb.width() as i32 {
                if fb.get_pixel(x as usize, y as usize).unwrap_or(0) != 0 {
                    points.push((x, y));
                }
            }
        }
        points
    }

    fn collect_rect_pts(top_left: (i32, i32), bottom_right: (i32, i32)) -> Vec<(i32, i32)> {
        let mut fb = crate::framebuffer::FrameBuffer::new(16, 16);
        {
            let mut renderer = Renderer::new(&mut fb);
            renderer.draw_rect_pts(top_left, bottom_right, Color::WHITE);
        }

        let mut points = Vec::new();
        for y in 0..fb.height() as i32 {
            for x in 0..fb.width() as i32 {
                if fb.get_pixel(x as usize, y as usize).unwrap_or(0) != 0 {
                    points.push((x, y));
                }
            }
        }
        points
    }

    #[test]
    fn fill_rect() {
        let top_left = (2, 2);
        let size = (3, 3);

        let points = collect_rect(top_left, size, |renderer, top_left, size, color| {
            renderer.fill_rect(top_left, size, color)
        });

        assert_eq!(
            points,
            vec![
                (2, 2),
                (3, 2),
                (4, 2),
                (2, 3),
                (3, 3),
                (4, 3),
                (2, 4),
                (3, 4),
                (4, 4),
            ]
        );
    }

    #[test]
    fn draws_minimum_rectangle() {
        let points = collect_rect((2, 2), (1, 1), |renderer, tl, sz, color| {
            renderer.draw_rect(tl, sz, color)
        });
        assert_eq!(points, vec![(2, 2)]);
    }

    #[test]
    fn draw_rect_with_pts() {
        let top_left = (2, 2);
        let bottom_right = (4, 4);

        let points = collect_rect_pts(top_left, bottom_right);
        assert_eq!(
            points,
            vec![
                (2, 2),
                (3, 2),
                (4, 2), // top edge (inclusive corners)
                (2, 3), // left edge (without corners)
                (4, 3), // right edge (without corners)
                (2, 4),
                (3, 4),
                (4, 4), // bottom edge (inclusive corners)
            ]
        );
    }

    #[test]
    fn draws_axis_aligned_rectangle() {
        let mut points = collect_rect((1, 1), (4, 3), |renderer, tl, sz, color| {
            renderer.draw_rect(tl, sz, color)
        });
        points.sort();
        let expected = vec![
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1), // top edge (inclusive corners)
            (1, 2), // left edge (without corners)
            (2, 3),
            (3, 3),
            (4, 3),
            (1, 3), // bottom edge (inclusive corners)
            (4, 2), // right edge (without corners)
        ];
        let mut expected_sorted = expected.clone();
        expected_sorted.sort();
        assert_eq!(points, expected_sorted);
    }
}
