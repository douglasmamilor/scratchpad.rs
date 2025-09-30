/// Radians to degrees
pub fn rad_to_deg(rad: f64) -> f64 {
    rad * (180.0 / std::f64::consts::PI)
}

// Degrees to radians
pub fn deg_ro_rad(deg: f64) -> f64 {
    deg * (std::f64::consts::PI / 180.0)
}
