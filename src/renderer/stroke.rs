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

    // Defensively reject NaN/∞ inputs (comparisons with NaN behave unexpectedly).
    if !dash_len.is_finite() || !gap_len.is_finite() || !phase.is_finite() {
        return vec![];
    }

    // Normalize to non-negative lengths.
    let dash_len = dash_len.max(0.0);
    let gap_len = gap_len.max(0.0);
    let period = dash_len + gap_len;

    // Degenerate cases.
    if dash_len == 0.0 || period == 0.0 {
        return vec![];
    }
    if gap_len == 0.0 {
        // All ON.
        return vec![PolyLine::new(poly.points().to_vec(), poly.is_closed())];
    }

    // Phase shifts where the pattern starts along the path.
    // Normalize into [0, period).
    let start = mod_pos(phase, period);

    // - Accumulate interval endpoints in f64 with repeated addition.
    // - Clamp in f64.
    // - Cast to f32 only at the `slice_by_len` boundary.
    let total64 = total as f64;
    let period64 = period as f64;
    let dash64 = dash_len as f64;
    let start64 = start as f64;

    let mut on_a = -start64;
    let mut on_b = on_a + dash64;

    // Compute how many cycles we need, then cap to avoid pathological tiny-period
    // inputs generating enormous work.
    // Add 1 as a buffer for floating point
    let needed_cycles = ((total64 + start64) / period64).ceil() as usize + 1;

    const MAX_DASH_CYCLES: usize = 1_000_000;
    if needed_cycles > MAX_DASH_CYCLES {
        // Pattern too dense to be meaningful; render as solid.
        return vec![PolyLine::new(poly.points().to_vec(), poly.is_closed())];
    }

    let mut out: Vec<PolyLine> = Vec::new();

    for _ in 0..needed_cycles {
        if on_a >= total64 {
            break;
        }

        if on_b > 0.0 {
            // Clamp symmetrically in f64 first.
            let a64 = on_a.max(0.0).min(total64);
            let b64 = on_b.max(0.0).min(total64);

            // Convert once at the API boundary.
            let a = a64 as f32;
            let b = b64 as f32;

            if b > a
                && let Some(seg) = poly.slice_by_len(a, b)
            {
                out.push(seg);
            }
        }

        // Advance to next ON interval.
        on_a += period64;
        on_b += period64;
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
