use std::ops::{Index, IndexMut};

use super::{interp::LinearInterp, vector::Vec2, Scalar, line::Line};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    begin: Vec2,
    end: Vec2,
}

impl Rect {
    // pub const ZERO: Rect = Rect::new(Vec2::ZERO, Vec2::ZERO);

    pub fn new(begin: Vec2, end: Vec2) -> Rect {
        let begin = Vec2::new(begin.x.min(end.x), begin.y.min(end.y));
        let end = Vec2::new(begin.x.max(end.x), begin.y.max(end.y));

        Self { begin, end }
    }

    pub fn new_from_size(pos: Vec2, size: Vec2) -> Rect {
        let mut pos = pos;

        if size.x.is_sign_negative() {
            pos.x -= size.x;
        }

        if size.y.is_sign_negative() {
            pos.y -= size.y;
        }

        Self {
            begin: pos,
            end: pos.add(size.abs()),
        }
    }

    pub fn new_from_scalars(
        begin_x: Scalar,
        begin_y: Scalar,
        end_x: Scalar,
        end_y: Scalar,
    ) -> Rect {
        Rect {
            begin: Vec2::new(begin_x.min(end_x), begin_y.min(end_y)),
            end: Vec2::new(begin_x.max(end_x), begin_y.max(end_y)),
        }
    }

    pub fn new_from_scalars_size(x: Scalar, y: Scalar, width: Scalar, height: Scalar) -> Rect {
        Rect {
            begin: Vec2::new(x, y),
            end: Vec2::new(x + width, y + height),
        }
    }

    pub fn get_begin(&self) -> Vec2 {
        self.begin
    }

    pub fn get_end(&self) -> Vec2 {
        self.end
    }

    // [tl, tr, br, bl]
    pub fn get_corners(&self) -> [Vec2; 4] {
        [
            self.begin,
            Vec2::new(self.end.x, self.begin.y),
            self.end,
            Vec2::new(self.begin.x, self.end.y),
        ]
    }

    pub fn size(&self) -> Vec2 {
        self.end.sub(self.begin)
    }

    pub fn set_size(&self, size: Vec2) -> Rect {
        Self::new(self.begin, self.begin.add(size))
    }

    pub fn area(&self) -> Scalar {
        self.size().x * self.size().y
    }

    pub fn has_area(&self, eps: Scalar) -> bool {
        self.area() <= eps
    }

    pub fn get_center(&self) -> Vec2 {
        self.begin.add(self.size().mul_scalar(0.5))
    }

    pub fn has_point(&self, point: Vec2) -> bool {
        self.begin.x < point.x
            && self.end.x > point.x
            && self.begin.y < point.y
            && self.end.y > point.y
    }

    pub fn has_point_inclusive(&self, point: Vec2) -> bool {
        self.begin.x <= point.x
            && self.end.x >= point.x
            && self.begin.y <= point.y
            && self.end.y >= point.y
    }

    pub fn intersection(&self, other: Rect) -> Rect {
        debug_assert!(self.intersects(other));

        Self::new(
            Vec2::new(
                self.begin.x.max(other.begin.x),
                self.begin.y.max(other.begin.y),
            ),
            Vec2::new(self.end.x.min(other.end.x), self.end.y.min(other.end.y)),
        )
    }

    pub fn intersects(&self, other: Rect) -> bool {
        self.begin.x.max(other.begin.x) < self.end.x.min(other.end.x)
            && self.begin.y.max(other.begin.y) < self.end.y.min(other.end.y)
    }

    pub fn intersects_inclusive(&self, other: Rect) -> bool {
        self.begin.x.max(other.begin.x) <= self.end.x.min(other.end.x)
            && self.begin.y.max(other.begin.y) <= self.end.y.min(other.end.y)
    }

    // #[inline]
    // pub fn line_intersection(&self, _line: Line) -> Vec2 {
    //     todo!()
    // }

    pub fn line_intersects(&self, _line: Line) -> bool {
        todo!()
    }

    pub fn encloses(&self, other: Rect) -> bool {
        self.has_point(other.begin)
            && self.has_point(other.end)
            && self.has_point(Vec2::new(other.begin.x, other.end.y))
            && self.has_point(Vec2::new(other.end.x, other.begin.y))
    }

    pub fn encloses_inclusive(&self, other: Rect) -> bool {
        self.has_point_inclusive(other.begin)
            && self.has_point_inclusive(other.end)
            && self.has_point_inclusive(Vec2::new(other.begin.x, other.end.y))
            && self.has_point_inclusive(Vec2::new(other.end.x, other.begin.y))
    }

    pub fn encapsulate(&self, other: Rect) -> Rect {
        self.ensapsulate_point(other.begin)
            .ensapsulate_point(other.end)
    }

    pub fn ensapsulate_point(&self, other: Vec2) -> Rect {
        Rect::new(
            Vec2::new(self.begin.x.min(other.x), self.begin.y.min(other.y)),
            Vec2::new(self.end.x.max(other.x), self.end.y.max(other.y)),
        )
    }

    pub fn shrink(&self, amount: Scalar) -> Rect {
        self.shrink_individual(amount, amount, amount, amount)
    }

    pub fn shrink_individual(
        &self,
        top: Scalar,
        right: Scalar,
        bottom: Scalar,
        left: Scalar,
    ) -> Rect {
        Rect::new(
            self.begin.add(Vec2::new(left, top)),
            self.end.sub(Vec2::new(right * 2.0, bottom * 2.0)),
        )
    }

    pub fn expand(&self, amount: Scalar) -> Rect {
        self.expand_individual(amount, amount, amount, amount)
    }

    pub fn expand_individual(
        &self,
        top: Scalar,
        right: Scalar,
        bottom: Scalar,
        left: Scalar,
    ) -> Rect {
        Rect::new(
            self.begin.sub(Vec2::new(left, top)),
            self.end.add(Vec2::new(right * 2.0, bottom * 2.0)),
        )
    }


}

impl Index<usize> for Rect {
    type Output = Vec2;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.begin,
            1 => &self.end,
            _ => panic!("Invalid index"),
        }
    }
}

impl IndexMut<usize> for Rect {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.begin,
            1 => &mut self.end,
            _ => panic!("Invalid index"),
        }
    }
}

impl LinearInterp for Rect {
    fn lerp(a: Self, b: Self, t: Scalar) -> Self {
        Self::new(a.begin.lerp_to(b.begin, t), a.end.lerp_to(b.end, t))
    }
}
