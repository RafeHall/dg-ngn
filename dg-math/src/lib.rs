#![feature(adt_const_params)]

pub mod aabb;
pub mod algorithms;
pub mod color;
pub mod frustum;
pub mod interp;
pub mod line;
pub mod matrix;
pub mod plane;
pub mod projection;
pub mod ray;
pub mod rect;
pub mod rotor;
pub mod transform;
pub mod vector;

#[cfg(not(feature = "high-precision"))]
pub type Scalar = f32;

#[cfg(feature = "high-precision")]
pub type Scalar = f64;

pub mod scalar {
    #[cfg(not(feature = "high-precision"))]
    pub use std::f32::consts;
    #[cfg(not(feature = "high-precision"))]
    pub use std::f32::*;

    #[cfg(feature = "high-precision")]
    pub use std::f64::consts;
    #[cfg(feature = "high-precision")]
    pub use std::f64::*;
}

#[cfg(not(feature = "large-integers"))]
pub type Integer = i32;

#[cfg(feature = "large-integers")]
pub type Integer = i64;

pub trait ApproxEq<Rhs = Self>
where
    Rhs: ?Sized
{
    const EPS: Scalar = 0.00001;

    fn approx_eq(&self, other: &Rhs) -> bool;

    fn approx_ne(&self, other: &Rhs) -> bool {
        !self.approx_eq(other)
    }
}

impl ApproxEq for Scalar {
    fn approx_eq(&self, to: &Self) -> bool {
        (self - to).abs() < Self::EPS
    }
}

impl ApproxEq for &Scalar {
    fn approx_eq(&self, to: &Self) -> bool {
        (*self - *to).abs() < Self::EPS
    }
}

#[macro_export]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(crate::ApproxEq::approx_eq(left_val, right_val)) {
                    panic!(
                        "{:?} is not approximately equal to {:?}",
                        left_val, right_val
                    );
                }
            }
        }
    };
}

#[macro_export]
macro_rules! assert_approx_ne {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (crate::ApproxEq::approx_eq(left_val, right_val)) {
                    panic!("{:?} is approximately equal to {:?}", left_val, right_val);
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Side {
    Inside,
    Outside,
    On,
}

#[cfg(test)]
mod tests {
    use super::ApproxEq;

    #[test]
    fn approx_eq() {
        let a = 1.0;
        let b = -1.0;

        assert!(!a.approx_eq(&b));

        let a = 1.0;
        let b = 1.0;

        assert!(a.approx_eq(&b));
    }
}
