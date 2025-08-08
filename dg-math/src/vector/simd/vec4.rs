use std::{
    ops::{Add, Deref, DerefMut, Div, Mul, Neg, Sub},
    simd::{num::SimdFloat, StdFloat},
};

use crate::Scalar;

#[cfg(feature = "high-precision")]
type Scalar4 = std::simd::f64x4;
#[cfg(not(feature = "high-precision"))]
type Scalar4 = std::simd::f32x4;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec4(pub(crate) Scalar4);

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

    #[inline]
    pub const fn new(x: Scalar, y: Scalar, z: Scalar, w: Scalar) -> Self {
        Self(Scalar4::from_array([x, y, z, w]))
    }

    #[inline]
    pub fn add(&self, other: Vec4) -> Vec4 {
        // Vec4::new(
        //     self.x + other.x,
        //     self.y + other.y,
        //     self.z + other.z,
        //     self.w + other.w,
        // )
        Self(self.0.add(other.0))
    }

    pub fn sub(&self, other: Vec4) -> Vec4 {
        // Vec4::new(
        //     self.x - other.x,
        //     self.y - other.y,
        //     self.z - other.z,
        //     self.w - other.w,
        // )
        Self(self.0.sub(other.0))
    }

    pub fn div(&self, other: Vec4) -> Vec4 {
        // Vec4::new(
        //     self.x / other.x,
        //     self.y / other.y,
        //     self.z / other.z,
        //     self.w / other.w,
        // )
        Self(self.0.div(other.0))
    }

    pub fn mul(&self, other: Vec4) -> Vec4 {
        // Vec4::new(
        //     self.x * other.x,
        //     self.y * other.y,
        //     self.z * other.z,
        //     self.w * other.w,
        // )
        Self(self.0.mul(other.0))
    }

    pub fn add_scalar(&self, other: Scalar) -> Vec4 {
        // Self::new(self.x + other, self.y + other, self.z + other, self.w + other)
        Self(self.0.add(Scalar4::splat(other)))
    }

    pub fn sub_scalar(&self, other: Scalar) -> Vec4 {
        // Self::new(self.x - other, self.y - other, self.z - other, self.w - other)
        Self(self.0.sub(Scalar4::splat(other)))
    }

    pub fn div_scalar(&self, other: Scalar) -> Vec4 {
        // Self::new(self.x / other, self.y / other, self.z / other, self.w / other)
        Self(self.0.div(Scalar4::splat(other)))
    }

    pub fn mul_scalar(&self, other: Scalar) -> Vec4 {
        // Self::new(self.x * other, self.y * other, self.z * other, self.w * other)
        Self(self.0.mul(Scalar4::splat(other)))
    }

    pub fn neg(&self) -> Vec4 {
        // Self::new(-self.x, -self.y, -self.z, -self.w)
        Self(self.0.neg())
    }

    pub fn inverse(&self) -> Vec4 {
        self.neg()
    }

    pub fn reciprocal(&self) -> Vec4 {
        // Self::new(1.0 / self.x, 1.0 / self.y, 1.0 / self.z, 1.0 / self.w)
        Self(self.0.recip())
    }

    // pub fn truncate(&self) -> Vec3 {
    //     Vec3::new(self.x, self.y, self.z)
    // }

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
        // self.x.max(self.y.max(self.z.max(self.w)))
        self.0.reduce_max()
    }

    pub fn vmax(&self, other: Vec4) -> Vec4 {
        // Self::new(
        //     self.x.max(other.x),
        //     self.y.max(other.y),
        //     self.z.max(other.z),
        //     self.w.max(other.w),
        // )
        Self(self.0.simd_max(other.0))
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
        // self.x.min(self.y.min(self.x))
        self.0.reduce_min()
    }

    pub fn vmin(&self, other: Vec4) -> Vec4 {
        // Self::new(
        //     self.x.min(other.x),
        //     self.y.min(other.y),
        //     self.z.min(other.z),
        //     self.w.min(other.w),
        // )
        Self(self.0.simd_min(other.0))
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

    #[inline]
    pub fn dot(&self, other: Vec4) -> Scalar {
        // self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
        self.0.mul(other.0).reduce_sum()
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
        // (to.x - self.x).powi(2) + (to.y - self.y).powi(2) + (to.z - self.z).powi(2) + (to.w - self.w).powi(2)

        let v = to.0.sub(self.0);
        v.mul(v).reduce_sum()
    }

    pub fn length(&self) -> Scalar {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> Scalar {
        self.0.mul(self.0).reduce_sum()
        // self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn normalized(&self) -> Vec4 {
        self.div_scalar(self.length())
    }

    pub fn clamp(&self, min: Vec4, max: Vec4) -> Vec4 {
        Self(self.0.simd_clamp(min.0, max.0))
    }

    pub fn clamp_scalar(&self, min: Scalar, max: Scalar) -> Vec4 {
        Self::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
            self.w.clamp(min, max),
        )
        // Self(self.0.simd_clamp(Scalar4::splat(min), Scalar4::splat(max)))
    }

    pub fn round(&self) -> Vec4 {
        Self(self.0.round())
    }

    pub fn floor(&self) -> Vec4 {
        Self(self.0.floor())
    }

    pub fn ceil(&self) -> Vec4 {
        Self(self.0.ceil())
    }

    pub fn abs(&self) -> Vec4 {
        Self(self.0.abs())
    }
}

impl Deref for Vec4 {
    type Target = crate::vector::Vec4;

    fn deref<'a>(&'a self) -> &'a Self::Target {
        unsafe { std::mem::transmute::<&'a Self, &'a Self::Target>(self) }
    }
}

impl DerefMut for Vec4 {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
        unsafe { std::mem::transmute::<&'a mut Self, &'a mut Self::Target>(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec4;

    #[test]
    fn test() {
        let v = Vec4::new(4.6425, 2.1278, 1.8716, 0.0);
        let v = v.clamp_scalar(1.0, 2.0);
        println!("{:#?}", v);
    }
}
