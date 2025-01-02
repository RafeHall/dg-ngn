use std::ops::Mul;

use crate::{vector::Vec2, Scalar};



pub struct Matrix2x2 {
    pub s00: Scalar, pub s10: Scalar,
    pub s01: Scalar, pub s11: Scalar,
}

impl Matrix2x2 {
    pub fn new(s00: Scalar, s10: Scalar, s01: Scalar, s11: Scalar) -> Self {
        Self {
            s00, s10,
            s01, s11,
        }
    }

    pub fn new_from_vecs(v0: Vec2, v1: Vec2) -> Self {
        Self {
            s00: v0.x, s10: v0.y,
            s01: v1.x, s11: v1.y,
        }
    }

    pub fn transform(&self, v: Vec2) -> Vec2 {
        Vec2::new(
            self.s00 * v.x + self.s10 * v.y,
            self.s01 * v.x + self.s11 * v.y,
        )
    }
}