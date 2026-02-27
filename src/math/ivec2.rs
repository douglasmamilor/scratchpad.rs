use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

impl IVec2 {
    /// Zero vector (0, 0)
    pub const ZERO: Self = Self { x: 0, y: 0 };

    /// Unit vector (1, 1)
    pub const ONE: Self = Self { x: 1, y: 1 };

    /// Unit vector along X-axis (1, 0)
    pub const X: Self = Self { x: 1, y: 0 };

    /// Unit vector along Y-axis (0, 1)
    pub const Y: Self = Self { x: 0, y: 1 };

    /// Creates a new IVec2 with the given x and y components
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::IVec2;
    /// let v = IVec2::new(3, 4);
    /// assert_eq!(v.x, 3);
    /// assert_eq!(v.y, 4);
    /// ```
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
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
    /// use scratchpad_rs::math::IVec2;
    /// let a = IVec2::new(1, 0);  // Right
    /// let b = IVec2::new(0, 1);  // Up
    /// assert_eq!(a.dot(b), 0);   // Perpendicular
    /// ```
    #[inline]
    pub fn dot(self, rhs: IVec2) -> i32 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Computes the cross product of two vectors
    ///
    /// For 2D vectors, this returns a scalar representing the "signed area"
    /// of the parallelogram formed by the two vectors.
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::IVec2;
    /// let a = IVec2::new(1, 0);
    /// let b = IVec2::new(0, 1);
    /// assert_eq!(a.cross(b), 1);  // Counter-clockwise
    /// ```
    #[inline]
    pub fn cross(self, rhs: Self) -> i32 {
        self.x * rhs.y - self.y * rhs.x
    }

    /// Computes the cross product using the perpendicular method
    ///
    /// This is equivalent to `self.cross(rhs)` but uses the perpendicular
    /// vector approach: `self.perp().dot(rhs)`
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::IVec2;
    /// let a = IVec2::new(1, 0);
    /// let b = IVec2::new(0, 1);
    /// assert_eq!(a.cross_via_perp(b), 1);
    /// ```
    #[inline]
    pub fn cross_via_perp(self, rhs: Self) -> i32 {
        self.perp().dot(rhs)
    }

    /// Computes the squared length of the vector
    ///
    /// This is faster than `len()` since it avoids the square root.
    /// Useful for distance comparisons where you don't need the actual distance.
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::IVec2;
    /// let v = IVec2::new(3, 4);
    /// assert_eq!(v.len_sq(), 25);
    /// ```
    #[inline]
    pub fn len_sq(self) -> i32 {
        self.dot(self)
    }

    /// Computes the length (magnitude) of the vector
    ///
    /// Returns the Euclidean distance from the origin to this point.
    /// Note: This returns f32 for precision, as sqrt of integers may not be integers.
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::IVec2;
    /// let v = IVec2::new(3, 4);
    /// assert!((v.len() - 5.0).abs() < 1e-6);
    /// ```
    #[inline]
    pub fn len(self) -> f32 {
        (self.len_sq() as f32).sqrt()
    }

    /// Normalizes the vector to unit length, or returns zero if length is zero
    ///
    /// For integer vectors, this returns the original vector since we cannot
    /// normalize to unit length while keeping integer coordinates.
    /// If the input vector has zero length, returns `IVec2::ZERO`.
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::IVec2;
    /// let v = IVec2::new(3, 4);
    /// let normalized = v.normalize_or_zero();
    /// assert_eq!(normalized, v); // Returns original vector for integers
    /// ```
    #[inline]
    pub fn normalize_or_zero(self) -> Self {
        let len_sq = self.len_sq();
        if len_sq == 0 {
            Self::ZERO
        } else {
            // For integer vectors, we can't normalize to unit length
            // Return the original vector as the best approximation
            self
        }
    }

    /// Checks if two vectors are approximately equal within a tolerance
    ///
    /// Since we're dealing with integers, this checks for exact equality.
    /// The `eps` parameter is ignored for integer vectors.
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::IVec2;
    /// let a = IVec2::new(3, 4);
    /// let b = IVec2::new(3, 4);
    /// assert!(a.near(b, 0.0));
    /// ```
    #[inline]
    pub fn near(self, rhs: Self, _eps: f32) -> bool {
        self == rhs
    }

    /// Linearly interpolates between two vectors
    ///
    /// For integer vectors, this performs integer interpolation.
    /// `t` should be in the range [0.0, 1.0].
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::IVec2;
    /// let a = IVec2::new(0, 0);
    /// let b = IVec2::new(10, 20);
    /// let result = a.lerp(b, 0.5);
    /// assert_eq!(result, IVec2::new(5, 10));
    /// ```
    #[inline]
    pub fn lerp(self, to: Self, t: f32) -> Self {
        Self {
            x: (self.x as f32 + (to.x - self.x) as f32 * t).round() as i32,
            y: (self.y as f32 + (to.y - self.y) as f32 * t).round() as i32,
        }
    }

