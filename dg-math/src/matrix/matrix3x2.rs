use std::ops::Mul;

use crate::{vector::Vec2, Scalar};



pub struct Matrix3x2 {
    pub s00: Scalar, pub s10: Scalar, pub s20: Scalar,
    pub s01: Scalar, pub s11: Scalar, pub s21: Scalar,
}

impl Matrix3x2 {
    pub fn new(s00: Scalar, s10: Scalar, s01: Scalar, s11: Scalar, s20: Scalar, s21: Scalar) -> Self {
        Self {
            s00, s10,
            s01, s11,
            s20, s21,
        }
    }

    pub fn transform(&self, v: Vec2) -> Vec2 {
        Vec2::new(
            self.s00 * v.x + self.s10 * v.y,
            self.s01 * v.x + self.s11 * v.y,
        )
    }
}