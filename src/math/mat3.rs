use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::math::vec3::Vec3;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Mat3 {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
}

impl Mat3 {
    /// Zero matrix (all elements are 0)
    pub const ZERO: Self = Self {
        m00: 0.0,
        m01: 0.0,
        m02: 0.0,
        m10: 0.0,
        m11: 0.0,
        m12: 0.0,
        m20: 0.0,
        m21: 0.0,
        m22: 0.0,
    };

    /// Identity matrix (diagonal elements are 1, others are 0)
    pub const IDENTITY: Self = Self {
        m00: 1.0,
        m01: 0.0,
        m02: 0.0,
        m10: 0.0,
        m11: 1.0,
        m12: 0.0,
        m20: 0.0,
        m21: 0.0,
        m22: 1.0,
    };

    /// Create a new matrix from elements
    #[inline]
    pub fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m20: f32,
        m21: f32,
        m22: f32,
    ) -> Self {
        Self {
            m00,
            m01,
            m02,
            m10,
            m11,
            m12,
            m20,
            m21,
            m22,
        }
    }

    /// Create a matrix from row vectors
    #[inline]
    pub fn from_rows(row0: Vec3, row1: Vec3, row2: Vec3) -> Self {
        Self {
            m00: row0.x,
            m01: row0.y,
            m02: row0.z,
            m10: row1.x,
            m11: row1.y,
            m12: row1.z,
            m20: row2.x,
            m21: row2.y,
            m22: row2.z,
        }
    }

    /// Create a matrix from column vectors
    #[inline]
    pub fn from_cols(col0: Vec3, col1: Vec3, col2: Vec3) -> Self {
        Self {
            m00: col0.x,
            m01: col1.x,
            m02: col2.x,
            m10: col0.y,
            m11: col1.y,
            m12: col2.y,
            m20: col0.z,
            m21: col1.z,
            m22: col2.z,
        }
    }

    /// Calculate the determinant of the matrix
    ///
    /// Uses the Sarrus rule for 3×3 matrices:
    /// det = m00*(m11*m22 - m12*m21) - m01*(m10*m22 - m12*m20) + m02*(m10*m21 - m11*m20)
    ///
    /// The determinant represents the scaling factor of the linear transformation
    /// and indicates whether the matrix is invertible (det ≠ 0).
    #[inline]
    pub fn det(self) -> f32 {
        self.m00 * (self.m11 * self.m22 - self.m12 * self.m21)
            - self.m01 * (self.m10 * self.m22 - self.m12 * self.m20)
            + self.m02 * (self.m10 * self.m21 - self.m11 * self.m20)
    }
}

/******************* Unary ******************/

/// Negation operator (-m)
///
/// Returns a new matrix with each component negated.
impl Neg for Mat3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            m00: -self.m00,
            m01: -self.m01,
            m02: -self.m02,
            m10: -self.m10,
            m11: -self.m11,
            m12: -self.m12,
            m20: -self.m20,
            m21: -self.m21,
            m22: -self.m22,
        }
    }
}

/******************* Add/Sub ******************/

/// Addition operator (m1 + m2)
///
/// Component-wise addition of two matrices
impl Add for Mat3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            m00: self.m00 + rhs.m00,
            m01: self.m01 + rhs.m01,
            m02: self.m02 + rhs.m02,
            m10: self.m10 + rhs.m10,
            m11: self.m11 + rhs.m11,
            m12: self.m12 + rhs.m12,
            m20: self.m20 + rhs.m20,
            m21: self.m21 + rhs.m21,
            m22: self.m22 + rhs.m22,
        }
    }
}

/// Addition assignment operator (m1 += m2)
///
/// Modifies the matrix in-place by adding the components of rhs.
impl AddAssign for Mat3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.m00 += rhs.m00;
        self.m01 += rhs.m01;
        self.m02 += rhs.m02;
        self.m10 += rhs.m10;
        self.m11 += rhs.m11;
        self.m12 += rhs.m12;
        self.m20 += rhs.m20;
        self.m21 += rhs.m21;
        self.m22 += rhs.m22;
    }
}

/// Subtraction operator (m1 - m2)
///
/// Component-wise subtraction of two matrices
impl Sub for Mat3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            m00: self.m00 - rhs.m00,
            m01: self.m01 - rhs.m01,
            m02: self.m02 - rhs.m02,
            m10: self.m10 - rhs.m10,
            m11: self.m11 - rhs.m11,
            m12: self.m12 - rhs.m12,
            m20: self.m20 - rhs.m20,
            m21: self.m21 - rhs.m21,
            m22: self.m22 - rhs.m22,
        }
    }
}

