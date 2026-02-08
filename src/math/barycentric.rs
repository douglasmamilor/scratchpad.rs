use crate::Color;
use crate::math::Vec2;

pub struct BarycentricCoords {
    pub u: f32,
    pub v: f32,
    pub w: f32,
}

/// Calculates the barycentric coordinates of point `p` with respect to triangle `abc`.
///
/// Barycentric coordinates represent a point inside a triangle as a weighted combination
/// of the three vertices. The weights (u, v, w) sum to 1.0 and represent the "pull"
/// or contribution from each vertex.
///
/// # Returns
/// - `Some(BarycentricCoords)` if the triangle is valid (non-degenerate)
/// - `None` if the triangle is degenerate (collinear points, zero area)
///
/// # Coordinates
/// - `u` is the weight for vertex `a` (corresponds to area of sub-triangle PBC)
/// - `v` is the weight for vertex `b` (corresponds to area of sub-triangle PCA)
/// - `w` is the weight for vertex `c` (corresponds to area of sub-triangle PAB)
///
/// # Properties
/// - If point is inside triangle: all coordinates are non-negative (u ≥ 0, v ≥ 0, w ≥ 0)
/// - If point is on an edge: one coordinate is 0
/// - If point is at a vertex: one coordinate is 1.0, others are 0
/// - u + v + w = 1.0 (always)
///
/// # Example
/// ```
/// use scratchpad_rs::math::{barycentric, Vec2};
/// let a = Vec2::new(0.0, 0.0);
/// let b = Vec2::new(1.0, 0.0);
/// let c = Vec2::new(0.0, 1.0);
/// let p = Vec2::new(0.5, 0.5);
/// let coords = barycentric::barycentric(p, a, b, c).unwrap();
/// // Point at centroid: u ≈ 0.0, v ≈ 0.5, w ≈ 0.5
/// ```
pub fn barycentric(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> Option<BarycentricCoords> {
    // Build local edge vectors
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;

    // Note that when then sub-triangles get divide by denom, the 2s cancel out
    // denom = area of the triangle * 2
    let denom = v0.cross(v1);

    // handle degenerate triangle
    if denom.abs() < f32::EPSILON {
        return None;
    }

    // As mentioned earlier, the 2s from the area calculations cancel out here
    let v = v2.cross(v1) / denom; // area of sub-triangle PCA / area of triangle ABC.
    let w = v0.cross(v2) / denom; // area of sub-triangle PAB / area of triangle ABC
    let u = 1.0 - v - w; // area of sub-triangle PBC / area of triangle ABC

    Some(BarycentricCoords { u, v, w })
}

/// Checks if a point is inside the triangle based on barycentric coordinates.
///
/// A point is inside (or on the boundary of) a triangle if all barycentric
/// coordinates are non-negative. This means the point is not outside any edge.
///
/// # Returns
/// - `true` if point is inside or on the triangle boundary
/// - `false` if point is outside the triangle
///
/// # Edge Cases
/// - Points exactly on edges: returns `true` (one coordinate = 0)
/// - Points at vertices: returns `true` (one coordinate = 1.0)
/// - Points outside: returns `false` (at least one coordinate < 0)
///
/// # Example
/// ```
/// use scratchpad_rs::math::{barycentric, Vec2};
/// let a = Vec2::new(0.0, 0.0);
/// let b = Vec2::new(1.0, 0.0);
/// let c = Vec2::new(0.0, 1.0);
///
/// // Point inside triangle
/// let p_inside = Vec2::new(0.3, 0.3);
/// let coords_inside = barycentric::barycentric(p_inside, a, b, c).unwrap();
/// assert!(barycentric::is_point_in_triangle(&coords_inside));
///
/// // Point outside triangle
/// let p_outside = Vec2::new(1.5, 1.5);
/// let coords_outside = barycentric::barycentric(p_outside, a, b, c).unwrap();
/// assert!(!barycentric::is_point_in_triangle(&coords_outside));
/// ```
pub fn is_point_in_triangle(bary_coords: &BarycentricCoords) -> bool {
    bary_coords.u >= 0.0 && bary_coords.v >= 0.0 && bary_coords.w >= 0.0
}

/// Interpolate a f32 value using barycentric coordinates.
///
/// Linearly interpolates between three f32 values (a, b, c) using the
/// barycentric weights (u, v, w) where:
/// - u is the weight for value a
/// - v is the weight for value b
/// - w is the weight for value c
///
/// # Formula
/// `result = u * a + v * b + w * c`
///
/// # Use Cases
/// - Interpolating colors (per channel: R, G, B, A)
/// - Interpolating depth values (Z-buffer)
/// - Interpolating any scalar attributes across a triangle
///
/// # Example
/// ```
/// use scratchpad_rs::math::{barycentric, Vec2};
/// let a = Vec2::new(0.0, 0.0);
/// let b = Vec2::new(1.0, 0.0);
/// let c = Vec2::new(0.0, 1.0);
/// let p = Vec2::new(0.5, 0.5);
/// let coords = barycentric::barycentric(p, a, b, c).unwrap();
/// let value_a = 10.0;
/// let value_b = 20.0;
/// let value_c = 30.0;
/// let interpolated = barycentric::interpolate_f32(&coords, value_a, value_b, value_c);
/// // Should be approximately 20.0 (midpoint)
/// ```
pub fn interpolate_f32(coords: &BarycentricCoords, a: f32, b: f32, c: f32) -> f32 {
    coords.u * a + coords.v * b + coords.w * c
}

/// Interpolate a Vec2 value using barycentric coordinates.
///
/// Linearly interpolates between three Vec2 values (a, b, c) using the
/// barycentric weights (u, v, w). This interpolates both the x and y components
/// independently using the same weights.
///
/// # Formula
/// `result = u * a + v * b + w * c`
///
/// Which expands to:
/// - `result.x = u * a.x + v * b.x + w * c.x`
/// - `result.y = u * a.y + v * b.y + w * c.y`
///
/// # Use Cases
/// - Interpolating texture coordinates (UV mapping)
/// - Interpolating normals for smooth shading
/// - Interpolating any 2D vector attributes across a triangle
///
/// # Example
/// ```
/// use scratchpad_rs::math::{barycentric, Vec2};
/// let a = Vec2::new(0.0, 0.0);
/// let b = Vec2::new(1.0, 0.0);
/// let c = Vec2::new(0.0, 1.0);
/// let p = Vec2::new(0.5, 0.5);
/// let coords = barycentric::barycentric(p, a, b, c).unwrap();
///
/// // Interpolate texture coordinates
/// let uv_a = Vec2::new(0.0, 0.0);
/// let uv_b = Vec2::new(1.0, 0.0);
/// let uv_c = Vec2::new(0.0, 1.0);
/// let interpolated_uv = barycentric::interpolate_vec2(&coords, uv_a, uv_b, uv_c);
/// // Result will be a blend of the three UV coordinates
/// ```
pub fn interpolate_vec2(coords: &BarycentricCoords, a: Vec2, b: Vec2, c: Vec2) -> Vec2 {
    coords.u * a + coords.v * b + coords.w * c
}

/// Interpolate a Color value using barycentric coordinates.
///
/// Linearly interpolates between three Color values (a, b, c) using the
/// barycentric weights (u, v, w). This interpolates each color channel
/// (R, G, B, A) independently using the same weights.
///
/// # Formula
/// For each channel (R, G, B, A):
/// `result.channel = u * a.channel + v * b.channel + w * c.channel`
///
/// # Use Cases
/// - Gouraud shading (smooth color gradients across triangles)
/// - Vertex color interpolation
/// - Creating smooth color transitions
///
/// # Example
/// ```
/// use scratchpad_rs::color::Color;
/// use scratchpad_rs::math::{barycentric, Vec2};
/// let a = Vec2::new(0.0, 0.0);
/// let b = Vec2::new(1.0, 0.0);
/// let c = Vec2::new(0.0, 1.0);
/// let p = Vec2::new(0.5, 0.5);
/// let coords = barycentric::barycentric(p, a, b, c).unwrap();
///
/// // Interpolate colors: red at A, green at B, blue at C
/// let color_a = Color::RED;
/// let color_b = Color::GREEN;
/// let color_c = Color::BLUE;
/// let interpolated = barycentric::interpolate_color(&coords, color_a, color_b, color_c);
/// // Result will be a blend of red, green, and blue
/// ```
pub fn interpolate_color(coords: &BarycentricCoords, a: Color, b: Color, c: Color) -> Color {
    let ri = (coords.u * a.r as f32 + coords.v * b.r as f32 + coords.w * c.r as f32).round() as u8;
    let gi = (coords.u * a.g as f32 + coords.v * b.g as f32 + coords.w * c.g as f32).round() as u8;
    let bi = (coords.u * a.b as f32 + coords.v * b.b as f32 + coords.w * c.b as f32).round() as u8;
    let alpha =
        (coords.u * a.a as f32 + coords.v * b.a as f32 + coords.w * c.a as f32).round() as u8;

    Color::RGBA(ri, gi, bi, alpha)
}
