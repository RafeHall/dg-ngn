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
/// +Roll spins clockwise
#[derive(Debug)]
pub struct Rotor {
    pub a: Scalar,
    pub b: Vec3,
}

impl Rotor {
    
}

impl SphericalInterp for Rotor {
    fn slerp(_a: Self, _b: Self, _t: Scalar) -> Self {
        todo!()
    }
}
