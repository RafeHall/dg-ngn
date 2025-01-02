use crate::Scalar;


pub struct Matrix3x3 {
    pub s00: Scalar, pub s10: Scalar, pub s20: Scalar,
    pub s01: Scalar, pub s11: Scalar, pub s21: Scalar,
    pub s02: Scalar, pub s12: Scalar, pub s22: Scalar,
}

impl Matrix3x3 {
    pub fn inverse(&self) -> Matrix3x3 {
        todo!()
    }
}