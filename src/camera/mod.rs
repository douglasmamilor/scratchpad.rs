use crate::math::{Mat3, Point2, Rect, Vec3};

pub struct Camera {
    // Position in world space
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

    // Get the view matrix (for use with TransformStack)
    pub fn view_matrix(&self) -> Mat3 {
        // Column-major order
        Mat3::translate(self.viewport.width / 2.0, self.viewport.height / 2.0) // move camera origin to center of viewport
        * Mat3::scale(1.0/self.zoom, 1.0/self.zoom) // scale world opposite to camera zoom
        * Mat3::rotate(-self.rotation) // rotate world opposite to camera rotation
        * Mat3::translate(-self.position.x, -self.position.y) // move world to camera
    }

    // Camera movement helpers
    pub fn translate(&mut self, delta: Point2) {
        self.position = self.position + delta;
    }

    pub fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
    }

    pub fn zoom_by(&mut self, factor: f32) {
        self.zoom *= factor;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(Point2::ZERO, 1.0, 0.0, Rect::new(0.0, 0.0, 800.0, 600.0))
    }
}
