use crate::math::Point2;
use crate::math::distance_point_to_line;
use crate::renderer::Color;
use crate::renderer::Renderer;

pub enum LineCap {
    Butt,
    Square,
    Round,
}

pub enum LineJoin {
    Miter { limit: f32 },
    Bevel,
    Round,
}

pub enum StrokeSpace {
    ScreenSpace { thickness: u64 },
    WorldSpace { thickness: u64 },
}

pub enum StrokePattern {
    Dashed {
        dash_length: f32,
        gap_length: f32,
        phase: f32,
        enabled: bool,
    },
    Dotted {
        dot_space: f32,
        dot_radius: f32,
        phase: f32,
        enabled: bool,
    },
}

pub struct StrokeStyle {
    space: StrokeSpace,
    pattern: StrokePattern,
    cap: LineCap,
    join: LineJoin,
    color: Color,
}

pub enum PathCommand {
    MoveTo(Point2),                  // start new subpath at P
    LineTo(Point2),                  // straight line
    QuadTo(Point2, Point2),          // quadratic Bezier: control P1, end P2
    CubicTo(Point2, Point2, Point2), // cubic Bezier: controls P1 P2, end P2
    ClosePath,                       // connect back to last MoveTo
}

pub struct Path {
    commands: Vec<PathCommand>,
}

pub struct Polyline {
    points: Vec<Point2>, // P[0..n-1]
    closed: bool,        // true if ClosePath
}

impl<'a> Renderer<'a> {
    pub fn flatten_path_to_polylines(&mut self, path: Path) -> Option<Vec<Polyline>> {
        let mut poly_lines: Vec<Polyline> = vec![];
        let mut current_points: Vec<Point2> = vec![];
        let mut start_point: Point2;
        let mut current_point: Point2;

        for cmd in path.commands.iter() {
            match cmd {
                PathCommand::MoveTo(p) => {
                    // if the list is non-empty, then we were building a polyline,
                    // and did not encounter a ClosePath.
                    // So finalise the polyline, marking it as open
                    if current_points.len() > 0 {
                        polyLines.push(Polyline {
                            points: current_points.clone(),
                            closed: false,
                        });
                    }
                    current_points.clear();
                    current_points.push(*p);
                    start_point = *p;
                }
                PathCommand::LineTo(p) => {
                    if current_points.is_empty() {
                        current_points.push(*p);
                        start_point = *p;
                    } else {
                        current_points.push(*p);
                    }
                    current_point = *p;
                }
                PathCommand::QuadTo(c, p1) => {
                    // flatten quadratic Bezier from: current_point → c → p1
                    let segment_points = flatten_quad(current_point, *p0, *p1, 0.5);
                    for i in 1..segment_points.len() {
                        current_points.push(segment_points.points[i]);
                    }
                }
            }
        }

        None
    }
}
pub fn flatten_cubic(
    p0: Point2,
    c1: Point2,
    c2: Point2,
    p3: Point2,
    tolerance: f32,
) -> Vec<Point2> {
    fn recursive_flatten_cubic(
        p0: Point2,
        c1: Point2,
        c2: Point2,
        p3: Point2,
        tolerance: f32,
        points: &mut Vec<Point2>,
    ) {
        // Flatness heuristic: max distance of control points from the chord (p0 -> p3)
        let d1 = distance_point_to_line(c1, (p0, p3));
        let d2 = distance_point_to_line(c2, (p0, p3));
        let deviation = d1.max(d2);

        if deviation <= tolerance {
            // Flat enough: approximate with a straight segment p0 -> p3
            points.push(p0);
            return;
        }

        // de Casteljau split at t = 0.5
        let p01 = p0.lerp(c1, 0.5);
        let p12 = c1.lerp(c2, 0.5);
        let p23 = c2.lerp(p3, 0.5);

        let p012 = p01.lerp(p12, 0.5);
        let p123 = p12.lerp(p23, 0.5);

        let p0123 = p012.lerp(p123, 0.5); // point on curve at t = 0.5

        // Left:  (p0,  p01,  p012,  p0123)
        // Right: (p0123, p123, p23,  p3)
        recursive_flatten_cubic(p0, p01, p012, p0123, tolerance, points);
        recursive_flatten_cubic(p0123, p123, p23, p3, tolerance, points);
    }

    let mut pts = Vec::new();
    recursive_flatten_cubic(p0, c1, c2, p3, tolerance, &mut pts);
    pts.push(p3); // add final endpoint once
    pts
}

pub fn flatten_quad(p0: Point2, c: Point2, p1: Point2, tolerance: f32) -> Vec<Point2> {
    let mut points: Vec<Point2> = vec![];
    recursive_flatten_quad(p0, c, p1, tolerance, &mut points);
    points.push(p1);

    points
}

pub fn recursive_flatten_quad(
    p0: Point2,
    c: Point2,
    p1: Point2,
    tolerance: f32,
    points: &mut Vec<Point2>,
) {
    let deviation = distance_point_to_line(c, (p0, p1));

    if deviation <= tolerance {
        points.push(p0);
        return;
    }

    // de Casteljau split at 0.5
    let q0 = p0.lerp(c, 0.5);
    let q1 = c.lerp(p1, 0.5);
    let r = q0.lerp(q1, 0.5);

    recursive_flatten_quad(p0, q0, r, tolerance, points);
    recursive_flatten_quad(r, q1, p1, tolerance, points);
}
