use crate::math::{Point2, distance_point_to_line, mod_pos};
use crate::renderer::{Color, PolyLine, Renderer};

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
    QuadTo(Point2, Point2),          // quadratic Bezier: c1, end P2
    CubicTo(Point2, Point2, Point2), // cubic Bezier: c1, c2, end P3
    ClosePath,                       // connect back to last MoveTo
}

pub struct Path {
    commands: Vec<PathCommand>,
}

impl<'a> Renderer<'a> {
    pub fn flatten_path_to_polylines(&mut self, path: Path) -> Option<Vec<PolyLine>> {
        let mut polylines: Vec<PolyLine> = Vec::new();
        let mut current_points: Vec<Point2> = Vec::new();
        let mut current_point: Option<Point2> = None;

        #[allow(unused_variables)]
        let mut start_point: Option<Point2> = None;

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
                    start_point = Some(*p);
                    current_point = Some(*p);
                }

                PathCommand::LineTo(p) => {
                    if current_points.is_empty() {
                        // implicit MoveTo
                        current_points.push(*p);
                        start_point = Some(*p);
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
                            start_point = Some(*p2);
                            current_point = Some(*p2);
                            continue;
                        }
                    };

                    // flatten quadratic Bezier from: p0 → c → p2 (append-only points)
                    let seg = flatten_quad(p0, *c, *p2, 0.5);
                    current_points.extend(seg);
                    current_point = Some(*p2);
                }

                PathCommand::CubicTo(c1, c2, p3) => {
                    let p0 = match current_point {
                        Some(p0) => p0,
                        None => {
                            // implicit MoveTo to the segment end (keeps state consistent)
                            current_points.push(*p3);
                            start_point = Some(*p3);
                            current_point = Some(*p3);
                            continue;
                        }
                    };

                    // flatten cubic Bezier from: p0 → c1 → c2 → p3 (append-only points)
                    let seg = flatten_cubic(p0, *c1, *c2, *p3, 0.5);
                    current_points.extend(seg);
                    current_point = Some(*p3);
                }

                PathCommand::ClosePath => {
                    if current_points.len() > 1 {
                        polylines.push(PolyLine::new(std::mem::take(&mut current_points), true));
                    } else {
                        current_points.clear();
                    }

                    start_point = None;
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

/// Given one polyline, return the ON dash segments as separate open polylines.
fn dash_polyline(poly: &PolyLine, dash_len: f32, gap_len: f32, phase: f32) -> Vec<PolyLine> {
    let total = poly.len();
    if total <= 0.0 {
        return vec![];
    }

    let dash_len = dash_len.max(0.0);
    let gap_len = gap_len.max(0.0);
    let period = dash_len + gap_len;

    // Degenerate cases:
    if dash_len == 0.0 || period == 0.0 {
        return vec![];
    }
    if gap_len == 0.0 {
        // All ON
        return vec![PolyLine::new(poly.points().to_vec(), poly.is_closed())];
    }

    // phase shifts where the pattern starts along the path.
    // We want a starting offset in [0, period).
    let start = mod_pos(phase, period);

    // We iterate k so that (k*period - start) spans the whole [0,total]
    // ON interval for a cycle: [k*period - start, k*period - start + dash_len]
    let mut out: Vec<PolyLine> = Vec::new();

    // Start k so first ON interval might begin before 0.
    // Using floor division:
    let mut k = ((0.0 + start) / period).floor() as i32;

    loop {
        let on_a = (k as f32) * period - start;
        let on_b = on_a + dash_len;

        if on_a >= total {
            break;
        }
        if on_b > 0.0 {
            let a = on_a.max(0.0);
            let b = on_b.min(total);
            if b > a
                && let Some(seg) = poly.slice_by_len(a, b)
            {
                out.push(seg);
            }
        }

        k += 1;
        // Safety: if somehow period is tiny, still won’t infinite-loop because on_a grows by period.
        if (k as f32) * period - start > total + period {
            break;
        }
    }

    out
}

fn circle_polyline(center: Point2, radius: f32, segments: usize) -> PolyLine {
    let segments = segments.max(6);
    let r = radius.max(0.0);

    // If radius is ~0, return a tiny “dot” as a 2-point line (fallback)
    if r <= 1e-6 {
        return PolyLine::new(vec![center, center], false);
    }

    let mut pts = Vec::with_capacity(segments);
    let tau = std::f32::consts::TAU;

    for i in 0..segments {
        let t = (i as f32) / (segments as f32);
        let a = t * tau;
        let (sa, ca) = a.sin_cos();

        // Adjust this if your Point2 construction differs
        pts.push(Point2 {
            x: center.x + ca * r,
            y: center.y + sa * r,
        });
    }

    PolyLine::new(pts, true)
}

fn dotted_polyline(poly: &PolyLine, dot_space: f32, dot_radius: f32, phase: f32) -> Vec<PolyLine> {
    let total = poly.len();
    if total <= 0.0 {
        return vec![];
    }

    let step = dot_space.max(1e-6); // avoid divide-by-zero
    let start = mod_pos(phase, step);

    // Choose how smooth your dot circles are
    let circle_segments = 16;

    let mut out = Vec::new();

    // Place dot centers at s = start + k*step within [0, total]
    // If you want a dot at s=0 when phase=0, this does it.
    let mut s = start;
    while s <= total {
        let c = poly.point_at_len(s);
        out.push(circle_polyline(c, dot_radius, circle_segments));
        s += step;
    }

    out
}

/// Apply stroke pattern to a list of polylines.
/// For now: only Dashed implemented (Dotted later).
pub fn apply_stroke_pattern(polylines: &[PolyLine], pattern: &StrokePattern) -> Vec<PolyLine> {
    match *pattern {
        StrokePattern::Dashed {
            dash_length,
            gap_length,
            phase,
            enabled,
        } => {
            if !enabled {
                return polylines.to_vec();
            }

            let mut out = Vec::new();
            for pl in polylines {
                // Typically you dash after flattening, before stroking caps/joins.
                out.extend(dash_polyline(pl, dash_length, gap_length, phase));
            }
            out
        }

        StrokePattern::Dotted {
            dot_space,
            dot_radius,
            phase,
            enabled,
        } => {
            if !enabled {
                polylines.to_vec()
            } else {
                let mut out = Vec::new();
                for pl in polylines {
                    out.extend(dotted_polyline(pl, dot_space, dot_radius, phase));
                }
                out
            }
        }
    }
}
