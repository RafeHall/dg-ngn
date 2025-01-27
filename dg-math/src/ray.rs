use crate::vector::Vec3;


#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub start: Vec3,
    pub end: Vec3,
}

impl Ray {
    pub const fn new(start: Vec3, end: Vec3) -> Ray {
        Ray {
            start,
            end,
        }
    }
}