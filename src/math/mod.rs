pub(crate) mod barycentric;
pub(crate) mod consts;
pub(crate) mod ivec2;
pub(crate) mod mat2;
pub(crate) mod mat3;
pub(crate) mod space;
pub(crate) mod util;
pub(crate) mod vec2;
pub(crate) mod vec3;

pub use barycentric::{
    BarycentricCoords, barycentric, interpolate_color, interpolate_f32, interpolate_vec2,
    is_point_in_triangle,
};
pub use consts::EPS;
pub use ivec2::IVec2;
pub use mat2::Mat2;
pub use mat3::{AffineDecomposition, Decomposition, Mat3};
pub use space::{Line, Point2, Point3, Rect, ScreenPoint, ScreenVec2, WorldPoint, WorldVec2};
pub use util::{angle_delta, distance_point_to_line, mod_pos, perp_left, rad_to_deg};
pub use vec2::Vec2;
pub use vec3::Vec3;
