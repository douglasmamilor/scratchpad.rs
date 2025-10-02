use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Zero vector (0, 0)
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    /// Unit vector (1, 1)
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    /// Unit vector along X-axis (1, 0)
    pub const X: Self = Self { x: 1.0, y: 0.0 };

    /// Unit vector along Y-axis (0, 1)
    pub const Y: Self = Self { x: 0.0, y: 1.0 };

    /// Creates a new Vec2 with the given x and y components
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::vec2::Vec2;
    /// let v = Vec2::new(3.0, 4.0);
    /// assert_eq!(v.x, 3.0);
    /// assert_eq!(v.y, 4.0);
    /// ```
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Computes the dot product of two vectors
    ///
    /// The dot product measures how "aligned" two vectors are:
    /// - Positive: vectors point in similar directions
    /// - Zero: vectors are perpendicular
    /// - Negative: vectors point in opposite directions
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::vec2::Vec2;
    /// let a = Vec2::new(1.0, 0.0);  // Right
    /// let b = Vec2::new(0.0, 1.0);  // Up
    /// assert_eq!(a.dot(b), 0.0);    // Perpendicular
    ///
    /// let c = Vec2::new(1.0, 1.0);  // Diagonal
    /// assert!(a.dot(c) > 0.0);      // Similar direction
    /// ```
    #[inline]
    pub fn dot(self, rhs: Vec2) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    /// Computes the 2D cross product (scalar result)
    ///
    /// Returns the signed area of the parallelogram formed by the two vectors.
    /// Positive when rhs is counter-clockwise from self, negative when clockwise.
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::vec2::Vec2;
    /// let right = Vec2::new(1.0, 0.0);
    /// let up = Vec2::new(0.0, 1.0);
    /// assert_eq!(right.cross(up), 1.0);   // Right to up is counter-clockwise
    /// assert_eq!(up.cross(right), -1.0);  // Up to right is clockwise
    /// ```
    #[inline]
    pub fn cross(self, rhs: Self) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    /// Alternative cross product implementation using perp + dot
    ///
    /// This is mathematically equivalent to `cross()` but uses the relationship
    /// that a·b_perp = -(a × b) where b_perp is b rotated 90° counter-clockwise.
    #[inline]
    pub fn cross_via_perp(self, rhs: Self) -> f32 {
        -self.dot(rhs.perp()) // because a.b_perp = -(a cross b)
    }

    /// Returns the squared length of the vector
    ///
    /// Useful when you only need to compare lengths (avoids expensive sqrt).
    /// For example: `if v.len_sq() < threshold_sq { ... }`
    #[inline]
    pub fn len_sq(self) -> f32 {
        self.dot(self)
    }

    /// Returns the length (magnitude) of the vector
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::vec2::Vec2;
    /// let v = Vec2::new(3.0, 4.0);
    /// assert_eq!(v.len(), 5.0);  // 3-4-5 triangle
    /// ```
    #[inline]
    pub fn len(self) -> f32 {
        self.len_sq().sqrt()
    }

    /// Normalizes the vector to unit length, or returns zero vector if length is zero
    ///
    /// This prevents NaN results that would occur from normalizing a zero vector.
    /// Returns a vector pointing in the same direction but with length 1.0.
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::vec2::Vec2;
    /// let v = Vec2::new(3.0, 4.0);
    /// let normalized = v.normalize_or_zero();
    /// assert_eq!(normalized.len(), 1.0);
    /// assert_eq!(normalized, Vec2::new(0.6, 0.8));
    ///
    /// let zero = Vec2::ZERO;
    /// assert_eq!(zero.normalize_or_zero(), Vec2::ZERO);
    /// ```
    #[inline]
    pub fn normalize_or_zero(self) -> Self {
        let len = self.len();
        if len > 0.0 {
            self / len
        } else {
            Self::default()
        }
    }

    /// Checks if two vectors are approximately equal within the given epsilon
    ///
    /// Useful for floating-point comparisons where exact equality is unlikely.
    /// Returns true if both x and y components are within `eps` of each other.
    #[inline]
    pub fn near(self, rhs: Self, eps: f32) -> bool {
        (self.x - rhs.x).abs() <= eps && (self.y - rhs.y).abs() <= eps
    }

    /// Linear interpolation between two vectors
    ///
    /// When t=0, returns self. When t=1, returns to. When t=0.5, returns the midpoint.
    /// Useful for smooth transitions, animations, and blending between positions.
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::vec2::Vec2;
    /// let start = Vec2::new(0.0, 0.0);
    /// let end = Vec2::new(10.0, 20.0);
    /// assert_eq!(start.lerp(end, 0.0), start);
    /// assert_eq!(start.lerp(end, 1.0), end);
    /// assert_eq!(start.lerp(end, 0.5), Vec2::new(5.0, 10.0));
    /// ```
    #[inline]
    pub fn lerp(self, to: Self, t: f32) -> Self {
        self + (to - self) * t
    }

    /// Returns the perpendicular vector (rotated 90° counter-clockwise)
    ///
    /// This is useful for computing normals, finding orthogonal directions,
    /// and implementing 2D rotations. The result has the same length as the original.
    #[inline]
    pub fn perp(self) -> Self {
        // rotate +90°
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// Reflects this vector across a normal vector
    ///
    /// Computes the reflection of this vector across the line defined by the normal.
    /// The normal should be normalized for correct results.
    /// Formula: reflected = self - 2 * (self · normal) * normal
    #[inline]
    pub fn reflect(self, normal: Self) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }

    /// Returns the angle in radians from the positive X-axis
    ///
    /// Range: [-π, π] (atan2 convention)
    /// - 0 radians = Right (+X axis)
    /// - π/2 radians = Up (+Y axis)
    /// - π radians = Left (-X axis)
    /// - -π/2 radians = Down (-Y axis)
    #[inline]
    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }

    /// Creates a unit vector from an angle in radians
    ///
    /// Angle 0 points along positive X-axis, π/2 points along positive Y-axis.
    /// Useful for converting polar coordinates to Cartesian, or creating
    /// direction vectors from angles.
    #[inline]
    pub fn from_angle(angle: f32) -> Self {
        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    /// Returns the distance between two points
    ///
    /// Computes the Euclidean distance: √((x₂-x₁)² + (y₂-y₁)²)
    /// Equivalent to the length of the displacement vector (to - self).
    #[inline]
    pub fn distance(self, to: Self) -> f32 {
        (self - to).len()
    }
}

