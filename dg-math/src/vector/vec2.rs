use std::{
    fmt::Display,
    iter::Sum,
    ops::{Index, IndexMut},
};

use crate::{
    interp::{LinearInterp, SphericalInterp},
    ApproxEq, Scalar,
};

use super::Vec3;

/// Representation of a vector in 2d space using `x` and `y` [`Scalar`]
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec2 {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vec2 {
    /// [`Vec2`] with `-1.0` in the `y` component
    pub const UP: Vec2 = Vec2::new(0.0, -1.0);
    /// [`Vec2`] with `1.0` in the `y` component
    pub const DOWN: Vec2 = Vec2::new(0.0, 1.0);
    /// [`Vec2`] with `-1.0` in the `x` component
    pub const LEFT: Vec2 = Vec2::new(-1.0, 0.0);
    /// [`Vec2`] with `1.0` in the `x` component
    pub const RIGHT: Vec2 = Vec2::new(1.0, 0.0);

    /// [`Vec2`] with `0.0` in all components
    pub const ZERO: Vec2 = Vec2::new(0.0, 0.0);
    /// [`Vec2`] with `1.0` in all components
    pub const ONE: Vec2 = Vec2::new(1.0, 1.0);
    /// [`Vec2`] with `-1.0` in all components
    pub const NEG_ONE: Vec2 = Vec2::new(-1.0, -1.0);

    /// [`Vec2`] with [`Scalar::INFINITY`] in all components
    pub const INFINITY: Vec2 = Vec2::new(Scalar::INFINITY, Scalar::INFINITY);
    /// [`Vec2`] with [`Scalar::NEG_INFINITY`] in all components
    pub const NEG_INFINITY: Vec2 = Vec2::new(Scalar::NEG_INFINITY, Scalar::NEG_INFINITY);

    /// [`Vec2`] with [`Scalar::NAN`] in all components
    pub const NAN: Vec2 = Vec2::new(Scalar::NAN, Scalar::NAN);

    /// Contruct [`Vec2`] using `x` and `y` [`Scalar`]
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

    /// Adds each component of the vectors together
    pub fn add(&self, other: Vec2) -> Vec2 {
        Self::new(self.x + other.x, self.y + other.y)
    }

    /// Subtracts each component of the vectors together
    pub fn sub(&self, other: Vec2) -> Vec2 {
        Self::new(self.x - other.x, self.y - other.y)
    }

    /// Divides each component of the vectors together
    pub fn div(&self, other: Vec2) -> Vec2 {
        Self::new(self.x / other.x, self.y / other.y)
    }

    /// Multiplies each component of the vectors together
    pub fn mul(&self, other: Vec2) -> Vec2 {
        Self::new(self.x * other.x, self.y * other.y)
    }

    /// Adds scalar to each component of vector
    pub fn add_scalar(&self, other: Scalar) -> Vec2 {
        Self::new(self.x + other, self.y + other)
    }

    /// Subtracts scalar to each component of vector
    pub fn sub_scalar(&self, other: Scalar) -> Vec2 {
        Self::new(self.x - other, self.y - other)
    }

    /// Divides scalar to each component of vector
    pub fn div_scalar(&self, other: Scalar) -> Vec2 {
        Self::new(self.x / other, self.y / other)
    }

    /// Multiplies scalar to each component of vector
    pub fn mul_scalar(&self, other: Scalar) -> Vec2 {
        Self::new(self.x * other, self.y * other)
    }

    /// Makes each component the negative of itself
    pub fn neg(&self) -> Vec2 {
        Self::new(-self.x, -self.y)
    }

    /// Returns vector that undoes transform
    pub fn inverse(&self) -> Vec2 {
        self.neg()
    }

    /// Returns vector with reciprocal of each component
    pub fn reciprocal(&self) -> Vec2 {
        Self::new(1.0 / self.x, 1.0 / self.y)
    }

    /// Truncates vector components removing `y`
    pub fn truncate(&self) -> Scalar {
        self.x
    }

    /// Extends vector component adding `z`
    pub fn extend(&self, z: Scalar) -> Vec3 {
        Vec3::new(self.x, self.y, z)
    }

    /// Returns true if all components of `self` are greater than `other`
    pub fn greater_than(&self, other: Vec2) -> bool {
        self.x > other.x && self.y > other.y
    }

    /// Returns true if all components of `self` are greater than or equal to `other`
    pub fn greater_than_equals(&self, other: Vec2) -> bool {
        self.x >= other.x && self.y >= other.y
    }

    /// Returns true if all components of `self` are greater than `other`
    pub fn less_than(&self, other: Vec2) -> bool {
        self.x < other.x && self.y < other.y
    }

    /// Returns true if all components of `self` are less than or equal to `other`
    pub fn less_than_equals(&self, other: Vec2) -> bool {
        self.x <= other.x && self.y <= other.y
    }

    /// Returns largest scalar component
    pub fn max(&self) -> Scalar {
        self.x.max(self.y)
    }

    /// Returns vector with largest of each component between `self` and `other`
    pub fn vmax(&self, other: Vec2) -> Vec2 {
        Vec2::new(self.x.max(other.x), self.y.max(other.y))
    }

    /// Returns index of largest scalar component
    pub fn imax(&self) -> usize {
        match (self.x, self.y) {
            (x, y) if x > y => 0,
            (x, y) if y > x => 1,
            _ => 0,
        }
    }

    /// Returns smallest scalar component
    pub fn min(&self) -> Scalar {
        self.x.min(self.y)
    }

    /// Returns vector with smallest of each component between `self` and `other`
    pub fn vmin(&self, other: Vec2) -> Vec2 {
        Vec2::new(self.x.min(other.x), self.y.min(other.y))
    }

    /// Returns index of smallest scalar component
    pub fn imin(&self) -> usize {
        match (self.x, self.y) {
            (x, y) if x < y => 0,
            (x, y) if y < x => 1,
            _ => 0,
        }
    }

    /// Returns dot product between `self` and `other`
    pub fn dot(&self, other: Vec2) -> Scalar {
        self.x * other.x + self.y * other.y
    }

    /// Returns cross product between `self` and `other`
    pub fn cross(&self, other: Vec2) -> Scalar {
        self.x * other.y - other.x * self.y
    }

    /// Returns vector rotation -90 degrees
    pub fn left(&self) -> Vec2 {
        Self::new(self.y, self.x)
    }

    /// Returns vector rotation 90 degrees
    pub fn right(&self) -> Vec2 {
        Self::new(-self.y, self.x)
    }

    /// Returns vector that goes from `self` to `other`
    pub fn to(&self, other: Vec2) -> Vec2 {
        other.sub(*self)
    }

    /// Returns normalized vector from `self` to `other`
    pub fn direction_to(&self, other: Vec2) -> Vec2 {
        self.to(other).normalized()
    }

    /// Returns distance from `self` to `other`
    pub fn distance_to(&self, other: Vec2) -> Scalar {
        self.distance_squared_to(other).sqrt()
    }

    /// Returns squared distance from `self` to `other`
    pub fn distance_squared_to(&self, other: Vec2) -> Scalar {
        // NOTE: x.powi(2) is just as fast as x * x
        (other.x - self.x).powi(2) + (other.y - self.y).powi(2)
    }

    /// Returns length of `self`
    pub fn length(&self) -> Scalar {
        self.length_squared().sqrt()
    }

    /// Returns length squared of `self`
    pub fn length_squared(&self) -> Scalar {
        self.x * self.x + self.y * self.y
    }

    pub fn normalized(&self) -> Vec2 {
        self.div_scalar(self.length())
    }

    pub fn clamp(&self, min: Vec2, max: Vec2) -> Vec2 {
        Self::new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }

    pub fn clamp_scalar(&self, min: Scalar, max: Scalar) -> Vec2 {
        Self::new(self.x.clamp(min, max), self.y.clamp(min, max))
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

impl ApproxEq for Vec2 {
    fn approx_eq(&self, other: &Self) -> bool {
        self.x.approx_eq(&other.x) && self.y.approx_eq(&other.y)
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

impl Sum for Vec2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec2::ZERO, |a, b| a.add(b))
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_approx_eq, assert_approx_ne, scalar};

    use super::Vec2;

    #[test]
    fn new_from_angle() {
        let v = Vec2::new_from_angle(0.0);
        assert_approx_eq!(v, Vec2::RIGHT);

        let v = Vec2::new_from_angle(scalar::consts::FRAC_PI_2);
        assert_approx_eq!(v, Vec2::DOWN);

        let v = Vec2::new_from_angle(scalar::consts::PI);
        assert_approx_eq!(v, Vec2::LEFT);
        assert_approx_ne!(v, Vec2::RIGHT);

        let v = Vec2::new_from_angle(scalar::consts::FRAC_PI_2 * 3.0);
        assert_approx_eq!(v, Vec2::UP);

        let v = Vec2::new_from_angle(scalar::consts::FRAC_PI_4);
        assert_approx_eq!(v, Vec2::new(1.0, 1.0).normalized());

        let v = Vec2::new_from_angle(scalar::consts::FRAC_PI_8);
        assert_approx_eq!(
            v,
            Vec2::new(1.0, scalar::consts::FRAC_PI_8.tan()).normalized()
        );
    }

    #[test]
    fn add() {
        let a = Vec2::new(1.0, -5.0);
        let b = Vec2::new(2.0, 2.0);
        let r = a.add(b);

        assert_approx_eq!(r, Vec2::new(3.0, -3.0));
    }

    #[test]
    fn sub() {
        let a = Vec2::new(1.0, -5.0);
        let b = Vec2::new(2.0, 2.0);
        let r = a.sub(b);

        assert_approx_eq!(r, Vec2::new(-1.0, -7.0));
    }

    #[test]
    fn div() {
        let a = Vec2::new(1.0, -5.0);
        let b = Vec2::new(3.0, 2.0);
        let r = a.div(b);

        assert_approx_eq!(r, Vec2::new(1.0 / 3.0, -5.0 / 2.0));
    }

    #[test]
    fn mul() {
        let a = Vec2::new(1.0, -5.0);
        let b = Vec2::new(3.0, 2.0);
        let r = a.mul(b);

        assert_approx_eq!(r, Vec2::new(3.0, -10.0));
    }

    #[test]
    fn neg() {
        let v = Vec2::new(104.0, -24.0);
        let r = v.neg();

        assert_approx_eq!(r, Vec2::new(-104.0, 24.0));
    }

    #[test]
    fn reciprocal() {
        let v = Vec2::new(10.0, 0.5);
        let r = v.reciprocal();

        assert_approx_eq!(r, Vec2::new(0.1, 2.0));
        // assert!(r.approx_eq(Vec2::new(0.1, 2.0)));
    }

    #[test]
    fn max() {
        let v = Vec2::new(-1.0, 5.0);
        let r = v.max();

        assert_approx_eq!(r, 5.0);
        // assert!((r - 5.0).abs() < Vec2::EPS);

        let v = Vec2::new(-1.0, -5.0);
        let r = v.max();

        assert_approx_eq!(r, -1.0);
        // assert!((r + 1.0).abs() < Vec2::EPS);
    }

    #[test]
    fn imax() {
        let v = Vec2::new(-1.0, 5.0);
        let r = v.imax();

        assert_eq!(r, 1);

        let v = Vec2::new(-1.0, -5.0);
        let r = v.imax();

        assert_eq!(r, 0);

        let v = Vec2::new(-1.0, -1.0);
        let r = v.imax();

        assert_eq!(r, 0);
    }

    #[test]
    fn min() {
        let v = Vec2::new(-1.0, 5.0);
        let r = v.min();

        assert_approx_eq!(r, -1.0);

        let v = Vec2::new(-1.0, -5.0);
        let r = v.min();

        assert_approx_eq!(r, -5.0);
    }

    #[test]
    fn imin() {
        let v = Vec2::new(-1.0, 5.0);
        let r = v.imin();

        assert_eq!(r, 0);

        let v = Vec2::new(-1.0, -5.0);
        let r = v.imin();

        assert_eq!(r, 1);

        let v = Vec2::new(-1.0, -1.0);
        let r = v.imin();

        assert_eq!(r, 0);
    }
}