/// Subtraction assignment operator (m1 -= m2)
///
/// Modifies the matrix in-place by subtracting the components of rhs.
impl SubAssign for Mat3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.m00 -= rhs.m00;
        self.m01 -= rhs.m01;
        self.m02 -= rhs.m02;
        self.m10 -= rhs.m10;
        self.m11 -= rhs.m11;
        self.m12 -= rhs.m12;
        self.m20 -= rhs.m20;
        self.m21 -= rhs.m21;
        self.m22 -= rhs.m22;
    }
}

/******************* Mul/Div ******************/

/// Scalar multiplication operator (m * scalar)
///
/// Scales each component of the matrix by the given scalar value.
impl Mul<f32> for Mat3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            m00: rhs * self.m00,
            m01: rhs * self.m01,
            m02: rhs * self.m02,
            m10: rhs * self.m10,
            m11: rhs * self.m11,
            m12: rhs * self.m12,
            m20: rhs * self.m20,
            m21: rhs * self.m21,
            m22: rhs * self.m22,
        }
    }
}

/// Scalar multiplication assignment operator (m *= scalar)
///
/// Scales each component of the matrix by the given scalar value in place.
impl MulAssign<f32> for Mat3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.m00 *= rhs;
        self.m01 *= rhs;
        self.m02 *= rhs;
        self.m10 *= rhs;
        self.m11 *= rhs;
        self.m12 *= rhs;
        self.m20 *= rhs;
        self.m21 *= rhs;
        self.m22 *= rhs;
    }
}

/// Scalar multiplication operator (scalar * m)
///
/// Allows scalar-matrix multiplication in both orders: `2.0 * m` and `m * 2.0`
impl Mul<Mat3> for f32 {
    type Output = Mat3;

    #[inline]
    fn mul(self, rhs: Mat3) -> Self::Output {
        rhs * self
    }
}

/// Scalar division operator (m / scalar)
///
/// Divides each component of the matrix by the given scalar value.
impl Div<f32> for Mat3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        debug_assert!(rhs != 0.0, "Division by zero in Mat3::div");

        let inv = 1.0 / rhs;
        Self {
            m00: self.m00 * inv,
            m01: self.m01 * inv,
            m02: self.m02 * inv,
            m10: self.m10 * inv,
            m11: self.m11 * inv,
            m12: self.m12 * inv,
            m20: self.m20 * inv,
            m21: self.m21 * inv,
            m22: self.m22 * inv,
        }
    }
}

/// Scalar division assignment operator (m /= scalar)
///
/// Divides each component of the matrix by the given scalar value in place.
impl DivAssign<f32> for Mat3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        debug_assert!(rhs != 0.0, "Division by zero in Mat3::div_assign");

        let inv = 1.0 / rhs;
        self.m00 *= inv;
        self.m01 *= inv;
        self.m02 *= inv;
        self.m10 *= inv;
        self.m11 *= inv;
        self.m12 *= inv;
        self.m20 *= inv;
        self.m21 *= inv;
        self.m22 *= inv;
    }
}

/******************* Indexing ******************/

impl Index<usize> for Mat3 {
    type Output = f32;

    /// Index into the matrix as a flat array in **row-major order**.
    ///
    /// Valid indices:
    /// * `0` → `m00`
    /// * `1` → `m01`
    /// * `2` → `m02`
    /// * `3` → `m10`
    /// * `4` → `m11`
    /// * `5` → `m12`
    /// * `6` → `m20`
    /// * `7` → `m21`
    /// * `8` → `m22`
    ///
    /// # Panics
    /// Panics if the index is not in `0..9`.
    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.m00,
            1 => &self.m01,
            2 => &self.m02,
            3 => &self.m10,
            4 => &self.m11,
            5 => &self.m12,
            6 => &self.m20,
            7 => &self.m21,
            8 => &self.m22,
            _ => panic!("Mat3 index out of range: {i}"),
        }
    }
}

