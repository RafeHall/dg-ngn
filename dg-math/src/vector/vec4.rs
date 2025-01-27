use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::{interp::LinearInterp, ApproxEq, Scalar};

use super::Vec3;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec4 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
    pub w: Scalar,
}

impl Vec4 {
    pub const UP: Vec4 = Vec4::new(0.0, 1.0, 0.0, 1.0);
    pub const DOWN: Vec4 = Vec4::new(0.0, -1.0, 0.0, 1.0);
    pub const RIGHT: Vec4 = Vec4::new(1.0, 0.0, 0.0, 1.0);
    pub const LEFT: Vec4 = Vec4::new(-1.0, 0.0, 0.0, 1.0);
    pub const FORWARD: Vec4 = Vec4::new(0.0, 0.0, -1.0, 1.0);
    pub const BACK: Vec4 = Vec4::new(0.0, 0.0, 1.0, 1.0);

    pub const X: Vec4 = Vec4::new(1.0, 0.0, 0.0, 0.0);
    pub const Y: Vec4 = Vec4::new(0.0, 1.0, 0.0, 0.0);
    pub const Z: Vec4 = Vec4::new(0.0, 0.0, 1.0, 0.0);
    pub const W: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);

    pub const ZERO: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.0);
    pub const ONE: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);
    pub const NEG_ONE: Vec4 = Vec4::new(-1.0, -1.0, -1.0, -1.0);

    pub const INFINITY: Vec4 = Vec4::new(
        Scalar::INFINITY,
        Scalar::INFINITY,
        Scalar::INFINITY,
        Scalar::INFINITY,
    );
    pub const NEG_INFINITY: Vec4 = Vec4::new(
        Scalar::NEG_INFINITY,
        Scalar::NEG_INFINITY,
        Scalar::NEG_INFINITY,
        Scalar::NEG_INFINITY,
    );

    pub const fn new(x: Scalar, y: Scalar, z: Scalar, w: Scalar) -> Self {
        Self { x, y, z, w }
    }

    pub fn add(&self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }

    pub fn sub(&self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }

    pub fn div(&self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.x / other.x,
            self.y / other.y,
            self.z / other.z,
            self.w / other.w,
        )
    }

    pub fn mul(&self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w,
        )
    }

    pub fn add_scalar(&self, other: Scalar) -> Vec4 {
        Self::new(self.x + other, self.y + other, self.z + other, self.w + other)
    }

    pub fn sub_scalar(&self, other: Scalar) -> Vec4 {
        Self::new(self.x - other, self.y - other, self.z - other, self.w - other)
    }

    pub fn div_scalar(&self, other: Scalar) -> Vec4 {
        Self::new(self.x / other, self.y / other, self.z / other, self.w / other)
    }

    pub fn mul_scalar(&self, other: Scalar) -> Vec4 {
        Self::new(self.x * other, self.y * other, self.z * other, self.w * other)
    }

    pub fn neg(&self) -> Vec4 {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }

    pub fn inverse(&self) -> Vec4 {
        self.neg()
    }

    pub fn reciprocal(&self) -> Vec4 {
        Self::new(1.0 / self.x, 1.0 / self.y, 1.0 / self.z, 1.0 / self.w)
    }

    pub fn truncate(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    /// Returns true if all components of `self` are greater than `other`
    pub fn greater_than(&self, other: Vec4) -> bool {
        self.x > other.x && self.y > other.y && self.z > other.z && self.w > other.w
    }

    /// Returns true if all components of `self` are greater than or equal to `other`
    pub fn greater_than_equals(&self, other: Vec4) -> bool {
        self.x >= other.x && self.y >= other.y && self.z >= other.z && self.w >= other.w
    }

    /// Returns true if all components of `self` are greater than `other`
    pub fn less_than(&self, other: Vec4) -> bool {
        self.x < other.x && self.y < other.y && self.z < other.z && self.w < other.w
    }

    /// Returns true if all components of `self` are less than or equal to `other`
    pub fn less_than_equals(&self, other: Vec4) -> bool {
        self.x <= other.x && self.y <= other.y && self.z <= other.z && self.w <= other.w
    }

    pub fn max(&self) -> Scalar {
        self.x.max(self.y.max(self.z.max(self.w)))
    }

    pub fn vmax(&self, other: Vec4) -> Vec4 {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
            self.w.max(other.w),
        )
    }

    pub fn imax(&self) -> usize {
        match (self.x, self.y, self.z, self.w) {
            (x, y, z, w) if x > y && x > z && x > w => 0,
            (x, y, z, w) if y > x && y > z && y > w => 1,
            (x, y, z, w) if z > x && z > y && z > w => 2,
            (x, y, z, w) if w > x && w > y && w > z => 3,
            _ => 0,
        }
    }

    pub fn min(&self) -> Scalar {
        self.x.min(self.y.min(self.x))
    }

    pub fn vmin(&self, other: Vec4) -> Vec4 {
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
            self.w.min(other.w),
        )
    }

    pub fn imin(&self) -> usize {
        match (self.x, self.y, self.z, self.w) {
            (x, y, z, w) if x < y && x < z && x < w => 0,
            (x, y, z, w) if y < x && y < z && y < w => 1,
            (x, y, z, w) if z < x && z < y && z < w => 2,
            (x, y, z, w) if w < x && w < y && w < z => 3,
            _ => 0,
        }
    }

    pub fn dot(&self, other: Vec4) -> Scalar {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: Vec4) -> Vec4 {
        todo!()
    }

    pub fn to(&self, to: Vec4) -> Vec4 {
        to.sub(*self)
    }

    pub fn direction_to(&self, to: Vec4) -> Vec4 {
        self.to(to).normalized()
    }

    pub fn distance_to(&self, to: Vec4) -> Scalar {
        self.distance_squared_to(to).sqrt()
    }

    pub fn distance_squared_to(&self, to: Vec4) -> Scalar {
        // NOTE: x.powi(2) is just as fast as x * x
        (to.x - self.x).powi(2) + (to.y - self.y).powi(2) + (to.z - self.z).powi(2) + (to.w - self.w).powi(2)
    }

    pub fn length(&self) -> Scalar {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> Scalar {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn normalized(&self) -> Vec4 {
        self.div_scalar(self.length())
    }

    pub fn clamp(&self, min: Vec4, max: Vec4) -> Vec4 {
        Self::new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
            self.z.clamp(min.z, max.z),
            self.w.clamp(min.w, max.w),
        )
    }

    pub fn clamp_scalar(&self, min: Scalar, max: Scalar) -> Vec4 {
        Self::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
            self.w.clamp(min, max),
        )
    }

    pub fn round(&self) -> Vec4 {
        Self::new(self.x.round(), self.y.round(), self.z.round(), self.w.round())
    }

    pub fn floor(&self) -> Vec4 {
        Self::new(self.x.floor(), self.y.floor(), self.z.floor(), self.w.floor())
    }

    pub fn ceil(&self) -> Vec4 {
        Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil(), self.w.ceil())
    }

    pub fn abs(&self) -> Vec4 {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
    }
}

