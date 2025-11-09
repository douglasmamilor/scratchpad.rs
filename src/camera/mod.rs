mod viewport;

use crate::math::space::{ScreenPoint, WorldPoint};
use crate::math::{Mat3, Point2, Rect, Vec3};

pub struct Camera {
    // Position in world space. A cameras position is the point it is **centred on**.
    position: Point2,
    // Zoom level (1.0 = normal, >1.0 = zoomed in, <1.0 = zoomed out)
    zoom: f32,
    // Rotation in radians (0.0 = no rotation)
    rotation: f32,
    // Viewport bounds for screen space
    viewport: Rect,
}

impl Camera {
    // Create a new camera
    pub fn new(position: Point2, zoom: f32, rotation: f32, viewport: Rect) -> Self {
        Self {
            position,
            zoom,
            rotation,
            viewport,
        }
    }

    pub fn default(viewport: Rect) -> Self {
        Self::new(Point2::ZERO, 1.0, 0.0, viewport)
    }

    // Convert world coordinates to screen coordinates
    pub fn world_to_screen(&self, world_point: Point2) -> Point2 {
        let world_vec3 = Vec3::new(world_point.x, world_point.y, 1.0);
        let view_matrix = self.view_matrix();

        let screen_point = view_matrix * world_vec3;

        Point2::new(screen_point.x, screen_point.y)
    }

    // Convert screen coordinates to world coordinates
    pub fn screen_to_world(&self, screen_point: Point2) -> Point2 {
        let screenvec3 = Vec3::new(screen_point.x, screen_point.y, 1.0);
        let inv_view_matrix = self
            .view_matrix()
            .inverse()
            .expect("View matrix must be invertible");

        let world_point = inv_view_matrix * screenvec3;

        Point2::new(world_point.x, world_point.y)
    }

    // Type-safe conversion: WorldPoint to ScreenPoint
    pub fn world_to_screen_space(&self, world_point: WorldPoint) -> ScreenPoint {
        let screen = self.world_to_screen(world_point.to_point2());
        ScreenPoint::from_point2(screen)
    }

    // Type-safe conversion: ScreenPoint to WorldPoint
    pub fn screen_to_world_space(&self, screen_point: ScreenPoint) -> WorldPoint {
        let world = self.screen_to_world(screen_point.to_point2());
        WorldPoint::from_point2(world)
    }

    // Get the view matrix (for use with TransformStack).
    // Applied from right to left since this is column major order
    pub fn view_matrix(&self) -> Mat3 {
        // Apply Y-flip in screen space and then reposition to viewport. This needs to be applied
        // AFTER the transformations have moved everything into screen space.
        Mat3::translate(0.0, self.viewport.height)  // y = -y + height
        * Mat3::scale(1.0, -1.0)
        // Viewport transformations get applied first
        * Mat3::translate(self.viewport.width / 2.0, self.viewport.height / 2.0) // move camera origin to center of viewport
        * Mat3::scale(1.0/self.zoom, 1.0/self.zoom) // scale world opposite to camera zoom
        * Mat3::rotate(-self.rotation) // rotate world opposite to camera rotation
        * Mat3::translate(-self.position.x, -self.position.y) // move world based on camera pos
    }

    // Camera movement helpers
    pub fn translate(&mut self, delta: Point2) {
        self.position += delta;
    }

