use crate::vector::Vec3;

#[derive(Debug, Clone)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        let n_min = min.vmin(max);
        let n_max = min.vmax(max);

        Self {
            min: n_min,
            max: n_max,
        }
    }

    pub const fn new_unchecked(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn has_point(&self, point: Vec3) -> bool {
        self.min.less_than(point) && self.max.greater_than(point)
    }

    pub fn has_point_inclusive(&self, point: Vec3) -> bool {
        self.min.less_than_equals(point) && self.max.greater_than_equals(point)
    }

    pub fn intersection(&self, other: &AABB) -> Option<AABB> {
        if !self.intersects(other) {
            return None;
        }

        Some(Self::new_unchecked(
            self.min.vmax(other.min),
            self.max.vmin(other.max),
        ))
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        let min = self.min.vmax(other.min);
        let max = self.max.vmin(other.max);

        min.less_than(max)
    }

    pub fn intersects_inclusive(&self, other: &AABB) -> bool {
        let min = self.min.vmax(other.min);
        let max = self.max.vmin(other.max);

        min.less_than_equals(max)
    }

    pub fn center(&self) -> Vec3 {
        self.min.add(self.max.sub(self.min).div_scalar(2.0))
    }

    pub fn set_center(&mut self, center: Vec3) {
        let cur_center = self.center();
        let offset = center.sub(cur_center);
        self.offset(offset)
    }

    pub fn offset(&mut self, offset: Vec3) {
        self.min = self.min.add(offset);
        self.max = self.max.add(offset);
    }
}
