use crate::math::Vec2;

pub trait Lerp: Copy {
    fn lerp(start: Self, end: Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(start: Self, end: Self, t: f32) -> Self {
        start + (end - start) * t
    }
}

impl Lerp for Vec2 {
    fn lerp(start: Self, end: Self, t: f32) -> Self {
        start.lerp(end, t)
    }
}
