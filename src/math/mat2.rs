use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::math::vec2::Vec2;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Mat2 {
    pub m00: f32,
    pub m01: f32,
    pub m10: f32,
    pub m11: f32,
}

impl Mat2 {
    /// Zero mat ((0, 0), (0, 0))
    pub const ZERO: Self = Self {
        m00: 0.0,
        m01: 0.0,
        m10: 0.0,
        m11: 0.0,
    };

    /// Identity mat ((1, 0), (0, 1))
    pub const IDENTITY: Self = Self {
        m00: 1.0,
        m01: 0.0,
        m10: 0.0,
        m11: 1.0,
    };

    /// Create a new matrix from elements
    #[inline]
    pub fn new(m00: f32, m01: f32, m10: f32, m11: f32) -> Self {
        Self { m00, m01, m10, m11 }
    }

    /// Create a matrix from row vectors
    #[inline]
    pub fn from_rows(row0: Vec2, row1: Vec2) -> Self {
        Self {
            m00: row0.x,
            m01: row0.y,
            m10: row1.x,
            m11: row1.y,
        }
    }

    /// Create a matrix from column vectors
    #[inline]
    pub fn from_cols(col0: Vec2, col1: Vec2) -> Self {
        Self {
            m00: col0.x,
            m01: col1.x,
            m10: col0.y,
            m11: col1.y,
        }
    }

    /// Calculate the determinant of the matrix
    #[inline]
    pub fn det(self) -> f32 {
        (self.m00 * self.m11) - (self.m01 * self.m10)
    }

    /// Return the trace of this matrix (sum of diagonal elements)
    #[inline]
    pub fn trace(self) -> f32 {
        self.m00 + self.m11
    }

    /// Return the Frobenius norm of this matrix
    ///
    /// The Frobenius norm is the square root of the sum of squares of all elements.
    /// It's a measure of the "size" of the matrix.
    #[inline]
    pub fn frobenius_norm(self) -> f32 {
        (self.m00 * self.m00 + self.m01 * self.m01 + self.m10 * self.m10 + self.m11 * self.m11)
            .sqrt()
    }

    /// Return a transposed version of the matrix
    #[inline]
    pub fn transpose(self) -> Self {
        Self {
            m00: self.m00,
            m01: self.m10,
            m10: self.m01,
            m11: self.m11,
        }
    }

    /// Return the inverse of the matrix
    #[inline]
    pub fn inverse(self) -> Self {
        let det = self.det();
        debug_assert!(det != 0.0, "Matrix is not invertible");

        let inv_det = 1.0 / det;

        Self {
            m00: self.m11 * inv_det,
            m01: -self.m01 * inv_det,
            m10: -self.m10 * inv_det,
            m11: self.m00 * inv_det,
        }
    }

    /// Return a rotation matrix for a given angle in radians
    #[inline]
    pub fn rotate(angle: f32) -> Self {
        Self {
            m00: angle.cos(),
            m01: -angle.sin(),
            m10: angle.sin(),
            m11: angle.cos(),
        }
    }

    /// Return a scaling matrix with given x and y scale factors
    #[inline]
    pub fn scale(sx: f32, sy: f32) -> Self {
        Self {
            m00: sx,
            m01: 0.0,
            m10: 0.0,
            m11: sy,
        }
    }

    /// Return a scaling matrix scaled uniformly by s
    #[inline]
    pub fn scale_uniform(s: f32) -> Self {
        Self {
            m00: s,
            m01: 0.0,
            m10: 0.0,
            m11: s,
        }
    }

    /// Return a shear matrix with given x and y shear factors
    #[inline]
    pub fn shear(sx: f32, sy: f32) -> Self {
        Self {
            m00: 1.0,
            m01: sx,
            m10: sy,
            m11: 1.0,
        }
    }

    /// From angle in radians, create a rotation matrix
    pub fn from_angle(angle: f32) -> Self {
        Self::rotate(angle)
    }

    /// Returns true if this is the identity matrix
    #[inline]
    pub fn is_identity(&self) -> bool {
        self.m00 == 1.0 && self.m01 == 0.0 && self.m10 == 0.0 && self.m11 == 1.0
    }

