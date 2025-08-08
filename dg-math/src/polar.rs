use crate::{vector::Vec2, Scalar};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Polar {
    pub radius: Scalar,
    pub angle: Scalar,
}

impl Polar {
    pub fn new(radius: Scalar, angle: Scalar) -> Self {
        Self {
            radius,
            angle,
        }
    }
}

impl Into<Vec2> for Polar {
    fn into(self) -> Vec2 {
        Vec2::new(
            self.angle.cos() * self.radius,
            self.angle.sin() * self.radius,
        )
    }
}
