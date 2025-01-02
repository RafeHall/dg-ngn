use std::{fmt::Display, ops::{Index, IndexMut}};

use crate::vector::{Vec3, Vec4};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix4x4 {
    pub r0: Vec4,
    pub r1: Vec4,
    pub r2: Vec4,
    pub r3: Vec4,
}

#[rustfmt::skip]
impl Matrix4x4 {
    pub const ZERO: Matrix4x4 = Self::new_rows(Vec4::ZERO, Vec4::ZERO, Vec4::ZERO, Vec4::ZERO);
    pub const IDENTITY: Matrix4x4 = Self::new_rows(Vec4::X, Vec4::Y, Vec4::Z, Vec4::W);

    pub const fn new_rows(r0: Vec4, r1: Vec4, r2: Vec4, r3: Vec4) -> Self {
        Self {
            r0,
            r1,
            r2,
            r3,
        }
    }

    pub const fn new_columns(c0: Vec4, c1: Vec4, c2: Vec4, c3: Vec4) -> Self {
        Self {
            r0: Vec4::new(c0.x, c1.x, c2.x, c3.x),
            r1: Vec4::new(c0.y, c1.y, c2.y, c3.y),
            r2: Vec4::new(c0.z, c1.z, c2.z, c3.z),
            r3: Vec4::new(c0.w, c1.w, c2.w, c3.w),
        }
    }

    pub fn xform_matrix(&self, other: &Matrix4x4) -> Matrix4x4 {
        todo!()
    }

    pub fn xform_vec4(&self, other: Vec4) -> Vec4 {
        todo!()
    }

    pub fn xform_vec3(&self, other: Vec3) -> Vec3 {
        todo!()
    }

    pub fn inverse(&self) -> Matrix4x4 {
        todo!()
    }

    pub fn get_row(&self, r: usize) -> Vec4 {
        assert!(r < 4, "index out of bounds");

        *self.index(r)
    }

    pub fn get_column(&self, c: usize) -> Vec4 {
        assert!(c < 4, "index out of bounds");

        Vec4::new(self.r0[c], self.r1[c], self.r2[c], self.r3[c])
    }

    pub fn swap_columns(&mut self, a: usize, b: usize) {
        assert!(a < 4, "index out of bounds");
        assert!(b < 4, "index out of bounds");

        if a == b {
            return;
        }

        let (a0, b0) = match (a, b) {
            (0, 1) | (1, 0) => (&mut self.r0.x, &mut self.r0.y),
            (0, 2) | (2, 0) => (&mut self.r0.x, &mut self.r0.z),
            (0, 3) | (3, 0) => (&mut self.r0.x, &mut self.r0.w),
            (1, 2) | (2, 1) => (&mut self.r0.y, &mut self.r0.z),
            (1, 3) | (3, 1) => (&mut self.r0.y, &mut self.r0.w),
            (2, 3) | (3, 2) => (&mut self.r0.z, &mut self.r0.w),
            _ => unreachable!(),
        };

        let (a1, b1) = match (a, b) {
            (0, 1) | (1, 0) => (&mut self.r1.x, &mut self.r1.y),
            (0, 2) | (2, 0) => (&mut self.r1.x, &mut self.r1.z),
            (0, 3) | (3, 0) => (&mut self.r1.x, &mut self.r1.w),
            (1, 2) | (2, 1) => (&mut self.r1.y, &mut self.r1.z),
            (1, 3) | (3, 1) => (&mut self.r1.y, &mut self.r1.w),
            (2, 3) | (3, 2) => (&mut self.r1.z, &mut self.r1.w),
            _ => unreachable!(),
        };

        let (a2, b2) = match (a, b) {
            (0, 1) | (1, 0) => (&mut self.r2.x, &mut self.r2.y),
            (0, 2) | (2, 0) => (&mut self.r2.x, &mut self.r2.z),
            (0, 3) | (3, 0) => (&mut self.r2.x, &mut self.r2.w),
            (1, 2) | (2, 1) => (&mut self.r2.y, &mut self.r2.z),
            (1, 3) | (3, 1) => (&mut self.r2.y, &mut self.r2.w),
            (2, 3) | (3, 2) => (&mut self.r2.z, &mut self.r2.w),
            _ => unreachable!(),
        };

        let (a3, b3) = match (a, b) {
            (0, 1) | (1, 0) => (&mut self.r3.x, &mut self.r3.y),
            (0, 2) | (2, 0) => (&mut self.r3.x, &mut self.r3.z),
            (0, 3) | (3, 0) => (&mut self.r3.x, &mut self.r3.w),
            (1, 2) | (2, 1) => (&mut self.r3.y, &mut self.r3.z),
            (1, 3) | (3, 1) => (&mut self.r3.y, &mut self.r3.w),
            (2, 3) | (3, 2) => (&mut self.r3.z, &mut self.r3.w),
            _ => unreachable!(),
        };

        std::mem::swap(a0, b0);
        std::mem::swap(a1, b1);
        std::mem::swap(a2, b2);
        std::mem::swap(a3, b3);
    }

    pub fn swap_rows(&mut self, a: usize, b: usize) {
        assert!(a < 4, "index out of bounds");
        assert!(b < 4, "index out of bounds");

        if a == b {
            return;
        }

        let (a, b) = match (a, b) {
            (0, 1) | (1, 0) => (&mut self.r0, &mut self.r1),
            (0, 2) | (2, 0) => (&mut self.r0, &mut self.r2),
            (0, 3) | (3, 0) => (&mut self.r0, &mut self.r3),
            (1, 2) | (2, 1) => (&mut self.r1, &mut self.r2),
            (1, 3) | (3, 1) => (&mut self.r1, &mut self.r3),
            (2, 3) | (3, 2) => (&mut self.r2, &mut self.r3),
            _ => unreachable!(),
        };

        std::mem::swap(a, b);
    }
}

impl Index<usize> for Matrix4x4 {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r0,
            1 => &self.r1,
            2 => &self.r2,
            3 => &self.r3,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Matrix4x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r0,
            1 => &mut self.r1,
            2 => &mut self.r2,
            3 => &mut self.r3,
            _ => panic!("index out of bounds"),
        }
    }
}

impl Display for Matrix4x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    c0 c1 c2 c3")?;
        writeln!(f, "r0 {}", self.r0)?;
        writeln!(f, "r1 {}", self.r1)?;
        writeln!(f, "r2 {}", self.r2)?;
        writeln!(f, "r3 {}", self.r3)
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vec4;

    use super::Matrix4x4;

    #[test]
    fn test_swap() {
        let mut m = Matrix4x4::new_rows(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(2.0, 2.0, 0.0, 0.0),
            Vec4::new(3.0, 0.0, 3.0, 0.0),
            Vec4::new(4.0, 0.0, 0.0, 4.0),
        );

        println!("{}", m);

        m.swap_rows(0, 3);

        println!("{}", m);

        m.swap_rows(0, 3);
        m.swap_columns(0, 3);
        
        println!("{}", m);
    }
}