use std::ops::{Index, IndexMut};

use crate::ApproxEq;

use super::{interp::LinearInterp, line::Line, vector::Vec2, Scalar};

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    begin: Vec2,
    end: Vec2,
}

impl Rect {
    // pub const ZERO: Rect = Rect::new(Vec2::ZERO, Vec2::ZERO);

    pub fn new(a: Vec2, b: Vec2) -> Rect {
        let begin = a.vmin(b);
        let end = a.vmax(b);

        Self { begin, end }
    }

    pub const fn new_unchecked(begin: Vec2, end: Vec2) -> Rect {
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

    pub fn begin(&self) -> Vec2 {
        self.begin
    }

    pub fn end(&self) -> Vec2 {
        self.end
    }

    // [tl, tr, br, bl]
    pub fn corners(&self) -> [Vec2; 4] {
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

    pub fn center(&self) -> Vec2 {
        self.begin.add(self.size().mul_scalar(0.5))
    }

    pub fn set_center(&self, center: Vec2) -> Rect {
        let cur_center = self.center();
        let offset = center.sub(cur_center);
        self.offset(offset)
    }

    pub fn offset(&self, offset: Vec2) -> Rect {
        Rect::new_unchecked(self.begin.add(offset), self.end.add(offset))
    }

    pub fn has_point(&self, point: Vec2) -> bool {
        self.begin.less_than(point) && self.end.greater_than(point)
    }

    pub fn has_point_inclusive(&self, point: Vec2) -> bool {
        self.begin.less_than_equals(point) && self.end.greater_than_equals(point)
    }

    pub fn intersection(&self, other: Rect) -> Option<Rect> {
        if !self.intersects(other) {
            return None;
        }

        Some(Self::new(
            self.begin.vmax(other.begin),
            self.end.vmin(other.end),
        ))
    }

    pub fn intersects(&self, other: Rect) -> bool {
        let begin = self.begin.vmax(other.begin);
        let end = self.end.vmin(other.end);

        begin.less_than(end)
    }

    pub fn intersects_inclusive(&self, other: Rect) -> bool {
        let begin = self.begin.vmax(other.begin);
        let end = self.end.vmin(other.end);

        begin.less_than_equals(end)
    }

    // pub fn line_intersection(&self, _line: Line) -> Vec2 {
    //     todo!()
    // }

    pub fn line_intersects(&self, _line: Line) -> bool {
        todo!()
    }

    pub fn encloses(&self, other: Rect) -> bool {
        self.has_point(other.begin) && self.has_point(other.end)
    }

    pub fn encloses_inclusive(&self, other: Rect) -> bool {
        self.has_point_inclusive(other.begin) && self.has_point_inclusive(other.end)
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

impl ApproxEq for Rect {
    fn approx_eq(&self, other: &Self) -> bool {
        self.begin.approx_eq(&other.begin) && self.end.approx_eq(&other.end)
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

#[cfg(test)]
mod tests {
    use crate::{assert_approx_eq, assert_approx_ne, vector::Vec2};

    use super::Rect;

    #[test]
    fn has_point() {
        {
            let rect = Rect::new(Vec2::new(-16.0, -16.0), Vec2::new(16.0, 16.0));
            let point = Vec2::new(0.0, 0.0);
    
            assert_eq!(rect.has_point(point), true);
            assert_eq!(rect.has_point_inclusive(point), true);
        }

        {
            let rect = Rect::new(Vec2::new(-16.0, -16.0), Vec2::new(16.0, 16.0));
            let point = Vec2::new(16.0, 0.0);
    
            assert_eq!(rect.has_point(point), false);
            assert_eq!(rect.has_point_inclusive(point), true);
        }

        {
            let rect = Rect::new(Vec2::new(-16.0, -16.0), Vec2::new(16.0, 16.0));
            let point = Vec2::new(-15.0, -15.0);
    
            assert_eq!(rect.has_point(point), true);
            assert_eq!(rect.has_point_inclusive(point), true);
        }

        {
            let rect = Rect::new(Vec2::new(-16.0, -16.0), Vec2::new(16.0, 16.0));
            let point = Vec2::new(-16.0, -8.0);
    
            assert_eq!(rect.has_point(point), false);
            assert_eq!(rect.has_point_inclusive(point), true);
        }
    }

    #[test]
    fn intersection() {
        {
            let a = Rect::new(Vec2::new(-16.0, -16.0), Vec2::new(16.0, 16.0));
            let b = Rect::new(Vec2::new(-16.0, -16.0), Vec2::new(16.0, 16.0));
            let intersection = a.intersection(b).unwrap();
    
            assert_approx_eq!(a, b);
            assert_approx_eq!(a, intersection);
            assert_approx_eq!(b, intersection);
        }

        {
            let a = Rect::new(Vec2::new(-16.0, -16.0), Vec2::new(16.0, 16.0));
            let b = Rect::new(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0));
            let intersection = a.intersection(b).unwrap();
    
            assert_approx_ne!(a, b);
            assert_approx_ne!(a, intersection);
            assert_approx_eq!(b, intersection);
        }

        {
            let a = Rect::new(Vec2::new(-16.0, -16.0), Vec2::new(16.0, 16.0));
            let b = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(32.0, 32.0));
            let intersection = a.intersection(b).unwrap();
    
            assert_approx_ne!(a, b);
            assert_approx_ne!(a, intersection);
            assert_approx_ne!(b, intersection);
            assert_approx_eq!(
                intersection,
                Rect::new(Vec2::new(0.0, 0.0), Vec2::new(16.0, 16.0))
            );
        }

        {
            let a = Rect::new(Vec2::new(-16.0, -16.0), Vec2::new(0.0, 0.0));
            let b = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(16.0, 16.0));
            let intersection = a.intersection(b);
    
            assert!(intersection.is_none());
        }
    }

    #[test]
    fn encloses() {
        // A and B same rect
        {
            let a = Rect::new(Vec2::new(-16.0, -16.0), Vec2::new(16.0, 16.0));
            let b = a.clone();
            let encloses = a.encloses(b);
            let encloses_inclusive = a.encloses_inclusive(b);

            assert!(!encloses, "{:?} should not enclose {:?}", a, b);
            assert!(
                encloses_inclusive,
                "{:?} should enclose inclusively {:?}",
                a, b
            );
        }

        // A similar rect to B, A larger than B
        {
            let a = Rect::new(Vec2::new(-16.0, -16.0), Vec2::new(16.0, 16.0));
            let b = Rect::new(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0));
            let encloses = a.encloses(b);
            let encloses_inclusive = a.encloses_inclusive(b);

            assert!(encloses, "{:?} should enclose {:?}", a, b);
            assert!(
                encloses_inclusive,
                "{:?} should enclose inclusively {:?}",
                a, b
            );

            let encloses = b.encloses(a);
            let encloses_inclusive = b.encloses_inclusive(a);

            assert!(!encloses, "{:?} should not enclose {:?}", b, a);
            assert!(
                !encloses_inclusive,
                "{:?} should not enclose inclusively {:?}",
                b, a
            );
        }

        // A and B intersect but do not enclose eachother
        {
            let a = Rect::new(Vec2::new(-16.0, -16.0), Vec2::new(16.0, 16.0));
            let b = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(32.0, 32.0));
            let encloses = a.encloses(b);
            let encloses_inclusive = a.encloses_inclusive(b);

            assert!(!encloses, "{:?} should not enclose {:?}", a, b);
            assert!(
                !encloses_inclusive,
                "{:?} should not enclose inclusively {:?}",
                a, b
            );

            let encloses = b.encloses(a);
            let encloses_inclusive = b.encloses_inclusive(a);

            assert!(!encloses, "{:?} should not enclose {:?}", b, a);
            assert!(
                !encloses_inclusive,
                "{:?} should not enclose inclusively {:?}",
                b, a
            );
        }

        // A and B share a corner but do not intersect
        {
            let a = Rect::new(Vec2::new(-16.0, -16.0), Vec2::new(0.0, 0.0));
            let b = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(16.0, 16.0));
            let encloses = a.encloses(b);
            let encloses_inclusive = a.encloses_inclusive(b);

            assert!(!encloses, "{:?} should not enclose {:?}", a, b);
            assert!(
                !encloses_inclusive,
                "{:?} should not enclose inclusively {:?}",
                a, b
            );
        }
    }
}
