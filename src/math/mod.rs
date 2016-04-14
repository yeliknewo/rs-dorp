mod vec2;
mod vec3;
mod vec4;
mod mat4;

pub use self::vec2::Vec2;
pub use self::vec3::Vec3;
pub use self::vec4::Vec4;
pub use self::mat4::Mat4;

use std::f32::consts::{PI};

pub const DEG_TO_RAD: f32 = PI / 180.0;
