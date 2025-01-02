use std::ops::Add;

use crate::{interp::SphericalInterp, vector::Vec3};

use super::Scalar;

/// -Z forward, +Y up, +X right
///
/// yxz euler order
///
/// yaw, pitch, roll
///
/// when looking forward (-Z)
///
/// +Yaw turns right
///
/// +Pitch tilts down
///
/// +Roll spins counterclockwise
#[derive(Debug)]
pub struct Rotor {
    pub v: Vec3,
    pub a: Scalar,
}

impl Rotor {
    pub fn new(v: Vec3, angle: Scalar) -> Self {
        Self {
            v: v * angle.sin(),
            a: angle.cos(),
        }
    }

    pub fn new_yaw(angle: Scalar) -> Self {
        Self {
            v: Vec3::new(0.0, 0.0, angle.sin()),
            a: angle.cos(),
        }
    }

    pub fn new_pitch(angle: Scalar) -> Self {
        Self {
            v: Vec3::new(0.0, angle.sin(), 0.0),
            a: angle.cos(),
        }
    }

    pub fn new_roll(angle: Scalar) -> Self {
        Self {
            v: Vec3::new(angle.sin(), 0.0, 0.0),
            a: angle.cos(),
        }
    }

    // TODO: expand and optimize this
    pub fn new_euler(yaw: Scalar, pitch: Scalar, roll: Scalar) -> Self {
        Self::new_yaw(yaw)
            .mul(&Self::new_pitch(pitch))
            .mul(&Self::new_roll(roll))
    }

    pub fn new_look(from: Vec3, to: Vec3, roll: Scalar) -> Self {
        let dir = (to - from).normalized();

        Self::new_euler(dir.y.atan2(dir.x) * 0.5, -dir.z.asin() * 0.5, roll)
    }

    pub fn invert(&self) -> Self {
        Self {
            v: -self.v,
            a: self.a,
        }
    }

    // TODO: expand and optimize this
    pub fn mul(&self, other: &Rotor) -> Self {
        Self::new(
            other.v * self.a + self.v * other.a + self.v.cross(other.v),
            self.a * other.a - self.v.dot(other.v),
        )
    }

    pub fn add(&self, other: &Rotor) -> Self {
        Self::new(self.v + other.v, self.a + other.a)
    }

    // TODO: expand and optimize this
    pub fn rotate(&self, v: Vec3) -> Vec3 {
        let c = self.v.cross(v) * 2.0;
        v + c * self.a - c.cross(self.v)
    }
}

impl Add<Rotor> for Rotor {
    type Output = Rotor;

    fn add(self, rhs: Rotor) -> Self::Output {
        Rotor::add(&self, &rhs)
    }
}

impl SphericalInterp for Rotor {
    fn slerp(_a: Self, _b: Self, _t: Scalar) -> Self {
        todo!()
    }
}
