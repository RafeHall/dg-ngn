use crate::{vector::Vec2, Scalar};

#[derive(Debug)]
pub enum Projection {
    Orthographic {
        offset: Vec2,
        size: Vec2,
        near: Scalar,
        far: Scalar,
    },
    Perspective {
        fov_y: Scalar,
        aspect: Scalar,
        near: Scalar,
        far: Scalar,
    },
}

impl Projection {}

impl Default for Projection {
    fn default() -> Self {
        Self::Orthographic {
            offset: Vec2::ZERO,
            size: Vec2::new(1280.0, 720.0),
            near: -100.0,
            far: 100.0,
        }
    }
}
