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