    /// Returns the perpendicular vector (rotated 90° counter-clockwise)
    ///
    /// This creates a vector that is perpendicular to the original.
    /// For a vector (x, y), the perpendicular is (-y, x).
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::IVec2;
    /// let v = IVec2::new(1, 0);  // Right
    /// let perp = v.perp();
    /// assert_eq!(perp, IVec2::new(0, 1));  // Up
    /// ```
    #[inline]
    pub fn perp(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// Reflects this vector off a surface with the given normal
    ///
    /// The reflection formula is: `v - 2 * (v · n) * n`
    /// where `v` is this vector and `n` is the normal.
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::IVec2;
    /// let v = IVec2::new(1, -1);  // Moving down-right
    /// let normal = IVec2::new(0, 1);  // Surface pointing up
    /// let reflected = v.reflect(normal);
    /// assert_eq!(reflected, IVec2::new(1, 1));  // Moving up-right
    /// ```
    #[inline]
    pub fn reflect(self, normal: Self) -> Self {
        self - normal * (2 * self.dot(normal))
    }

    /// Computes the angle of this vector in radians
    ///
    /// Returns the angle from the positive X-axis to this vector.
    /// Range: [-π, π]
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::IVec2;
    /// let v = IVec2::new(1, 0);
    /// assert!((v.angle() - 0.0).abs() < 1e-6);
    /// ```
    #[inline]
    pub fn angle(self) -> f32 {
        (self.y as f32).atan2(self.x as f32)
    }

    /// Creates a unit vector from an angle in radians
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::IVec2;
    /// let v = IVec2::from_angle(0.0);
    /// assert_eq!(v, IVec2::new(1, 0));
    /// ```
    #[inline]
    pub fn from_angle(angle: f32) -> Self {
        Self {
            x: angle.cos().round() as i32,
            y: angle.sin().round() as i32,
        }
    }

    /// Computes the distance between two points
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::math::IVec2;
    /// let a = IVec2::new(0, 0);
    /// let b = IVec2::new(3, 4);
    /// assert!((a.distance(b) - 5.0).abs() < 1e-6);
    /// ```
    #[inline]
    pub fn distance(self, to: Self) -> f32 {
        (self - to).len()
    }
}

impl From<(i32, i32)> for IVec2 {
    #[inline]
    fn from(v: (i32, i32)) -> Self {
        Self::new(v.0, v.1)
    }
}

impl From<IVec2> for (i32, i32) {
    #[inline]
    fn from(v: IVec2) -> Self {
        (v.x, v.y)
    }
}

impl Neg for IVec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for IVec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for IVec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for IVec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for IVec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<i32> for IVec2 {
    type Output = Self;
    #[inline]
    fn mul(self, scalar: i32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl MulAssign<i32> for IVec2 {
    #[inline]
    fn mul_assign(&mut self, scalar: i32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Div<i32> for IVec2 {
    type Output = Self;
    #[inline]
    fn div(self, scalar: i32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl DivAssign<i32> for IVec2 {
    #[inline]
    fn div_assign(&mut self, scalar: i32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl Index<usize> for IVec2 {
    type Output = i32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index {} out of bounds for IVec2", index),
        }
    }
}

impl IndexMut<usize> for IVec2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index {} out of bounds for IVec2", index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let v = IVec2::new(3, 4);
        assert_eq!(v.x, 3);
        assert_eq!(v.y, 4);
    }

    #[test]
    fn test_constants() {
        assert_eq!(IVec2::ZERO, IVec2::new(0, 0));
        assert_eq!(IVec2::ONE, IVec2::new(1, 1));
        assert_eq!(IVec2::X, IVec2::new(1, 0));
        assert_eq!(IVec2::Y, IVec2::new(0, 1));
    }

    #[test]
    fn test_arithmetic() {
        let a = IVec2::new(3, 4);
        let b = IVec2::new(1, 2);

        assert_eq!(a + b, IVec2::new(4, 6));
        assert_eq!(a - b, IVec2::new(2, 2));
        assert_eq!(a * 2, IVec2::new(6, 8));
        assert_eq!(a / 2, IVec2::new(1, 2));
        assert_eq!(-a, IVec2::new(-3, -4));
    }

    #[test]
    fn test_dot_product() {
        let a = IVec2::new(3, 4);
        let b = IVec2::new(1, 2);
        assert_eq!(a.dot(b), 11); // 3*1 + 4*2 = 11
    }

    #[test]
    fn test_cross_product() {
        let a = IVec2::new(3, 4);
        let b = IVec2::new(1, 2);
        assert_eq!(a.cross(b), 2); // 3*2 - 4*1 = 2
    }

    #[test]
    fn test_length() {
        let v = IVec2::new(3, 4);
        assert_eq!(v.len_sq(), 25);
        assert!((v.len() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_utility_methods() {
        let v = IVec2::new(3, 4);
        assert_eq!(v.perp(), IVec2::new(-4, 3));
        assert!(v.near(IVec2::new(3, 4), 0.0));
        assert!(!v.near(IVec2::new(3, 5), 0.0));
    }

    #[test]
    fn test_indexing() {
        let mut v = IVec2::new(3, 4);
        assert_eq!(v[0], 3);
        assert_eq!(v[1], 4);

        v[0] = 5;
        v[1] = 6;
        assert_eq!(v, IVec2::new(5, 6));
    }

    #[test]
    fn test_conversions() {
        let tuple = (3, 4);
        let ivec = IVec2::from(tuple);
        assert_eq!(ivec, IVec2::new(3, 4));

        let back: (i32, i32) = ivec.into();
        assert_eq!(back, (3, 4));
    }
}