    /// Returns `true` if this matrix is approximately the identity matrix,
    /// within the given epsilon tolerance.
    ///
    /// Useful for floating-point results where rounding errors are possible.
    ///
    /// # Arguments
    /// * `eps` - maximum allowed difference from the ideal values.
    #[inline]
    pub fn is_identity_eps(&self, eps: f32) -> bool {
        (self.m00 - 1.0).abs() <= eps
            && self.m01.abs() <= eps
            && self.m10.abs() <= eps
            && (self.m11 - 1.0).abs() <= eps
    }

    /// Returns true if this is the zero matrix
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.m00 == 0.0 && self.m01 == 0.0 && self.m10 == 0.0 && self.m11 == 0.0
    }

    /// Returns `true` if this matrix is approximately the zero matrix,
    /// within the given epsilon tolerance.
    ///
    /// Useful for floating-point results where rounding errors are possible.
    ///
    /// # Arguments
    /// * `eps` - maximum allowed difference from zero.
    #[inline]
    pub fn is_zero_eps(&self, eps: f32) -> bool {
        self.m00.abs() <= eps
            && self.m01.abs() <= eps
            && self.m10.abs() <= eps
            && self.m11.abs() <= eps
    }

    /// Convert the matrix to a flat array in **row-major order**.
    #[inline]
    pub fn to_array(self) -> [f32; 4] {
        [self.m00, self.m01, self.m10, self.m11]
    }

    /// Create a matrix from a flat array in **row-major order**.
    #[inline]
    pub fn from_array(arr: [f32; 4]) -> Self {
        Self {
            m00: arr[0],
            m01: arr[1],
            m10: arr[2],
            m11: arr[3],
        }
    }

    /// Lerp between two matrices
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            m00: self.m00 + (other.m00 - self.m00) * t,
            m01: self.m01 + (other.m01 - self.m01) * t,
            m10: self.m10 + (other.m10 - self.m10) * t,
            m11: self.m11 + (other.m11 - self.m11) * t,
        }
    }

    /// Checks if two matrices are approximately equal within the given epsilon
    ///
    /// Useful for floating-point comparisons where exact equality is unlikely.
    /// Returns true if all components are within `eps` of each other.
    #[inline]
    pub fn near(self, rhs: Self, eps: f32) -> bool {
        (self.m00 - rhs.m00).abs() <= eps
            && (self.m01 - rhs.m01).abs() <= eps
            && (self.m10 - rhs.m10).abs() <= eps
            && (self.m11 - rhs.m11).abs() <= eps
    }

    /// Angle of rotation in radians represented by this rotation matrix.
    /// m00 is sin(theta), m10 is cos(theta),
    /// so atan2 of (m10, m00) gives the angle.
    pub fn angle(&self) -> f32 {
        self.m10.atan2(self.m00)
    }

    /// Spherical interpolation between two 2D rotations.
    /// `self` and `other` must be valid rotation matrices.
    /// `t` in [0,1].
    pub fn slerp(&self, other: Mat2, t: f32) -> Mat2 {
        // extract angles
        let theta1 = self.angle(); // you’d need a helper: atan2(m10, m00)
        let theta2 = other.angle();

        // shortest path interpolation
        let mut delta = theta2 - theta1;
        if delta > std::f32::consts::PI {
            delta -= 2.0 * std::f32::consts::PI;
        } else if delta < -std::f32::consts::PI {
            delta += 2.0 * std::f32::consts::PI;
        }

        let theta = theta1 + t * delta;
        Self::from_angle(theta)
    }
}

/******************* Unary ******************/

/// Negation operator (-m)
///
/// Returns a new matrix with each component negated.
impl Neg for Mat2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            m00: -self.m00,
            m01: -self.m01,
            m10: -self.m10,
            m11: -self.m11,
        }
    }
}

/******************* Add/Sub ******************/

/// Addition operator (m1 + m2)
///
/// Component-wise addition of two matrices
impl Add for Mat2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            m00: self.m00 + rhs.m00,
            m01: self.m01 + rhs.m01,
            m10: self.m10 + rhs.m10,
            m11: self.m11 + rhs.m11,
        }
    }
}

/// Addition assignment operator (m1 += m2)
///
/// Modifies the matrix in-place by adding the components of rhs.
impl AddAssign for Mat2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.m00 += rhs.m00;
        self.m01 += rhs.m01;
        self.m10 += rhs.m10;
        self.m11 += rhs.m11;
    }
}