/******************* Unary ******************/
/// Negation operator (-v)
///
/// Returns a vector with the same length but opposite direction.
impl Neg for Vec2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

/******************* Add/Sub ******************/

/// Addition operator (v1 + v2)
///
/// Component-wise addition: (x₁+x₂, y₁+y₂)
impl Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Addition assignment operator (v1 += v2)
///
/// Modifies the vector in-place by adding the components of rhs.
impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

/// Subtraction operator (v1 - v2)
///
/// Component-wise subtraction: (x₁-x₂, y₁-y₂)
impl Sub for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// Subtraction assignment operator (v1 -= v2)
///
/// Modifies the vector in-place by subtracting the components of rhs.
impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/******************* Mul/Div ******************/

/// Scalar multiplication operator (v * scalar)
///
/// Scales the vector by the given scalar: (x*s, y*s)
impl Mul<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// Scalar multiplication assignment operator (v *= scalar)
///
/// Modifies the vector in-place by scaling its components.
impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

/// Scalar multiplication operator (scalar * v)
///
/// Allows scalar-vector multiplication in both orders: `2.0 * v` and `v * 2.0`
impl Mul<Vec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs * self
    }
}

/// Scalar division operator (v / scalar)
///
/// Scales the vector by the inverse of the scalar: (x/s, y/s)
/// Uses multiplication by inverse for better numerical stability.
impl Div<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        debug_assert!(rhs != 0.0, "division by zero in Vec2::div");

        let inv = 1.0 / rhs;
        Self {
            x: self.x * inv,
            y: self.y * inv,
        }
    }
}

/// Scalar division assignment operator (v /= scalar)
///
/// Modifies the vector in-place by dividing its components by the scalar.
impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        debug_assert!(rhs != 0.0, "division by zero in Vec2::div");

        let inv = 1.0 / rhs;
        self.x *= inv;
        self.y *= inv;
    }
}

/******************* Indexing ******************/

/// Indexing operator (v[i])
///
/// Allows accessing vector components by index:
/// - v[0] returns the x component
/// - v[1] returns the y component
///
/// Panics if index is out of range (not 0 or 1).
impl Index<usize> for Vec2 {
    type Output = f32;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vec2 index out of range: {i}"),
        }
    }
}

