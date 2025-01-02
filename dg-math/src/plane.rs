use crate::Side;

use super::{vector::Vec3, Scalar};

// #[derive(Debug, Clone, Copy)]
// pub struct Plane(Vec3, Vec3, Vec3);

// impl Plane {
//     pub const XY_PLANE: Plane = Self::new(Vec3::RIGHT, Vec3::UP, Vec3::ZERO);
//     pub const XZ_PLANE: Plane = Self::new(Vec3::RIGHT, Vec3::FORWARD, Vec3::ZERO);
//     pub const YZ_PLANE: Plane = Self::new(Vec3::UP, Vec3::FORWARD, Vec3::ZERO);

//     pub const fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
//         Self(a, b, c)
//     }

//     pub fn new_from_normal(normal: Vec3, distance: Scalar) -> Self {
//         let normal = normal.normalized();
//         let center = normal.mul_scalar(distance);
//         let right = normal.cross(Vec3::UP);
//         let up = right.cross(normal);

//         Self(center, center.add(right), center.add(up))
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    normal: Vec3,
    distance: Scalar,
}

impl Plane {
    pub fn new(normal: Vec3, distance: Scalar) -> Self {
        Self {
            normal,
            distance,
        }
    }

    pub fn distance_to(&self, point: Vec3) -> Scalar {
        self.normal.dot(point) - self.distance
    }

    pub fn side(&self, point: Vec3, eps: Scalar) -> Side {
        match self.normal.dot(point) - self.distance {
            v if v > eps => Side::Outside,
            v if v < -eps => Side::Inside,
            _ => Side::On,
        }
        // if self.normal.dot(point) < self.distance {
        //     Side::Inside
        // } else {
        //     Side::Outside
        // }
    }

    pub fn project(&self, point: Vec3) -> Vec3 {
        point.sub(self.normal).mul_scalar(self.distance_to(point))
    }

    // pub fn on_plane(&self, point: Vec3, eps: Scalar) -> bool {
    //     let v = self.normal.dot(point) - self.distance;

    //     v.abs() <= eps
    // }
}