impl IndexMut<usize> for Mat3 {
    /// Mutable index into the matrix as a flat array in **row-major order**.
    ///
    /// Valid indices:
    /// * `0` → `m00`
    /// * `1` → `m01`
    /// * `2` → `m02`
    /// * `3` → `m10`
    /// * `4` → `m11`
    /// * `5` → `m12`
    /// * `6` → `m20`
    /// * `7` → `m21`
    /// * `8` → `m22`
    ///
    /// # Panics
    /// Panics if the index is not in `0..9`.
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.m00,
            1 => &mut self.m01,
            2 => &mut self.m02,
            3 => &mut self.m10,
            4 => &mut self.m11,
            5 => &mut self.m12,
            6 => &mut self.m20,
            7 => &mut self.m21,
            8 => &mut self.m22,
            _ => panic!("Mat3 index out of range: {i}"),
        }
    }
}

impl Index<(usize, usize)> for Mat3 {
    type Output = f32;

    /// Index into the matrix by `(row, col)`.
    ///
    /// Valid indices (row, col):
    /// * `m(0,0)` → `m00`
    /// * `m(0,1)` → `m01`
    /// * `m(0,2)` → `m02`
    /// * `m(1,0)` → `m10`
    /// * `m(1,1)` → `m11`
    /// * `m(1,2)` → `m12`
    /// * `m(2,0)` → `m20`
    /// * `m(2,1)` → `m21`
    /// * `m(2,2)` → `m22`
    ///
    /// # Panics
    /// Panics if either row or col is not in `0..3`.
    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        match index {
            (0, 0) => &self.m00,
            (0, 1) => &self.m01,
            (0, 2) => &self.m02,
            (1, 0) => &self.m10,
            (1, 1) => &self.m11,
            (1, 2) => &self.m12,
            (2, 0) => &self.m20,
            (2, 1) => &self.m21,
            (2, 2) => &self.m22,
            _ => panic!("Mat3 index out of range: {index:?}"),
        }
    }
}

impl IndexMut<(usize, usize)> for Mat3 {
    /// Mutable index into the matrix by `(row, col)`.
    ///
    /// Valid indices (row, col):
    /// * `m(0,0)` → `m00`
    /// * `m(0,1)` → `m01`
    /// * `m(0,2)` → `m02`
    /// * `m(1,0)` → `m10`
    /// * `m(1,1)` → `m11`
    /// * `m(1,2)` → `m12`
    /// * `m(2,0)` → `m20`
    /// * `m(2,1)` → `m21`
    /// * `m(2,2)` → `m22`
    ///
    /// # Panics
    /// Panics if either row or col is not in `0..3`.
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        match index {
            (0, 0) => &mut self.m00,
            (0, 1) => &mut self.m01,
            (0, 2) => &mut self.m02,
            (1, 0) => &mut self.m10,
            (1, 1) => &mut self.m11,
            (1, 2) => &mut self.m12,
            (2, 0) => &mut self.m20,
            (2, 1) => &mut self.m21,
            (2, 2) => &mut self.m22,
            _ => panic!("Mat3 index out of range: {index:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        // Test basic construction
        let m = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m.m00, 1.0);
        assert_eq!(m.m01, 2.0);
        assert_eq!(m.m02, 3.0);
        assert_eq!(m.m10, 4.0);
        assert_eq!(m.m11, 5.0);
        assert_eq!(m.m12, 6.0);
        assert_eq!(m.m20, 7.0);
        assert_eq!(m.m21, 8.0);
        assert_eq!(m.m22, 9.0);

        // Test constants
        assert_eq!(
            Mat3::IDENTITY,
            Mat3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,)
        );
        assert_eq!(
            Mat3::ZERO,
            Mat3::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
        );

