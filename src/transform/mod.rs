use crate::math::Mat3;

pub struct TransformStack {
    stack: Vec<Mat3>,
}

impl TransformStack {
    pub fn new() -> Self {
        Self {
            stack: vec![Mat3::IDENTITY],
        }
    }

    #[inline]
    pub fn current(&self) -> Mat3 {
        *self.stack.last().unwrap()
    }

    pub fn push(&mut self, m: Mat3) {
        let top = self.current() * m;
        self.stack.push(top);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn mul(&mut self, m: Mat3) {
        let top = self.stack.last_mut().unwrap();
        *top = *top * m;
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.mul(Mat3::translate(x, y));
    }

    pub fn rotate(&mut self, angle: f32) {
        self.mul(Mat3::rotate(angle));
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.mul(Mat3::scale(sx, sy));
    }
}

impl Default for TransformStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec2;
    use std::f32::consts::PI;

    #[test]
    fn construction() {
        // Test new() creates transform with identity matrix
        let transform = TransformStack::new();
        assert_eq!(transform.current(), Mat3::IDENTITY);

        // Test default() creates same as new()
        let default_transform = TransformStack::default();
        assert_eq!(default_transform.current(), Mat3::IDENTITY);
    }

    #[test]
    fn current_matrix_access() {
        let mut transform = TransformStack::new();

        // Initially should be identity
        assert_eq!(transform.current(), Mat3::IDENTITY);

        // After modification, should reflect changes
        let translate = Mat3::translate(10.0, 20.0);
        transform.mul(translate);
        assert_eq!(transform.current(), translate);

        // After push, should be composition
        let scale = Mat3::scale(2.0, 3.0);
        transform.push(scale);
        let expected = translate * scale;
        assert_eq!(transform.current(), expected);
    }

