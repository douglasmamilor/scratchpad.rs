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
