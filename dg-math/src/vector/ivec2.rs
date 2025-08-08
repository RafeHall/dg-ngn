use std::{
    fmt::Display,
    iter::Sum,
    ops::{Index, IndexMut},
};

use crate::{Integer, Scalar};

use super::Vec2;

/// Representation of a vector in 2d space using `x` and `y` [`Integer`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct IVec2 {
    pub x: Integer,
    pub y: Integer,
}

impl IVec2 {
    /// [`IVec2`] with `-1` in the `y` component
    pub const UP: IVec2 = IVec2::new(0, -1);
    /// [`IVec2`] with `1` in the `y` component
    pub const DOWN: IVec2 = IVec2::new(0, 1);
    /// [`IVec2`] with `-1` in the `x` component
    pub const LEFT: IVec2 = IVec2::new(-1, 0);
    /// [`IVec2`] with `1` in the `x` component
    pub const RIGHT: IVec2 = IVec2::new(1, 0);

    /// [`IVec2`] with `0` in all components
    pub const ZERO: IVec2 = IVec2::new(0, 0);
    /// [`IVec2`] with `1` in all components
    pub const ONE: IVec2 = IVec2::new(1, 1);
    /// [`IVec2`] with `-1` in all components
    pub const NEG_ONE: IVec2 = IVec2::new(-1, -1);

    /// Contruct [`IVec2`] using `x` and `y` [`Integer`]
    pub const fn new(x: Integer, y: Integer) -> IVec2 {
        Self { x, y }
    }

    /// Adds each component of the vectors together
    pub fn add(&self, other: IVec2) -> IVec2 {
        Self::new(self.x + other.x, self.y + other.y)
    }

    /// Subtracts each component of the vectors together
    pub fn sub(&self, other: IVec2) -> IVec2 {
        Self::new(self.x - other.x, self.y - other.y)
    }

    /// Divides each component of the vectors together
    pub fn div(&self, other: IVec2) -> IVec2 {
        Self::new(self.x / other.x, self.y / other.y)
    }

    /// Multiplies each component of the vectors together
    pub fn mul(&self, other: IVec2) -> IVec2 {
        Self::new(self.x * other.x, self.y * other.y)
    }

    /// Adds Integer to each component of vector
    pub fn add_integer(&self, other: Integer) -> IVec2 {
        Self::new(self.x + other, self.y + other)
    }

    /// Subtracts Integer to each component of vector
    pub fn sub_integer(&self, other: Integer) -> IVec2 {
        Self::new(self.x - other, self.y - other)
    }

    /// Divides Integer to each component of vector
    pub fn div_integer(&self, other: Integer) -> IVec2 {
        Self::new(self.x / other, self.y / other)
    }

    /// Multiplies Integer to each component of vector
    pub fn mul_integer(&self, other: Integer) -> IVec2 {
        Self::new(self.x * other, self.y * other)
    }

    /// Makes each component the negative of itself
    pub fn neg(&self) -> IVec2 {
        Self::new(-self.x, -self.y)
    }

    /// Returns vector that undoes transform
    pub fn inverse(&self) -> IVec2 {
        self.neg()
    }

    /// Returns vector with reciprocal of each component
    pub fn reciprocal(&self) -> Vec2 {
        Vec2::new(1.0 / self.x as Scalar, 1.0 / self.y as Scalar)
    }

    /// Truncates vector components removing `y`
    pub fn truncate(&self) -> Integer {
        self.x
    }

    /// Extends vector component adding `z`
    // pub fn extend(&self, z: Integer) -> IVec3 {
    //     Vec3::new(self.x, self.y, z)
    // }

    /// Returns true if all components of `self` are greater than `other`
    pub fn greater_than(&self, other: IVec2) -> bool {
        self.x > other.x && self.y > other.y
    }

    /// Returns true if all components of `self` are greater than or equal to `other`
    pub fn greater_than_equals(&self, other: IVec2) -> bool {
        self.x >= other.x && self.y >= other.y
    }

    /// Returns true if all components of `self` are greater than `other`
    pub fn less_than(&self, other: IVec2) -> bool {
        self.x < other.x && self.y < other.y
    }

    /// Returns true if all components of `self` are less than or equal to `other`
    pub fn less_than_equals(&self, other: IVec2) -> bool {
        self.x <= other.x && self.y <= other.y
    }

    /// Returns largest Integer component
    pub fn max(&self) -> Integer {
        self.x.max(self.y)
    }

    /// Returns vector with largest of each component between `self` and `other`
    pub fn vmax(&self, other: IVec2) -> IVec2 {
        IVec2::new(self.x.max(other.x), self.y.max(other.y))
    }

    /// Returns index of largest Integer component
    pub fn imax(&self) -> usize {
        match (self.x, self.y) {
            (x, y) if x > y => 0,
            (x, y) if y > x => 1,
            _ => 0,
        }
    }

    /// Returns smallest Integer component
    pub fn min(&self) -> Integer {
        self.x.min(self.y)
    }

    /// Returns vector with smallest of each component between `self` and `other`
    pub fn vmin(&self, other: IVec2) -> IVec2 {
        IVec2::new(self.x.min(other.x), self.y.min(other.y))
    }

    /// Returns index of smallest Integer component
    pub fn imin(&self) -> usize {
        match (self.x, self.y) {
            (x, y) if x < y => 0,
            (x, y) if y < x => 1,
            _ => 0,
        }
    }

    /// Returns dot product between `self` and `other`
    pub fn dot(&self, other: IVec2) -> Integer {
        self.x * other.x + self.y * other.y
    }

    /// Returns cross product between `self` and `other`
    pub fn cross(&self, other: IVec2) -> Integer {
        self.x * other.y - other.x * self.y
    }

