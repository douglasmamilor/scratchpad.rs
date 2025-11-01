use crate::math::{Point2, Vec2};

/// A point in screen space coordinates.
///
/// Screen space is the coordinate system of the viewport/framebuffer.
/// Origin (0, 0) is at the top-left, with Y-axis pointing down.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenPoint(Point2);

/// A vector in screen space coordinates.
///
/// Represents a displacement or direction in screen space (pixel-based).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenVec2(Vec2);

impl ScreenPoint {
    /// Create a new screen point from x, y coordinates.
    pub fn new(x: f32, y: f32) -> Self {
        ScreenPoint(Point2::new(x, y))
    }

    /// Create a screen point at the origin (top-left).
    pub fn zero() -> Self {
        ScreenPoint(Point2::ZERO)
    }

    /// Create a screen point from a Point2.
    pub fn from_point2(p: Point2) -> Self {
        ScreenPoint(p)
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

    /// Calculate distance to another screen point.
    pub fn distance_to(self, other: ScreenPoint) -> f32 {
        (self.0 - other.0).len()
    }

    /// Linear interpolation between two screen points.
    pub fn lerp(self, other: ScreenPoint, t: f32) -> ScreenPoint {
        ScreenPoint(self.0 + (other.0 - self.0) * t)
    }

    /// Translate by a screen vector.
    pub fn translate(self, delta: ScreenVec2) -> ScreenPoint {
        ScreenPoint(self.0 + delta.0)
    }

    /// Check if point is within screen bounds.
    pub fn is_in_bounds(self, width: f32, height: f32) -> bool {
        self.0.x >= 0.0 && self.0.x < width && self.0.y >= 0.0 && self.0.y < height
    }
}

impl ScreenVec2 {
    /// Create a new screen vector from x, y components.
    pub fn new(x: f32, y: f32) -> Self {
        ScreenVec2(Vec2::new(x, y))
    }

    /// Create a zero screen vector.
    pub fn zero() -> Self {
        ScreenVec2(Vec2::ZERO)
    }

    /// Create a screen vector from a Vec2.
    pub fn from_vec2(v: Vec2) -> Self {
        ScreenVec2(v)
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
        ScreenVec2(self.0.normalize_or_zero())
    }

    /// Dot product with another screen vector.
    pub fn dot(self, other: ScreenVec2) -> f32 {
        self.0.dot(other.0)
    }

    /// Cross product with another screen vector (returns scalar).
    pub fn cross(self, other: ScreenVec2) -> f32 {
        self.0.cross(other.0)
    }

    /// Linear interpolation between two screen vectors.
    pub fn lerp(self, other: ScreenVec2, t: f32) -> ScreenVec2 {
        ScreenVec2(self.0.lerp(other.0, t))
    }

    /// Scale the vector by a scalar.
    pub fn scale(self, scalar: f32) -> ScreenVec2 {
        ScreenVec2(self.0 * scalar)
    }
}

impl std::ops::Add<ScreenVec2> for ScreenPoint {
    type Output = ScreenPoint;

    fn add(self, rhs: ScreenVec2) -> Self::Output {
        ScreenPoint(self.0 + rhs.0)
    }
}

impl std::ops::Sub<ScreenVec2> for ScreenPoint {
    type Output = ScreenPoint;

    fn sub(self, rhs: ScreenVec2) -> Self::Output {
        ScreenPoint(self.0 - rhs.0)
    }
}

impl std::ops::Sub<ScreenPoint> for ScreenPoint {
    type Output = ScreenVec2;

    fn sub(self, rhs: ScreenPoint) -> Self::Output {
        ScreenVec2(self.0 - rhs.0)
    }
}

impl std::ops::Add for ScreenVec2 {
    type Output = ScreenVec2;

    fn add(self, rhs: ScreenVec2) -> Self::Output {
        ScreenVec2(self.0 + rhs.0)
    }
}

impl std::ops::Sub for ScreenVec2 {
    type Output = ScreenVec2;

    fn sub(self, rhs: ScreenVec2) -> Self::Output {
        ScreenVec2(self.0 - rhs.0)
    }
}

impl std::ops::Mul<f32> for ScreenVec2 {
    type Output = ScreenVec2;

    fn mul(self, rhs: f32) -> Self::Output {
        ScreenVec2(self.0 * rhs)
    }
}

impl std::ops::Mul<ScreenVec2> for f32 {
    type Output = ScreenVec2;

    fn mul(self, rhs: ScreenVec2) -> Self::Output {
        ScreenVec2(self * rhs.0)
    }
}

impl std::ops::Neg for ScreenVec2 {
    type Output = ScreenVec2;

    fn neg(self) -> Self::Output {
        ScreenVec2(-self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_point_creation() {
        let p = ScreenPoint::new(100.0, 200.0);
        assert_eq!(p.x(), 100.0);
        assert_eq!(p.y(), 200.0);
    }

    #[test]
    fn screen_point_bounds_check() {
        let p1 = ScreenPoint::new(50.0, 50.0);
        let p2 = ScreenPoint::new(-10.0, 50.0);
        let p3 = ScreenPoint::new(50.0, 1000.0);
        
        assert!(p1.is_in_bounds(800.0, 600.0));
        assert!(!p2.is_in_bounds(800.0, 600.0));
        assert!(!p3.is_in_bounds(800.0, 600.0));
    }

    #[test]
    fn screen_point_arithmetic() {
        let p1 = ScreenPoint::new(100.0, 200.0);
        let p2 = ScreenPoint::new(50.0, 150.0);
        let delta = p1 - p2;
        assert_eq!(delta.x(), 50.0);
        assert_eq!(delta.y(), 50.0);
        
        let translated = p2 + delta;
        assert_eq!(translated, p1);
    }

    #[test]
    fn screen_vec2_operations() {
        let v = ScreenVec2::new(3.0, 4.0);
        assert_eq!(v.len(), 5.0);
        assert_eq!(v.len_squared(), 25.0);
        
        let normalized = v.normalize();
        assert!((normalized.len() - 1.0).abs() < 1e-6);
    }
}