impl ApproxEq for Vec4 {
    fn approx_eq(&self, other: &Self) -> bool {
        self.x.approx_eq(&other.x)
            && self.y.approx_eq(&other.y)
            && self.z.approx_eq(&other.z)
            && self.w.approx_eq(&other.w)
    }
}

impl Index<usize> for Vec4 {
    type Output = Scalar;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => {
                panic!("Invalid index");
            }
        }
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => {
                panic!("Invalid index");
            }
        }
    }
}

impl LinearInterp for Vec4 {
    fn lerp(a: Self, b: Self, t: Scalar) -> Self {
        Vec4::new(
            a.x.lerp_to(b.x, t),
            a.y.lerp_to(b.y, t),
            a.z.lerp_to(b.z, t),
            a.w.lerp_to(b.w, t),
        )
    }
}

impl Display for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

#[cfg(test)]
mod tests {
    use crate::interp::LinearInterp;

    use super::Vec4;

    #[test]
    fn interpolation() {
        let a = Vec4::new(-1.0, 0.0, 0.0, 1.0);
        let b = Vec4::new(1.0, 0.0, 0.0, 1.0);

        let _c = Vec4::lerp(a, b, 0.5);

        // assert_eq!(c, Vec4::new(0.0, 0.0, 0.0, 1.0));
    }
}
