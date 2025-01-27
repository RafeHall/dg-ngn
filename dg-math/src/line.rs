use std::ops::{Index, IndexMut};

use crate::{vector::Vec3, ApproxEq};

use super::{interp::LinearInterp, vector::Vec2, Scalar};

/// Line used for slicing
#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
}

impl Line {
    pub const CENTER_HORIZONTAL: Line = Line::new(Vec2::RIGHT, Vec2::LEFT);
    pub const CENTER_VERTICAL: Line = Line::new(Vec2::UP, Vec2::DOWN);

    pub const fn new(start: Vec2, end: Vec2) -> Line {
        Self { start, end }
    }

    pub fn new_from_normal(normal: Vec2, distance: Scalar) -> Line {
        let center = normal.mul_scalar(distance);

        Self {
            start: center.add(normal.right()),
            end: center.add(normal.right().neg()),
        }
    }

    pub fn new_from_slope_intercept(slope: Scalar, y_intercept: Scalar) -> Line {
        Self {
            start: Vec2::new(0.0, y_intercept),
            end: Vec2::new(1.0 / slope, y_intercept + 1.0),
        }
    }

    pub fn distance_from_origin(&self) -> Scalar {
        self.closest_point(Vec2::ZERO).length()
    }

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

    pub fn segment(&self) -> Vec2 {
        self.end.sub(self.start)
    }

    pub fn direction(&self) -> Vec2 {
        self.start.direction_to(self.end)
    }

    pub fn normal(&self) -> Vec2 {
        self.direction().right()
    }

    pub fn intersection(&self, other: Line) -> Option<Vec2> {
        let a = Vec3::new(
            self.end.y - self.start.y,
            self.start.x - self.end.x,
            -self.start.x * (self.end.y - self.start.y)
                + self.start.y * (self.end.x - self.start.x),
        );

        let b = Vec3::new(
            other.end.y - other.start.y,
            other.start.x - other.end.x,
            -other.start.x * (other.end.y - other.start.y)
                + other.start.y * (other.end.x - other.start.x),
        );

        let h = a.cross(b);
        if h.z.approx_eq(&0.0) {
            return None;
        }

        Some(Vec2::new(h.x / h.z, h.y / h.z))
    }

    pub fn parallel(&self, other: Line, eps: Scalar) -> bool {
        let a = self.start.direction_to(self.end);
        let b = other.start.direction_to(other.end);

        (1.0 - a.dot(b).abs()).abs() < eps
    }

    pub fn perpendicular(&self, other: Line, eps: Scalar) -> bool {
        let a = self.start.direction_to(self.end);
        let b = other.start.direction_to(other.end);

        a.dot(b).abs() < eps
    }

    pub fn offset_x(&self, offset: Scalar) -> Line {
        self.offset(Vec2::new(offset, 0.0))
    }

    pub fn offset_y(&self, offset: Scalar) -> Line {
        self.offset(Vec2::new(0.0, offset))
    }

    pub fn offset(&self, offset: Vec2) -> Line {
        Line::new(self.start.add(offset), self.end.add(offset))
    }

    pub fn with_distance(&self, distance: Scalar) -> Line {
        let normal = if distance.is_sign_negative() {
            self.normal().neg()
        } else {
            self.normal()
        };
        let distance = distance.abs();

        Line::new_from_normal(normal, distance)
    }

    pub fn with_normal(&self, normal: Vec2) -> Line {
        let distance = self.distance_from_origin();

        Line::new_from_normal(normal, distance)
    }
}

impl ApproxEq for Line {
    fn approx_eq(&self, other: &Self) -> bool {
        self.start.approx_eq(&other.start) && self.end.approx_eq(&other.end)
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
        Self::new(a.start.lerp_to(b.start, t), a.end.lerp_to(b.end, t))
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_approx_eq, vector::Vec2};

    use super::Line;

    #[test]
    fn parallel_perpendicular() {
        {
            let a = Line::new(Vec2::new(5.0, 10.0), Vec2::new(10.0, 15.0));
            let b = Line::new(Vec2::new(5.0, 20.0), Vec2::new(10.0, 25.0));
    
            assert!(a.parallel(b, 0.001), "{:?} should be parallel to {:?}", a, b);
            assert!(!a.perpendicular(b, 0.001), "{:?} should not be perpendicular to {:?}", a, b);
        }

        {
            let a = Line::new(Vec2::new(0.0, 5.0), Vec2::new(0.0, -10.0));
            let b = Line::new(Vec2::new(5.0, 0.0), Vec2::new(-10.0, 0.0));
    
            assert!(!a.parallel(b, 0.001), "{:?} should not be parallel to {:?}", a, b);
            assert!(a.perpendicular(b, 0.001), "{:?} should be perpendicular to {:?}", a, b);
        }
    }

    #[test]
    fn intersections() {
        // Two lines that intersect at an integer position
        {
            let a = Line::new(Vec2::new(5.0, -5.0), Vec2::new(0.0, 5.0));
            let b = Line::new(Vec2::new(1.0, -5.0), Vec2::new(1.0, 5.0));
            let intersection = a.intersection(b).unwrap();

            assert_approx_eq!(intersection, Vec2::new(1.0, 3.0));
        }

        // Two lines that intersect at a decimal position
        {
            let a = Line::new(Vec2::new(5.0, -15.0), Vec2::new(51.0, 5.0));
            let b = Line::new(Vec2::new(25.0, 5.0), Vec2::new(1.0, 15.0));
            let intersection = a.intersection(b).unwrap();

            assert_approx_eq!(intersection, Vec2::new(38.2765957447, -0.531914893617));
        }

        // Parallel lines, do not intersect
        {
            let a = Line::new(Vec2::new(5.0, -15.0), Vec2::new(51.0, 5.0));
            let b = a.clone().offset_x(5.0);

            let intersection = a.intersection(b);

            assert!(intersection.is_none(), "{:?} should not intersect {:?}", a, b);
        }
    }
}
