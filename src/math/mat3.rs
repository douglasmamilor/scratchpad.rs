use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;

/// Decomposition result containing translation, rotation, and scale components
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Decomposition {
    /// Translation component (tx, ty)
    pub translation: Vec2,
    /// Rotation angle in radians
    pub rotation: f32,
    /// Scale factors (sx, sy)
    pub scale: Vec2,
}

/// Affine decomposition result with potential shear
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AffineDecomposition {
    /// Translation component (tx, ty)
    pub translation: Vec2,
    /// Rotation angle in radians
    pub rotation: f32,
    /// Scale factors (sx, sy)
    pub scale: Vec2,
    /// Shear factors (shx, shy)
    pub shear: Vec2,
}

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

    /// Calculate the transpose of the matrix
    ///
    /// Swaps rows and columns: transpose[i,j] = original[j,i]
    ///
    /// For a 3×3 matrix, this swaps:
    /// - m01 ↔ m10
    /// - m02 ↔ m20
    /// - m12 ↔ m21
    ///
    /// The diagonal elements (m00, m11, m22) remain unchanged.
    #[inline]
    pub fn transpose(self) -> Self {
        Self {
            m00: self.m00,
            m01: self.m10,
            m02: self.m20,
            m10: self.m01,
            m11: self.m11,
            m12: self.m21,
            m20: self.m02,
            m21: self.m12,
            m22: self.m22,
        }
    }

    /// Calculate the inverse of the matrix
    ///
    /// Uses the adjugate method: inverse = (1/det) * adjugate
    ///
    /// The adjugate is the transpose of the cofactor matrix, where each cofactor
    /// is the determinant of the 2×2 submatrix formed by removing the row and
    /// column of that element, multiplied by (-1)^(i+j).
    ///
    /// # Returns
    /// Returns `None` if the matrix is singular (determinant ≈ 0).
    #[inline]
    pub fn inverse(self) -> Option<Self> {
        let det = self.det();

        // Check for singularity
        if det.abs() < 1e-6 {
            return None;
        }

        let inv_det = 1.0 / det;

        // Calculate adjugate matrix (transpose of cofactor matrix)
        let adj = Self {
            m00: (self.m11 * self.m22 - self.m12 * self.m21) * inv_det,
            m01: (self.m02 * self.m21 - self.m01 * self.m22) * inv_det,
            m02: (self.m01 * self.m12 - self.m02 * self.m11) * inv_det,
            m10: (self.m12 * self.m20 - self.m10 * self.m22) * inv_det,
            m11: (self.m00 * self.m22 - self.m02 * self.m20) * inv_det,
            m12: (self.m02 * self.m10 - self.m00 * self.m12) * inv_det,
            m20: (self.m10 * self.m21 - self.m11 * self.m20) * inv_det,
            m21: (self.m01 * self.m20 - self.m00 * self.m21) * inv_det,
            m22: (self.m00 * self.m11 - self.m01 * self.m10) * inv_det,
        };

        Some(adj)
    }

    /// Calculate the Frobenius norm of the matrix
    ///
    /// The Frobenius norm is the square root of the sum of squares of all elements:
    /// ||A||_F = sqrt(sum(|a_ij|^2))
    ///
    /// This is useful for measuring the "size" of a matrix and for approximate equality tests.
    #[inline]
    pub fn frobenius_norm(self) -> f32 {
        (self.m00 * self.m00
            + self.m01 * self.m01
            + self.m02 * self.m02
            + self.m10 * self.m10
            + self.m11 * self.m11
            + self.m12 * self.m12
            + self.m20 * self.m20
            + self.m21 * self.m21
            + self.m22 * self.m22)
            .sqrt()
    }

    /// Check if this matrix is the identity matrix
    ///
    /// Returns `true` if all diagonal elements are 1.0 and all off-diagonal elements are 0.0.
    #[inline]
    pub fn is_identity(self) -> bool {
        self == Self::IDENTITY
    }

    /// Check if this matrix is approximately the identity matrix
    ///
    /// Uses epsilon comparison for floating-point precision issues.
    ///
    /// # Arguments
    /// * `eps` - Tolerance for comparison (default: 1e-6)
    #[inline]
    pub fn is_identity_eps(self, eps: f32) -> bool {
        (self.m00 - 1.0).abs() < eps
            && self.m01.abs() < eps
            && self.m02.abs() < eps
            && self.m10.abs() < eps
            && (self.m11 - 1.0).abs() < eps
            && self.m12.abs() < eps
            && self.m20.abs() < eps
            && self.m21.abs() < eps
            && (self.m22 - 1.0).abs() < eps
    }

    /// Check if this matrix is the zero matrix
    ///
    /// Returns `true` if all elements are 0.0.
    #[inline]
    pub fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    /// Check if this matrix is approximately the zero matrix
    ///
    /// Uses epsilon comparison for floating-point precision issues.
    ///
    /// # Arguments
    /// * `eps` - Tolerance for comparison (default: 1e-6)
    #[inline]
    pub fn is_zero_eps(self, eps: f32) -> bool {
        self.m00.abs() < eps
            && self.m01.abs() < eps
            && self.m02.abs() < eps
            && self.m10.abs() < eps
            && self.m11.abs() < eps
            && self.m12.abs() < eps
            && self.m20.abs() < eps
            && self.m21.abs() < eps
            && self.m22.abs() < eps
    }

    /// Check if this matrix is approximately equal to another matrix
    ///
    /// Uses epsilon comparison for floating-point precision issues.
    ///
    /// # Arguments
    /// * `other` - The matrix to compare against
    /// * `eps` - Tolerance for comparison (default: 1e-6)
    #[inline]
    pub fn near(self, other: Self, eps: f32) -> bool {
        (self.m00 - other.m00).abs() < eps
            && (self.m01 - other.m01).abs() < eps
            && (self.m02 - other.m02).abs() < eps
            && (self.m10 - other.m10).abs() < eps
            && (self.m11 - other.m11).abs() < eps
            && (self.m12 - other.m12).abs() < eps
            && (self.m20 - other.m20).abs() < eps
            && (self.m21 - other.m21).abs() < eps
            && (self.m22 - other.m22).abs() < eps
    }

    /// Calculate the trace of the matrix
    ///
    /// The trace is the sum of the diagonal elements: trace = m00 + m11 + m22
    ///
    /// This is useful for various matrix properties and eigenvalue calculations.
    #[inline]
    pub fn trace(self) -> f32 {
        self.m00 + self.m11 + self.m22
    }

    /// Convert the matrix to a flat array in row-major order
    ///
    /// Returns a 9-element array where elements are arranged as:
    /// [m00, m01, m02, m10, m11, m12, m20, m21, m22]
    ///
    /// This is useful for passing matrices to graphics APIs or serialization.
    #[inline]
    pub fn to_array(self) -> [f32; 9] {
        [
            self.m00, self.m01, self.m02, self.m10, self.m11, self.m12, self.m20, self.m21,
            self.m22,
        ]
    }

    /// Create a matrix from a flat array in row-major order
    ///
    /// Expects a 9-element array arranged as:
    /// [m00, m01, m02, m10, m11, m12, m20, m21, m22]
    ///
    /// # Arguments
    /// * `arr` - Array of 9 elements in row-major order
    #[inline]
    pub fn from_array(arr: [f32; 9]) -> Self {
        Self {
            m00: arr[0],
            m01: arr[1],
            m02: arr[2],
            m10: arr[3],
            m11: arr[4],
            m12: arr[5],
            m20: arr[6],
            m21: arr[7],
            m22: arr[8],
        }
    }

    /// Convert the matrix to a slice of 9 elements in row-major order
    ///
    /// Returns a slice reference to the matrix elements arranged as:
    /// [m00, m01, m02, m10, m11, m12, m20, m21, m22]
    ///
    /// This is useful for passing matrices to functions that expect slices.
    #[inline]
    pub fn as_slice(&self) -> &[f32; 9] {
        unsafe { std::mem::transmute(self) }
    }

    /// Convert the matrix to a mutable slice of 9 elements in row-major order
    ///
    /// Returns a mutable slice reference to the matrix elements arranged as:
    /// [m00, m01, m02, m10, m11, m12, m20, m21, m22]
    ///
    /// This is useful for modifying matrix elements through slice operations.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [f32; 9] {
        unsafe { std::mem::transmute(self) }
    }

    /// Create a matrix from a slice of 9 elements in row-major order
    ///
    /// Expects a slice with exactly 9 elements arranged as:
    /// [m00, m01, m02, m10, m11, m12, m20, m21, m22]
    ///
    /// # Arguments
    /// * `slice` - Slice of 9 elements in row-major order
    ///
    /// # Panics
    /// Panics if the slice doesn't have exactly 9 elements.
    #[inline]
    pub fn from_slice(slice: &[f32]) -> Self {
        assert_eq!(slice.len(), 9, "Slice must have exactly 9 elements");
        Self {
            m00: slice[0],
            m01: slice[1],
            m02: slice[2],
            m10: slice[3],
            m11: slice[4],
            m12: slice[5],
            m20: slice[6],
            m21: slice[7],
            m22: slice[8],
        }
    }

    /// Create a translation matrix
    ///
    /// Creates a matrix that translates points by (tx, ty).
    /// The translation is applied to the homogeneous coordinate (w=1).
    ///
    /// # Arguments
    /// * `tx` - Translation in X direction
    /// * `ty` - Translation in Y direction
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::mat3::Mat3;
    /// let translate = Mat3::translate(10.0, 20.0);
    /// // This matrix will move points by (10, 20)
    /// ```
    #[inline]
    pub fn translate(tx: f32, ty: f32) -> Self {
        Self {
            m00: 1.0,
            m01: 0.0,
            m02: tx,
            m10: 0.0,
            m11: 1.0,
            m12: ty,
            m20: 0.0,
            m21: 0.0,
            m22: 1.0,
        }
    }

    /// Create a rotation matrix
    ///
    /// Creates a matrix that rotates points around the origin by the given angle.
    /// Positive angles rotate counter-clockwise.
    ///
    /// # Arguments
    /// * `angle` - Rotation angle in radians
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::mat3::Mat3;
    /// use std::f32::consts::PI;
    /// let rotate = Mat3::rotate(PI / 4.0); // 45 degrees
    /// ```
    #[inline]
    pub fn rotate(angle: f32) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Self {
            m00: cos_a,
            m01: -sin_a,
            m02: 0.0,
            m10: sin_a,
            m11: cos_a,
            m12: 0.0,
            m20: 0.0,
            m21: 0.0,
            m22: 1.0,
        }
    }

    /// Create a scaling matrix
    ///
    /// Creates a matrix that scales points by (sx, sy).
    /// Scaling is applied around the origin.
    ///
    /// # Arguments
    /// * `sx` - Scale factor in X direction
    /// * `sy` - Scale factor in Y direction
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::mat3::Mat3;
    /// let scale = Mat3::scale(2.0, 0.5); // Double width, half height
    /// ```
    #[inline]
    pub fn scale(sx: f32, sy: f32) -> Self {
        Self {
            m00: sx,
            m01: 0.0,
            m02: 0.0,
            m10: 0.0,
            m11: sy,
            m12: 0.0,
            m20: 0.0,
            m21: 0.0,
            m22: 1.0,
        }
    }

    /// Create a uniform scaling matrix
    ///
    /// Creates a matrix that scales points uniformly by the given factor.
    ///
    /// # Arguments
    /// * `s` - Uniform scale factor
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::mat3::Mat3;
    /// let scale = Mat3::scale_uniform(2.0); // Double size
    /// ```
    #[inline]
    pub fn scale_uniform(s: f32) -> Self {
        Self::scale(s, s)
    }

    /// Create a shearing matrix
    ///
    /// Creates a matrix that shears points by the given factors.
    /// Shearing skews the coordinate system.
    ///
    /// # Arguments
    /// * `shx` - Shear factor in X direction (affects Y coordinates)
    /// * `shy` - Shear factor in Y direction (affects X coordinates)
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::mat3::Mat3;
    /// let shear = Mat3::shear(0.5, 0.0); // Shear X by 0.5
    /// ```
    #[inline]
    pub fn shear(shx: f32, shy: f32) -> Self {
        Self {
            m00: 1.0,
            m01: shy,
            m02: 0.0,
            m10: shx,
            m11: 1.0,
            m12: 0.0,
            m20: 0.0,
            m21: 0.0,
            m22: 1.0,
        }
    }

    /// Create a transformation matrix from translation, rotation, and scale
    ///
    /// Creates a matrix that applies transformations in the order:
    /// 1. Scale
    /// 2. Rotate
    /// 3. Translate
    ///
    /// This is the most common transformation order for 2D graphics.
    ///
    /// # Arguments
    /// * `tx` - Translation in X direction
    /// * `ty` - Translation in Y direction
    /// * `angle` - Rotation angle in radians
    /// * `sx` - Scale factor in X direction
    /// * `sy` - Scale factor in Y direction
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::mat3::Mat3;
    /// use std::f32::consts::PI;
    /// let transform = Mat3::transform(10.0, 20.0, PI/4.0, 2.0, 1.5);
    /// ```
    #[inline]
    pub fn transform(tx: f32, ty: f32, angle: f32, sx: f32, sy: f32) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Self {
            m00: sx * cos_a,
            m01: -sy * sin_a,
            m02: tx,
            m10: sx * sin_a,
            m11: sy * cos_a,
            m12: ty,
            m20: 0.0,
            m21: 0.0,
            m22: 1.0,
        }
    }

    /// Transform a 2D vector by this matrix
    ///
    /// Applies the transformation matrix to a Vec2, treating it as a point
    /// with homogeneous coordinate w=1. This means translation is applied.
    ///
    /// # Arguments
    /// * `vec` - The 2D vector to transform
    ///
    /// # Returns
    /// A new Vec2 representing the transformed point
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::mat3::Mat3;
    /// use scratchpad_rs::math::vec2::Vec2;
    /// let translate = Mat3::translate(10.0, 20.0);
    /// let point = Vec2::new(5.0, 5.0);
    /// let transformed = translate.transform_vec2(point);
    /// // Result: Vec2::new(15.0, 25.0)
    /// ```
    #[inline]
    pub fn transform_vec2(self, vec: Vec2) -> Vec2 {
        Vec2::new(
            self.m00 * vec.x + self.m01 * vec.y + self.m02,
            self.m10 * vec.x + self.m11 * vec.y + self.m12,
        )
    }

    /// Transform a 2D vector by this matrix (direction only)
    ///
    /// Applies the transformation matrix to a Vec2, treating it as a direction
    /// vector with homogeneous coordinate w=0. This means translation is NOT applied.
    ///
    /// # Arguments
    /// * `vec` - The 2D direction vector to transform
    ///
    /// # Returns
    /// A new Vec2 representing the transformed direction
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::mat3::Mat3;
    /// use scratchpad_rs::math::vec2::Vec2;
    /// use std::f32::consts::PI;
    /// let rotate = Mat3::rotate(PI / 2.0); // 90 degrees
    /// let direction = Vec2::new(1.0, 0.0); // Right
    /// let transformed = rotate.transform_vec2_direction(direction);
    /// // Result: Vec2::new(0.0, 1.0) - Up
    /// ```
    #[inline]
    pub fn transform_vec2_direction(self, vec: Vec2) -> Vec2 {
        Vec2::new(
            self.m00 * vec.x + self.m01 * vec.y,
            self.m10 * vec.x + self.m11 * vec.y,
        )
    }

    /// Decompose a transformation matrix into translation, rotation, and scale
    ///
    /// Extracts the individual transform components from a composed matrix.
    /// This method works best for matrices created from translation, rotation, and scale
    /// operations without shear.
    ///
    /// # Returns
    /// A `Decomposition` struct containing:
    /// - `translation`: The translation vector (tx, ty)
    /// - `rotation`: The rotation angle in radians
    /// - `scale`: The scale factors (sx, sy)
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::mat3::Mat3;
    /// use std::f32::consts::PI;
    ///
    /// let matrix = Mat3::translate(10.0, 20.0)
    ///     * Mat3::rotate(PI / 4.0)
    ///     * Mat3::scale(2.0, 3.0);
    /// let decomp = matrix.decompose();
    ///
    /// // Note: Due to composition order, exact values may vary
    /// assert!((decomp.translation.x - 10.0).abs() < 1e-5);
    /// ```
    pub fn decompose(self) -> Decomposition {
        // Extract translation (last column, top two rows)
        let translation = Vec2::new(self.m02, self.m12);

        // Extract the 2x2 linear transformation matrix
        // [m00 m01]
        // [m10 m11]
        // This represents rotation and scale combined

        // Extract scale from column vectors
        // Scale X is the length of the first column vector
        let scale_x = (self.m00 * self.m00 + self.m10 * self.m10).sqrt();
        // Scale Y is the length of the second column vector
        let scale_y = (self.m01 * self.m01 + self.m11 * self.m11).sqrt();

        // Extract rotation from the first column (or average of both columns for better accuracy)
        // For a pure rotation+scale matrix:
        // [sx*cos(θ)  -sy*sin(θ)]
        // [sx*sin(θ)   sy*cos(θ)]
        //
        // From first column: atan2(sx*sin(θ), sx*cos(θ)) = atan2(sin(θ), cos(θ)) = θ
        let rotation = f32::atan2(self.m10, self.m00);

        Decomposition {
            translation,
            rotation,
            scale: Vec2::new(scale_x, scale_y),
        }
    }

    /// Decompose an affine transformation matrix with potential shear
    ///
    /// Extracts translation, rotation, scale, and shear components using QR decomposition.
    /// This method handles matrices with shearing (skewing) in addition to rotation and scale.
    ///
    /// # Returns
    /// An `AffineDecomposition` struct containing all transform components
    ///
    /// # Algorithm
    /// Uses QR decomposition to separate the linear part into rotation and scale+shear:
    /// 1. Extract translation (easy - last column)
    /// 2. Apply QR decomposition to the 2x2 linear part
    /// 3. Extract rotation from Q (orthogonal matrix)
    /// 4. Extract scale and shear from R (upper triangular)
    pub fn decompose_affine(self) -> AffineDecomposition {
        // Extract translation
        let translation = Vec2::new(self.m02, self.m12);

        // QR Decomposition of the 2x2 linear part [m00 m01; m10 m11]
        // Q is orthogonal (rotation), R is upper triangular (scale + shear)

        let a = self.m00;
        let b = self.m01;
        let c = self.m10;
        let d = self.m11;

        // Compute first column vector length (for Q normalization)
        let col0_len = (a * a + c * c).sqrt();

        if col0_len < 1e-6 {
            // Degenerate case - matrix has no rotation/scale
            return AffineDecomposition {
                translation,
                rotation: 0.0,
                scale: Vec2::new(0.0, 0.0),
                shear: Vec2::new(0.0, 0.0),
            };
        }

        // Normalize first column to get first column of Q (rotation matrix)
        let q00 = a / col0_len;
        let q10 = c / col0_len;

        // Compute second column of Q (orthogonal to first)
        let q01 = -q10; // Perpendicular
        let q11 = q00;

        // Extract rotation angle from Q
        let rotation = f32::atan2(q10, q00);

        // R = Q^T * A (where A is the original 2x2 matrix [a b; c d])
        // Since Q^T = [q00 q10; q01 q11] (transpose of Q)
        // R = Q^T * A = [q00 q10; q01 q11] * [a b; c d]
        let r00 = q00 * a + q10 * c; // Should be positive (length of first column = col0_len)
        let r01 = q00 * b + q10 * d; // Projection of second column onto first
        let r11 = q01 * b + q11 * d; // Component of second column orthogonal to first

        // Verify R is upper triangular (r10 should be 0)
        // r10 = q01 * a + q11 * c
        // Since Q is orthogonal: q01 = -q10, q11 = q00
        // r10 = -q10 * a + q00 * c
        // But q00 = a/col0_len, q10 = c/col0_len
        // r10 = -(c/col0_len)*a + (a/col0_len)*c = (-ac + ac)/col0_len = 0 ✓

        // Extract scale from R diagonal
        let scale_x = r00.abs(); // Always positive
        let scale_y = r11.abs(); // Always positive (but could be negative for reflection)

        // Extract shear from R off-diagonal
        // In upper triangular R, r01 represents the shear component
        // Store r01 directly (not normalized) - it will be reconstructed correctly
        let shear_x = r01;
        let shear_y = 0.0; // For 2D, we typically only have one shear component

        AffineDecomposition {
            translation,
            rotation,
            scale: Vec2::new(scale_x, scale_y),
            shear: Vec2::new(shear_x, shear_y),
        }
    }

    /// Recompose a matrix from decomposition components
    ///
    /// Rebuilds a transformation matrix from translation, rotation, and scale.
    /// This is the inverse operation of `decompose()`.
    ///
    /// # Arguments
    /// * `decomp` - The decomposition result to recompose
    ///
    /// # Returns
    /// A new `Mat3` matrix representing the composed transformation
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::mat3::{Mat3, Decomposition};
    /// use scratchpad_rs::math::vec2::Vec2;
    /// use std::f32::consts::PI;
    ///
    /// let original = Mat3::translate(10.0, 20.0)
    ///     * Mat3::rotate(PI / 4.0)
    ///     * Mat3::scale(2.0, 3.0);
    /// let decomp = original.decompose();
    /// let recomposed = Mat3::recompose(decomp);
    ///
    /// // Recomposed should match original (within floating point precision)
    /// assert!(original.near(recomposed, 1e-5));
    /// ```
    pub fn recompose(decomp: Decomposition) -> Self {
        // Order: translate * rotate * scale
        Self::translate(decomp.translation.x, decomp.translation.y)
            * Self::rotate(decomp.rotation)
            * Self::scale(decomp.scale.x, decomp.scale.y)
    }

    /// Recompose a matrix from affine decomposition components
    ///
    /// Rebuilds a transformation matrix including shear components.
    /// The composition order is: translate * rotate * scale * shear
    ///
    /// # Arguments
    /// * `decomp` - The affine decomposition result to recompose
    ///
    /// # Returns
    /// A new `Mat3` matrix representing the composed transformation
    pub fn recompose_affine(decomp: AffineDecomposition) -> Self {
        // Build matrix as: translate * Q * R
        // Where Q is the rotation matrix and R is upper triangular (scale + shear)
        //
        // From QR decomposition: A = Q * R
        // Where Q = rotation matrix, R = [sx  shear*sy; 0  sy]

        let cos_r = decomp.rotation.cos();
        let sin_r = decomp.rotation.sin();

        // Q (rotation matrix)
        let q00 = cos_r;
        let q01 = -sin_r;
        let q10 = sin_r;
        let q11 = cos_r;

        // R (upper triangular from QR decomposition)
        // R = [scale.x  shear.x; 0  scale.y]
        // Note: shear.x was stored as r01 directly from QR decomposition
        let r00 = decomp.scale.x;
        let r01 = decomp.shear.x; // Stored directly as r01 from QR
        let r11 = decomp.scale.y;

        // Q * R
        let m00_linear = q00 * r00 + q01 * 0.0; // q00*r00 + q01*0
        let m01_linear = q00 * r01 + q01 * r11; // q00*r01 + q01*r11
        let m10_linear = q10 * r00 + q11 * 0.0; // q10*r00 + q11*0
        let m11_linear = q10 * r01 + q11 * r11; // q10*r01 + q11*r11

        // Then apply translation: T * (Q * R)
        Self {
            m00: m00_linear,
            m01: m01_linear,
            m02: decomp.translation.x,
            m10: m10_linear,
            m11: m11_linear,
            m12: decomp.translation.y,
            m20: 0.0,
            m21: 0.0,
            m22: 1.0,
        }
    }

    /// Transform a 3D vector by this matrix
    ///
    /// Applies the transformation matrix to a Vec3, treating it as a point
    /// with homogeneous coordinate w=1. The z-component is preserved.
    ///
    /// # Arguments
    /// * `vec` - The 3D vector to transform
    ///
    /// # Returns
    /// A new Vec3 representing the transformed point
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::mat3::Mat3;
    /// use scratchpad_rs::math::vec3::Vec3;
    /// let translate = Mat3::translate(10.0, 20.0);
    /// let point = Vec3::new(5.0, 5.0, 1.0);
    /// let transformed = translate.transform_vec3(point);
    /// // Result: Vec3::new(15.0, 25.0, 1.0)
    /// ```
    #[inline]
    pub fn transform_vec3(self, vec: Vec3) -> Vec3 {
        Vec3::new(
            self.m00 * vec.x + self.m01 * vec.y + self.m02 * vec.z,
            self.m10 * vec.x + self.m11 * vec.y + self.m12 * vec.z,
            self.m20 * vec.x + self.m21 * vec.y + self.m22 * vec.z,
        )
    }

    /// Linear interpolation between two matrices
    ///
    /// Performs component-wise linear interpolation between this matrix and another.
    /// This is useful for interpolating between transformation matrices, but note that
    /// the result may not be a valid transformation matrix (e.g., determinant may change).
    ///
    /// # Arguments
    /// * `other` - The target matrix to interpolate towards
    /// * `t` - Interpolation parameter in [0, 1] (0 = this matrix, 1 = other matrix)
    ///
    /// # Returns
    /// A new matrix with each component linearly interpolated
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::mat3::Mat3;
    /// let scale1 = Mat3::scale(1.0, 1.0);
    /// let scale2 = Mat3::scale(2.0, 2.0);
    /// let halfway = scale1.lerp(scale2, 0.5);
    /// // Result: approximately Mat3::scale(1.5, 1.5)
    /// ```
    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            m00: self.m00 + t * (other.m00 - self.m00),
            m01: self.m01 + t * (other.m01 - self.m01),
            m02: self.m02 + t * (other.m02 - self.m02),
            m10: self.m10 + t * (other.m10 - self.m10),
            m11: self.m11 + t * (other.m11 - self.m11),
            m12: self.m12 + t * (other.m12 - self.m12),
            m20: self.m20 + t * (other.m20 - self.m20),
            m21: self.m21 + t * (other.m21 - self.m21),
            m22: self.m22 + t * (other.m22 - self.m22),
        }
    }

    /// Spherical linear interpolation for rotation matrices
    ///
    /// Interpolates between two rotation matrices using spherical linear interpolation.
    /// This preserves the rotation properties and ensures smooth rotation transitions.
    ///
    /// The matrices are decomposed into rotation and scale components, then:
    /// - Rotation is interpolated using quaternion slerp
    /// - Scale is linearly interpolated
    /// - Translation is linearly interpolated
    ///
    /// # Arguments
    /// * `other` - The target rotation matrix to interpolate towards
    /// * `t` - Interpolation parameter in [0, 1]
    ///
    /// # Returns
    /// A new matrix representing the interpolated transformation
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::mat3::Mat3;
    /// use std::f32::consts::PI;
    /// let rotate1 = Mat3::rotate(0.0);
    /// let rotate2 = Mat3::rotate(PI / 2.0);
    /// let halfway = rotate1.slerp(rotate2, 0.5);
    /// // Result: approximately Mat3::rotate(PI / 4.0)
    /// ```
    #[inline]
    pub fn slerp(self, other: Self, t: f32) -> Self {
        use std::f32::consts::PI;

        // Extract translation components
        let trans1 = Vec2::new(self.m02, self.m12);
        let trans2 = Vec2::new(other.m02, other.m12);
        let interp_trans = trans1.lerp(trans2, t);

        // Extract scale components (assuming uniform scaling for simplicity)
        let scale1 = Vec2::new(
            (self.m00 * self.m00 + self.m10 * self.m10).sqrt(),
            (self.m11 * self.m11 + self.m01 * self.m01).sqrt(),
        );
        let scale2 = Vec2::new(
            (other.m00 * other.m00 + other.m10 * other.m10).sqrt(),
            (other.m11 * other.m11 + other.m01 * other.m01).sqrt(),
        );
        let interp_scale = scale1.lerp(scale2, t);

        // Extract rotation angles
        let angle1 = self.m10.atan2(self.m00);
        let angle2 = other.m10.atan2(other.m00);

        // Handle angle wrapping
        let mut angle_diff = angle2 - angle1;
        if angle_diff > PI {
            angle_diff -= 2.0 * PI;
        } else if angle_diff < -PI {
            angle_diff += 2.0 * PI;
        }

        let interp_angle = angle1 + t * angle_diff;

        // Reconstruct the interpolated matrix
        Self::transform(
            interp_trans.x,
            interp_trans.y,
            interp_angle,
            interp_scale.x,
            interp_scale.y,
        )
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

/// Matrix multiplication operator (m1 * m2)
///
/// Performs standard matrix multiplication where each element of the result
/// is the dot product of the corresponding row from the first matrix and
/// column from the second matrix.
impl Mul<Mat3> for Mat3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            m00: self.m00 * rhs.m00 + self.m01 * rhs.m10 + self.m02 * rhs.m20,
            m01: self.m00 * rhs.m01 + self.m01 * rhs.m11 + self.m02 * rhs.m21,
            m02: self.m00 * rhs.m02 + self.m01 * rhs.m12 + self.m02 * rhs.m22,
            m10: self.m10 * rhs.m00 + self.m11 * rhs.m10 + self.m12 * rhs.m20,
            m11: self.m10 * rhs.m01 + self.m11 * rhs.m11 + self.m12 * rhs.m21,
            m12: self.m10 * rhs.m02 + self.m11 * rhs.m12 + self.m12 * rhs.m22,
            m20: self.m20 * rhs.m00 + self.m21 * rhs.m10 + self.m22 * rhs.m20,
            m21: self.m20 * rhs.m01 + self.m21 * rhs.m11 + self.m22 * rhs.m21,
            m22: self.m20 * rhs.m02 + self.m21 * rhs.m12 + self.m22 * rhs.m22,
        }
    }
}

