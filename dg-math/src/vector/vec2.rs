use std::{
    fmt::Display, iter::Sum, ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign}
};

use crate::{interp::{LinearInterp, SphericalInterp}, Scalar};

use super::Vec3;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vec2 {
    pub const UP: Vec2 = Vec2::new(0.0, -1.0);
    pub const DOWN: Vec2 = Vec2::new(0.0, 1.0);
    pub const LEFT: Vec2 = Vec2::new(-1.0, 0.0);
    pub const RIGHT: Vec2 = Vec2::new(1.0, 0.0);

    pub const ZERO: Vec2 = Vec2::new(0.0, 0.0);
    pub const ONE: Vec2 = Vec2::new(1.0, 1.0);
    pub const NEG_ONE: Vec2 = Vec2::new(-1.0, -1.0);

    pub const INFINITY: Vec2 = Vec2::new(Scalar::INFINITY, Scalar::INFINITY);
    pub const NEG_INFINITY: Vec2 = Vec2::new(Scalar::NEG_INFINITY, Scalar::NEG_INFINITY);

    pub const NAN: Vec2 = Vec2::new(Scalar::NAN, Scalar::NAN);

    pub const fn new(x: Scalar, y: Scalar) -> Vec2 {
        Self { x, y }
    }

    /// Takes an angle in **radians** and creates a **normalized** vector that points in the direction of the angle.
    ///
    /// Starts pointing towards the [`Vec2::RIGHT`] direction turning clockwise with increasing angle.
    ///
    /// * `angle` in **radians**
    ///
    pub fn new_from_angle(angle: Scalar) -> Vec2 {
        Self::new(angle.cos(), angle.sin())
    }

    pub fn add(&self, other: Vec2) -> Vec2 {
        Self::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(&self, other: Vec2) -> Vec2 {
        Self::new(self.x - other.x, self.y - other.y)
    }

    pub fn div(&self, other: Vec2) -> Vec2 {
        Self::new(self.x / other.x, self.y / other.y)
    }

    pub fn mul(&self, other: Vec2) -> Vec2 {
        Self::new(self.x * other.x, self.y * other.y)
    }

    pub fn add_scalar(&self, other: Scalar) -> Vec2 {
        Self::new(self.x + other, self.y + other)
    }

    pub fn sub_scalar(&self, other: Scalar) -> Vec2 {
        Self::new(self.x - other, self.y - other)
    }

    pub fn div_scalar(&self, other: Scalar) -> Vec2 {
        Self::new(self.x / other, self.y / other)
    }

    pub fn mul_scalar(&self, other: Scalar) -> Vec2 {
        Self::new(self.x * other, self.y * other)
    }

    pub fn neg(&self) -> Vec2 {
        Self::new(-self.x, -self.y)
    }

    pub fn inverse(&self) -> Vec2 {
        Self::new(1.0 / self.x, 1.0 / self.y)
    }

    pub fn truncate(&self, z: Scalar) -> Vec3 {
        Vec3::new(self.x, self.y, z)
    }

    pub fn max(&self) -> Scalar {
        self.x.max(self.y)
    }

    pub fn imax(&self) -> usize {
        match (self.x, self.y) {
            (x, y) if x > y => 0,
            (x, y) if y > x => 1,
            _ => 0,
        }
    }

    pub fn min(&self) -> Scalar {
        self.x.min(self.y)
    }

    pub fn imin(&self) -> usize {
        if self.x < self.y {
            0
        } else {
            1
        }
    }

    pub fn dot(&self, other: Vec2) -> Scalar {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: Vec2) -> Scalar {
        self.x * other.y - other.x * self.y
    }

    pub fn left(&self) -> Vec2 {
        Self::new(self.y, self.x)
    }

    pub fn right(&self) -> Vec2 {
        Self::new(-self.y, self.x)
    }

    pub fn to(&self, to: Vec2) -> Vec2 {
        to.sub(*self)
    }

    pub fn direction_to(&self, to: Vec2) -> Vec2 {
        self.to(to).normalized()
    }

    pub fn distance_to(&self, to: Vec2) -> Scalar {
        self.distance_squared_to(to).sqrt()
    }

    pub fn distance_squared_to(&self, to: Vec2) -> Scalar {
        // NOTE: x.powi(2) is just as fast as x * x
        (to.x - self.x).powi(2) + (to.y - self.y).powi(2)
    }

    pub fn length(&self) -> Scalar {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> Scalar {
        self.x * self.x + self.y * self.y
    }

    pub fn normalized(&self) -> Vec2 {
        self.div_scalar(self.length())
    }

    pub fn clamp(&self, min: Vec2, max: Vec2) -> Vec2 {
        Self::new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }

    pub fn round(&self) -> Vec2 {
        Self::new(self.x.round(), self.y.round())
    }

    pub fn floor(&self) -> Vec2 {
        Self::new(self.x.floor(), self.y.floor())
    }

    pub fn ceil(&self) -> Vec2 {
        Self::new(self.x.ceil(), self.y.ceil())
    }

    pub fn abs(&self) -> Vec2 {
        Self::new(self.x.abs(), self.y.abs())
    }

    pub fn angle(&self) -> Scalar {
        self.y.atan2(self.x)
    }

    pub fn angle_to(&self, other: Vec2) -> Scalar {
        self.cross(other).atan2(self.dot(other))
    }

    pub fn rotate(&self, by: Scalar) -> Vec2 {
        Self::new_from_angle(self.angle() + by)
    }

    pub fn reflect(&self, normal: Vec2) -> Vec2 {
        let dot2 = self.dot(normal) * 2.0;

        Self::new(self.x - dot2 * normal.x, self.y - dot2 * normal.y)
    }

    pub fn bounce(&self, normal: Vec2, strength: Scalar) -> Vec2 {
        self.reflect(normal).mul_scalar(strength)
    }
}

impl Index<usize> for Vec2 {
    type Output = Scalar;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => {
                panic!("Invalid index");
            }
        }
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => {
                panic!("Invalid index");
            }
        }
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}, {}", self.x, self.y))
    }
}

impl LinearInterp for Vec2 {
    fn lerp(a: Self, b: Self, t: Scalar) -> Self {
        Vec2::new(a.x.lerp_to(b.x, t), a.y.lerp_to(b.y, t))
    }
}

impl SphericalInterp for Vec2 {
    fn slerp(_a: Self, _b: Self, _t: Scalar) -> Self {
        todo!()
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::add(&self, rhs)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Scalar> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Scalar) -> Self::Output {
        Vec2::add_scalar(&self, rhs)
    }
}

impl AddAssign<Scalar> for Vec2 {
    fn add_assign(&mut self, rhs: Scalar) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::sub(&self, rhs)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Div for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Self) -> Self::Output {
        Vec2::div(&self, rhs)
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Div<Scalar> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Scalar) -> Self::Output {
        Vec2::div_scalar(&self, rhs)
    }
}

impl DivAssign<Scalar> for Vec2 {
    fn div_assign(&mut self, rhs: Scalar) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec2::mul(&self, rhs)
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<Scalar> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Vec2::mul_scalar(&self, rhs)
    }
}

impl MulAssign<Scalar> for Vec2 {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2::neg(&self)
    }
}

impl Sum for Vec2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            Vec2::ZERO,
            |a, b| a + b,
        )
    }
}