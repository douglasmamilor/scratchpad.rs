use crate::{Vec2, math::Point2};

/// Radians to degrees
pub fn rad_to_deg(rad: f32) -> f32 {
    rad * (180.0 / std::f32::consts::PI)
}

// Degrees to radians
pub fn deg_ro_rad(deg: f32) -> f32 {
    deg * (std::f32::consts::PI / 180.0)
}

/// Find the distance from a point to a line defined by two points.
/// Based on the formula: https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line
pub fn distance_point_to_line(p: Point2, line: (Point2, Point2)) -> f32 {
    let (a, b) = (line.0, line.1);

    (p - a).reject_from(b - a).len()
}

pub fn perp_left(v: Vec2) -> Vec2 {
    Vec2::new(-v.y, v.x)
}

/// Positive modulo that works for floats.
pub fn mod_pos(x: f32, m: f32) -> f32 {
    let r = x % m;
    if r < 0.0 { r + m } else { r }
}

/// Return the signed angle delta from a0 to a1 in the chosen direction.
///
/// - If sweep_ccw = true: delta is in [0, 2π)
/// - If sweep_ccw = false: delta is in (−2π, 0]
pub fn angle_delta(a0: f32, a1: f32, sweep_ccw: bool) -> f32 {
    let mut d = a1 - a0;

    // Wrap into (-π, π]
    while d <= -std::f32::consts::PI {
        d += 2.0 * std::f32::consts::PI;
    }
    while d > std::f32::consts::PI {
        d -= 2.0 * std::f32::consts::PI;
    }

    if sweep_ccw && d < 0.0 {
        d += 2.0 * std::f32::consts::PI;
    } else if !sweep_ccw && d > 0.0 {
        d -= 2.0 * std::f32::consts::PI;
    }

    d
}