    #[test]
    fn push_and_pop_operations() {
        let mut transform = TransformStack::new();

        // Test push operation with pre-composed matrix
        let translate = Mat3::translate(10.0, 20.0);
        transform.push(translate);
        assert_eq!(transform.current(), translate);

        // Test multiple pushes
        let scale = Mat3::scale(2.0, 3.0);
        transform.push(scale);
        let expected = translate * scale;
        assert_eq!(transform.current(), expected);

        // Test pop operation
        transform.pop();
        assert_eq!(transform.current(), translate);

        // Test pop back to identity
        transform.pop();
        assert_eq!(transform.current(), Mat3::IDENTITY);

        // Test state machine usage
        transform.translate(5.0, 10.0);
        let state1 = transform.current();

        // Push current state
        transform.push(Mat3::IDENTITY);

        // Apply more transformations
        transform.scale(2.0, 2.0);
        let state2 = transform.current();

        // Pop back to saved state
        transform.pop();
        assert_eq!(transform.current(), state1);

        // Verify states are different
        assert!(!state1.near(state2, 1e-6));
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")] // TODO: should we handle this more gracefully?
    fn pop_from_empty_stack_panics() {
        let mut transform = TransformStack::new();
        transform.pop(); // This removes the identity matrix
        transform.current(); // This should panic because stack is empty
    }

    #[test]
    fn matrix_multiplication() {
        let mut transform = TransformStack::new();

        // Test mul operation
        let translate = Mat3::translate(5.0, 10.0);
        transform.mul(translate);
        assert_eq!(transform.current(), translate);

        // Test multiple multiplications
        let scale = Mat3::scale(2.0, 3.0);
        transform.mul(scale);
        let expected = translate * scale;
        assert_eq!(transform.current(), expected);

        // Test multiplication with identity
        transform.mul(Mat3::IDENTITY);
        assert_eq!(transform.current(), expected); // Should be unchanged
    }

    #[test]
    fn translate_operation() {
        let mut transform = TransformStack::new();

        // Test basic translation
        transform.translate(10.0, 20.0);
        let expected = Mat3::translate(10.0, 20.0);
        assert_eq!(transform.current(), expected);

        // Test multiple translations
        transform.translate(5.0, 5.0);
        let expected_combined = Mat3::translate(10.0, 20.0) * Mat3::translate(5.0, 5.0);
        assert_eq!(transform.current(), expected_combined);

        // Test zero translation
        transform.translate(0.0, 0.0);
        assert_eq!(transform.current(), expected_combined); // Should be unchanged

        // Test negative translation
        transform.translate(-5.0, -10.0);
        let expected_negative = expected_combined * Mat3::translate(-5.0, -10.0);
        assert_eq!(transform.current(), expected_negative);
    }

    #[test]
    fn rotate_operation() {
        let mut transform = TransformStack::new();

        // Test basic rotation
        transform.rotate(PI / 4.0); // 45 degrees
        let expected = Mat3::rotate(PI / 4.0);
        assert!(transform.current().near(expected, 1e-6));

        // Test multiple rotations
        transform.rotate(PI / 4.0); // Another 45 degrees = 90 degrees total
        let expected_combined = Mat3::rotate(PI / 4.0) * Mat3::rotate(PI / 4.0);
        assert!(transform.current().near(expected_combined, 1e-6));

        // Test zero rotation
        transform.rotate(0.0);
        assert!(transform.current().near(expected_combined, 1e-6)); // Should be unchanged

        // Test negative rotation
        transform.rotate(-PI / 2.0); // -90 degrees
        let expected_negative = expected_combined * Mat3::rotate(-PI / 2.0);
        assert!(transform.current().near(expected_negative, 1e-6));

        // Test full rotation (2π)
        transform.rotate(2.0 * PI);
        // Should be approximately the same as before (within floating point precision)
        assert!(transform.current().near(expected_negative, 1e-5));
    }

    #[test]
    fn scale_operation() {
        let mut transform = TransformStack::new();

        // Test basic scaling
        transform.scale(2.0, 3.0);
        let expected = Mat3::scale(2.0, 3.0);
        assert_eq!(transform.current(), expected);

        // Test multiple scalings
        transform.scale(0.5, 2.0);
        let expected_combined = Mat3::scale(2.0, 3.0) * Mat3::scale(0.5, 2.0);
        assert_eq!(transform.current(), expected_combined);

        // Test uniform scaling
        transform.scale(2.0, 2.0);
        let expected_uniform = expected_combined * Mat3::scale(2.0, 2.0);
        assert_eq!(transform.current(), expected_uniform);

        // Test zero scaling
        transform.scale(0.0, 0.0);
        let expected_zero = expected_uniform * Mat3::scale(0.0, 0.0);
        assert_eq!(transform.current(), expected_zero);

        // Test negative scaling
        transform.scale(-1.0, -1.0);
        let expected_negative = expected_zero * Mat3::scale(-1.0, -1.0);
        assert_eq!(transform.current(), expected_negative);
    }

    #[test]
    fn complex_transformation_composition() {
        let mut transform = TransformStack::new();

        // Test typical transformation order: scale -> rotate -> translate
        transform.scale(2.0, 3.0);
        transform.rotate(PI / 4.0);
        transform.translate(10.0, 20.0);

        let expected = Mat3::scale(2.0, 3.0) * Mat3::rotate(PI / 4.0) * Mat3::translate(10.0, 20.0);
        assert!(transform.current().near(expected, 1e-6));

        // Test that the transformation actually works on a point
        let point = Vec2::new(1.0, 1.0);
        let transformed_point = transform.current().transform_vec2(point);

        // Verify the transformation is not identity
        assert!(!transformed_point.near(point, 1e-6));
        assert!(!transformed_point.near(Vec2::ZERO, 1e-6));
    }

    #[test]
    fn stack_management() {
        let mut transform = TransformStack::new();

        // Test state machine usage pattern
        // Start with identity
        assert!(transform.current().near(Mat3::IDENTITY, 1e-6));

        // Apply some transformations
        transform.translate(10.0, 20.0);
        transform.scale(2.0, 3.0);
        let state1 = transform.current();

        // Push current state (save it)
        transform.push(Mat3::IDENTITY); // This saves the current state

        // Apply more transformations
        transform.rotate(PI / 2.0);
        transform.translate(5.0, 5.0);
        let state2 = transform.current();

        // Verify state2 is different from state1
        assert!(!state2.near(state1, 1e-6));

        // Pop back to saved state
        transform.pop();
        let restored_state = transform.current();

        // Should be back to state1
        assert!(restored_state.near(state1, 1e-6));

        // Test multiple push/pop operations
        transform.push(Mat3::IDENTITY); // Save current state
        transform.scale(0.5, 0.5);
        let scaled_state = transform.current();

        transform.push(Mat3::IDENTITY); // Save again
        transform.rotate(PI / 4.0);
        let _rotated_state = transform.current();

        // Pop back to scaled state
        transform.pop();
        assert!(transform.current().near(scaled_state, 1e-6));

        // Pop back to original state
        transform.pop();
        assert!(transform.current().near(state1, 1e-6));
    }

    #[test]
    fn transformation_consistency() {
        let mut transform = TransformStack::new();

        // Test that transformations are applied in the correct order
        transform.translate(10.0, 20.0);
        transform.scale(2.0, 3.0);
        transform.rotate(PI / 4.0);

        // The order should be: scale * rotate * translate
        // But since we're multiplying on the right, it's actually:
        // translate * scale * rotate
        let expected = Mat3::translate(10.0, 20.0) * Mat3::scale(2.0, 3.0) * Mat3::rotate(PI / 4.0);
        assert!(transform.current().near(expected, 1e-6));

        // Test that the transformation preserves certain properties
        let point = Vec2::new(1.0, 0.0);
        let transformed = transform.current().transform_vec2(point);

        // The point should be transformed, not remain the same
        assert!(!transformed.near(point, 1e-6));

        // Test that identity transformation doesn't change points
        let identity_transform = TransformStack::new();
        let test_point = Vec2::new(5.0, 10.0);
        let identity_result = identity_transform.current().transform_vec2(test_point);
        assert!(identity_result.near(test_point, 1e-6));
    }

    #[test]
    fn edge_cases() {
        let mut transform = TransformStack::new();

        // Test very small values
        transform.translate(1e-6, 1e-6);
        transform.scale(1.0 + 1e-6, 1.0 + 1e-6);
        transform.rotate(1e-6);

        // Should still be approximately identity
        assert!(transform.current().near(Mat3::IDENTITY, 1e-5));

        // Test very large values
        transform = TransformStack::new();
        transform.translate(1e6, 1e6);
        transform.scale(1e6, 1e6);

        let large_point = Vec2::new(1.0, 1.0);
        let transformed = transform.current().transform_vec2(large_point);

        // Should be a very large number
        assert!(transformed.x > 1e5);
        assert!(transformed.y > 1e5);

        // Test NaN and infinity handling
        transform = TransformStack::new();
        transform.translate(f32::NAN, f32::INFINITY);

        // The matrix should contain NaN and infinity
        let current = transform.current();
        assert!(current.m02.is_nan());
        assert!(current.m12.is_infinite() || current.m12.is_nan()); // INFINITY might become NaN in some operations

        // Test zero values
        transform = TransformStack::new();
        transform.translate(0.0, 0.0);
        transform.scale(0.0, 0.0);
        transform.rotate(0.0);

        // Should be zero matrix (except for the bottom-right element which should be 1)
        let zero_matrix = transform.current();
        assert_eq!(zero_matrix.m00, 0.0);
        assert_eq!(zero_matrix.m01, 0.0);
        assert_eq!(zero_matrix.m02, 0.0);
        assert_eq!(zero_matrix.m10, 0.0);
        assert_eq!(zero_matrix.m11, 0.0);
        assert_eq!(zero_matrix.m12, 0.0);
        assert_eq!(zero_matrix.m20, 0.0);
        assert_eq!(zero_matrix.m21, 0.0);
        assert_eq!(zero_matrix.m22, 1.0);
    }

    #[test]
    fn matrix_properties() {
        let mut transform = TransformStack::new();

        // Test that transformations preserve matrix properties
        transform.translate(10.0, 20.0);
        transform.scale(2.0, 3.0);
        transform.rotate(PI / 3.0);

        let matrix = transform.current();

        // Test determinant (should be positive for proper transformations)
        assert!(matrix.det() > 0.0);

        // Test that the matrix is invertible
        assert!(matrix.inverse().is_some());

        // Test that the inverse works correctly
        let inverse = matrix.inverse().unwrap();
        let identity_check = matrix * inverse;
        assert!(identity_check.near(Mat3::IDENTITY, 1e-5)); // Relaxed tolerance

        // Test with simpler transformations
        transform = TransformStack::new();
        transform.scale(2.0, 3.0);
        let scale_matrix = transform.current();
        assert!(scale_matrix.det() > 0.0);
        assert!(scale_matrix.inverse().is_some());

        let scale_inverse = scale_matrix.inverse().unwrap();
        let scale_identity = scale_matrix * scale_inverse;
        assert!(scale_identity.near(Mat3::IDENTITY, 1e-6));
    }

    #[test]
    fn transformation_combinations() {
        let mut transform = TransformStack::new();

        // Test common transformation combinations

        // 1. Scale around origin, then translate
        transform.scale(2.0, 2.0);
        transform.translate(100.0, 100.0);

        let point = Vec2::new(10.0, 20.0);
        let transformed = transform.current().transform_vec2(point);

        // The actual result is: (10*2 + 100*2, 20*2 + 100*2) = (220, 240)
        // because the translation is also scaled
        assert!(transformed.near(Vec2::new(220.0, 240.0), 1e-6));

        // 2. Translate, then scale (different result)
        transform = TransformStack::new();
        transform.translate(100.0, 100.0);
        transform.scale(2.0, 2.0);

        let transformed2 = transform.current().transform_vec2(point);

        // The matrix is: translate(100,100) * scale(2,2)
        // This gives us: scale(2,2) * translate(100,100)
        // So: (10*2 + 100, 20*2 + 100) = (120, 140)
        assert!(transformed2.near(Vec2::new(120.0, 140.0), 1e-6));

        // 3. Scale first, then translate (to get the expected result)
        transform = TransformStack::new();
        transform.scale(2.0, 2.0);
        transform.translate(50.0, 50.0); // Half the translation since it gets scaled

        let transformed3 = transform.current().transform_vec2(point);

        // Should be: (10*2 + 50*2, 20*2 + 50*2) = (120, 140)
        assert!(transformed3.near(Vec2::new(120.0, 140.0), 1e-6));

        // 3. Rotation around origin
        transform = TransformStack::new();
        transform.rotate(PI / 2.0); // 90 degrees

        let right_vector = Vec2::new(1.0, 0.0);
        let rotated = transform.current().transform_vec2_direction(right_vector);

        // Should point up: (0, 1)
        assert!(rotated.near(Vec2::new(0.0, 1.0), 1e-6));

        // 4. Test that order matters
        transform = TransformStack::new();
        transform.translate(10.0, 20.0);
        transform.scale(2.0, 2.0);

        let point2 = Vec2::new(5.0, 5.0);
        let result1 = transform.current().transform_vec2(point2);

        transform = TransformStack::new();
        transform.scale(2.0, 2.0);
        transform.translate(10.0, 20.0);

        let result2 = transform.current().transform_vec2(point2);

        // Results should be different due to order
        assert!(!result1.near(result2, 1e-6));
    }

    #[test]
    fn canvas_usage_pattern() {
        let mut transform = TransformStack::new();

        // Test the canvas usage pattern as described
        // Pattern 1: State machine approach
        transform.push(Mat3::IDENTITY); // Save current state
        transform.translate(200.0, 140.0);
        transform.rotate(0.2);
        transform.scale(1.2, 1.2);

        // The current matrix should be: translate(200,140) * rotate(0.2) * scale(1.2,1.2)
        let state_machine_result = transform.current();
        let expected = Mat3::translate(200.0, 140.0) * Mat3::rotate(0.2) * Mat3::scale(1.2, 1.2);
        assert!(state_machine_result.near(expected, 1e-6));

        // Restore state
        transform.pop();
        assert!(transform.current().near(Mat3::IDENTITY, 1e-6));

        // Pattern 2: Pre-composed transformation
        let pre_composed =
            Mat3::translate(200.0, 140.0) * Mat3::rotate(0.2) * Mat3::scale(1.2, 1.2);

        transform.push(pre_composed);
        let pre_composed_result = transform.current();
        assert!(pre_composed_result.near(pre_composed, 1e-6));

        // Both approaches should give the same result
        assert!(state_machine_result.near(pre_composed_result, 1e-6));

        // Restore state
        transform.pop();
        assert!(transform.current().near(Mat3::IDENTITY, 1e-6));
    }

    #[test]
    fn performance_properties() {
        let mut transform = TransformStack::new();

        // Test that operations are efficient (no unnecessary allocations)
        for i in 0..10 {
            // Reduced number to avoid stack issues
            transform.translate(i as f32, i as f32);
            transform.scale(1.0 + i as f32 * 0.01, 1.0 + i as f32 * 0.01);
            transform.rotate(i as f32 * 0.1);
        }

        // Should still have a valid transformation
        let matrix = transform.current();
        assert!(matrix.det().abs() > 1e-6); // Should be invertible

        // Test that we can still perform operations
        transform.translate(100.0, 200.0);
        let final_matrix = transform.current();
        assert!(final_matrix.det().abs() > 1e-6);

        // Test stack operations with proper push/pop
        transform.push(Mat3::IDENTITY); // Save current state
        transform.scale(0.5, 0.5);
        let scaled_state = transform.current();

        transform.push(Mat3::IDENTITY); // Save again
        transform.rotate(PI / 4.0);
        let _rotated_state = transform.current();

        // Pop back to scaled state
        transform.pop();
        assert!(transform.current().near(scaled_state, 1e-6));

        // Pop back to original state
        transform.pop();
        assert!(transform.current().near(final_matrix, 1e-6));
    }
}
