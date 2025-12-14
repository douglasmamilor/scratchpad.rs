use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// Zero vector (0, 0, 0)
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    /// Unit vector (1, 1, 1)
    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    /// Unit vector along X-axis (1, 0, 0)
    pub const X: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };

    /// Unit vector along Y-axis (0, 1, 0)
    pub const Y: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    /// Unit vector along Z-axis (0, 0, 1)
    pub const Z: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    /// Creates a new Vec3 with the given x, y, and z components
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::vec3::Vec3;
    /// let v = Vec3::new(1.0, 2.0, 3.0);
    /// assert_eq!(v.x, 1.0);
    /// assert_eq!(v.y, 2.0);
    /// assert_eq!(v.z, 3.0);
    /// ```
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
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
    /// use scratchpad_rs::math::vec3::Vec3;
    /// let a = Vec3::new(1.0, 0.0, 0.0);  // X-axis
    /// let b = Vec3::new(0.0, 1.0, 0.0);  // Y-axis
    /// assert_eq!(a.dot(b), 0.0);         // Perpendicular
    ///
    /// let c = Vec3::new(1.0, 1.0, 0.0);  // Diagonal
    /// assert!(a.dot(c) > 0.0);           // Similar direction
    /// ```
    #[inline]
    pub fn dot(self, rhs: Vec3) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Computes the 3D cross product (vector result)
    ///
    /// Returns a vector perpendicular to both input vectors.
    /// The result follows the right-hand rule: if you point your right thumb
    /// along self and curl your fingers toward rhs, the result points in the
    /// direction of your palm.
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::vec3::Vec3;
    /// let x = Vec3::new(1.0, 0.0, 0.0);
    /// let y = Vec3::new(0.0, 1.0, 0.0);
    /// let z = x.cross(y);
    /// assert_eq!(z, Vec3::new(0.0, 0.0, 1.0));  // Z-axis
    /// ```
    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
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
    /// use scratchpad_rs::math::vec3::Vec3;
    /// let v = Vec3::new(3.0, 4.0, 0.0);
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
    /// Returns true if all x, y, and z components are within `eps` of each other.
    #[inline]
    pub fn near(self, rhs: Self, eps: f32) -> bool {
        (self.x - rhs.x).abs() <= eps
            && (self.y - rhs.y).abs() <= eps
            && (self.z - rhs.z).abs() <= eps
    }

    /// Linear interpolation between two vectors
    ///
    /// When t=0, returns self. When t=1, returns to. When t=0.5, returns the midpoint.
    /// Useful for smooth transitions, animations, and blending between positions.
    #[inline]
    pub fn lerp(self, to: Self, t: f32) -> Self {
        self + (to - self) * t
    }

    /// Returns the distance between two points
    ///
    /// Computes the Euclidean distance: √((x₂-x₁)² + (y₂-y₁)² + (z₂-z₁)²)
    /// Equivalent to the length of the displacement vector (to - self).
    #[inline]
    pub fn distance(self, to: Self) -> f32 {
        (self - to).len()
    }

    /// Reflects this vector across a normal vector
    ///
    /// Computes the reflection of this vector across the plane defined by the normal.
    /// The normal should be normalized for correct results.
    /// Formula: reflected = self - 2 * (self · normal) * normal
    #[inline]
    pub fn reflect(self, normal: Self) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }

    /// Projects this vector onto another vector
    ///
    /// Returns the component of this vector that lies along the direction of `onto`.
    /// The result is a scalar multiple of `onto` that represents the projection.
    /// i.e ((b.a)*a)/len_q(a)
    #[inline]
    pub fn project_onto(self, onto: Self) -> Self {
        let onto_len_sq = onto.len_sq();
        if onto_len_sq > 0.0 {
            onto * (self.dot(onto) / onto_len_sq)
        } else {
            Self::ZERO
        }
    }

    /// Returns the component of this vector perpendicular to another vector
    ///
    /// Computes the rejection: self - project_onto(self, onto)
    /// This gives the component of self that is orthogonal to onto.
    #[inline]
    pub fn reject_from(self, from: Self) -> Self {
        self - self.project_onto(from)
    }

    /// Spherical linear interpolation between two unit vectors
    ///
    /// Interpolates along the shortest path on the unit sphere between two vectors.
    /// Both vectors should be normalized for correct results.
    ///
    /// # Arguments
    /// * `to` - The target vector (should be normalized)
    /// * `t` - Interpolation parameter in [0, 1]
    ///
    /// # Returns
    /// A normalized vector interpolated between self and to.
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::vec3::Vec3;
    /// let x = Vec3::new(1.0, 0.0, 0.0);
    /// let y = Vec3::new(0.0, 1.0, 0.0);
    /// let interpolated = x.slerp(y, 0.5);
    /// // Result is a unit vector halfway between X and Y axes
    /// assert!((interpolated.len() - 1.0).abs() < 1e-6);
    /// ```
    #[inline]
    pub fn slerp(self, to: Self, t: f32) -> Self {
        let dot = self.dot(to).clamp(-1.0, 1.0);

        // If vectors are very close, use linear interpolation to avoid precision issues
        if dot > 0.9995 {
            return self.lerp(to, t).normalize_or_zero();
        }

        let theta = dot.acos();
        let sin_theta = theta.sin();

        if sin_theta.abs() < 1e-6 {
            return self;
        }

        let a = ((1.0 - t) * theta).sin() / sin_theta;
        let b = (t * theta).sin() / sin_theta;

        self * a + to * b
    }
}

