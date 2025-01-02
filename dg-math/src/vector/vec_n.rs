use std::{fmt::Display, ops::{Index, IndexMut}};

use crate::Scalar;

#[derive(Debug, Clone)]
pub struct VecN<const N: usize>([Scalar; N]);

impl<const N: usize> VecN<N> {
    pub const ZERO: VecN<N> = VecN::new([0.0; N]);
    pub const ONE: VecN<N> = VecN::new([1.0; N]);
    pub const NEG_ONE: VecN<N> = VecN::new([-1.0; N]);

    pub const INFINITY: VecN<N> = VecN::new([Scalar::INFINITY; N]);
    pub const NEG_INFINITY: VecN<N> = VecN::new([Scalar::NEG_INFINITY; N]);

    pub const NAN: VecN<N> = VecN::new([Scalar::NAN; N]);

    pub const fn new(a: [Scalar; N]) -> Self {
        Self(a)
    }

    pub fn add(&mut self, other: &VecN<N>) {
        self.0.iter_mut().zip(other.0.iter()).for_each(|(a, b)| {
            *a += *b;
        });
    }

    pub fn sub(&mut self, other: &VecN<N>) {
        self.0.iter_mut().zip(other.0.iter()).for_each(|(a, b)| {
            *a -= *b;
        });
    }

    pub fn div(&mut self, other: &VecN<N>) {
        self.0.iter_mut().zip(other.0.iter()).for_each(|(a, b)| {
            *a /= *b;
        });
    }

    pub fn mul(&mut self, other: &VecN<N>) {
        self.0.iter_mut().zip(other.0.iter()).for_each(|(a, b)| {
            *a *= *b;
        });
    }

    pub fn add_scalar(&mut self, other: Scalar) {
        self.0.iter_mut().for_each(|a| {
            *a += other;
        });
    }

    pub fn sub_scalar(&mut self, other: Scalar) {
        self.0.iter_mut().for_each(|a| {
            *a -= other;
        });
    }

    pub fn div_scalar(&mut self, other: Scalar) {
        self.0.iter_mut().for_each(|a| {
            *a /= other;
        });
    }

    pub fn mul_scalar(&mut self, other: Scalar) {
        self.0.iter_mut().for_each(|a| {
            *a *= other;
        });
    }

    pub fn neg(&mut self) {
        self.0.iter_mut().for_each(|a| {
            *a = -*a;
        });
    }

    pub fn inverse(&mut self) {
        self.0.iter_mut().for_each(|a| {
            *a = 1.0 / *a;
        });
    }

    pub fn max(&self) -> Scalar {
        *self
            .0
            .iter()
            .max_by(|a, b| Scalar::total_cmp(a, b))
            .unwrap()
    }

    pub fn imax(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .max_by(|a, b| Scalar::total_cmp(a.1, b.1))
            .unwrap()
            .0
    }

    pub fn min(&self) -> Scalar {
        *self
            .0
            .iter()
            .min_by(|a, b| Scalar::total_cmp(a, b))
            .unwrap()
    }

    pub fn imin(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .min_by(|a, b| Scalar::total_cmp(a.1, b.1))
            .unwrap()
            .0
    }

    pub fn dot(&self, other: &VecN<N>) -> Scalar {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(0.0, |r, (a, b)| r + a * b)
    }

    pub fn cross(&self, other: &VecN<N>) {
        todo!("please god help me")
    }

    pub fn distance_to(&self, other: &VecN<N>) -> Scalar {
        self.distance_squared_to(other).sqrt()
    }

    pub fn distance_squared_to(&self, other: &VecN<N>) -> Scalar {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(0.0, |r, (a, b)| r + (b - a).powi(2))
    }

    pub fn length(&self) -> Scalar {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> Scalar {
        self.0.iter().fold(0.0, |r, a| r + a * a)
    }

    pub fn normalized(&mut self) {
        self.div_scalar(self.length());
    }

    pub fn clamped(&mut self, min: &VecN<N>, max: &VecN<N>) {
        self.0
            .iter_mut()
            .zip(min.0.iter())
            .zip(max.0.iter())
            .for_each(|((a, min), max)| {
                *a = a.clamp(*min, *max);
            });
    }

    pub fn clamped_scalar(&mut self, min: Scalar, max: Scalar) {
        self.0.iter_mut().for_each(|a| {
            *a = a.clamp(min, max);
        });
    }

    pub fn round(&mut self) {
        self.0.iter_mut().for_each(|a| *a = a.round());
    }

    pub fn floor(&mut self) {
        self.0.iter_mut().for_each(|a| *a = a.floor());
    }

    pub fn ceil(&mut self) {
        self.0.iter_mut().for_each(|a| *a = a.ceil());
    }

    pub fn abs(&mut self) {
        self.0.iter_mut().for_each(|a| *a = a.abs());
    }
}

impl<const N: usize> Default for VecN<N> {
    fn default() -> Self {
        Self { 0: [0.0; N] }
    }
}

impl<const N: usize> Index<usize> for VecN<N> {
    type Output = Scalar;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<const N: usize> IndexMut<usize> for VecN<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<const N: usize> Display for VecN<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, v) in self.0.iter().enumerate() {
            write!(f, "{}", v)?;
            if i != self.0.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}