use super::Renderer;

#[inline]
fn ipart(x: f32) -> i32 {
    x.floor() as i32
}
#[inline]
fn roundi(x: f32) -> i32 {
    (x + 0.5).floor() as i32
}
#[inline]
fn fpart(x: f32) -> f32 {
    x.fract()
}
#[inline]
fn rfpart(x: f32) -> f32 {
    1.0 - fpart(x)
}

impl<'a> Renderer<'a> {
    // draw antialiased line using Wu's line drawing algorithm
    pub fn plot_line_aa(start: &(f32, f32), end: &(f32, f32)) -> Vec<(i32, i32, f32)> {
        let (mut x0, mut y0) = *start;
        let (mut x, mut y) = *end;

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

        if (x0 == x) && (y0 == y) {
            let mut pts = Vec::with_capacity(1);
            if steep {
                pts.push((y0 as i32, x0 as i32, 1.0));
            } else {
                pts.push((x0 as i32, y0 as i32, 1.0));
            }
            return pts;
        }

        let dx = x - x0;
        let dy = y - y0;
        let m = dy / dx;

        // Pre-allocate after normalisation
        let vec_start = x0.floor() as i32;
        let vec_end = x.ceil() as i32;
        // Each column can plot up to 2 pixels
        // two pixels per column, each with a brightness
        let mut points: Vec<(i32, i32, f32)> =
            Vec::with_capacity(((vec_end - vec_start + 1) * 2) as usize);

        // Post-normalisation

        // Step 2: Process head endpoint (i.e endpoint at x0)
        let mut x_endpt = roundi(x0);
        let mut y_endpt = y0 + m * (x_endpt as f32 - x0);
        let mut x_endpt_gap = rfpart(x0 + 0.5);
        let x_start = x_endpt;
        let y_start = ipart(y_endpt);

        // Wu's algorithm means we paint two pixels, making one dimmer
        // than the other based on closeness to the real line
        if steep {
            points.push((y_start, x_start, rfpart(y_endpt) * x_endpt_gap));
            points.push((y_start + 1, x_start, fpart(y_endpt) * x_endpt_gap));
        } else {
            points.push((x_start, y_start, rfpart(y_endpt) * x_endpt_gap));
            points.push((x_start, y_start + 1, fpart(y_endpt) * x_endpt_gap));
        }

        let mut y_nxt = y_endpt + m; // unit step in x direction

        // Step 3: Process tail endpoint (i.e endpoint at x)
        x_endpt = roundi(x);
        y_endpt = y + m * (x_endpt as f32 - x);
        x_endpt_gap = fpart(x + 0.5);
        let x_end = x_endpt;
        let y_end = ipart(y_endpt);

        // Step 4: Process intermediate pixels
        for x in (x_start + 1)..x_end {
            // Wu's algorithm means we paint two pixels, making one dimmer
            // than the other based on closeness to the real line
            if steep {
                points.push((ipart(y_nxt), x, rfpart(y_nxt)));
                points.push((ipart(y_nxt) + 1, x, fpart(y_nxt)));
            } else {
                points.push((x, ipart(y_nxt), rfpart(y_nxt)));
                points.push((x, ipart(y_nxt) + 1, fpart(y_nxt)));
            }

            y_nxt += m;
        }

        // Stept 5: Process tail endpoint (i.e endpoint at x)
        if steep {
            points.push((y_end, x_end, rfpart(y_endpt) * x_endpt_gap));
            points.push((y_end + 1, x_end, fpart(y_endpt) * x_endpt_gap));
        } else {
            points.push((x_end, y_end, rfpart(y_endpt) * x_endpt_gap));
            points.push((x_end, y_end + 1, fpart(y_endpt) * x_endpt_gap));
        }

        points
    }
}