/******************* Unary ******************/
/// Negation operator (-v)
///
/// Returns a vector with the same length but opposite direction.
impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/******************* Add/Sub ******************/

/// Addition operator (v1 + v2)
///
/// Component-wise addition: (x₁+x₂, y₁+y₂, z₁+z₂)
impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// Addition assignment operator (v1 += v2)
///
/// Modifies the vector in-place by adding the components of rhs.
impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

/// Subtraction operator (v1 - v2)
///
/// Component-wise subtraction: (x₁-x₂, y₁-y₂, z₁-z₂)
impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// Subtraction assignment operator (v1 -= v2)
///
/// Modifies the vector in-place by subtracting the components of rhs.
impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

/******************* Mul/Div ******************/

/// Scalar multiplication operator (v * scalar)
///
/// Scales the vector by the given scalar: (x*s, y*s, z*s)
impl Mul<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

/// Scalar multiplication assignment operator (v *= scalar)
///
/// Modifies the vector in-place by scaling its components.
impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

/// Scalar multiplication operator (scalar * v)
///
/// Allows scalar-vector multiplication in both orders: `2.0 * v` and `v * 2.0`
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

/// Scalar division operator (v / scalar)
///
/// Scales the vector by the inverse of the scalar: (x/s, y/s, z/s)
/// Uses multiplication by inverse for better numerical stability.
impl Div<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        debug_assert!(rhs != 0.0, "division by zero in Vec3::div");

        let inv = 1.0 / rhs;
        Self {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        }
    }
}

/// Scalar division assignment operator (v /= scalar)
///
/// Modifies the vector in-place by dividing its components by the scalar.
impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        debug_assert!(rhs != 0.0, "division by zero in Vec3::div");

        let inv = 1.0 / rhs;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
    }
}

/******************* Indexing ******************/

/// Indexing operator (v[i])
///
/// Allows accessing vector components by index:
/// - v[0] returns the x component
/// - v[1] returns the y component
/// - v[2] returns the z component
///
/// Panics if index is out of range (not 0, 1, or 2).
impl Index<usize> for Vec3 {
    type Output = f32;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 index out of range: {i}"),
        }
    }
}