    pub fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
    }

    pub fn zoom_by(&mut self, factor: f32) {
        self.zoom *= factor;
    }

    // Viewport management
    /// Update viewport when window resizes
    pub fn set_viewport(&mut self, viewport: Rect) {
        self.viewport = viewport;
    }

    /// Get current viewport
    pub fn viewport(&self) -> Rect {
        self.viewport
    }

    /// Get viewport center in screen coordinates
    pub fn viewport_center(&self) -> Point2 {
        Point2::new(
            self.viewport.x + self.viewport.width / 2.0,
            self.viewport.y + self.viewport.height / 2.0,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(Point2::ZERO, 1.0, 0.0, Rect::new(0.0, 0.0, 800.0, 600.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn y_is_up_validation() {
        // Simple targeted test to verify Y is pointing up in world space
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera = Camera::default(viewport);

        // Get screen center (where world origin maps to)
        let world_origin = Point2::new(0.0, 0.0);
        let screen_center = camera.world_to_screen(world_origin);

        // Test 1: World point above origin (Y+ = up) should map to screen Y BELOW center
        // In world: Y+ means "up", in screen: Y+ means "down"
        // So after Y-flip: world Y+ should map to screen Y- (lower Y value)
        let world_above = Point2::new(0.0, 100.0); // 100 units UP in world
        let screen_above = camera.world_to_screen(world_above);
        
        assert!(screen_above.y < screen_center.y, 
                "Y-flip validation failed: World Y+100 (up) should map to screen Y BELOW center. \
                 Got: screen_above.y={}, screen_center.y={}", 
                screen_above.y, screen_center.y);
        
        // Test 2: World point below origin (Y- = down) should map to screen Y ABOVE center
        let world_below = Point2::new(0.0, -100.0); // 100 units DOWN in world
        let screen_below = camera.world_to_screen(world_below);
        
        assert!(screen_below.y > screen_center.y,
                "Y-flip validation failed: World Y-100 (down) should map to screen Y ABOVE center. \
                 Got: screen_below.y={}, screen_center.y={}",
                screen_below.y, screen_center.y);

        // Test 3: Verify the distance is correct (100 world units = 100 screen pixels)
        let distance_above = screen_center.y - screen_above.y;
        let distance_below = screen_below.y - screen_center.y;
        
        assert!((distance_above - 100.0).abs() < 1e-5,
                "Y-flip validation failed: World Y+100 should be exactly 100 pixels from center. Got: {}", distance_above);
        assert!((distance_below - 100.0).abs() < 1e-5,
                "Y-flip validation failed: World Y-100 should be exactly 100 pixels from center. Got: {}", distance_below);
    }

    #[test]
    fn construction() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let position = Point2::new(100.0, 50.0);
        let camera = Camera::new(position, 2.0, PI / 4.0, viewport);

        // Check that camera stores values correctly
        // Note: Since fields are private, we verify through behavior
        assert!(!camera.view_matrix().is_identity());
    }

    #[test]
    fn default_construction() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera = Camera::default(viewport);

        // Default camera should have identity-like behavior (no transform)
        let world_point = Point2::new(0.0, 0.0);
        let screen_point = camera.world_to_screen(world_point);

        // World origin should map to viewport center (with Y-flip: 400, 300)
        assert!((screen_point.x - 400.0).abs() < 1e-5);
        assert!((screen_point.y - 300.0).abs() < 1e-5);
    }

    #[test]
    fn default_trait() {
        use std::default::Default;
        let camera = <Camera as Default>::default();

        // Test through behavior - default camera should use default viewport
        let world_origin = Point2::new(0.0, 0.0);
        let screen_point = camera.world_to_screen(world_origin);

        // Default viewport is 800x600, with Y-flip: world (0, 0) -> screen (400, 300)
        assert!((screen_point.x - 400.0).abs() < 1e-5);
        assert!((screen_point.y - 300.0).abs() < 1e-5);
    }

    #[test]
    fn world_to_screen_identity() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera = Camera::default(viewport);

        // Test world origin maps to screen center (with Y-flip: 400, 300)
        let world_origin = Point2::new(0.0, 0.0);
        let screen_point = camera.world_to_screen(world_origin);
        assert!((screen_point.x - 400.0).abs() < 1e-5);
        assert!((screen_point.y - 300.0).abs() < 1e-5);
    }

    #[test]
    fn world_to_screen_translation() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let position = Point2::new(100.0, 50.0);
        let camera = Camera::new(position, 1.0, 0.0, viewport);

        // World point at camera position should map to screen center
        let world_point = Point2::new(100.0, 50.0);
        let screen_point = camera.world_to_screen(world_point);
        assert!((screen_point.x - 400.0).abs() < 1e-5);
        assert!((screen_point.y - 300.0).abs() < 1e-5); // With Y-flip: (400, 300)
    }

    #[test]
    fn world_to_screen_zoom() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera = Camera::new(Point2::ZERO, 2.0, 0.0, viewport);

        // With 2x zoom, world point (100, 100) should appear at half the distance from center
        let world_point = Point2::new(100.0, 100.0);
        let screen_point = camera.world_to_screen(world_point);

        // Screen center is (400, 300), world (100, 100) with 2x zoom scales to (50, 50) relative to origin
        // After Y-flip: world Y+100 becomes screen Y-50 from center, so (450, 250)
        assert!((screen_point.x - 450.0).abs() < 1e-5);
        assert!((screen_point.y - 250.0).abs() < 1e-5);
    }

    #[test]
    fn world_to_screen_rotation() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera = Camera::new(Point2::ZERO, 1.0, PI / 2.0, viewport);

        // Point at (100, 0) in world should rotate 90 degrees
        let world_point = Point2::new(100.0, 0.0);
        let screen_point = camera.world_to_screen(world_point);

        // After 90-degree rotation: (100, 0) -> (0, -100) relative to origin
        // Screen center is (400, 300), with Y-flip: world Y-100 becomes screen Y+100 from center, so (400, 400)
        assert!((screen_point.x - 400.0).abs() < 1e-5);
        assert!((screen_point.y - 400.0).abs() < 1e-4); // Slightly increased tolerance for floating point precision
    }

    #[test]
    fn screen_to_world_identity() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera = Camera::default(viewport);

        // Screen center (400, 300) should map to world origin (0, 0)
        let screen_center = Point2::new(400.0, 300.0);
        let world_point = camera.screen_to_world(screen_center);
        assert!((world_point.x - 0.0).abs() < 1e-5);
        assert!((world_point.y - 0.0).abs() < 1e-5);
    }

    #[test]
    fn screen_to_world_round_trip() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera = Camera::new(Point2::new(50.0, 25.0), 1.5, PI / 6.0, viewport);

        let test_points = vec![
            Point2::new(0.0, 0.0),
            Point2::new(100.0, 200.0),
            Point2::new(-50.0, -100.0),
            Point2::new(500.0, 300.0),
        ];

        for world_point in test_points {
            let screen_point = camera.world_to_screen(world_point);
            let back_to_world = camera.screen_to_world(screen_point);

            // Increased tolerance for Y-flip precision (matrix inversion can accumulate errors)
            assert!(
                (back_to_world.x - world_point.x).abs() < 1e-3,
                "Round trip failed for world point {:?}, got {:?}",
                world_point,
                back_to_world
            );
            assert!(
                (back_to_world.y - world_point.y).abs() < 1e-3,
                "Round trip failed for world point {:?}, got {:?}",
                world_point,
                back_to_world
            );
        }
    }

    #[test]
    fn view_matrix_composition() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera = Camera::new(Point2::new(100.0, 50.0), 2.0, PI / 4.0, viewport);
        let view_matrix = camera.view_matrix();

        // View matrix should be invertible (determinant != 0)
        let det = view_matrix.det();
        assert!(det.abs() > 1e-6, "View matrix should be invertible");

        // View matrix should have an inverse
        assert!(view_matrix.inverse().is_some());
    }

    #[test]
    fn translate_movement() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let mut camera = Camera::default(viewport);

        // Initial position should be at origin
        let world_origin = Point2::new(0.0, 0.0);
        let screen_center_initial = camera.world_to_screen(world_origin);
        assert!((screen_center_initial.x - 400.0).abs() < 1e-5);
        assert!((screen_center_initial.y - 300.0).abs() < 1e-5);

        // Translate camera by (50, 25) - camera moves to (50, 25) in world
        camera.translate(Point2::new(50.0, 25.0));

        // Now world origin (0, 0) is at (-50, -25) relative to camera
        // With Y-flip: world Y-25 becomes screen Y+25 from center
        let screen_center_after = camera.world_to_screen(world_origin);
        assert!((screen_center_after.x - 350.0).abs() < 1e-5); // Moved left by 50
        assert!((screen_center_after.y - 325.0).abs() < 1e-5); // Moved down by 25: 300 + 25 = 325
    }

    #[test]
    fn rotate_movement() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let mut camera = Camera::default(viewport);

        // Point at (100, 0) initially maps to (500, 300) on screen
        let world_point = Point2::new(100.0, 0.0);
        let initial_screen = camera.world_to_screen(world_point);

        // Rotate camera 90 degrees
        camera.rotate(PI / 2.0);

        // After rotation, the point should appear rotated
        let rotated_screen = camera.world_to_screen(world_point);

        // Should be different from initial
        assert!(
            (initial_screen.x - rotated_screen.x).abs() > 1e-5
                || (initial_screen.y - rotated_screen.y).abs() > 1e-5
        );
    }

    #[test]
    fn zoom_by_movement() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let mut camera = Camera::default(viewport);

        // Initial 1x zoom
        let world_point = Point2::new(100.0, 100.0);
        let initial_screen = camera.world_to_screen(world_point);
        let initial_center = camera.world_to_screen(Point2::new(0.0, 0.0));
        let initial_dist_from_center = ((initial_screen.x - initial_center.x).powi(2)
            + (initial_screen.y - initial_center.y).powi(2))
        .sqrt();

        // Zoom in by 2x
        camera.zoom_by(2.0);

        // Point should appear closer to center (half distance)
        let zoomed_screen = camera.world_to_screen(world_point);
        let zoomed_center = camera.world_to_screen(Point2::new(0.0, 0.0));
        let zoomed_dist_from_center = ((zoomed_screen.x - zoomed_center.x).powi(2)
            + (zoomed_screen.y - zoomed_center.y).powi(2))
        .sqrt();

        assert!(
            (zoomed_dist_from_center - initial_dist_from_center / 2.0).abs() < 1e-5,
            "Zoom should halve the distance from center: initial={}, zoomed={}",
            initial_dist_from_center,
            zoomed_dist_from_center
        );
    }

    #[test]
    fn zoom_by_multiple() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let mut camera = Camera::default(viewport);

        camera.zoom_by(2.0);
        // Verify zoom through behavior - point should be closer to center
        let world_point = Point2::new(100.0, 100.0);
        let screen_2x = camera.world_to_screen(world_point);
        let center_2x = camera.world_to_screen(Point2::new(0.0, 0.0));
        let dist_2x =
            ((screen_2x.x - center_2x.x).powi(2) + (screen_2x.y - center_2x.y).powi(2)).sqrt();

        camera.zoom_by(1.5); // Now at 3x total
        let screen_3x = camera.world_to_screen(world_point);
        let center_3x = camera.world_to_screen(Point2::new(0.0, 0.0));
        let dist_3x =
            ((screen_3x.x - center_3x.x).powi(2) + (screen_3x.y - center_3x.y).powi(2)).sqrt();

        // With higher zoom, the point should be closer to center (smaller distance)
        // Note: zoom scales world down, so higher zoom = smaller world units = closer to center
        assert!(
            dist_3x < dist_2x,
            "3x zoom dist ({}) should be smaller than 2x zoom dist ({})",
            dist_3x,
            dist_2x
        );

        camera.zoom_by(0.5); // Now at 1.5x total
        let screen_1_5x = camera.world_to_screen(world_point);
        let center_1_5x = camera.world_to_screen(Point2::new(0.0, 0.0));
        let dist_1_5x = ((screen_1_5x.x - center_1_5x.x).powi(2)
            + (screen_1_5x.y - center_1_5x.y).powi(2))
        .sqrt();
        // 1.5x should be further from center than 3x (lower zoom = larger distance)
        assert!(dist_1_5x > dist_3x);
    }

    #[test]
    fn rotate_accumulation() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let mut camera = Camera::default(viewport);

        // Test rotation through behavior
        let world_point = Point2::new(100.0, 0.0);
        let initial_screen = camera.world_to_screen(world_point);

        camera.rotate(PI / 4.0);
        let screen_45 = camera.world_to_screen(world_point);
        assert!(
            (initial_screen.x - screen_45.x).abs() > 1e-5
                || (initial_screen.y - screen_45.y).abs() > 1e-5
        );

        camera.rotate(PI / 4.0);
        let screen_90 = camera.world_to_screen(world_point);
        assert!(
            (screen_45.x - screen_90.x).abs() > 1e-5 || (screen_45.y - screen_90.y).abs() > 1e-5
        );
    }

    #[test]
    fn translate_accumulation() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let mut camera = Camera::default(viewport);

        camera.translate(Point2::new(10.0, 20.0));
        camera.translate(Point2::new(5.0, 10.0));

        // Position should be accumulated: camera is now at (15, 30)
        let world_origin = Point2::new(0.0, 0.0);
        let screen_point = camera.world_to_screen(world_origin);

        // World origin is at (-15, -30) relative to camera
        // With Y-flip: world Y-30 becomes screen Y+30 from center
        assert!((screen_point.x - 385.0).abs() < 1e-5); // 400 - 15 = 385
        assert!((screen_point.y - 330.0).abs() < 1e-5); // 300 + 30 = 330
    }

    #[test]
    fn complex_transformation() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera = Camera::new(Point2::new(200.0, 100.0), 1.5, PI / 6.0, viewport);

        let world_point = Point2::new(50.0, 25.0);
        let screen_point = camera.world_to_screen(world_point);
        let back_to_world = camera.screen_to_world(screen_point);

        // Should round-trip accurately
        assert!((back_to_world.x - world_point.x).abs() < 1e-4);
        assert!((back_to_world.y - world_point.y).abs() < 1e-4);
    }

    #[test]
    fn viewport_changes() {
        let viewport1 = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera1 = Camera::default(viewport1);

        let viewport2 = Rect::new(0.0, 0.0, 1920.0, 1080.0);
        let camera2 = Camera::default(viewport2);

        let world_origin = Point2::new(0.0, 0.0);
        let screen1 = camera1.world_to_screen(world_origin);
        let screen2 = camera2.world_to_screen(world_origin);

        // Both should map to their respective viewport centers
        assert!((screen1.x - 400.0).abs() < 1e-5); // 800/2 = 400
        assert!((screen1.y - 300.0).abs() < 1e-5); // 600/2 = 300

        assert!((screen2.x - 960.0).abs() < 1e-5); // 1920/2 = 960
        assert!((screen2.y - 540.0).abs() < 1e-5); // 1080/2 = 540
    }

    #[test]
    fn world_to_screen_space_type_safe() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera = Camera::new(Point2::new(0.0, 0.0), 1.0, 0.0, viewport);

        let world_point = WorldPoint::new(100.0, 50.0);
        let screen_point = camera.world_to_screen_space(world_point);

        // Verify type safety - these are different types
        // Screen center is (400, 300), world (100, 50) relative to camera
        // With Y-flip: world Y+50 becomes screen Y-50 from center
        assert_eq!(screen_point.x(), 500.0); // 400 + 100 = 500
        assert_eq!(screen_point.y(), 250.0); // 300 - 50 = 250
    }

    #[test]
    fn screen_to_world_space_type_safe() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera = Camera::new(Point2::new(0.0, 0.0), 1.0, 0.0, viewport);

        // Screen (500, 300) is at center (400, 300) + (100, 0)
        // With Y-flip: screen Y 0 (at center) means world Y 0
        let screen_point = ScreenPoint::new(500.0, 300.0);
        let world_point = camera.screen_to_world_space(screen_point);

        // Should convert back correctly: world (100, 0)
        assert_eq!(world_point.x(), 100.0);
        assert_eq!(world_point.y(), 0.0);
    }

    #[test]
    fn coordinate_space_round_trip() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera = Camera::new(Point2::new(0.0, 0.0), 1.0, 0.0, viewport);

        let original = WorldPoint::new(123.0, 456.0);
        let screen = camera.world_to_screen_space(original);
        let back_to_world = camera.screen_to_world_space(screen);

        // Should round-trip correctly
        assert!((original.x() - back_to_world.x()).abs() < 1e-5);
        assert!((original.y() - back_to_world.y()).abs() < 1e-5);
    }

    #[test]
    fn edge_cases() {
        // Very small zoom
        let viewport1 = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera_small_zoom = Camera::new(Point2::ZERO, 0.01, 0.0, viewport1);
        let world_point = Point2::new(1000.0, 1000.0);
        let _screen_point = camera_small_zoom.world_to_screen(world_point);
        // Should still be invertible
        assert!(camera_small_zoom.view_matrix().inverse().is_some());

        // Very large zoom
        let viewport2 = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera_large_zoom = Camera::new(Point2::ZERO, 100.0, 0.0, viewport2);
        let _screen_point2 = camera_large_zoom.world_to_screen(world_point);
        // Should still be invertible
        assert!(camera_large_zoom.view_matrix().inverse().is_some());

        // Negative translation (should work fine)
        let viewport3 = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera_negative = Camera::new(Point2::new(-100.0, -50.0), 1.0, 0.0, viewport3);
        let screen_point3 = camera_negative.world_to_screen(Point2::new(-100.0, -50.0));
        assert!((screen_point3.x - 400.0).abs() < 1e-5);
    }

    #[test]
    fn rotation_wrapping() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let mut camera = Camera::default(viewport);

        // Rotate multiple times
        camera.rotate(PI * 2.0); // Full rotation
        camera.rotate(PI * 2.0); // Another full rotation

        // Rotation accumulates (no automatic wrapping)
        assert!((camera.rotation - PI * 4.0).abs() < 1e-6);
    }

    #[test]
    fn viewport_getter() {
        let viewport = Rect::new(10.0, 20.0, 800.0, 600.0);
        let camera = Camera::default(viewport);

        let retrieved_viewport = camera.viewport();
        assert_eq!(retrieved_viewport.x, 10.0);
        assert_eq!(retrieved_viewport.y, 20.0);
        assert_eq!(retrieved_viewport.width, 800.0);
        assert_eq!(retrieved_viewport.height, 600.0);
    }

    #[test]
    fn viewport_setter() {
        let initial_viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let mut camera = Camera::default(initial_viewport);

        // Change viewport
        let new_viewport = Rect::new(0.0, 0.0, 1920.0, 1080.0);
        camera.set_viewport(new_viewport);

        // Verify viewport changed
        let retrieved = camera.viewport();
        assert_eq!(retrieved.width, 1920.0);
        assert_eq!(retrieved.height, 1080.0);

        // Verify it affects coordinate conversion
        let world_origin = Point2::new(0.0, 0.0);
        let screen_point = camera.world_to_screen(world_origin);
        // Should now map to new viewport center (960, 540)
        assert!((screen_point.x - 960.0).abs() < 1e-5); // 1920/2 = 960
        assert!((screen_point.y - 540.0).abs() < 1e-5); // 1080/2 = 540
    }

    #[test]
    fn viewport_center() {
        let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
        let camera = Camera::default(viewport);

        let center = camera.viewport_center();
        assert_eq!(center.x, 400.0);
        assert_eq!(center.y, 300.0);
    }

    #[test]
    fn viewport_center_offset() {
        let viewport = Rect::new(100.0, 50.0, 800.0, 600.0);
        let camera = Camera::default(viewport);

        let center = camera.viewport_center();
        // Center should be (100 + 400, 50 + 300) = (500, 350)
        assert_eq!(center.x, 500.0);
        assert_eq!(center.y, 350.0);
    }
}
