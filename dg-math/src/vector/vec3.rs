use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{interp::{LinearInterp, SphericalInterp}, Scalar};

use super::Vec4;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Vec3 {
    pub const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    pub const DOWN: Vec3 = Vec3::new(0.0, -1.0, 0.0);
    pub const RIGHT: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    pub const LEFT: Vec3 = Vec3::new(-1.0, 0.0, 0.0);
    pub const FORWARD: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    pub const BACK: Vec3 = Vec3::new(0.0, 0.0, 1.0);

    pub const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    pub const ONE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    pub const NEG_ONE: Vec3 = Vec3::new(-1.0, -1.0, -1.0);

    pub const INFINITY: Vec3 = Vec3::new(Scalar::INFINITY, Scalar::INFINITY, Scalar::INFINITY);
    pub const NEG_INFINITY: Vec3 = Vec3::new(
        Scalar::NEG_INFINITY,
        Scalar::NEG_INFINITY,
        Scalar::NEG_INFINITY,
    );

    pub const NAN: Vec3 = Vec3::new(Scalar::NAN, Scalar::NAN, Scalar::NAN);

    #[inline]
    pub const fn new(x: Scalar, y: Scalar, z: Scalar) -> Vec3 {
        Self { x, y, z }
    }

    #[inline]
    pub fn add(&self, other: Vec3) -> Vec3 {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    #[inline]
    pub fn sub(&self, other: Vec3) -> Vec3 {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    #[inline]
    pub fn div(&self, other: Vec3) -> Vec3 {
        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }

    #[inline]
    pub fn mul(&self, other: Vec3) -> Vec3 {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    #[inline]
    pub fn add_scalar(&self, other: Scalar) -> Vec3 {
        Self::new(self.x + other, self.y + other, self.z + other)
    }

    #[inline]
    pub fn sub_scalar(&self, other: Scalar) -> Vec3 {
        Self::new(self.x - other, self.y - other, self.z - other)
    }

    #[inline]
    pub fn div_scalar(&self, other: Scalar) -> Vec3 {
        Self::new(self.x / other, self.y / other, self.z / other)
    }

    #[inline]
    pub fn mul_scalar(&self, other: Scalar) -> Vec3 {
        Self::new(self.x * other, self.y * other, self.z * other)
    }

    #[inline]
    pub fn neg(&self) -> Vec3 {
        Self::new(-self.x, -self.y, -self.z)
    }

    #[inline]
    pub fn inverse(&self) -> Vec3 {
        Self::new(1.0 / self.x, 1.0 / self.y, 1.0 / self.z)
    }

    #[inline]
    pub fn truncate(&self, w: Scalar) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, w)
    }

    pub fn max(&self) -> Scalar {
        self.x.max(self.y.max(self.z))
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

    pub fn imin(&self) -> usize {
        match (self.x, self.y, self.z) {
            (x, y, z) if x < y && x < z => 0,
            (x, y, z) if y < x && y < z => 1,
            (x, y, z) if z < x && z < y => 2,
            _ => 0,
        }
    }

    #[inline]
    pub fn dot(&self, other: Vec3) -> Scalar {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    #[inline]
    pub fn distance_to(&self, to: Vec3) -> Scalar {
        self.distance_squared_to(to).sqrt()
    }

    #[inline]
    pub fn distance_squared_to(&self, to: Vec3) -> Scalar {
        // NOTE: x.powi(2) is just as fast as x * x
        (to.x - self.x).powi(2) + (to.y - self.y).powi(2) + (to.z - self.z).powi(2)
    }

    #[inline]
    pub fn length(&self) -> Scalar {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> Scalar {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn normalized(&self) -> Vec3 {
        self.div_scalar(self.length())
    }

    #[inline]
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

    #[inline]
    pub fn round(&self) -> Vec3 {
        Self::new(self.x.round(), self.y.round(), self.z.round())
    }

    #[inline]
    pub fn floor(&self) -> Vec3 {
        Self::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    #[inline]
    pub fn ceil(&self) -> Vec3 {
        Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }

    #[inline]
    pub fn abs(&self) -> Vec3 {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
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

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::add(&self, rhs)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Add<Scalar> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Scalar) -> Self::Output {
        Vec3::add_scalar(&self, rhs)
    }
}

impl AddAssign<Scalar> for Vec3 {
    fn add_assign(&mut self, rhs: Scalar) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::sub(&self, rhs)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3::div(&self, rhs)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Div<Scalar> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Scalar) -> Self::Output {
        Vec3::div_scalar(&self, rhs)
    }
}

impl DivAssign<Scalar> for Vec3 {
    fn div_assign(&mut self, rhs: Scalar) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::mul(&self, rhs)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Mul<Scalar> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Vec3::mul_scalar(&self, rhs)
    }
}

impl MulAssign<Scalar> for Vec3 {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::neg(&self)
    }
}