/// Mutable indexing operator (v[i] = value)
///
/// Allows modifying vector components by index:
/// - v[0] = value sets the x component
/// - v[1] = value sets the y component
/// - v[2] = value sets the z component
///
/// Panics if index is out of range (not 0, 1, or 2).
impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index out of range: {i}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ops() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a + b, Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(-a, Vec3::new(-1.0, -2.0, -3.0));
        assert_eq!(a * 2.0, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(2.0 * a, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(a / 2.0, Vec3::new(0.5, 1.0, 1.5));
        let mut c = a;
        c[2] += 10.0;
        assert_eq!(c, Vec3::new(1.0, 2.0, 13.0));
    }

    #[test]
    fn math() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(v.len(), 5.0);
        assert_eq!(v.normalize_or_zero().len(), 1.0);
        assert_eq!(v.dot(Vec3::new(1.0, 1.0, 1.0)), 7.0);
    }

    #[test]
    fn constants() {
        assert_eq!(Vec3::ZERO, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(Vec3::ONE, Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(Vec3::X, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(Vec3::Y, Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(Vec3::Z, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn cross_product() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);

        // Right-handed coordinate system: a × b should be +Z
        assert_eq!(a.cross(b), Vec3::Z);
        assert_eq!(b.cross(a), -Vec3::Z);

        // Parallel vectors should have zero cross product
        assert_eq!(a.cross(a), Vec3::ZERO);
        assert_eq!(a.cross(a * 2.0), Vec3::ZERO);

        // Test the right-hand rule
        let x = Vec3::X;
        let y = Vec3::Y;
        let z = Vec3::Z;
        assert_eq!(x.cross(y), z);
        assert_eq!(y.cross(z), x);
        assert_eq!(z.cross(x), y);
    }

    #[test]
    fn length_and_normalization() {
        let v = Vec3::new(3.0, 4.0, 0.0);

        // Test length
        assert_eq!(v.len(), 5.0);
        assert_eq!(v.len_sq(), 25.0);

        // Test normalization
        let normalized = v.normalize_or_zero();
        assert_eq!(normalized.len(), 1.0);
        assert_eq!(normalized, Vec3::new(0.6, 0.8, 0.0));

        // Test zero vector normalization
        let zero = Vec3::ZERO;
        assert_eq!(zero.normalize_or_zero(), Vec3::ZERO);
        assert_eq!(zero.len(), 0.0);
        assert_eq!(zero.len_sq(), 0.0);
    }

    #[test]
    fn negation() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(-v, Vec3::new(-1.0, -2.0, -3.0));
        assert_eq!(-Vec3::ZERO, Vec3::ZERO);
    }

    #[test]
    fn lerp() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(10.0, 20.0, 30.0);

        // Test interpolation endpoints
        assert_eq!(a.lerp(b, 0.0), a);
        assert_eq!(a.lerp(b, 1.0), b);

        // Test midpoint
        assert_eq!(a.lerp(b, 0.5), Vec3::new(5.0, 10.0, 15.0));

        // Test quarter points
        assert_eq!(a.lerp(b, 0.25), Vec3::new(2.5, 5.0, 7.5));
        assert_eq!(a.lerp(b, 0.75), Vec3::new(7.5, 15.0, 22.5));
    }

    #[test]
    fn near_approximation() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(1.0001, 2.0001, 3.0001);
        let c = Vec3::new(1.1, 2.1, 3.1);

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
        let mut v = Vec3::new(1.0, 2.0, 3.0);

        // Test +=
        v += Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(v, Vec3::new(4.0, 6.0, 8.0));

        // Test -=
        v -= Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v, Vec3::new(3.0, 4.0, 5.0));

        // Test *=
        v *= 2.0;
        assert_eq!(v, Vec3::new(6.0, 8.0, 10.0));

        // Test /=
        v /= 2.0;
        assert_eq!(v, Vec3::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn indexing() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);

        // Test read access
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);

        // Test write access
        v[0] = 5.0;
        v[1] = 6.0;
        v[2] = 7.0;
        assert_eq!(v, Vec3::new(5.0, 6.0, 7.0));
    }

    #[test]
    fn dot_product_properties() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let c = Vec3::new(7.0, 8.0, 9.0);

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
        assert_eq!(Vec3::X.dot(Vec3::Y), 0.0);
        assert_eq!(Vec3::X.dot(Vec3::Z), 0.0);
        assert_eq!(Vec3::Y.dot(Vec3::Z), 0.0);
    }

    #[test]
    fn reflection() {
        let incident = Vec3::new(1.0, -1.0, 0.0);
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let reflected = incident.reflect(normal);

        // Should reflect across the Y=0 plane
        assert_eq!(reflected, Vec3::new(1.0, 1.0, 0.0));

        // Test reflection of a vector along the normal
        let along_normal = Vec3::new(0.0, 1.0, 0.0);
        let reflected_along = along_normal.reflect(normal);
        assert_eq!(reflected_along, Vec3::new(0.0, -1.0, 0.0));
    }

    #[test]
    fn projection() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let onto = Vec3::new(1.0, 0.0, 0.0);

        // Project onto X-axis should give (3, 0, 0)
        let projected = v.project_onto(onto);
        assert_eq!(projected, Vec3::new(3.0, 0.0, 0.0));

        // Project onto zero vector should give zero
        let zero_proj = v.project_onto(Vec3::ZERO);
        assert_eq!(zero_proj, Vec3::ZERO);

        // Project onto itself should give itself
        let self_proj = v.project_onto(v);
        assert_eq!(self_proj, v);
    }

    #[test]
    fn rejection() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let from = Vec3::new(1.0, 0.0, 0.0);

        // Reject from X-axis should give (0, 4, 0)
        let rejected = v.reject_from(from);
        assert_eq!(rejected, Vec3::new(0.0, 4.0, 0.0));

        // Reject from itself should give zero
        let self_reject = v.reject_from(v);
        assert_eq!(self_reject, Vec3::ZERO);
    }

    #[test]
    fn slerp() {
        // Test slerp between unit vectors
        let a = Vec3::new(1.0, 0.0, 0.0); // X-axis
        let b = Vec3::new(0.0, 1.0, 0.0); // Y-axis

        // Test endpoints
        let result0 = a.slerp(b, 0.0);
        assert!(result0.near(a, 1e-6));

        let result1 = a.slerp(b, 1.0);
        assert!(result1.near(b, 1e-6));

        // Test midpoint (should be on the diagonal)
        let result_mid = a.slerp(b, 0.5);
        let expected = Vec3::new(1.0, 1.0, 0.0).normalize_or_zero();
        assert!(result_mid.near(expected, 1e-6));

        // Test that result is always normalized
        assert!((result_mid.len() - 1.0).abs() < 1e-6);

        // Test slerp between opposite vectors
        let c = Vec3::new(1.0, 0.0, 0.0);
        let d = Vec3::new(-1.0, 0.0, 0.0);
        let result_opposite = c.slerp(d, 0.5);
        // Should be perpendicular to both (any perpendicular vector)
        assert!((result_opposite.len() - 1.0).abs() < 1e-6);

        // Test slerp with same vector
        let result_same = a.slerp(a, 0.5);
        assert!(result_same.near(a, 1e-6));

        // Test slerp with very close vectors (should use linear interpolation fallback)
        let e = Vec3::new(1.0, 0.0, 0.0);
        let f = Vec3::new(0.9999, 0.01, 0.0);
        let result_close = e.slerp(f, 0.5);
        assert!((result_close.len() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn edge_cases() {
        // Test with very small numbers
        let tiny = Vec3::new(1e-10, 1e-10, 1e-10);
        assert!(tiny.len() > 0.0);
        assert!((tiny.normalize_or_zero().len() - 1.0).abs() < 1e-6);

        // Test with very large numbers
        let huge = Vec3::new(1e10, 1e10, 1e10);
        assert!(huge.len() > 0.0);
        assert!((huge.normalize_or_zero().len() - 1.0).abs() < 1e-6);

        // Test infinity and NaN handling
        let inf = Vec3::new(f32::INFINITY, 0.0, 0.0);
        assert_eq!(inf.len(), f32::INFINITY);

        // Test negative zero
        let neg_zero = Vec3::new(-0.0, -0.0, -0.0);
        assert_eq!(neg_zero.len(), 0.0);
        assert_eq!(neg_zero.normalize_or_zero(), Vec3::ZERO);
    }
}
