#![allow(dead_code)]
pub mod camera;
pub mod canvas;
pub mod color;
pub mod framebuffer;
pub mod math;
pub mod renderer;
pub mod transform;
pub mod ui;
pub mod window;

// Re-export commonly used types for convenience
pub use camera::Camera;
pub use math::space::{ScreenPoint, ScreenVec2, WorldPoint, WorldVec2};
pub use math::{Mat3, Point2, Rect, Vec2};
pub use transform::TransformStack;
pub use ui::Anchor;