        // Test from_rows
        let row0 = Vec3::new(1.0, 2.0, 3.0);
        let row1 = Vec3::new(4.0, 5.0, 6.0);
        let row2 = Vec3::new(7.0, 8.0, 9.0);
        let m = Mat3::from_rows(row0, row1, row2);
        assert_eq!(m, Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,));

        // Test from_cols
        let col0 = Vec3::new(1.0, 4.0, 7.0);
        let col1 = Vec3::new(2.0, 5.0, 8.0);
        let col2 = Vec3::new(3.0, 6.0, 9.0);
        let m = Mat3::from_cols(col0, col1, col2);
        assert_eq!(m, Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,));
    }

    #[test]
    fn arithmetic_operations() {
        let a = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let b = Mat3::new(10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0);

        // Addition
        assert_eq!(
            a + b,
            Mat3::new(11.0, 13.0, 15.0, 17.0, 19.0, 21.0, 23.0, 25.0, 27.0,)
        );

        // Subtraction
        assert_eq!(
            a - b,
            Mat3::new(-9.0, -9.0, -9.0, -9.0, -9.0, -9.0, -9.0, -9.0, -9.0,)
        );

        // Scalar multiplication
        assert_eq!(
            a * 2.0,
            Mat3::new(2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0,)
        );
        assert_eq!(
            2.0 * a,
            Mat3::new(2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0,)
        );

        // Scalar division
        assert_eq!(
            a / 2.0,
            Mat3::new(0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5,)
        );

        // Negation
        assert_eq!(
            -a,
            Mat3::new(-1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0, -9.0,)
        );
    }

    #[test]
    fn assignment_operators() {
        let mut m = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let other = Mat3::new(10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0);

        // Add assign
        m += other;
        assert_eq!(
            m,
            Mat3::new(11.0, 13.0, 15.0, 17.0, 19.0, 21.0, 23.0, 25.0, 27.0,)
        );

        // Sub assign
        m -= other;
        assert_eq!(m, Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,));

        // Mul assign
        m *= 2.0;
        assert_eq!(
            m,
            Mat3::new(2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0,)
        );

        // Div assign
        m /= 2.0;
        assert_eq!(m, Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,));
    }

    #[test]
    fn indexing() {
        let mut m = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);

        // Test flat indexing
        assert_eq!(m[0], 1.0); // m00
        assert_eq!(m[1], 2.0); // m01
        assert_eq!(m[2], 3.0); // m02
        assert_eq!(m[3], 4.0); // m10
        assert_eq!(m[4], 5.0); // m11
        assert_eq!(m[5], 6.0); // m12
        assert_eq!(m[6], 7.0); // m20
        assert_eq!(m[7], 8.0); // m21
        assert_eq!(m[8], 9.0); // m22

        // Test (row, col) indexing
        assert_eq!(m[(0, 0)], 1.0); // m00
        assert_eq!(m[(0, 1)], 2.0); // m01
        assert_eq!(m[(0, 2)], 3.0); // m02
        assert_eq!(m[(1, 0)], 4.0); // m10
        assert_eq!(m[(1, 1)], 5.0); // m11
        assert_eq!(m[(1, 2)], 6.0); // m12
        assert_eq!(m[(2, 0)], 7.0); // m20
        assert_eq!(m[(2, 1)], 8.0); // m21
        assert_eq!(m[(2, 2)], 9.0); // m22

        // Test mutable indexing
        m[0] = 10.0;
        assert_eq!(m[0], 10.0);

        m[(1, 1)] = 20.0;
        assert_eq!(m[(1, 1)], 20.0);
    }

    #[test]
    fn determinant() {
        // Test identity matrix (determinant = 1)
        let identity = Mat3::IDENTITY;
        assert_eq!(identity.det(), 1.0);

        // Test zero matrix (determinant = 0)
        let zero = Mat3::ZERO;
        assert_eq!(zero.det(), 0.0);

        // Test simple 3x3 matrix with known determinant
        // [1 2 3]
        // [4 5 6]  det = 1*(5*9-6*8) - 2*(4*9-6*7) + 3*(4*8-5*7)
        // [7 8 9]     = 1*(45-48) - 2*(36-42) + 3*(32-35)
        //              = 1*(-3) - 2*(-6) + 3*(-3)
        //              = -3 + 12 - 9 = 0
        let matrix = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(matrix.det(), 0.0);

        // Test matrix with non-zero determinant
        // [2 1 0]
        // [1 2 1]  det = 2*(2*1-1*1) - 1*(1*1-1*0) + 0*(1*1-2*0)
        // [0 1 1]     = 2*(2-1) - 1*(1-0) + 0*(1-0)
        //              = 2*1 - 1*1 + 0*1 = 2 - 1 = 1
        let matrix2 = Mat3::new(2.0, 1.0, 0.0, 1.0, 2.0, 1.0, 0.0, 1.0, 1.0);
        assert_eq!(matrix2.det(), 1.0);

        // Test scaling matrix (determinant = product of diagonal elements)
        let scale = Mat3::new(2.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 4.0);
        assert_eq!(scale.det(), 2.0 * 3.0 * 4.0); // 24

        // Test singular matrix (determinant = 0)
        // Two identical rows
        let singular = Mat3::new(
            1.0, 2.0, 3.0, 1.0, 2.0, 3.0, // Same as first row
            4.0, 5.0, 6.0,
        );
        assert_eq!(singular.det(), 0.0);

        // Test negative determinant
        let negative = Mat3::new(-1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(negative.det(), -1.0);
    }
}

