use std::{
    fmt::Display,
    iter::Sum,
    ops::{Index, IndexMut},
};

use crate::{
    interp::{LinearInterp, SphericalInterp},
    ApproxEq, Scalar,
};

use super::{Vec2, Vec4};

/// Representation of a vector in 3d space using `x`, `y`, and `z` [`Scalar`]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Vec3 {
    /// [`Vec3`] with `1.0` in the `y` component
    pub const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    /// [`Vec3`] with `-1.0` in the `y` component
    pub const DOWN: Vec3 = Vec3::new(0.0, -1.0, 0.0);
    /// [`Vec3`] with `1.0` in the `x` component
    pub const RIGHT: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    /// [`Vec3`] with `-1.0` in the `x` component
    pub const LEFT: Vec3 = Vec3::new(-1.0, 0.0, 0.0);
    /// [`Vec3`] with `-1.0` in the `z` component
    pub const FORWARD: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    /// [`Vec3`] with `1.0` in the `z` component
    pub const BACK: Vec3 = Vec3::new(0.0, 0.0, 1.0);

    /// [`Vec3`] with `0.0` in all components
    pub const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    /// [`Vec3`] with `1.0` in all components
    pub const ONE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    /// [`Vec3`] with `-1.0` in all components
    pub const NEG_ONE: Vec3 = Vec3::new(-1.0, -1.0, -1.0);

    /// [`Vec3`] with [`Scalar::INFINITY`] in all components
    pub const INFINITY: Vec3 = Vec3::new(Scalar::INFINITY, Scalar::INFINITY, Scalar::INFINITY);
    /// [`Vec3`] with [`Scalar::NEG_INFINITY`] in all components
    pub const NEG_INFINITY: Vec3 = Vec3::new(
        Scalar::NEG_INFINITY,
        Scalar::NEG_INFINITY,
        Scalar::NEG_INFINITY,
    );

    /// [`Vec3`] with [`Scalar::NAN`] in all components
    pub const NAN: Vec3 = Vec3::new(Scalar::NAN, Scalar::NAN, Scalar::NAN);

    /// Contruct [`Vec3`] using `x`, `y` and `z` [`Scalar`]
    pub const fn new(x: Scalar, y: Scalar, z: Scalar) -> Vec3 {
        Self { x, y, z }
    }

    pub fn add(&self, other: Vec3) -> Vec3 {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: Vec3) -> Vec3 {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn div(&self, other: Vec3) -> Vec3 {
        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }

    pub fn mul(&self, other: Vec3) -> Vec3 {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    pub fn add_scalar(&self, other: Scalar) -> Vec3 {
        Self::new(self.x + other, self.y + other, self.z + other)
    }

    pub fn sub_scalar(&self, other: Scalar) -> Vec3 {
        Self::new(self.x - other, self.y - other, self.z - other)
    }

    pub fn div_scalar(&self, other: Scalar) -> Vec3 {
        Self::new(self.x / other, self.y / other, self.z / other)
    }

    pub fn mul_scalar(&self, other: Scalar) -> Vec3 {
        Self::new(self.x * other, self.y * other, self.z * other)
    }

    pub fn neg(&self) -> Vec3 {
        Self::new(-self.x, -self.y, -self.z)
    }

    pub fn inverse(&self) -> Vec3 {
        self.neg()
    }

    pub fn reciprocal(&self) -> Vec3 {
        Self::new(1.0 / self.x, 1.0 / self.y, 1.0 / self.z)
    }

    pub fn truncate(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn extend(&self, w: Scalar) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, w)
    }

    /// Returns true if all components of `self` are greater than `other`
    pub fn greater_than(&self, other: Vec3) -> bool {
        self.x > other.x && self.y > other.y && self.z > other.z
    }

    /// Returns true if all components of `self` are greater than or equal to `other`
    pub fn greater_than_equals(&self, other: Vec3) -> bool {
        self.x >= other.x && self.y >= other.y && self.z >= other.z
    }

    /// Returns true if all components of `self` are greater than `other`
    pub fn less_than(&self, other: Vec3) -> bool {
        self.x < other.x && self.y < other.y && self.z < other.z
    }

    /// Returns true if all components of `self` are less than or equal to `other`
    pub fn less_than_equals(&self, other: Vec3) -> bool {
        self.x <= other.x && self.y <= other.y && self.z <= other.z
    }

    pub fn max(&self) -> Scalar {
        self.x.max(self.y.max(self.z))
    }

    pub fn vmax(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    pub fn imax(&self) -> usize {
        match (self.x, self.y, self.z) {
            (x, y, z) if x > y && x > z => 0,
            (x, y, z) if y > x && y > z => 1,
            (x, y, z) if z > x && z > y => 2,
            _ => 0,
        }
    }

    pub fn min(&self) -> Scalar {
        self.x.min(self.y.min(self.x))
    }

    pub fn vmin(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    pub fn imin(&self) -> usize {
        match (self.x, self.y, self.z) {
            (x, y, z) if x < y && x < z => 0,
            (x, y, z) if y < x && y < z => 1,
            (x, y, z) if z < x && z < y => 2,
            _ => 0,
        }
    }

    pub fn dot(&self, other: Vec3) -> Scalar {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn to(&self, to: Vec3) -> Vec3 {
        to.sub(*self)
    }

    pub fn direction_to(&self, to: Vec3) -> Vec3 {
        self.to(to).normalized()
    }

    pub fn distance_to(&self, to: Vec3) -> Scalar {
        self.distance_squared_to(to).sqrt()
    }

    pub fn distance_squared_to(&self, to: Vec3) -> Scalar {
        // NOTE: x.powi(2) is just as fast as x * x
        (to.x - self.x).powi(2) + (to.y - self.y).powi(2) + (to.z - self.z).powi(2)
    }

    pub fn length(&self) -> Scalar {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> Scalar {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalized(&self) -> Vec3 {
        self.div_scalar(self.length())
    }

    pub fn clamp(&self, min: Vec3, max: Vec3) -> Vec3 {
        Self::new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
            self.z.clamp(min.z, max.z),
        )
    }

    pub fn clamp_scalar(&self, min: Scalar, max: Scalar) -> Vec3 {
        Self::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
        )
    }

    pub fn round(&self) -> Vec3 {
        Self::new(self.x.round(), self.y.round(), self.z.round())
    }

    pub fn floor(&self) -> Vec3 {
        Self::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    pub fn ceil(&self) -> Vec3 {
        Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }

    pub fn abs(&self) -> Vec3 {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        let dot2 = self.dot(normal) * 2.0;

        Self::new(
            self.x - dot2 * normal.x,
            self.y - dot2 * normal.y,
            self.z - dot2 * normal.z,
        )
    }

    pub fn bounce(&self, normal: Vec3, strength: Scalar) -> Vec3 {
        self.reflect(normal).mul_scalar(strength)
    }

    pub fn tangents(&self) -> (Vec3, Vec3) {
        let mut tangent = self.cross(Vec3::UP);
        if tangent.length_squared() < Self::EPS {
            tangent = self.cross(Vec3::RIGHT);
        }

        let cotangent = self.cross(tangent);

        (tangent, cotangent)
    }
}

impl ApproxEq for Vec3 {
    fn approx_eq(&self, other: &Self) -> bool {
        self.x.approx_eq(&other.x) && self.y.approx_eq(&other.y) && self.z.approx_eq(&other.z)
    }
}

impl Into<[f32; 3]> for Vec3 {
    fn into(self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }
}

impl Into<[f64; 3]> for Vec3 {
    fn into(self) -> [f64; 3] {
        [self.x as f64, self.y as f64, self.z as f64]
    }
}

impl Index<usize> for Vec3 {
    type Output = Scalar;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => {
                panic!("Invalid index");
            }
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => {
                panic!("Invalid index");
            }
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}, {}, {}", self.x, self.y, self.z))
    }
}

impl LinearInterp for Vec3 {
    fn lerp(a: Self, b: Self, t: Scalar) -> Self {
        Vec3::new(
            a.x.lerp_to(b.x, t),
            a.y.lerp_to(b.y, t),
            a.z.lerp_to(b.z, t),
        )
    }
}

impl SphericalInterp for Vec3 {
    fn slerp(_a: Self, _b: Self, _t: Scalar) -> Self {
        todo!()
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::ZERO, |a, b| a.add(b))
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn cross() {
        let a = Vec3::RIGHT;
        let b = Vec3::FORWARD;
        let _r = a.cross(b);
    }
}