/// Subtraction operator (m1 - m2)
///
/// Component-wise subtraction of two matrices
impl Sub for Mat2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            m00: self.m00 - rhs.m00,
            m01: self.m01 - rhs.m01,
            m10: self.m10 - rhs.m10,
            m11: self.m11 - rhs.m11,
        }
    }
}

/// Subtraction assignment operator (m1 -= m2)
///
/// Modifies the matrix in-place by subtracting the components of rhs.
impl SubAssign for Mat2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.m00 -= rhs.m00;
        self.m01 -= rhs.m01;
        self.m10 -= rhs.m10;
        self.m11 -= rhs.m11;
    }
}

/******************* Mul/Div ******************/

/// Scalar multiplication operator (m * scalar)
///
/// Scales each component of the matrix by the given scalar value.
impl Mul<f32> for Mat2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            m00: rhs * self.m00,
            m01: rhs * self.m01,
            m10: rhs * self.m10,
            m11: rhs * self.m11,
        }
    }
}

/// Scalar multiplication assignment operator (m *= scalar)
///
/// Scales each component of the matrix by the given scalar value in place.
impl MulAssign<f32> for Mat2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.m00 *= rhs;
        self.m01 *= rhs;
        self.m10 *= rhs;
        self.m11 *= rhs;
    }
}

/// Scalar multiplication operator (scalar * m)
///
/// Allows scalar-matrix multiplication in both orders: `2.0 * m` and `m * 2.0`
impl Mul<Mat2> for f32 {
    type Output = Mat2;

    #[inline]
    fn mul(self, rhs: Mat2) -> Self::Output {
        rhs * self
    }
}

/// Scalar division operator (m / scalar)
///
/// Divides each component of the matrix by the given scalar value.
impl Div<f32> for Mat2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        debug_assert!(rhs != 0.0, "Division by zero in Mat2::div");

        let inv = 1.0 / rhs;
        Self {
            m00: self.m00 * inv,
            m01: self.m01 * inv,
            m10: self.m10 * inv,
            m11: self.m11 * inv,
        }
    }
}

/// Scalar division assignment operator (m /= scalar)
///
/// Divides each component of the matrix by the given scalar value in place.
impl DivAssign<f32> for Mat2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        debug_assert!(rhs != 0.0, "Division by zero in Mat2::div_assign");

        let inv = 1.0 / rhs;
        self.m00 *= inv;
        self.m01 *= inv;
        self.m10 *= inv;
        self.m11 *= inv;
    }
}

impl Mul<Mat2> for Mat2 {
    type Output = Self;

    /// Matrix multiplication operator (m1 * m2)
    ///
    /// Performs matrix multiplication between two 2x2 matrices.
    #[inline]
    fn mul(self, rhs: Mat2) -> Self::Output {
        Self {
            m00: self.m00 * rhs.m00 + self.m01 * rhs.m10,
            m01: self.m00 * rhs.m01 + self.m01 * rhs.m11,
            m10: self.m10 * rhs.m00 + self.m11 * rhs.m10,
            m11: self.m10 * rhs.m01 + self.m11 * rhs.m11,
        }
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;

    /// Matrix-vector multiplication operator (m * v)
    /// This convention is column-major,
    /// meaning the vector is treated as a column vector.
    ///
    /// Transforms a 2D vector by the matrix.
    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.m00 * rhs.x + self.m01 * rhs.y,
            y: self.m10 * rhs.x + self.m11 * rhs.y,
        }
    }
}

/******************* Mul/Div ******************/

impl Index<usize> for Mat2 {
    type Output = f32;

    /// Index into the matrix as a flat array in **row-major order**.
    ///
    /// Valid indices:
    /// * `0` → `m00`
    /// * `1` → `m01`
    /// * `2` → `m10`
    /// * `3` → `m11`
    ///
    /// # Panics
    /// Panics if the index is not in `0..4`.
    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.m00,
            1 => &self.m01,
            2 => &self.m10,
            3 => &self.m11,
            _ => panic!("Mat2 index out of range: {i}"),
        }
    }
}