/// Mutable indexing operator (v[i] = value)
///
/// Allows modifying vector components by index:
/// - v[0] = value sets the x component
/// - v[1] = value sets the y component
///
/// Panics if index is out of range (not 0 or 1).
impl IndexMut<usize> for Vec2 {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vec2 index out of range: {i}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ops() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        assert_eq!(a + b, Vec2::new(4.0, 6.0));
        assert_eq!(-a, Vec2::new(-1.0, -2.0));
        assert_eq!(a * 2.0, Vec2::new(2.0, 4.0));
        assert_eq!(2.0 * a, Vec2::new(2.0, 4.0));
        assert_eq!(a / 2.0, Vec2::new(0.5, 1.0));
        let mut c = a;
        c[1] += 10.0;
        assert_eq!(c, Vec2::new(1.0, 12.0));
    }

    #[test]
    fn math() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.len(), 5.0);
        assert_eq!(v.normalize_or_zero().len(), 1.0);
        assert_eq!(v.dot(Vec2::new(1.0, 1.0)), 7.0);
    }

    #[test]
    fn angle_functions() {
        use std::f32::consts::PI;

        // Test basic angles
        assert_eq!(Vec2::X.angle(), 0.0); // Right
        assert_eq!(Vec2::Y.angle(), PI / 2.0); // Up
        assert!(((-Vec2::X).angle() - (-PI)).abs() < 1e-6); // Left (atan2 gives -π for negative x)
        assert_eq!((-Vec2::Y).angle(), -PI / 2.0); // Down

        // Test round-trip conversion
        let angles = [0.0, PI / 4.0, PI / 2.0, PI, -PI / 2.0, -PI / 4.0];
        for &angle in &angles {
            let v = Vec2::from_angle(angle);
            assert!(v.angle().abs() - angle.abs() < 1e-6);
        }

        // Test distance
        let a = Vec2::new(1.0, 1.0);
        let b = Vec2::new(4.0, 5.0);
        assert_eq!(a.distance(b), 5.0); // 3-4-5 triangle
    }

    #[test]
    fn reflection() {
        let incident = Vec2::new(1.0, -1.0);
        let normal = Vec2::new(0.0, 1.0);
        let reflected = incident.reflect(normal);

        // Should reflect across the Y=0 line
        assert_eq!(reflected, Vec2::new(1.0, 1.0));

        // Test reflection of a vector along the normal
        let along_normal = Vec2::new(0.0, 1.0);
        let reflected_along = along_normal.reflect(normal);
        assert_eq!(reflected_along, Vec2::new(0.0, -1.0));

        // Test reflection across X-axis
        let incident_x = Vec2::new(-1.0, 1.0);
        let normal_x = Vec2::new(1.0, 0.0);
        let reflected_x = incident_x.reflect(normal_x);
        assert_eq!(reflected_x, Vec2::new(1.0, 1.0));
    }

    #[test]
    fn constants() {
        assert_eq!(Vec2::ZERO, Vec2::new(0.0, 0.0));
        assert_eq!(Vec2::ONE, Vec2::new(1.0, 1.0));
        assert_eq!(Vec2::X, Vec2::new(1.0, 0.0));
        assert_eq!(Vec2::Y, Vec2::new(0.0, 1.0));
    }

    #[test]
    fn cross_product() {
        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(0.0, 1.0);

        // Right-handed coordinate system: a × b should be positive
        assert_eq!(a.cross(b), 1.0);
        assert_eq!(b.cross(a), -1.0);

        // Test cross_via_perp equivalence
        assert_eq!(a.cross(b), a.cross_via_perp(b));
        assert_eq!(b.cross(a), b.cross_via_perp(a));

        // Parallel vectors should have zero cross product
        assert_eq!(a.cross(a), 0.0);
        assert_eq!(a.cross(a * 2.0), 0.0);
    }

    #[test]
    fn length_and_normalization() {
        let v = Vec2::new(3.0, 4.0);

        // Test length
        assert_eq!(v.len(), 5.0);
        assert_eq!(v.len_sq(), 25.0);

        // Test normalization
        let normalized = v.normalize_or_zero();
        assert_eq!(normalized.len(), 1.0);
        assert_eq!(normalized, Vec2::new(0.6, 0.8));

        // Test zero vector normalization
        let zero = Vec2::ZERO;
        assert_eq!(zero.normalize_or_zero(), Vec2::ZERO);
        assert_eq!(zero.len(), 0.0);
        assert_eq!(zero.len_sq(), 0.0);
    }

    #[test]
    fn perp_and_negation() {
        let v = Vec2::new(1.0, 2.0);
        let perp = v.perp();

        // Perpendicular should be (-y, x)
        assert_eq!(perp, Vec2::new(-2.0, 1.0));

        // Perpendicular should be orthogonal (dot product = 0)
        assert_eq!(v.dot(perp), 0.0);

        // Test negation
        assert_eq!(-v, Vec2::new(-1.0, -2.0));
        assert_eq!(-Vec2::ZERO, Vec2::ZERO);
    }

    #[test]
    fn lerp() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(10.0, 20.0);

        // Test interpolation endpoints
        assert_eq!(a.lerp(b, 0.0), a);
        assert_eq!(a.lerp(b, 1.0), b);

        // Test midpoint
        assert_eq!(a.lerp(b, 0.5), Vec2::new(5.0, 10.0));

        // Test quarter points
        assert_eq!(a.lerp(b, 0.25), Vec2::new(2.5, 5.0));
        assert_eq!(a.lerp(b, 0.75), Vec2::new(7.5, 15.0));
    }

    #[test]
    fn near_approximation() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(1.0001, 2.0001);
        let c = Vec2::new(1.1, 2.1);

        // Should be near with small epsilon
        assert!(a.near(b, 0.001));
        assert!(!a.near(c, 0.001));

        // Should not be near with tiny epsilon
        assert!(!a.near(b, 0.00001));

        // Exact equality should be near with any epsilon
        assert!(a.near(a, 0.0));
    }

    #[test]
    fn assignment_operators() {
        let mut v = Vec2::new(1.0, 2.0);

        // Test +=
        v += Vec2::new(3.0, 4.0);
        assert_eq!(v, Vec2::new(4.0, 6.0));

        // Test -=
        v -= Vec2::new(1.0, 2.0);
        assert_eq!(v, Vec2::new(3.0, 4.0));

        // Test *=
        v *= 2.0;
        assert_eq!(v, Vec2::new(6.0, 8.0));

        // Test /=
        v /= 2.0;
        assert_eq!(v, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn indexing() {
        let mut v = Vec2::new(1.0, 2.0);

        // Test read access
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);

        // Test write access
        v[0] = 5.0;
        v[1] = 6.0;
        assert_eq!(v, Vec2::new(5.0, 6.0));

        // Test bounds checking (should panic)
        // Note: These would panic in debug mode, but we can't test panics easily
        // assert!(std::panic::catch_unwind(|| v[2]).is_err());
    }

    #[test]
    fn dot_product_properties() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        let c = Vec2::new(5.0, 6.0);

        // Commutative: a·b = b·a
        assert_eq!(a.dot(b), b.dot(a));

        // Distributive: a·(b+c) = a·b + a·c
        assert_eq!(a.dot(b + c), a.dot(b) + a.dot(c));

        // Scalar multiplication: (ka)·b = k(a·b)
        let k = 2.0;
        assert_eq!((k * a).dot(b), k * a.dot(b));

        // Dot product with self is length squared
        assert_eq!(a.dot(a), a.len_sq());

        // Orthogonal vectors have zero dot product
        assert_eq!(Vec2::X.dot(Vec2::Y), 0.0);
        assert_eq!(Vec2::X.dot(Vec2::X.perp()), 0.0);
    }

    #[test]
    fn edge_cases() {
        // Test with very small numbers
        let tiny = Vec2::new(1e-10, 1e-10);
        assert!(tiny.len() > 0.0);
        assert!((tiny.normalize_or_zero().len() - 1.0).abs() < 1e-6);

        // Test with very large numbers
        let huge = Vec2::new(1e10, 1e10);
        assert!(huge.len() > 0.0);
        assert!((huge.normalize_or_zero().len() - 1.0).abs() < 1e-6);

        // Test infinity and NaN handling
        let inf = Vec2::new(f32::INFINITY, 0.0);
        assert_eq!(inf.len(), f32::INFINITY);

        // Test negative zero
        let neg_zero = Vec2::new(-0.0, -0.0);
        assert_eq!(neg_zero.len(), 0.0);
        assert_eq!(neg_zero.normalize_or_zero(), Vec2::ZERO);
    }
}
