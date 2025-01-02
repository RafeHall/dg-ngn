use std::{fmt::Display, ops::{Index, IndexMut}};

use crate::{interp::LinearInterp, Scalar};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
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

    #[inline]
    pub fn add(&self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }

    #[inline]
    pub fn sub(&self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }

    #[inline]
    pub fn div(&self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.x / other.x,
            self.y / other.y,
            self.z / other.z,
            self.w / other.w,
        )
    }

    #[inline]
    pub fn mul(&self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w,
        )
    }

    #[inline]
    pub fn inverse(&self) -> Vec4 {
        Self::new(1.0 / self.x, 1.0 / self.y, 1.0 / self.z, 1.0 / self.w)
    }

    pub fn max(&self) -> Scalar {
        self.x.max(self.y.max(self.z.max(self.w)))
    }

    pub fn imax(&self) -> usize {
        match (self.x, self.y, self.z, self.w) {
            (x, y, z, w) if x > y && x > z && x > w => 0,
            (x, y, z, w) if y > x && y > z && w > w => 1,
            (x, y, z, w) if z > x && z > y && z > w => 2,
            (x, y, z, w) if w > x && w > y && w > z => 3,
            _ => 0,
        }
    }

    pub fn min(&self) -> Scalar {
        self.x.min(self.y.min(self.x.min(self.w)))
    }

    pub fn imin(&self) -> usize {
        match (self.x, self.y, self.z, self.w) {
            (x, y, z, w) if x < y && x < z && x < w => 0,
            (x, y, z, w) if y < x && y < z && w < w => 1,
            (x, y, z, w) if z < x && z < y && z < w => 2,
            (x, y, z, w) if w < x && w < y && w < z => 3,
            _ => 0,
        }
    }

    pub fn abs(&self) -> Vec4 {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
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

        let c = Vec4::lerp(a, b, 0.5);

        assert_eq!(c, Vec4::new(0.0, 0.0, 0.0, 1.0));
    }
}