/// Matrix-vector multiplication operator (Mat3 * Vec2)
///
/// Transforms a 2D vector by the matrix, treating it as a point with w=1.
/// This applies translation, rotation, scaling, and shearing.
impl Mul<Vec2> for Mat3 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        self.transform_vec2(rhs)
    }
}

/// Matrix-vector multiplication operator (Mat3 * Vec3)
///
/// Transforms a 3D vector by the matrix, treating it as a point with w=1.
/// This applies translation, rotation, scaling, and shearing to all components.
impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.transform_vec3(rhs)
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

    #[test]
    fn transpose() {
        // Test identity matrix (transpose = identity)
        let identity = Mat3::IDENTITY;
        assert_eq!(identity.transpose(), identity);

        // Test zero matrix (transpose = zero)
        let zero = Mat3::ZERO;
        assert_eq!(zero.transpose(), zero);

        // Test general matrix
        let matrix = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let expected = Mat3::new(1.0, 4.0, 7.0, 2.0, 5.0, 8.0, 3.0, 6.0, 9.0);
        assert_eq!(matrix.transpose(), expected);

        // Test transpose of transpose equals original
        assert_eq!(matrix.transpose().transpose(), matrix);

        // Test symmetric matrix (transpose = original)
        let symmetric = Mat3::new(1.0, 2.0, 3.0, 2.0, 4.0, 5.0, 3.0, 5.0, 6.0);
        assert_eq!(symmetric.transpose(), symmetric);
    }

    #[test]
    fn matrix_multiplication() {
        // Test identity multiplication
        let identity = Mat3::IDENTITY;
        let matrix = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(identity * matrix, matrix);
        assert_eq!(matrix * identity, matrix);

        // Test zero multiplication
        let zero = Mat3::ZERO;
        assert_eq!(zero * matrix, zero);
        assert_eq!(matrix * zero, zero);

        // Test general matrix multiplication
        let a = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let b = Mat3::new(2.0, 0.0, 1.0, 1.0, 2.0, 0.0, 0.0, 1.0, 2.0);

        // Manual calculation:
        // Row 0: [1,2,3] · [2,1,0] = 1*2 + 2*1 + 3*0 = 4
        //        [1,2,3] · [0,2,1] = 1*0 + 2*2 + 3*1 = 7
        //        [1,2,3] · [1,0,2] = 1*1 + 2*0 + 3*2 = 7
        // Row 1: [4,5,6] · [2,1,0] = 4*2 + 5*1 + 6*0 = 13
        //        [4,5,6] · [0,2,1] = 4*0 + 5*2 + 6*1 = 16
        //        [4,5,6] · [1,0,2] = 4*1 + 5*0 + 6*2 = 16
        // Row 2: [7,8,9] · [2,1,0] = 7*2 + 8*1 + 9*0 = 22
        //        [7,8,9] · [0,2,1] = 7*0 + 8*2 + 9*1 = 25
        //        [7,8,9] · [1,0,2] = 7*1 + 8*0 + 9*2 = 25
        let expected = Mat3::new(4.0, 7.0, 7.0, 13.0, 16.0, 16.0, 22.0, 25.0, 25.0);
        assert_eq!(a * b, expected);

        // Test associativity: (A * B) * C = A * (B * C)
        let c = Mat3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!((a * b) * c, a * (b * c));

        // Test scaling matrix multiplication
        let scale = Mat3::new(2.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 4.0);
        let result = scale * a;
        assert_eq!(result.m00, 2.0 * a.m00);
        assert_eq!(result.m11, 3.0 * a.m11);
        assert_eq!(result.m22, 4.0 * a.m22);
    }

    #[test]
    fn inverse() {
        // Test identity matrix (inverse = identity)
        let identity = Mat3::IDENTITY;
        let inv_identity = identity.inverse().unwrap();
        assert_eq!(inv_identity, identity);

        // Test scaling matrix
        let scale = Mat3::new(2.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 4.0);
        let inv_scale = scale.inverse().unwrap();
        let expected_scale = Mat3::new(0.5, 0.0, 0.0, 0.0, 1.0 / 3.0, 0.0, 0.0, 0.0, 0.25);
        assert_eq!(inv_scale, expected_scale);

        // Test general invertible matrix
        let matrix = Mat3::new(1.0, 2.0, 0.0, 0.0, 1.0, 1.0, 2.0, 0.0, 1.0);
        let inv_matrix = matrix.inverse().unwrap();

        // Verify A * A^(-1) = I
        let product = matrix * inv_matrix;
        assert!((product - Mat3::IDENTITY).frobenius_norm() < 1e-6);

        // Verify A^(-1) * A = I
        let product2 = inv_matrix * matrix;
        assert!((product2 - Mat3::IDENTITY).frobenius_norm() < 1e-6);

        // Test singular matrix (should return None)
        let singular = Mat3::new(
            1.0, 2.0, 3.0, 1.0, 2.0, 3.0, // Same as first row
            4.0, 5.0, 6.0,
        );
        assert!(singular.inverse().is_none());

        // Test zero matrix (should return None)
        let zero = Mat3::ZERO;
        assert!(zero.inverse().is_none());

        // Test inverse of inverse equals original
        let original = Mat3::new(2.0, 1.0, 0.0, 1.0, 2.0, 1.0, 0.0, 1.0, 2.0);
        let inv_original = original.inverse().unwrap();
        let inv_inv_original = inv_original.inverse().unwrap();
        assert!((inv_inv_original - original).frobenius_norm() < 1e-6);

        // Test (A * B)^(-1) = B^(-1) * A^(-1)
        let a = Mat3::new(1.0, 2.0, 0.0, 0.0, 1.0, 1.0, 2.0, 0.0, 1.0);
        let b = Mat3::new(2.0, 0.0, 1.0, 1.0, 2.0, 0.0, 0.0, 1.0, 2.0);
        let ab_inv = (a * b).inverse().unwrap();
        let b_inv_a_inv = b.inverse().unwrap() * a.inverse().unwrap();
        assert!((ab_inv - b_inv_a_inv).frobenius_norm() < 1e-6);
    }

    #[test]
    fn utility_methods() {
        // Test is_identity
        let identity = Mat3::IDENTITY;
        assert!(identity.is_identity());
        assert!(identity.is_identity_eps(1e-6));

        let not_identity = Mat3::new(
            1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, // Should be 1.0
        );
        assert!(!not_identity.is_identity());
        assert!(!not_identity.is_identity_eps(1e-6));

        // Test is_zero
        let zero = Mat3::ZERO;
        assert!(zero.is_zero());
        assert!(zero.is_zero_eps(1e-6));

        let not_zero = Mat3::new(
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1e-7, // Very small but not zero
        );
        assert!(!not_zero.is_zero());
        assert!(not_zero.is_zero_eps(1e-6)); // Should be true with epsilon

        // Test near
        let a = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let b = Mat3::new(
            1.0 + 1e-7,
            2.0 + 1e-7,
            3.0 + 1e-7,
            4.0 + 1e-7,
            5.0 + 1e-7,
            6.0 + 1e-7,
            7.0 + 1e-7,
            8.0 + 1e-7,
            9.0 + 1e-7,
        );
        assert!(!a.near(b, 1e-8)); // Too strict
        assert!(a.near(b, 1e-6)); // Should be true
        assert!(a.near(a, 1e-6)); // Same matrix

        // Test trace
        let matrix = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(matrix.trace(), 1.0 + 5.0 + 9.0); // 15.0

        let identity = Mat3::IDENTITY;
        assert_eq!(identity.trace(), 3.0); // 1 + 1 + 1

        let zero = Mat3::ZERO;
        assert_eq!(zero.trace(), 0.0);

        // Test frobenius_norm
        let identity = Mat3::IDENTITY;
        let expected_norm = (1.0_f32 + 1.0_f32 + 1.0_f32).sqrt(); // sqrt(3)
        assert!((identity.frobenius_norm() - expected_norm).abs() < 1e-6);

        let zero = Mat3::ZERO;
        assert_eq!(zero.frobenius_norm(), 0.0);

        let matrix = Mat3::new(3.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0);
        let expected_norm = (9.0_f32 + 16.0_f32).sqrt(); // sqrt(25) = 5.0
        assert!((matrix.frobenius_norm() - expected_norm).abs() < 1e-6);

        // Test edge cases with floating point precision
        let almost_identity = Mat3::new(1.0, 1e-7, 1e-7, 1e-7, 1.0, 1e-7, 1e-7, 1e-7, 1.0);
        assert!(!almost_identity.is_identity());
        assert!(almost_identity.is_identity_eps(1e-6));
        assert!(!almost_identity.is_identity_eps(1e-8));

        // Test trace properties
        let a = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let b = Mat3::new(2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0);

        // trace(A + B) = trace(A) + trace(B)
        assert!(((a + b).trace() - (a.trace() + b.trace())).abs() < 1e-6);

        // trace(cA) = c * trace(A)
        let c = 2.5;
        assert!(((c * a).trace() - c * a.trace()).abs() < 1e-6);
    }

    #[test]
    fn conversion_methods() {
        // Test to_array and from_array
        let original = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);

        let arr = original.to_array();
        let expected_arr = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        assert_eq!(arr, expected_arr);

        let reconstructed = Mat3::from_array(arr);
        assert_eq!(reconstructed, original);

        // Test identity matrix conversion
        let identity = Mat3::IDENTITY;
        let identity_arr = identity.to_array();
        let expected_identity = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        assert_eq!(identity_arr, expected_identity);

        let identity_reconstructed = Mat3::from_array(identity_arr);
        assert_eq!(identity_reconstructed, identity);

        // Test zero matrix conversion
        let zero = Mat3::ZERO;
        let zero_arr = zero.to_array();
        let expected_zero = [0.0; 9];
        assert_eq!(zero_arr, expected_zero);

        let zero_reconstructed = Mat3::from_array(zero_arr);
        assert_eq!(zero_reconstructed, zero);

        // Test as_slice and as_mut_slice
        let mut matrix = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);

        let slice = matrix.as_slice();
        assert_eq!(slice, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

        // Test mutable slice modification
        let mut_slice = matrix.as_mut_slice();
        mut_slice[0] = 10.0; // Change m00
        mut_slice[4] = 20.0; // Change m11
        mut_slice[8] = 30.0; // Change m22

        assert_eq!(matrix.m00, 10.0);
        assert_eq!(matrix.m11, 20.0);
        assert_eq!(matrix.m22, 30.0);

        // Test from_slice
        let slice_data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let from_slice_matrix = Mat3::from_slice(&slice_data);
        assert_eq!(
            from_slice_matrix,
            Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,)
        );

        // Test round-trip conversion
        let test_matrix = Mat3::new(1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5);

        // Array round-trip
        let arr_round_trip = Mat3::from_array(test_matrix.to_array());
        assert_eq!(arr_round_trip, test_matrix);

        // Slice round-trip
        let slice_round_trip = Mat3::from_slice(test_matrix.as_slice());
        assert_eq!(slice_round_trip, test_matrix);

        // Test memory layout consistency
        let matrix = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);

        // Verify that as_slice gives us the same memory layout as to_array
        let array_data = matrix.to_array();
        let slice_data = matrix.as_slice();
        assert_eq!(array_data.as_slice(), slice_data);

        // Test edge cases with floating point precision
        let precision_matrix = Mat3::new(
            1.0 + 1e-7,
            2.0 + 1e-7,
            3.0 + 1e-7,
            4.0 + 1e-7,
            5.0 + 1e-7,
            6.0 + 1e-7,
            7.0 + 1e-7,
            8.0 + 1e-7,
            9.0 + 1e-7,
        );

        let precision_arr = precision_matrix.to_array();
        let precision_reconstructed = Mat3::from_array(precision_arr);
        assert!(precision_reconstructed.near(precision_matrix, 1e-6));
    }

    #[test]
    #[should_panic(expected = "Slice must have exactly 9 elements")]
    fn from_slice_wrong_length() {
        let wrong_slice = [1.0, 2.0, 3.0]; // Only 3 elements
        Mat3::from_slice(&wrong_slice);
    }

    #[test]
    fn transformations() {
        use std::f32::consts::PI;

        // Test translation matrix
        let translate = Mat3::translate(10.0, 20.0);
        let expected_translate = Mat3::new(1.0, 0.0, 10.0, 0.0, 1.0, 20.0, 0.0, 0.0, 1.0);
        assert_eq!(translate, expected_translate);

        // Test rotation matrix (90 degrees)
        let rotate_90 = Mat3::rotate(PI / 2.0);
        let expected_rotate_90 = Mat3::new(0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert!(rotate_90.near(expected_rotate_90, 1e-6));

        // Test rotation matrix (180 degrees)
        let rotate_180 = Mat3::rotate(PI);
        let expected_rotate_180 = Mat3::new(-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0);
        assert!(rotate_180.near(expected_rotate_180, 1e-6));

        // Test scaling matrix
        let scale = Mat3::scale(2.0, 3.0);
        let expected_scale = Mat3::new(2.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(scale, expected_scale);

        // Test uniform scaling
        let scale_uniform = Mat3::scale_uniform(2.5);
        let expected_scale_uniform = Mat3::new(2.5, 0.0, 0.0, 0.0, 2.5, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(scale_uniform, expected_scale_uniform);

        // Test shearing matrix
        let shear = Mat3::shear(0.5, 0.3);
        let expected_shear = Mat3::new(1.0, 0.3, 0.0, 0.5, 1.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(shear, expected_shear);

        // Test identity transformations
        assert_eq!(Mat3::translate(0.0, 0.0), Mat3::IDENTITY);
        assert_eq!(Mat3::rotate(0.0), Mat3::IDENTITY);
        assert_eq!(Mat3::scale(1.0, 1.0), Mat3::IDENTITY);
        assert_eq!(Mat3::scale_uniform(1.0), Mat3::IDENTITY);
        assert_eq!(Mat3::shear(0.0, 0.0), Mat3::IDENTITY);

        // Test transformation composition
        let translate = Mat3::translate(10.0, 20.0);
        let rotate = Mat3::rotate(PI / 4.0);
        let scale = Mat3::scale(2.0, 3.0);

        // Test that individual transformations work
        assert!(!translate.is_identity());
        assert!(!rotate.is_identity());
        assert!(!scale.is_identity());

        // Test combined transformation
        let combined = translate * rotate * scale;
        assert!(!combined.is_identity());
        assert!((combined.det() - 6.0).abs() < 1e-6); // det = 2 * 3 * 1 = 6

        // Test transform method (scale * rotate * translate)
        let transform = Mat3::transform(10.0, 20.0, PI / 4.0, 2.0, 3.0);
        assert!(!transform.is_identity());

        // Test that transform method produces a valid transformation matrix
        assert!(!transform.is_zero());
        assert!(transform.det().abs() > 1e-6); // Should be invertible

        // Test negative transformations
        let neg_translate = Mat3::translate(-5.0, -10.0);
        let neg_scale = Mat3::scale(-1.0, -1.0);
        assert_eq!(neg_translate.m02, -5.0);
        assert_eq!(neg_translate.m12, -10.0);
        assert_eq!(neg_scale.m00, -1.0);
        assert_eq!(neg_scale.m11, -1.0);

        // Test zero transformations
        let zero_scale = Mat3::scale(0.0, 0.0);
        assert_eq!(zero_scale.m00, 0.0);
        assert_eq!(zero_scale.m11, 0.0);
        assert_eq!(zero_scale.det(), 0.0); // Singular matrix

        // Test transformation properties
        let t1 = Mat3::translate(1.0, 2.0);
        let t2 = Mat3::translate(3.0, 4.0);
        let combined_translate = t1 * t2;
        let expected_combined = Mat3::translate(4.0, 6.0); // (1+3, 2+4)
        assert_eq!(combined_translate, expected_combined);

        // Test rotation composition
        let r1 = Mat3::rotate(PI / 6.0); // 30 degrees
        let r2 = Mat3::rotate(PI / 6.0); // 30 degrees
        let combined_rotate = r1 * r2;
        let expected_combined_rotate = Mat3::rotate(PI / 3.0); // 60 degrees
        assert!(combined_rotate.near(expected_combined_rotate, 1e-6));

        // Test scaling composition
        let s1 = Mat3::scale(2.0, 3.0);
        let s2 = Mat3::scale(4.0, 5.0);
        let combined_scale = s1 * s2;
        let expected_combined_scale = Mat3::scale(8.0, 15.0); // (2*4, 3*5)
        assert_eq!(combined_scale, expected_combined_scale);
    }

    #[test]
    fn vector_transformations() {
        use std::f32::consts::PI;

        // Test Vec2 point transformation (with translation)
        let translate = Mat3::translate(10.0, 20.0);
        let point = Vec2::new(5.0, 5.0);
        let transformed_point = translate.transform_vec2(point);
        let expected_point = Vec2::new(15.0, 25.0); // (5+10, 5+20)
        assert_eq!(transformed_point, expected_point);

        // Test Vec2 direction transformation (without translation)
        let rotate_90 = Mat3::rotate(PI / 2.0);
        let direction = Vec2::new(1.0, 0.0); // Right
        let transformed_direction = rotate_90.transform_vec2_direction(direction);
        let expected_direction = Vec2::new(0.0, 1.0); // Up
        assert!(transformed_direction.near(expected_direction, 1e-6));

        // Test Vec3 transformation
        let scale = Mat3::scale(2.0, 3.0);
        let point_3d = Vec3::new(1.0, 2.0, 1.0);
        let transformed_3d = scale.transform_vec3(point_3d);
        let expected_3d = Vec3::new(2.0, 6.0, 1.0); // (1*2, 2*3, 1*1)
        assert_eq!(transformed_3d, expected_3d);

        // Test operator overloads
        let matrix = Mat3::translate(5.0, 10.0);
        let vec2 = Vec2::new(1.0, 2.0);
        let vec3 = Vec3::new(1.0, 2.0, 3.0);

        // Test Mat3 * Vec2
        let result_vec2 = matrix * vec2;
        let expected_vec2 = Vec2::new(6.0, 12.0); // (1+5, 2+10)
        assert_eq!(result_vec2, expected_vec2);

        // Test Mat3 * Vec3
        let result_vec3 = matrix * vec3;
        let expected_vec3 = Vec3::new(16.0, 32.0, 3.0); // (1+5*3, 2+10*3, 3*1)
        assert_eq!(result_vec3, expected_vec3);

        // Test identity transformation
        let identity = Mat3::IDENTITY;
        let test_vec2 = Vec2::new(3.0, 4.0);
        let test_vec3 = Vec3::new(3.0, 4.0, 5.0);

        assert_eq!(identity.transform_vec2(test_vec2), test_vec2);
        assert_eq!(identity.transform_vec2_direction(test_vec2), test_vec2);
        assert_eq!(identity.transform_vec3(test_vec3), test_vec3);

        // Test zero transformation
        let zero = Mat3::ZERO;
        let test_vec2 = Vec2::new(1.0, 2.0);
        let test_vec3 = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(zero.transform_vec2(test_vec2), Vec2::ZERO);
        assert_eq!(zero.transform_vec2_direction(test_vec2), Vec2::ZERO);
        assert_eq!(zero.transform_vec3(test_vec3), Vec3::ZERO);

        // Test complex transformation
        let complex = Mat3::transform(10.0, 20.0, PI / 4.0, 2.0, 3.0);
        let test_point = Vec2::new(1.0, 1.0);
        let transformed = complex.transform_vec2(test_point);

        // Verify the transformation is not identity and produces valid results
        assert!(!transformed.near(test_point, 1e-6));
        assert!(!transformed.near(Vec2::ZERO, 1e-6));

        // Test direction vs point transformation difference
        let translate_only = Mat3::translate(5.0, 5.0);
        let test_direction = Vec2::new(1.0, 0.0);

        let point_result = translate_only.transform_vec2(test_direction);
        let direction_result = translate_only.transform_vec2_direction(test_direction);

        // Point transformation should include translation
        assert_eq!(point_result, Vec2::new(6.0, 5.0)); // (1+5, 0+5)
        // Direction transformation should NOT include translation
        assert_eq!(direction_result, Vec2::new(1.0, 0.0)); // (1+0, 0+0)

        // Test rotation preserves vector length for directions
        let rotate_45 = Mat3::rotate(PI / 4.0);
        let unit_vector = Vec2::new(1.0, 0.0);
        let rotated = rotate_45.transform_vec2_direction(unit_vector);

        // Length should be preserved (approximately due to floating point)
        assert!((rotated.len() - 1.0).abs() < 1e-6);

        // Test scaling affects vector length
        let scale_2x = Mat3::scale(2.0, 2.0);
        let test_vector = Vec2::new(3.0, 4.0); // Length = 5
        let scaled = scale_2x.transform_vec2_direction(test_vector);

        // Length should be doubled
        assert!((scaled.len() - 10.0).abs() < 1e-6); // 5 * 2 = 10

        // Test shearing
        let shear_x = Mat3::shear(0.5, 0.0); // Shear X by 0.5
        let test_vec = Vec2::new(1.0, 1.0);
        let sheared = shear_x.transform_vec2_direction(test_vec);

        // X should remain unchanged: x' = x = 1
        // Y should be affected by X: y' = 0.5*x + y = 0.5*1 + 1 = 1.5
        assert!(sheared.near(Vec2::new(1.0, 1.5), 1e-6));
    }

    #[test]
    fn interpolation() {
        use std::f32::consts::PI;

        // Test linear interpolation (lerp)
        let scale1 = Mat3::scale(1.0, 1.0);
        let scale2 = Mat3::scale(2.0, 2.0);

        // Test t=0 (should return first matrix)
        assert_eq!(scale1.lerp(scale2, 0.0), scale1);

        // Test t=1 (should return second matrix)
        assert_eq!(scale1.lerp(scale2, 1.0), scale2);

        // Test t=0.5 (should be halfway)
        let halfway = scale1.lerp(scale2, 0.5);
        let expected_halfway = Mat3::scale(1.5, 1.5);
        assert!(halfway.near(expected_halfway, 1e-6));

        // Test translation interpolation
        let trans1 = Mat3::translate(0.0, 0.0);
        let trans2 = Mat3::translate(10.0, 20.0);
        let interp_trans = trans1.lerp(trans2, 0.5);
        let expected_trans = Mat3::translate(5.0, 10.0);
        assert!(interp_trans.near(expected_trans, 1e-6));

        // Test spherical linear interpolation (slerp)
        let rotate1 = Mat3::rotate(0.0);
        let rotate2 = Mat3::rotate(PI / 2.0); // 90 degrees

        // Test t=0 (should return first matrix)
        assert!(rotate1.slerp(rotate2, 0.0).near(rotate1, 1e-6));

        // Test t=1 (should return second matrix)
        assert!(rotate1.slerp(rotate2, 1.0).near(rotate2, 1e-6));

        // Test t=0.5 (should be 45 degrees)
        let halfway_rot = rotate1.slerp(rotate2, 0.5);
        let expected_45 = Mat3::rotate(PI / 4.0);
        assert!(halfway_rot.near(expected_45, 1e-6));

        // Test complex transformation interpolation
        let transform1 = Mat3::transform(0.0, 0.0, 0.0, 1.0, 1.0);
        let transform2 = Mat3::transform(10.0, 20.0, PI / 4.0, 2.0, 3.0);

        let interp_transform = transform1.slerp(transform2, 0.5);

        // Verify it's not the same as either input
        assert!(!interp_transform.near(transform1, 1e-6));
        assert!(!interp_transform.near(transform2, 1e-6));

        // Verify it's a valid transformation matrix
        assert!(interp_transform.det().abs() > 1e-6); // Should be invertible

        // Test angle wrapping in slerp
        let rot_350 = Mat3::rotate(350.0 * PI / 180.0); // 350 degrees
        let rot_10 = Mat3::rotate(10.0 * PI / 180.0); // 10 degrees

        // Should interpolate the short way (20 degrees, not 340 degrees)
        let interp_wrap = rot_350.slerp(rot_10, 0.5);
        let expected_0 = Mat3::rotate(0.0); // Should be close to 0 degrees
        assert!(interp_wrap.near(expected_0, 1e-2));

        // Test interpolation properties
        let matrix_a = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let matrix_b = Mat3::new(2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0);

        // Test commutativity: lerp(a, b, t) should equal lerp(b, a, 1-t)
        let lerp_ab = matrix_a.lerp(matrix_b, 0.3);
        let lerp_ba = matrix_b.lerp(matrix_a, 0.7);
        assert!(lerp_ab.near(lerp_ba, 1e-6));

        // Test edge cases
        let identity = Mat3::IDENTITY;
        let zero = Mat3::ZERO;

        // Interpolating with identity
        let interp_id = identity.lerp(zero, 0.5);
        let expected_half = Mat3::new(0.5, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5);
        assert!(interp_id.near(expected_half, 1e-6));

        // Test interpolation with same matrix
        let same_interp = matrix_a.lerp(matrix_a, 0.5);
        assert!(same_interp.near(matrix_a, 1e-6));
    }

    #[test]
    fn decompose_translation_only() {
        let matrix = Mat3::translate(10.0, 20.0);
        let decomp = matrix.decompose();

        assert!((decomp.translation.x - 10.0).abs() < 1e-6);
        assert!((decomp.translation.y - 20.0).abs() < 1e-6);
        assert!((decomp.rotation - 0.0).abs() < 1e-6);
        assert!((decomp.scale.x - 1.0).abs() < 1e-6);
        assert!((decomp.scale.y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn decompose_rotation_only() {
        use std::f32::consts::PI;

        let angle = PI / 4.0; // 45 degrees
        let matrix = Mat3::rotate(angle);
        let decomp = matrix.decompose();

        assert!((decomp.translation.x - 0.0).abs() < 1e-6);
        assert!((decomp.translation.y - 0.0).abs() < 1e-6);
        assert!((decomp.rotation - angle).abs() < 1e-5);
        assert!((decomp.scale.x - 1.0).abs() < 1e-5);
        assert!((decomp.scale.y - 1.0).abs() < 1e-5);
    }

    #[test]
    fn decompose_scale_only() {
        let matrix = Mat3::scale(2.0, 3.0);
        let decomp = matrix.decompose();

        assert!((decomp.translation.x - 0.0).abs() < 1e-6);
        assert!((decomp.translation.y - 0.0).abs() < 1e-6);
        assert!((decomp.rotation - 0.0).abs() < 1e-6);
        assert!((decomp.scale.x - 2.0).abs() < 1e-5);
        assert!((decomp.scale.y - 3.0).abs() < 1e-5);
    }

    #[test]
    fn decompose_combined_transform() {
        use std::f32::consts::PI;

        let tx = 10.0;
        let ty = 20.0;
        let angle = PI / 6.0; // 30 degrees
        let sx = 2.0;
        let sy = 3.0;

        // Create matrix: translate * rotate * scale
        let matrix = Mat3::translate(tx, ty) * Mat3::rotate(angle) * Mat3::scale(sx, sy);
        let decomp = matrix.decompose();

        // Translation should match (not affected by rotation/scale when applied first)
        assert!((decomp.translation.x - tx).abs() < 1e-5);
        assert!((decomp.translation.y - ty).abs() < 1e-5);

        // Rotation should match
        assert!((decomp.rotation - angle).abs() < 1e-5);

        // Scale should match
        assert!((decomp.scale.x - sx).abs() < 1e-5);
        assert!((decomp.scale.y - sy).abs() < 1e-5);
    }

    #[test]
    fn recompose_from_decomposition() {
        use std::f32::consts::PI;

        let original = Mat3::translate(10.0, 20.0) * Mat3::rotate(PI / 4.0) * Mat3::scale(2.0, 3.0);

        let decomp = original.decompose();
        let recomposed = Mat3::recompose(decomp);

        // Recomposed should be very close to original
        assert!(original.near(recomposed, 1e-4));
    }

    #[test]
    fn decompose_recompose_round_trip() {
        use std::f32::consts::PI;

        let test_cases = vec![
            Mat3::IDENTITY,
            Mat3::translate(5.0, 10.0),
            Mat3::rotate(PI / 3.0),
            Mat3::scale(2.0, 3.0),
            Mat3::translate(10.0, 20.0) * Mat3::rotate(PI / 6.0) * Mat3::scale(1.5, 2.5),
            Mat3::rotate(PI / 4.0) * Mat3::scale_uniform(2.0),
        ];

        for matrix in test_cases {
            let decomp = matrix.decompose();
            let recomposed = Mat3::recompose(decomp);

            // Round-trip should be accurate
            assert!(
                matrix.near(recomposed, 1e-4),
                "Round-trip failed for matrix {:?}",
                matrix
            );
        }
    }

    #[test]
    fn decompose_affine_with_shear() {
        let matrix = Mat3::shear(0.5, 0.3);
        let decomp = matrix.decompose_affine();

        // Shear matrix should have translation = 0
        assert!((decomp.translation.x).abs() < 1e-6);
        assert!((decomp.translation.y).abs() < 1e-6);

        // Verify round-trip works
        let recomposed = Mat3::recompose_affine(decomp);
        assert!(matrix.near(recomposed, 1e-4));
    }

    #[test]
    fn decompose_affine_combined() {
        use std::f32::consts::PI;

        // Create matrix with translation, rotation, scale, and shear
        let matrix = Mat3::translate(10.0, 20.0)
            * Mat3::rotate(PI / 4.0)
            * Mat3::scale(2.0, 3.0)
            * Mat3::shear(0.2, 0.1);

        let decomp = matrix.decompose_affine();

        // Verify round-trip works (more important than exact values due to composition order)
        let recomposed = Mat3::recompose_affine(decomp);
        assert!(matrix.near(recomposed, 1e-3));
    }

    #[test]
    fn recompose_affine_round_trip() {
        use std::f32::consts::PI;

        let matrix = Mat3::translate(10.0, 20.0)
            * Mat3::rotate(PI / 6.0)
            * Mat3::scale(2.0, 3.0)
            * Mat3::shear(0.2, 0.1);

        let decomp = matrix.decompose_affine();
        let recomposed = Mat3::recompose_affine(decomp);

        // Round-trip should be accurate
        assert!(matrix.near(recomposed, 1e-4));
    }

    #[test]
    fn decompose_identity() {
        let decomp = Mat3::IDENTITY.decompose();

        assert!((decomp.translation.x - 0.0).abs() < 1e-6);
        assert!((decomp.translation.y - 0.0).abs() < 1e-6);
        assert!((decomp.rotation - 0.0).abs() < 1e-6);
        assert!((decomp.scale.x - 1.0).abs() < 1e-6);
        assert!((decomp.scale.y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn decompose_negative_scale() {
        let matrix = Mat3::scale(-2.0, -3.0);
        let decomp = matrix.decompose();

        // Negative scale should be preserved
        assert!((decomp.scale.x + 2.0).abs() < 1e-5 || (decomp.scale.x - 2.0).abs() < 1e-5);
        assert!((decomp.scale.y + 3.0).abs() < 1e-5 || (decomp.scale.y - 3.0).abs() < 1e-5);
    }

    #[test]
    fn decompose_edge_cases() {
        // Very small scale
        let matrix = Mat3::scale(0.001, 0.001);
        let decomp = matrix.decompose();
        assert!((decomp.scale.x - 0.001).abs() < 1e-6);

        // Large scale
        let matrix = Mat3::scale(1000.0, 1000.0);
        let decomp = matrix.decompose();
        assert!((decomp.scale.x - 1000.0).abs() < 1e-5);

        // Small rotation
        let matrix = Mat3::rotate(0.001);
        let decomp = matrix.decompose();
        assert!((decomp.rotation - 0.001).abs() < 1e-6);
    }

    #[test]
    fn decompose_non_uniform_scale_and_rotation() {
        use std::f32::consts::PI;

        // Non-uniform scale with rotation
        let matrix = Mat3::rotate(PI / 4.0) * Mat3::scale(2.0, 0.5);
        let decomp = matrix.decompose();

        // Should extract correct rotation
        assert!((decomp.rotation - PI / 4.0).abs() < 1e-5);

        // Scale should be preserved (though order matters)
        assert!((decomp.scale.x - 2.0).abs() < 1e-4 || (decomp.scale.x - 0.5).abs() < 1e-4);
    }
}
