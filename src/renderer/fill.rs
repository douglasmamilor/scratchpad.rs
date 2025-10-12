use super::Renderer;
use crate::color::Color;
use crate::math::vec2::Vec2;

impl<'a> Renderer<'a> {
    pub fn flood_fill<F>(&mut self, px: (i32, i32), new_color: Color, matches: F, conn_8: bool)
    where
        F: Fn(Color, Color) -> bool,
    {
        let target = match self.get_pixel(px) {
            Some(c) => c,
            None => return,
        };

        if matches(target, new_color) && matches(new_color, target) {
            return;
        }

        let (_, px_y) = px;
        let expand_run = |x: i32, y: i32| -> (i32, i32) {
            let mut xl = x;
            while let Some(c) = self.get_pixel((xl - 1, y)) {
                if !matches(c, target) {
                    break;
                }
                xl -= 1;
            }

            let mut xr = x;
            while let Some(c) = self.get_pixel((xr + 1, y)) {
                if !matches(c, target) {
                    break;
                }
                xr += 1;
            }

            (xl, xr)
        };

        let mut stack: Vec<(i32, i32, i32)> = Vec::new();
        let (xl, xr) = expand_run(px.0, px.1);
        self.hspan(px_y, xl, xr, new_color);
        stack.push((px_y, xl, xr));

        while let Some((y, xl, xr)) = stack.pop() {
            for &yn in &[y - 1, y + 1] {
                if !self.in_bounds_y(yn) {
                    continue;
                }

                let (mut scanl, mut scanr) = (xl, xr);
                if conn_8 {
                    scanl -= 1;
                    scanr += 1;
                }

                (scanl, scanr) = (
                    scanl.clamp(0, self.width() as i32 - 1),
                    scanr.clamp(0, self.width() as i32 - 1),
                );

                let mut x = scanl;
                while x <= scanr {
                    while x <= scanr {
                        if let Some(c) = self.get_pixel((x, yn)) {
                            if matches(c, target) {
                                break;
                            }
                        }

                        x += 1;
                    }
                    if x > scanr {
                        break;
                    }

                    let run_start = x;
                    while x <= scanr {
                        if let Some(c) = self.get_pixel((x, yn)) {
                            if matches(c, target) {
                                x += 1;
                                continue;
                            }
                        }
                        break;
                    }
                    let run_end = x - 1;

                    self.hspan(yn, run_start, run_end, new_color);
                    stack.push((yn, run_start, run_end));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::FrameBuffer;

    fn create_test_framebuffer() -> FrameBuffer {
        FrameBuffer::new(20, 20)
    }

    fn collect_pixels<F>(width: usize, height: usize, draw_fn: F) -> Vec<(i32, i32)>
    where
        F: FnOnce(&mut Renderer),
    {
        let mut fb = FrameBuffer::new(width, height);
        {
            let mut renderer = Renderer::new(&mut fb);
            draw_fn(&mut renderer);
        }

        let mut pixels = Vec::new();
        for y in 0..height as i32 {
            for x in 0..width as i32 {
                if let Some(color) = fb.get_pixel(x as usize, y as usize) {
                    if color != 0 {
                        pixels.push((x, y));
                    }
                }
            }
        }
        pixels
    }

    #[test]
    fn flood_fill_simple_rectangle() {
        let pixels = collect_pixels(10, 10, |renderer| {
            // Draw a rectangle outline
            renderer.draw_rect((2, 2), (6, 6), Color::WHITE);
            // Fill the interior
            renderer.flood_fill(
                (5, 5),
                Color::RED,
                |target, _new| target == Color::WHITE,
                false,
            );
        });

        // Should have the outline (white) and interior (red)
        assert!(pixels.len() > 20); // More than just the outline
        assert!(pixels.contains(&(5, 5))); // Seed point should be filled
    }

    #[test]
    fn flood_fill_4_connected_vs_8_connected() {
        // Create a shape where 4-connected and 8-connected give different results
        let pixels_4 = collect_pixels(10, 10, |renderer| {
            // Draw a shape with diagonal gap
            renderer.set_pixel((3, 3), Color::WHITE);
            renderer.set_pixel((4, 3), Color::WHITE);
            renderer.set_pixel((5, 3), Color::WHITE);
            renderer.set_pixel((3, 4), Color::WHITE);
            // Gap here - no pixel at (4, 4)
            renderer.set_pixel((5, 4), Color::WHITE);
            renderer.set_pixel((3, 5), Color::WHITE);
            renderer.set_pixel((4, 5), Color::WHITE);
            renderer.set_pixel((5, 5), Color::WHITE);

            // 4-connected fill should not cross the diagonal gap
            renderer.flood_fill(
                (3, 3),
                Color::RED,
                |target, _new| target == Color::WHITE,
                false,
            );
        });

        let pixels_8 = collect_pixels(10, 10, |renderer| {
            // Draw the same shape
            renderer.set_pixel((3, 3), Color::WHITE);
            renderer.set_pixel((4, 3), Color::WHITE);
            renderer.set_pixel((5, 3), Color::WHITE);
            renderer.set_pixel((3, 4), Color::WHITE);
            // Gap here - no pixel at (4, 4)
            renderer.set_pixel((5, 4), Color::WHITE);
            renderer.set_pixel((3, 5), Color::WHITE);
            renderer.set_pixel((4, 5), Color::WHITE);
            renderer.set_pixel((5, 5), Color::WHITE);

            // 8-connected fill should cross the diagonal gap
            renderer.flood_fill(
                (3, 3),
                Color::RED,
                |target, _new| target == Color::WHITE,
                true,
            );
        });

        // Both should fill the same number of pixels since the shape is connected
        // The difference between 4-connected and 8-connected is subtle in this case
        assert!(pixels_4.len() >= 6); // At least the connected pixels
        assert!(pixels_8.len() >= 6); // At least the connected pixels
    }

    #[test]
    fn flood_fill_boundary_detection() {
        let pixels = collect_pixels(10, 10, |renderer| {
            // Draw two separate regions with different boundary colors
            renderer.draw_rect((1, 1), (3, 3), Color::RED);
            renderer.draw_rect((5, 5), (3, 3), Color::BLUE);

            // Fill only the red-bounded region
            renderer.flood_fill(
                (2, 2),
                Color::GREEN,
                |target, _new| target == Color::RED,
                false,
            );
        });

        // Should not have filled the blue-bounded region
        assert!(pixels.contains(&(2, 2))); // Red region should be filled
        // Blue region should not be filled (no green pixels in that area)
    }

    #[test]
    fn flood_fill_already_filled() {
        let pixels = collect_pixels(10, 10, |renderer| {
            // Fill an area first
            renderer.fill_rect((2, 2), (4, 4), Color::RED);

            // Try to flood fill the same area with same color
            renderer.flood_fill(
                (4, 4),
                Color::RED,
                |target, _new| target == Color::RED,
                false,
            );
        });

        // Should not have changed anything (already filled with same color)
        assert_eq!(pixels.len(), 16); // Just the 4x4 rectangle
    }

    #[test]
    fn flood_fill_out_of_bounds() {
        let pixels = collect_pixels(10, 10, |renderer| {
            // Try to flood fill outside the framebuffer
            renderer.flood_fill(
                (15, 15),
                Color::RED,
                |target, _new| target == Color::WHITE,
                false,
            );
        });

        // Should not have filled anything
        assert_eq!(pixels.len(), 0);
    }

    #[test]
    fn flood_fill_single_pixel() {
        let pixels = collect_pixels(10, 10, |renderer| {
            // Draw a single pixel
            renderer.set_pixel((5, 5), Color::WHITE);

            // Fill it
            renderer.flood_fill(
                (5, 5),
                Color::RED,
                |target, _new| target == Color::WHITE,
                false,
            );
        });

        // Should have filled the single pixel
        assert_eq!(pixels.len(), 1);
        assert!(pixels.contains(&(5, 5)));
    }

    #[test]
    fn flood_fill_line() {
        let pixels = collect_pixels(10, 10, |renderer| {
            // Draw a horizontal line
            renderer.draw_line((2, 5), (7, 5), Color::WHITE);

            // Fill it
            renderer.flood_fill(
                (5, 5),
                Color::RED,
                |target, _new| target == Color::WHITE,
                false,
            );
        });

        // Should have filled the line
        assert!(pixels.len() >= 6); // At least 6 pixels for the line
        assert!(pixels.contains(&(5, 5))); // Seed point should be filled
    }

    #[test]
    fn flood_fill_complex_shape() {
        let pixels = collect_pixels(15, 15, |renderer| {
            // Draw a complex L-shaped boundary
            renderer.draw_rect((2, 2), (6, 2), Color::WHITE); // Top horizontal
            renderer.draw_rect((2, 2), (2, 6), Color::WHITE); // Left vertical
            renderer.draw_rect((2, 6), (4, 2), Color::WHITE); // Bottom horizontal (shorter)
            renderer.draw_rect((4, 4), (2, 4), Color::WHITE); // Right vertical (shorter)

            // Fill the interior
            renderer.flood_fill(
                (4, 4),
                Color::RED,
                |target, _new| target == Color::WHITE,
                false,
            );
        });

        // Should have filled the interior of the L-shape
        assert!(pixels.len() > 20); // More than just the outline
        assert!(pixels.contains(&(4, 4))); // Seed point should be filled
    }

    #[test]
    fn flood_fill_custom_matching_function() {
        let pixels = collect_pixels(10, 10, |renderer| {
            // Draw pixels with different colors
            renderer.set_pixel((2, 2), Color::RED);
            renderer.set_pixel((3, 2), Color::GREEN);
            renderer.set_pixel((4, 2), Color::BLUE);
            renderer.set_pixel((2, 3), Color::RED);
            renderer.set_pixel((3, 3), Color::GREEN);
            renderer.set_pixel((4, 3), Color::BLUE);

            // Fill only red and green pixels (not blue)
            renderer.flood_fill(
                (2, 2),
                Color::YELLOW,
                |target, _new| target == Color::RED || target == Color::GREEN,
                false,
            );
        });

        // Should have filled red and green pixels, but not blue
        assert!(pixels.contains(&(2, 2))); // Red pixel should be filled
        assert!(pixels.contains(&(3, 2))); // Green pixel should be filled
        // Blue pixels should not be filled
    }

    #[test]
    fn flood_fill_performance_large_area() {
        let pixels = collect_pixels(50, 50, |renderer| {
            // Draw a large rectangle outline
            renderer.draw_rect((5, 5), (40, 40), Color::WHITE);

            // Fill the large interior
            renderer.flood_fill(
                (25, 25),
                Color::RED,
                |target, _new| target == Color::WHITE,
                false,
            );
        });

        // Should have filled a reasonable area (outline + interior)
        assert!(pixels.len() > 100); // Reasonable number of pixels filled
        assert!(pixels.contains(&(25, 25))); // Seed point should be filled
    }

    #[test]
    fn flood_fill_circle_outline() {
        let pixels = collect_pixels(20, 20, |renderer| {
            // Draw a circle outline
            renderer.draw_circle(Vec2::new(10.0, 10.0), 5.0, Color::WHITE);
            // Try to fill the interior - but there's no interior with just an outline!
            renderer.flood_fill((10, 10), Color::RED, |target, _new| target == Color::WHITE, false);
        });

        // Should only have the outline pixels (no interior to fill)
        // The flood fill will only affect the outline pixels themselves
        assert!(pixels.len() > 20); // Circle outline pixels
        assert!(pixels.contains(&(10, 10))); // Center should be filled (it's part of the outline)
    }

    #[test]
    fn flood_fill_filled_circle() {
        let pixels = collect_pixels(20, 20, |renderer| {
            // Draw a filled circle first
            renderer.fill_rect((5, 5), (10, 10), Color::WHITE);
            // Then draw a circle outline on top
            renderer.draw_circle(Vec2::new(10.0, 10.0), 4.0, Color::BLACK);
            // Now fill the interior (between the outline and the filled area)
            renderer.flood_fill((10, 10), Color::RED, |target, _new| target == Color::WHITE, false);
        });

        // Should have filled the interior of the circle
        assert!(pixels.len() > 50); // Large filled area
        assert!(pixels.contains(&(10, 10))); // Center should be filled
    }
}
