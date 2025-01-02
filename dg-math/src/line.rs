use std::ops::{IndexMut, Index};

use super::{vector::Vec2, Scalar, interp::LinearInterp};

/// Line used for slicing
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
}

impl Line {
    pub const CENTER_HORIZONTAL: Line = Line::new(Vec2::RIGHT, Vec2::LEFT);
    pub const CENTER_VERTICAL: Line = Line::new(Vec2::UP, Vec2::DOWN);

    #[inline]
    pub const fn new(start: Vec2, end: Vec2) -> Line {
        Self { start, end }
    }

    #[inline]
    pub fn new_from_normal(normal: Vec2, distance: Scalar) -> Line {
        let center = normal.mul_scalar(distance);

        Self {
            start: center.add(normal.right()),
            end: center.add(normal.right().neg()),
        }
    }

    #[inline]
    pub fn new_from_slope_intercept(slope: Scalar, y_intercept: Scalar) -> Line {
        Self {
            start: Vec2::new(0.0, y_intercept),
            end: Vec2::new(1.0 / slope, y_intercept + 1.0),
        }
    }

    #[inline]
    pub fn distance_from_origin(&self) -> Scalar {
        self.closest_point(Vec2::ZERO).length()
    }

    #[inline]
    pub fn closest_point(&self, to: Vec2) -> Vec2 {
        let direction = self.direction();
        let offset = to.sub(self.end);

        if direction.x.abs() < Scalar::EPSILON {
            Vec2::new(self.start.x, to.y)
        } else {
            let d = offset.dot(direction);

            self.end.add(direction.mul_scalar(d))
        }
    }

    #[inline]
    pub fn segment_closest_point(&self, to: Vec2) -> Vec2 {
        let direction = self.direction();
        let offset = to.sub(self.end);

        if direction.x.abs() < Scalar::EPSILON {
            Vec2::new(self.start.x, to.y.clamp(self.start.y, self.end.y))
        } else {
            let d = offset.dot(direction);
            let d = d.clamp(0.0, 1.0);

            self.end.add(direction.mul_scalar(d))
        }
    }

    #[inline]
    pub fn segment(&self) -> Vec2 {
        self.end.sub(self.start)
    }

    #[inline]
    pub fn direction(&self) -> Vec2 {
        self.start.direction_to(self.end)
    }

    #[inline]
    pub fn normal(&self) -> Vec2 {
        self.direction().right()
    }

    #[inline]
    pub fn intersection(&self, other: Line) -> Vec2 {
        let divisor = (self.start.x - self.end.x) * (other.start.y - other.end.y)
            - (self.start.y - self.end.y) * (other.start.x - other.end.x);
        
        if divisor == 0.0 {
            return Vec2::INFINITY;
        }

        let a = self.start.cross(self.end);
        let b = other.start.cross(other.end);

        let x_dividen = a * (other.start.x - other.end.x) - b * (self.start.x - self.end.x);
        let y_dividen = a * (other.start.y - other.end.y) - b * (self.start.y - self.end.y);

        Vec2::new(x_dividen, y_dividen).div_scalar(divisor)
    }

    #[inline]
    pub fn parallel(&self, other: Line, eps: Scalar) -> bool {
        let a = self.start.direction_to(self.end);
        let b = other.start.direction_to(other.end);

        (1.0 - a.dot(b).abs()).abs() < eps
    }

    #[inline]
    pub fn perpendicular(&self, other: Line, eps: Scalar) -> bool {
        let a = self.start.direction_to(self.end);
        let b = other.start.direction_to(other.end);

        a.dot(b).abs() < eps
    }

    #[inline]
    pub fn offset_x(&self, offset: Scalar) -> Line {
        self.offset(Vec2::new(offset, 0.0))
    }

    #[inline]
    pub fn offset_y(&self, offset: Scalar) -> Line {
        self.offset(Vec2::new(0.0, offset))
    }

    #[inline]
    pub fn offset(&self, offset: Vec2) -> Line {
        Line::new(self.start.add(offset), self.end.add(offset))
    }

    #[inline]
    pub fn with_distance(&self, distance: Scalar) -> Line {
        let normal = if distance.is_sign_negative() {
            self.normal().neg()
        } else {
            self.normal()
        };
        let distance = distance.abs();

        Line::new_from_normal(normal, distance)
    }

    #[inline]
    pub fn with_normal(&self, normal: Vec2) -> Line {
        let distance = self.distance_from_origin();

        Line::new_from_normal(normal, distance)
    }
}

impl Index<usize> for Line {
    type Output = Vec2;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.start,
            1 => &self.end,
            _ => {
                panic!("Invalid index");
            }
        }
    }
}

impl IndexMut<usize> for Line {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.start,
            1 => &mut self.end,
            _ => {
                panic!("Invalid index");
            }
        }
    }
}

impl LinearInterp for Line {
    fn lerp(a: Self, b: Self, t: Scalar) -> Self {
        Self::new(
            a.start.lerp_to(b.start, t),
            a.end.lerp_to(b.end, t)
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vec2;

    use super::Line;

    #[test]
    fn parallel_perpendicular() {
        let a = Line::new(Vec2::new(5.0, 10.0), Vec2::new(10.0, 15.0));
        let b = Line::new(Vec2::new(5.0, 20.0), Vec2::new(10.0, 25.0));

        assert!(a.parallel(b, 0.001));

        let a = Line::new(Vec2::new(0.0, 5.0), Vec2::new(0.0, -10.0));
        let b = Line::new(Vec2::new(5.0, 0.0), Vec2::new(-10.0, 0.0));

        assert!(a.perpendicular(b, 0.001));
    }

    #[test]
    fn intersections() {
        let a = Line::new(Vec2::new(0.0, 5.0), Vec2::new(0.0, -10.0));
        let b = Line::new(Vec2::new(0.0, 10.0), Vec2::new(0.0, -15.0));

        assert_eq!(a.intersection(b), Vec2::INFINITY);

        let a = Line::new(Vec2::new(0.0, 5.0), Vec2::new(0.0, -10.0));
        let b = Line::new(Vec2::new(5.0, 0.0), Vec2::new(-10.0, 0.0));

        assert_eq!(a.intersection(b), Vec2::ZERO);

        let a = Line::new(Vec2::new(1.0, 1.0), Vec2::new(-1.0, -1.0));
        let b = Line::new(Vec2::new(-1.0, 1.0), Vec2::new(1.0, -1.0));

        assert_eq!(a.intersection(b), Vec2::ZERO);

        let a = Line::new(Vec2::new(2.0, 2.0), Vec2::new(-0.0, -0.0));
        let b = Line::new(Vec2::new(-0.0, 2.0), Vec2::new(2.0, -0.0));

        assert_eq!(a.intersection(b), Vec2::ONE);
    }
}