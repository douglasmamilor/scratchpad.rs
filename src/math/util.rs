use crate::math::Point2;

/// Radians to degrees
pub fn rad_to_deg(rad: f32) -> f32 {
    rad * (180.0 / std::f32::consts::PI)
}

// Degrees to radians
pub fn deg_ro_rad(deg: f32) -> f32 {
    deg * (std::f32::consts::PI / 180.0)
}

pub fn distance_point_to_line(p: Point2, line: (Point2, Point2)) -> f32 {
    let (a, b) = (line.0, line.1);

    (p - a).reject_from(b - a).len()
}

/// Positive modulo that works for floats.
pub fn mod_pos(x: f32, m: f32) -> f32 {
    let r = x % m;
    if r < 0.0 { r + m } else { r }
}