    /// Returns vector rotation -90 degrees
    pub fn left(&self) -> IVec2 {
        Self::new(self.y, self.x)
    }

    /// Returns vector rotation 90 degrees
    pub fn right(&self) -> IVec2 {
        Self::new(-self.y, self.x)
    }

    /// Returns vector that goes from `self` to `other`
    pub fn to(&self, other: IVec2) -> IVec2 {
        other.sub(*self)
    }

    // Returns normalized vector from `self` to `other`
    pub fn direction_to(&self, other: IVec2) -> Vec2 {
        self.to_vec2().direction_to(other.to_vec2())
    }

    /// Returns distance from `self` to `other`
    pub fn distance_to(&self, other: IVec2) -> Scalar {
        (self.distance_squared_to(other) as Scalar).sqrt()
    }

    /// Returns squared distance from `self` to `other`
    pub fn distance_squared_to(&self, other: IVec2) -> Integer {
        // NOTE: x.powi(2) is just as fast as x * x
        (other.x - self.x).pow(2) + (other.y - self.y).pow(2)
    }

    /// Returns length of `self`
    pub fn length(&self) -> Scalar {
        (self.length_squared() as Scalar).sqrt()
    }

    /// Returns length squared of `self`
    pub fn length_squared(&self) -> Integer {
        self.x * self.x + self.y * self.y
    }

    pub fn clamp(&self, min: IVec2, max: IVec2) -> IVec2 {
        Self::new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }

    pub fn clamp_integer(&self, min: Integer, max: Integer) -> IVec2 {
        Self::new(self.x.clamp(min, max), self.y.clamp(min, max))
    }

    pub fn abs(&self) -> IVec2 {
        Self::new(self.x.abs(), self.y.abs())
    }

    pub fn angle(&self) -> Scalar {
        (self.y as Scalar).atan2(self.x as Scalar)
    }

    pub fn angle_to(&self, other: IVec2) -> Scalar {
        (self.cross(other) as Scalar).atan2(self.dot(other) as Scalar)
    }

    pub fn rotate(&self, by: u8) -> IVec2 {
        match by % 4 {
            0 => Self::new(self.x, self.y),       //   0 degrees
            1 => Self::new(-self.y, self.x), //  90 degrees
            2 => Self::new(-self.x, -self.y),     // 180 degrees
            3 => Self::new(self.y, -self.x), // 270 degrees
            _ => unreachable!(),
        }
    }

    pub fn reflect(&self, normal: IVec2) -> IVec2 {
        let dot2 = self.dot(normal) * 2;

        Self::new(self.x - dot2 * normal.x, self.y - dot2 * normal.y)
    }

    pub fn bounce(&self, normal: IVec2, strength: Integer) -> IVec2 {
        self.reflect(normal).mul_integer(strength)
    }

    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x as Scalar, self.y as Scalar)
    }
}

impl Index<usize> for IVec2 {
    type Output = Integer;

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

impl IndexMut<usize> for IVec2 {
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

impl Display for IVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}, {}", self.x, self.y))
    }
}

impl Sum for IVec2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(IVec2::ZERO, |a, b| a.add(b))
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_approx_eq, vector::Vec2};

    use super::IVec2;

    #[test]
    fn add() {
        let a = IVec2::new(1, -5);
        let b = IVec2::new(2, 2);
        let r = a.add(b);

        assert_eq!(r, IVec2::new(3, -3));
    }

    #[test]
    fn sub() {
        let a = IVec2::new(1, -5);
        let b = IVec2::new(2, 2);
        let r = a.sub(b);

        assert_eq!(r, IVec2::new(-1, -7));
    }

    #[test]
    fn div() {
        let a = IVec2::new(1, -5);
        let b = IVec2::new(3, 2);
        let r = a.div(b);

        assert_eq!(r, IVec2::new(1 / 3, -5 / 2));
    }

    #[test]
    fn mul() {
        let a = IVec2::new(1, -5);
        let b = IVec2::new(3, 2);
        let r = a.mul(b);

        assert_eq!(r, IVec2::new(3, -10));
    }

    #[test]
    fn neg() {
        let v = IVec2::new(104, -24);
        let r = v.neg();

        assert_eq!(r, IVec2::new(-104, 24));
    }

    #[test]
    fn reciprocal() {
        let v = IVec2::new(10, 1);
        let r = v.reciprocal();

        assert_approx_eq!(r, Vec2::new(0.1, 1.0));
    }

    #[test]
    fn max() {
        let v = IVec2::new(-1, 5);
        let r = v.max();

        assert_eq!(r, 5);

        let v = IVec2::new(-1, -5);
        let r = v.max();

        assert_eq!(r, -1);
    }

    #[test]
    fn imax() {
        let v = IVec2::new(-1, 5);
        let r = v.imax();

        assert_eq!(r, 1);

        let v = IVec2::new(-1, -5);
        let r = v.imax();

        assert_eq!(r, 0);

        let v = IVec2::new(-1, -1);
        let r = v.imax();

        assert_eq!(r, 0);
    }

    #[test]
    fn min() {
        let v = IVec2::new(-1, 5);
        let r = v.min();

        assert_eq!(r, -1);

        let v = IVec2::new(-1, -5);
        let r = v.min();

        assert_eq!(r, -5);
    }

    #[test]
    fn imin() {
        let v = IVec2::new(-1, 5);
        let r = v.imin();

        assert_eq!(r, 0);

        let v = IVec2::new(-1, -5);
        let r = v.imin();

        assert_eq!(r, 1);

        let v = IVec2::new(-1, -1);
        let r = v.imin();

        assert_eq!(r, 0);
    }
}
