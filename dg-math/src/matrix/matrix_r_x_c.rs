use std::{fmt::Display, ops::{Index, IndexMut}};

use crate::Scalar;

#[derive(Debug, Clone)]
pub struct MatrixRxC<const R: usize, const C: usize>([[Scalar; C]; R]);

impl<const R: usize, const C: usize> MatrixRxC<R, C> {
    pub fn new(a: [[Scalar; C]; R]) -> Self {
        Self(a)
    }
}

impl<const R: usize, const C: usize> Default for MatrixRxC<R, C> {
    fn default() -> Self {
        Self([[0.0; C]; R])
    }
}

impl<const R: usize, const C: usize> Index<usize> for MatrixRxC<R, C> {
    type Output = [Scalar; C];

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<const R: usize, const C: usize> IndexMut<usize> for MatrixRxC<R, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<const R: usize, const C: usize> Display for MatrixRxC<R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.0.iter().enumerate() {
            write!(f, "r{} ", i)?;
            for column in row {
                write!(f, "{} ", column)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}