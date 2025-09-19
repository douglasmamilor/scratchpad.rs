use super::Renderer;

impl<'a> Renderer<'a> {
    // draw line using Bresenhams line drawing algorithm
    pub fn plot_line(start: &(i32, i32), end: &(i32, i32)) -> Vec<(i32, i32)> {
        let (mut x0, mut y0) = (start.0, start.1);
        let (mut x, mut y) = (end.0, end.1);

        // Step 1: Normalise

        // Determine if the line is steep (> 45 degrees)
        // and if steep, swap x and y coordinates
        let steep = (y - y0).abs() > (x - x0).abs();
        if steep {
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x, &mut y);
        }

        // If moving right to left, swap start and end points
        if x0 > x {
            std::mem::swap(&mut x0, &mut x);
            std::mem::swap(&mut y0, &mut y);
        }

        // Step 2: Separate y-direction from y-step using normalised coordinates and deltas
        let dx = x - x0;
        let dy = (y - y0).abs();
        let y_step = if (y - y0) >= 0 { 1 } else { -1 };

        // Now we write the Bresenham implicit line function
        #[allow(non_snake_case)]
        let mut D = 2 * dy - dx;
        let mut points: Vec<(i32, i32)> = Vec::with_capacity((x.max(x0) - x.min(x0) + 1) as usize);

        y = y0;
        for x in x0..x {
            if steep {
                points.push((y, x));
            } else {
                points.push((x, y));
            }

            if D >= 0 {
                // North East
                y += y_step;
                D += 2 * (dy - dx);
            } else {
                // East
                D += 2 * dy;
            }
        }

        points
    }

    // pub fn draw_line_aa() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_line() {
        let start = (0, 0);
        let end = (5, 0);
        let points = Renderer::plot_line(&start, &end);

        let expected = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)];
        assert_eq!(points, expected);
    }

    #[test]
    fn test_vertical_line() {
        let start = (0, 0);
        let end = (0, 5);
        let points = Renderer::plot_line(&start, &end);

        let expected = vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)];
        assert_eq!(points, expected);
    }

    #[test]
    fn test_diagonal_line() {
        let start = (0, 0);
        let end = (3, 3);
        let points = Renderer::plot_line(&start, &end);

        let expected = vec![(0, 0), (1, 1), (2, 2)];
        assert_eq!(points, expected);
    }

    #[test]
    fn test_reverse_direction() {
        let start = (5, 0);
        let end = (0, 0);
        let points = Renderer::plot_line(&start, &end);

        let expected = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)];
        assert_eq!(points, expected);
    }

    #[test]
    fn test_steep_line() {
        let start = (0, 0);
        let end = (2, 5);
        let points = Renderer::plot_line(&start, &end);

        // For steep lines, coordinates are swapped
        let expected = vec![(0, 0), (0, 1), (1, 2), (1, 3), (2, 4)];
        assert_eq!(points, expected);
    }

    #[test]
    fn test_single_point() {
        let start = (5, 5);
        let end = (5, 5);
        let points = Renderer::plot_line(&start, &end);

        assert_eq!(points, vec![]);
    }

    #[test]
    fn test_negative_coordinates() {
        let start = (-2, -2);
        let end = (2, 2);
        let points = Renderer::plot_line(&start, &end);

        let expected = vec![(-2, -2), (-1, -1), (0, 0), (1, 1)];
        assert_eq!(points, expected);
    }
}
