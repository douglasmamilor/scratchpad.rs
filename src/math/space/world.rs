use crate::math::{Point2, Vec2};

/// A point in world space coordinates.
///
/// World space is the absolute coordinate system where objects exist.
/// Origin (0, 0) is at the world center, with Y-axis pointing up.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WorldPoint(Point2);

/// A vector in world space coordinates.
///
/// Represents a displacement or direction in world space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WorldVec2(Vec2);

impl WorldPoint {
    /// Create a new world point from x, y coordinates.
    pub fn new(x: f32, y: f32) -> Self {
        WorldPoint(Point2::new(x, y))
    }

    /// Create a world point at the origin.
    pub fn zero() -> Self {
        WorldPoint(Point2::ZERO)
    }

    /// Create a world point from a Point2.
    pub fn from_point2(p: Point2) -> Self {
        WorldPoint(p)
    }

    /// Convert to Point2 (loses type safety).
    pub fn to_point2(self) -> Point2 {
        self.0
    }

    /// Get the x coordinate.
    pub fn x(&self) -> f32 {
        self.0.x
    }

    /// Get the y coordinate.
    pub fn y(&self) -> f32 {
        self.0.y
    }

    /// Calculate distance to another world point.
    pub fn distance_to(self, other: WorldPoint) -> f32 {
        (self.0 - other.0).len()
    }

    /// Linear interpolation between two world points.
    pub fn lerp(self, other: WorldPoint, t: f32) -> WorldPoint {
        WorldPoint(self.0 + (other.0 - self.0) * t)
    }

    /// Translate by a world vector.
    pub fn translate(self, delta: WorldVec2) -> WorldPoint {
        WorldPoint(self.0 + delta.0)
    }
}

impl WorldVec2 {
    /// Create a new world vector from x, y components.
    pub fn new(x: f32, y: f32) -> Self {
        WorldVec2(Vec2::new(x, y))
    }

    /// Create a zero world vector.
    pub fn zero() -> Self {
        WorldVec2(Vec2::ZERO)
    }

    /// Create a world vector from a Vec2.
    pub fn from_vec2(v: Vec2) -> Self {
        WorldVec2(v)
    }

    /// Convert to Vec2 (loses type safety).
    pub fn to_vec2(self) -> Vec2 {
        self.0
    }

    /// Get the x component.
    pub fn x(&self) -> f32 {
        self.0.x
    }

    /// Get the y component.
    pub fn y(&self) -> f32 {
        self.0.y
    }

    /// Calculate the length of the vector.
    pub fn len(self) -> f32 {
        self.0.len()
    }

    /// Calculate the squared length (faster, avoids sqrt).
    pub fn len_squared(self) -> f32 {
        self.0.len_sq()
    }

    /// Normalize the vector to unit length.
    pub fn normalize(self) -> Self {
        WorldVec2(self.0.normalize_or_zero())
    }

    /// Dot product with another world vector.
    pub fn dot(self, other: WorldVec2) -> f32 {
        self.0.dot(other.0)
    }

    /// Cross product with another world vector (returns scalar).
    pub fn cross(self, other: WorldVec2) -> f32 {
        self.0.cross(other.0)
    }

    /// Linear interpolation between two world vectors.
    pub fn lerp(self, other: WorldVec2, t: f32) -> WorldVec2 {
        WorldVec2(self.0.lerp(other.0, t))
    }

    /// Scale the vector by a scalar.
    pub fn scale(self, scalar: f32) -> WorldVec2 {
        WorldVec2(self.0 * scalar)
    }
}

impl std::ops::Add<WorldVec2> for WorldPoint {
    type Output = WorldPoint;

    fn add(self, rhs: WorldVec2) -> Self::Output {
        WorldPoint(self.0 + rhs.0)
    }
}

impl std::ops::Sub<WorldVec2> for WorldPoint {
    type Output = WorldPoint;

    fn sub(self, rhs: WorldVec2) -> Self::Output {
        WorldPoint(self.0 - rhs.0)
    }
}

impl std::ops::Sub<WorldPoint> for WorldPoint {
    type Output = WorldVec2;

    fn sub(self, rhs: WorldPoint) -> Self::Output {
        WorldVec2(self.0 - rhs.0)
    }
}

impl std::ops::Add for WorldVec2 {
    type Output = WorldVec2;

    fn add(self, rhs: WorldVec2) -> Self::Output {
        WorldVec2(self.0 + rhs.0)
    }
}

impl std::ops::Sub for WorldVec2 {
    type Output = WorldVec2;

    fn sub(self, rhs: WorldVec2) -> Self::Output {
        WorldVec2(self.0 - rhs.0)
    }
}

impl std::ops::Mul<f32> for WorldVec2 {
    type Output = WorldVec2;

    fn mul(self, rhs: f32) -> Self::Output {
        WorldVec2(self.0 * rhs)
    }
}

impl std::ops::Mul<WorldVec2> for f32 {
    type Output = WorldVec2;

    fn mul(self, rhs: WorldVec2) -> Self::Output {
        WorldVec2(self * rhs.0)
    }
}

impl std::ops::Neg for WorldVec2 {
    type Output = WorldVec2;

    fn neg(self) -> Self::Output {
        WorldVec2(-self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_point_creation() {
        let p = WorldPoint::new(10.0, 20.0);
        assert_eq!(p.x(), 10.0);
        assert_eq!(p.y(), 20.0);
        assert_eq!(p, WorldPoint::zero().translate(WorldVec2::new(10.0, 20.0)));
    }

    #[test]
    fn world_point_arithmetic() {
        let p1 = WorldPoint::new(10.0, 20.0);
        let p2 = WorldPoint::new(5.0, 15.0);
        let delta = p1 - p2;
        assert_eq!(delta.x(), 5.0);
        assert_eq!(delta.y(), 5.0);
        
        let translated = p2 + delta;
        assert_eq!(translated, p1);
    }

    #[test]
    fn world_vec2_operations() {
        let v = WorldVec2::new(3.0, 4.0);
        assert_eq!(v.len(), 5.0);
        assert_eq!(v.len_squared(), 25.0);
        
        let normalized = v.normalize();
        assert!((normalized.len() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn world_point_distance() {
        let p1 = WorldPoint::new(0.0, 0.0);
        let p2 = WorldPoint::new(3.0, 4.0);
        assert_eq!(p1.distance_to(p2), 5.0);
    }

    #[test]
    fn world_point_lerp() {
        let p1 = WorldPoint::new(0.0, 0.0);
        let p2 = WorldPoint::new(10.0, 10.0);
        let mid = p1.lerp(p2, 0.5);
        assert_eq!(mid.x(), 5.0);
        assert_eq!(mid.y(), 5.0);
    }
}
