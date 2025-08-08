#[cfg(feature = "simd")]
pub mod simd;

pub mod vec2;

pub mod vec3;
pub mod vec4;
pub mod vec_n;

pub mod ivec2;


pub use vec2::Vec2;

pub use vec3::Vec3;
pub use vec4::Vec4;
pub use vec_n::VecN;

pub use ivec2::IVec2;