impl IndexMut<usize> for Mat2 {
    /// Mutable index into the matrix as a flat array in **row-major order**.
    ///
    /// Valid indices:
    /// * `0` → `m00`
    /// * `1` → `m01`
    /// * `2` → `m10`
    /// * `3` → `m11`
    ///
    /// # Panics
    /// Panics if the index is not in `0..4`.
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.m00,
            1 => &mut self.m01,
            2 => &mut self.m10,
            3 => &mut self.m11,
            _ => panic!("Mat2 index out of range: {i}"),
        }
    }
}

impl Index<(usize, usize)> for Mat2 {
    type Output = f32;

    /// Index into the matrix by `(row, col)`.
    ///
    /// Valid indices (row, col):
    /// * `m(0,0)` → `m00`
    /// * `m(0,1)` → `m01`
    /// * `m(1,0)` → `m10`
    /// * `m(1,1)` → `m11`
    ///
    /// # Panics
    /// Panics if either row or col is not in `0..2`.
    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        match index {
            (0, 0) => &self.m00,
            (0, 1) => &self.m01,
            (1, 0) => &self.m10,
            (1, 1) => &self.m11,
            _ => panic!("Mat2 index out of range: {index:?}"),
        }
    }
}

impl IndexMut<(usize, usize)> for Mat2 {
    /// Mutable index into the matrix by `(row, col)`.
    ///
    /// Valid indices (row, col):
    /// * `m(0,0)` → `m00`
    /// * `m(0,1)` → `m01`
    /// * `m(1,0)` → `m10`
    /// * `m(1,1)` → `m11`
    ///
    /// # Panics
    /// Panics if either row or col is not in `0..2`.
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        match index {
            (0, 0) => &mut self.m00,
            (0, 1) => &mut self.m01,
            (1, 0) => &mut self.m10,
            (1, 1) => &mut self.m11,
            _ => panic!("Mat2 index out of range: {index:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn construction() {
        // Test basic construction
        let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(m.m00, 1.0);
        assert_eq!(m.m01, 2.0);
        assert_eq!(m.m10, 3.0);
        assert_eq!(m.m11, 4.0);

        // Test constants
        assert_eq!(Mat2::IDENTITY, Mat2::new(1.0, 0.0, 0.0, 1.0));
        assert_eq!(Mat2::ZERO, Mat2::new(0.0, 0.0, 0.0, 0.0));

        // Test from_rows
        let row0 = Vec2::new(1.0, 2.0);
        let row1 = Vec2::new(3.0, 4.0);
        let m = Mat2::from_rows(row0, row1);
        assert_eq!(m, Mat2::new(1.0, 2.0, 3.0, 4.0));

        // Test from_cols
        let col0 = Vec2::new(1.0, 3.0);
        let col1 = Vec2::new(2.0, 4.0);
        let m = Mat2::from_cols(col0, col1);
        assert_eq!(m, Mat2::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn arithmetic_operations() {
        let a = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let b = Mat2::new(5.0, 6.0, 7.0, 8.0);

        // Addition
        assert_eq!(a + b, Mat2::new(6.0, 8.0, 10.0, 12.0));

        // Subtraction
        assert_eq!(a - b, Mat2::new(-4.0, -4.0, -4.0, -4.0));

        // Scalar multiplication
        assert_eq!(a * 2.0, Mat2::new(2.0, 4.0, 6.0, 8.0));
        assert_eq!(2.0 * a, Mat2::new(2.0, 4.0, 6.0, 8.0));

        // Scalar division
        assert_eq!(a / 2.0, Mat2::new(0.5, 1.0, 1.5, 2.0));

        // Negation
        assert_eq!(-a, Mat2::new(-1.0, -2.0, -3.0, -4.0));
    }

    #[test]
    fn assignment_operators() {
        let mut m = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let other = Mat2::new(5.0, 6.0, 7.0, 8.0);

        // Add assign
        m += other;
        assert_eq!(m, Mat2::new(6.0, 8.0, 10.0, 12.0));

        // Sub assign
        m -= other;
        assert_eq!(m, Mat2::new(1.0, 2.0, 3.0, 4.0));

        // Mul assign
        m *= 2.0;
        assert_eq!(m, Mat2::new(2.0, 4.0, 6.0, 8.0));

        // Div assign
        m /= 2.0;
        assert_eq!(m, Mat2::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn matrix_operations() {
        let a = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let b = Mat2::new(5.0, 6.0, 7.0, 8.0);

        // Matrix multiplication
        let result = a * b;
        // [1 2]   [5 6]   [1*5+2*7  1*6+2*8]   [19 22]
        // [3 4] × [7 8] = [3*5+4*7  3*6+4*8] = [43 50]
        assert_eq!(result, Mat2::new(19.0, 22.0, 43.0, 50.0));

        // Determinant
        assert_eq!(a.det(), 1.0 * 4.0 - 2.0 * 3.0); // 4 - 6 = -2

        // Trace
        assert_eq!(a.trace(), 1.0 + 4.0); // 5

        // Transpose
        let transposed = a.transpose();
        assert_eq!(transposed, Mat2::new(1.0, 3.0, 2.0, 4.0));

        // Frobenius norm
        let norm = a.frobenius_norm();
        let expected = (1.0_f32 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0).sqrt();
        assert!((norm - expected).abs() < 1e-6);
    }

    #[test]
    fn inverse() {
        // Test invertible matrix
        let m = Mat2::new(2.0, 1.0, 1.0, 1.0);
        let inv = m.inverse();

        // Verify: m * inv = identity
        let identity_check = m * inv;
        assert!(identity_check.is_identity_eps(1e-6));

        // Test identity matrix inverse
        let identity_inv = Mat2::IDENTITY.inverse();
        assert_eq!(identity_inv, Mat2::IDENTITY);
    }

    #[test]
    fn transformations() {
        // Test rotation
        let rotation = Mat2::rotate(PI / 2.0); // 90 degrees
        let v = Vec2::new(1.0, 0.0);
        let rotated = rotation * v;
        assert!(rotated.near(Vec2::new(0.0, 1.0), 1e-6));

        // Test scaling
        let scale = Mat2::scale(2.0, 3.0);
        let v = Vec2::new(1.0, 1.0);
        let scaled = scale * v;
        assert_eq!(scaled, Vec2::new(2.0, 3.0));

        // Test uniform scaling
        let uniform_scale = Mat2::scale_uniform(2.0);
        let v = Vec2::new(1.0, 1.0);
        let scaled = uniform_scale * v;
        assert_eq!(scaled, Vec2::new(2.0, 2.0));

        // Test shearing
        let shear = Mat2::shear(0.5, 0.0); // horizontal shear
        let v = Vec2::new(1.0, 1.0);
        let sheared = shear * v;
        assert_eq!(sheared, Vec2::new(1.5, 1.0));
    }

    #[test]
    fn vector_transformation() {
        let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let v = Vec2::new(1.0, 2.0);
        let result = m * v;

        // [1 2]   [1]   [1*1+2*2]   [5]
        // [3 4] × [2] = [3*1+4*2] = [11]
        assert_eq!(result, Vec2::new(5.0, 11.0));
    }

    #[test]
    fn indexing() {
        let mut m = Mat2::new(1.0, 2.0, 3.0, 4.0);

        // Test flat indexing
        assert_eq!(m[0], 1.0); // m00
        assert_eq!(m[1], 2.0); // m01
        assert_eq!(m[2], 3.0); // m10
        assert_eq!(m[3], 4.0); // m11

        // Test (row, col) indexing
        assert_eq!(m[(0, 0)], 1.0); // m00
        assert_eq!(m[(0, 1)], 2.0); // m01
        assert_eq!(m[(1, 0)], 3.0); // m10
        assert_eq!(m[(1, 1)], 4.0); // m11

        // Test mutable indexing
        m[0] = 10.0;
        assert_eq!(m[0], 10.0);

        m[(1, 1)] = 20.0;
        assert_eq!(m[(1, 1)], 20.0);
    }

    #[test]
    fn utility_methods() {
        // Test is_identity
        assert!(Mat2::IDENTITY.is_identity());
        assert!(!Mat2::ZERO.is_identity());

        // Test is_identity_eps
        let almost_identity = Mat2::new(1.0001, 0.0001, 0.0001, 1.0001);
        assert!(almost_identity.is_identity_eps(0.001));
        assert!(!almost_identity.is_identity_eps(0.00001));

        // Test is_zero
        assert!(Mat2::ZERO.is_zero());
        assert!(!Mat2::IDENTITY.is_zero());

        // Test is_zero_eps
        let almost_zero = Mat2::new(0.0001, 0.0001, 0.0001, 0.0001);
        assert!(almost_zero.is_zero_eps(0.001));
        assert!(!almost_zero.is_zero_eps(0.00001));

        // Test near
        let a = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let b = Mat2::new(1.0001, 2.0001, 3.0001, 4.0001);
        assert!(a.near(b, 0.001));
        assert!(!a.near(b, 0.00001));
    }

    #[test]
    fn array_conversion() {
        let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let arr = m.to_array();
        assert_eq!(arr, [1.0, 2.0, 3.0, 4.0]);

        let m2 = Mat2::from_array([5.0, 6.0, 7.0, 8.0]);
        assert_eq!(m2, Mat2::new(5.0, 6.0, 7.0, 8.0));

        // Round-trip test
        let m3 = Mat2::from_array(m.to_array());
        assert_eq!(m, m3);
    }

    #[test]
    fn interpolation() {
        let a = Mat2::new(1.0, 0.0, 0.0, 1.0); // identity
        let b = Mat2::new(2.0, 0.0, 0.0, 2.0); // scale by 2

        // Test lerp
        let lerped = a.lerp(b, 0.5);
        assert_eq!(lerped, Mat2::new(1.5, 0.0, 0.0, 1.5));

        // Test endpoints
        assert_eq!(a.lerp(b, 0.0), a);
        assert_eq!(a.lerp(b, 1.0), b);
    }

    #[test]
    fn angle_extraction() {
        // Test 0 degree rotation
        let m0 = Mat2::rotate(0.0);
        assert!((m0.angle() - 0.0).abs() < 1e-6);

        // Test 90 degree rotation
        let m90 = Mat2::rotate(PI / 2.0);
        assert!((m90.angle() - PI / 2.0).abs() < 1e-6);

        // Test 180 degree rotation (atan2 can return -π or π, both are equivalent)
        let m180 = Mat2::rotate(PI);
        let angle = m180.angle();
        assert!((angle - PI).abs() < 1e-6 || (angle + PI).abs() < 1e-6);
    }

    #[test]
    fn slerp() {
        let a = Mat2::rotate(0.0);
        let b = Mat2::rotate(PI / 2.0);

        // Test endpoints
        let result0 = a.slerp(b, 0.0);
        assert!(result0.near(a, 1e-6));

        let result1 = a.slerp(b, 1.0);
        assert!(result1.near(b, 1e-6));

        // Test midpoint
        let result_mid = a.slerp(b, 0.5);
        let expected = Mat2::rotate(PI / 4.0);
        assert!(result_mid.near(expected, 1e-6));
    }

    #[test]
    fn mathematical_properties() {
        let a = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let b = Mat2::new(5.0, 6.0, 7.0, 8.0);
        let c = Mat2::new(9.0, 10.0, 11.0, 12.0);

        // Associativity of matrix multiplication
        assert_eq!((a * b) * c, a * (b * c));

        // Distributivity
        assert_eq!(a * (b + c), a * b + a * c);
        assert_eq!((a + b) * c, a * c + b * c);

        // Identity multiplication
        assert_eq!(a * Mat2::IDENTITY, a);
        assert_eq!(Mat2::IDENTITY * a, a);

        // Zero multiplication
        assert_eq!(a * Mat2::ZERO, Mat2::ZERO);
        assert_eq!(Mat2::ZERO * a, Mat2::ZERO);

        // Scalar multiplication properties
        let k = 2.0;
        assert_eq!(k * (a * b), (k * a) * b);
        assert_eq!(k * (a * b), a * (k * b));
    }

    #[test]
    fn edge_cases() {
        // Test very small numbers
        let tiny = Mat2::new(1e-10, 1e-10, 1e-10, 1e-10);
        assert!(tiny.frobenius_norm() > 0.0);

        // Test very large numbers
        let huge = Mat2::new(1e10, 1e10, 1e10, 1e10);
        assert!(huge.frobenius_norm() > 0.0);

        // Test negative zero
        let neg_zero = Mat2::new(-0.0, -0.0, -0.0, -0.0);
        assert!(neg_zero.is_zero());
    }
}
