use crate::{
    Point2,
    math::distance_point_to_line,
    renderer::{PolyLine, Renderer},
};

#[derive(Clone, Debug)]
pub enum PathCommand {
    MoveTo(Point2),                  // start new subpath at P
    LineTo(Point2),                  // straight line
    QuadTo(Point2, Point2),          // quadratic Bezier: c1, end P2
    CubicTo(Point2, Point2, Point2), // cubic Bezier: c1, c2, end P3
    ClosePath,                       // connect back to last MoveTo
}

#[derive(Clone, Debug)]
pub struct Path {
    commands: Vec<PathCommand>,
}

impl<'a> Renderer<'a> {
    /// Flatten a path into polylines using a configurable flatness tolerance.
    /// `tolerance` is the max distance control points may deviate from the chord
    /// before subdivision; non-finite/<=0 fall back to a reasonable default.
    pub fn flatten_path_to_polylines(&mut self, path: &Path, tolerance: f32) -> Option<Vec<PolyLine>> {
        let tol = if tolerance.is_finite() && tolerance > 0.0 {
            tolerance
        } else {
            0.5
        };
        let mut polylines: Vec<PolyLine> = Vec::new();
        let mut current_points: Vec<Point2> = Vec::new();
        let mut current_point: Option<Point2> = None;

        for cmd in path.commands.iter() {
            match cmd {
                PathCommand::MoveTo(p) => {
                    // If we were building a polyline and didn't see ClosePath,
                    // finalize it as open before starting a new subpath.
                    if current_points.len() > 1 {
                        polylines.push(PolyLine::new(std::mem::take(&mut current_points), false));
                    } else {
                        current_points.clear();
                    }

                    current_points.push(*p);
                    current_point = Some(*p);
                }

                PathCommand::LineTo(p) => {
                    if current_points.is_empty() {
                        // implicit MoveTo
                        current_points.push(*p);
                    } else {
                        current_points.push(*p);
                    }
                    current_point = Some(*p);
                }

                PathCommand::QuadTo(c, p2) => {
                    let p0 = match current_point {
                        Some(p0) => p0,
                        None => {
                            // implicit MoveTo to the segment end (keeps state consistent)
                            current_points.push(*p2);
                            current_point = Some(*p2);
                            continue;
                        }
                    };

                    // flatten quadratic Bezier from: p0 → c → p2 (append-only points)
                    let seg = flatten_quad(p0, *c, *p2, tol);
                    current_points.extend(seg);
                    current_point = Some(*p2);
                }

                PathCommand::CubicTo(c1, c2, p3) => {
                    let p0 = match current_point {
                        Some(p0) => p0,
                        None => {
                            // implicit MoveTo to the segment end (keeps state consistent)
                            current_points.push(*p3);
                            current_point = Some(*p3);
                            continue;
                        }
                    };

                    // flatten cubic Bezier from: p0 → c1 → c2 → p3 (append-only points)
                    let seg = flatten_cubic(p0, *c1, *c2, *p3, tol);
                    current_points.extend(seg);
                    current_point = Some(*p3);
                }

                PathCommand::ClosePath => {
                    if current_points.len() > 1 {
                        polylines.push(PolyLine::new(std::mem::take(&mut current_points), true));
                    } else {
                        current_points.clear();
                    }

                    current_point = None;
                }
            }
        }

        // End of commands: flush any remaining open polyline
        if current_points.len() > 1 {
            polylines.push(PolyLine::new(std::mem::take(&mut current_points), false));
        }

        if polylines.is_empty() {
            None
        } else {
            Some(polylines)
        }
    }
}

fn flatten_cubic(p0: Point2, c1: Point2, c2: Point2, p3: Point2, tolerance: f32) -> Vec<Point2> {
    fn recurse(
        p0: Point2,
        c1: Point2,
        c2: Point2,
        p3: Point2,
        tolerance: f32,
        out: &mut Vec<Point2>,
    ) {
        // Flatness heuristic: max distance of control points from the chord (p0 -> p3)
        let d1 = distance_point_to_line(c1, (p0, p3));
        let d2 = distance_point_to_line(c2, (p0, p3));
        let deviation = d1.max(d2);

        if deviation <= tolerance {
            // Append-only convention: emit only the endpoint for this leaf chord
            out.push(p3);
            return;
        }

        // de Casteljau split at t = 0.5
        let p01 = p0.lerp(c1, 0.5);
        let p12 = c1.lerp(c2, 0.5);
        let p23 = c2.lerp(p3, 0.5);

        let p012 = p01.lerp(p12, 0.5);
        let p123 = p12.lerp(p23, 0.5);

        let p0123 = p012.lerp(p123, 0.5); // point on curve at t = 0.5

        // Left:  (p0,    p01,  p012,  p0123)
        // Right: (p0123, p123, p23,   p3)
        recurse(p0, p01, p012, p0123, tolerance, out);
        recurse(p0123, p123, p23, p3, tolerance, out);
    }

    let mut out = Vec::new();
    recurse(p0, c1, c2, p3, tolerance, &mut out);
    out
}

fn flatten_quad(p0: Point2, c: Point2, p2: Point2, tolerance: f32) -> Vec<Point2> {
    fn recurse(p0: Point2, c: Point2, p2: Point2, tolerance: f32, out: &mut Vec<Point2>) {
        let deviation = distance_point_to_line(c, (p0, p2));

        if deviation <= tolerance {
            // Append-only convention: emit only the endpoint for this leaf chord
            out.push(p2);
            return;
        }

        // de Casteljau split at 0.5
        let q0 = p0.lerp(c, 0.5);
        let q1 = c.lerp(p2, 0.5);
        let r = q0.lerp(q1, 0.5);

        recurse(p0, q0, r, tolerance, out);
        recurse(r, q1, p2, tolerance, out);
    }

    let mut out: Vec<Point2> = Vec::new();
    recurse(p0, c, p2, tolerance, &mut out);
    out
}
