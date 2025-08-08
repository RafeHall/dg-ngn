use crate::{scalar, vector::Vec2, Scalar};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Circle {
    pub radius: Scalar,
    pub center: Vec2,
}

impl Circle {
    pub fn new(radius: Scalar, center: Vec2) -> Self {
        Self { radius, center }
    }

    pub fn points(&self, resolution: usize) -> Vec<Vec2> {
        (0..resolution)
            .map(|i| i as Scalar / resolution as Scalar * 2.0 * scalar::consts::PI)
            .map(|angle| {
                Vec2::new_from_angle(angle)
                    .mul_scalar(self.radius)
                    .add(self.center)
            })
            .collect()
    }
}
