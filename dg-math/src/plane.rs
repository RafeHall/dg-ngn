use crate::{ray::Ray, vector::Vec3, ApproxEq, Scalar, Side};

/// A plane defined by a normal and a distance from world origin
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: Scalar,
}

impl Plane {
    /// Plane that splits the x-axis of the world
    pub const X_INTERSECTION: Plane = Plane::new(Vec3::RIGHT, 0.0);
    /// Plane that splits the y-axis of the world
    pub const Y_INTERSECTION: Plane = Plane::new(Vec3::UP, 0.0);
    /// Plane that splits the z-axis of the world
    pub const Z_INTERSECTION: Plane = Plane::new(Vec3::FORWARD, 0.0);

    pub const fn new(normal: Vec3, distance: Scalar) -> Self {
        Self { normal, distance }
    }

    pub fn from_points_clockwise(a: Vec3, b: Vec3, c: Vec3) -> Self {
        let va = b.sub(a);
        let vb = c.sub(a);

        let normal = va.cross(vb).normalized().neg();
        let distance = normal.dot(a);

        Plane::new(normal, distance)
    }

    pub fn from_points_counter_clockwise(a: Vec3, b: Vec3, c: Vec3) -> Self {
        let va = b.sub(a);
        let vb = c.sub(a);

        let normal = va.cross(vb).normalized();
        let distance = normal.dot(a);

        Plane::new(normal, distance)
    }

    pub fn distance_to(&self, point: Vec3) -> Scalar {
        (self.normal.dot(point) - self.distance).abs()
    }

    pub fn signed_distance_to(&self, point: Vec3) -> Scalar {
        self.normal.dot(point) - self.distance
    }

    pub fn side(&self, point: Vec3, eps: Scalar) -> Side {
        match self.signed_distance_to(point) {
            v if v > eps => Side::Outside,
            v if v < -eps => Side::Inside,
            _ => Side::On,
        }
    }

    pub fn project(&self, point: Vec3) -> Vec3 {
        point.sub(self.normal.mul_scalar(self.distance_to(point)))
    }

    pub fn two_plane_intersection(&self, other: &Plane) -> Ray {
        let dot = self.normal.dot(other.normal);
        let c0 = self.distance - other.distance * dot;
        let c1 = other.distance - self.distance * dot;
        let origin = self.normal.mul_scalar(c0).add(other.normal.mul_scalar(c1));
        let normal = self.normal.cross(other.normal);
        Ray::new(origin, origin.add(normal))
    }

    // TODO: Implement using projective geometric algebra as that will likely be a faster solution than
    // solving the system of linear equations using guassian elimination...
    pub fn three_plane_intersection(&self, _b: &Plane, _c: &Plane) -> Vec3 {
        todo!()
    } 
}

impl ApproxEq for Plane {
    fn approx_eq(&self, other: &Self) -> bool {
        self.normal.approx_eq(&other.normal) && self.distance.approx_eq(&other.distance)
    }
}

// impl Into<Triplane> for Plane {
//     fn into(self) -> Triplane {
//         todo!()
//     }
// }

// /// A plane defined by three clockwise points that lie on the plane
// #[derive(Debug, Clone, Copy)]
// pub struct Triplane {
//     pub a: Vec3,
//     pub b: Vec3,
//     pub c: Vec3,
// }

// impl Triplane {
//     // TODO: Ensure the order of vec3s is correct for sides of the plane on consts
//     /// Plane that splits the x-axis of the world
//     pub const X_INTERSECTION: Triplane = Triplane::new(Vec3::ZERO, Vec3::UP, Vec3::FORWARD);
//     /// Plane that splits the y-axis of the world
//     pub const Y_INTERSECTION: Triplane = Triplane::new(Vec3::ZERO, Vec3::FORWARD, Vec3::RIGHT);
//     /// Plane that splits the z-axis of the world
//     pub const Z_INTERSECTION: Triplane = Triplane::new(Vec3::ZERO, Vec3::RIGHT, Vec3::UP);

//     pub const fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
//         Self { a, b, c }
//     }

//     pub fn distance_to(&self, point: Vec3) -> Scalar {
//         let n = self.normal();
//         let d = n.dot(self.a);

//         (n.dot(point) - d).abs()
//     }

//     pub fn signed_distance_to(&self, point: Vec3) -> Scalar {
//         let n = self.normal();
//         let d = n.dot(self.a);

//         n.dot(point) - d
//     }

//     pub fn side(&self, point: Vec3, eps: Scalar) -> Side {
//         match self.signed_distance_to(point) {
//             v if v > eps => Side::Outside,
//             v if v < -eps => Side::Inside,
//             _ => Side::On,
//         }
//     }

//     pub fn project(&self, point: Vec3) -> Vec3 {
//         point.sub(self.normal().mul_scalar(self.distance_to(point)))
//     }

//     pub fn normal(&self) -> Vec3 {
//         let a = self.b.sub(self.a);
//         let b = self.c.sub(self.a);

//         a.cross(b).normalized().neg()
//     }
// }

// impl ApproxEq for Triplane {
//     fn approx_eq(&self, other: &Self) -> bool {
//         self.a.approx_eq(&other.a) && self.b.approx_eq(&other.b) && self.c.approx_eq(&other.c)
//     }
// }

// impl Into<Plane> for Triplane {
//     fn into(self) -> Plane {
//         let normal = self.normal();
//         let distance = normal.dot(self.a);

//         Plane::new(self.normal(), distance)
//     }
// }

#[cfg(test)]
mod tests {
    use crate::{assert_approx_eq, vector::Vec3, Side};

    use super::Plane;
    // use super::Triplane;

    #[test]
    fn distance_to() {
        let p = Plane::new(Vec3::UP, 0.0);
        let v = Vec3::new(0.0, 5.0, 0.0);
        let r = p.distance_to(v);

        assert_approx_eq!(r, 5.0);

        let p = Plane::new(Vec3::UP, 0.0);
        let v = Vec3::new(0.0, -5.0, 0.0);
        let r = p.distance_to(v);

        assert_approx_eq!(r, 5.0);

        let p = Plane::new(Vec3::UP, 0.0);
        let v = Vec3::new(9967217.0, 0.0, -5928.0);
        let r = p.distance_to(v);

        assert_approx_eq!(r, 0.0);

        let p = Plane::new(Vec3::UP, 5.0);
        let v = Vec3::new(9967217.0, 0.0, -5928.0);
        let r = p.distance_to(v);

        assert_approx_eq!(r, 5.0);
    }

    #[test]
    fn signed_distance_to() {
        let p = Plane::new(Vec3::UP, 0.0);
        let v = Vec3::new(0.0, 5.0, 0.0);
        let r = p.signed_distance_to(v);

        assert_approx_eq!(r, 5.0);

        let p = Plane::new(Vec3::UP, 0.0);
        let v = Vec3::new(0.0, -5.0, 0.0);
        let r = p.signed_distance_to(v);

        assert_approx_eq!(r, -5.0);

        let p = Plane::new(Vec3::UP, 0.0);
        let v = Vec3::new(9967217.0, 0.0, -5928.0);
        let r = p.signed_distance_to(v);

        assert_approx_eq!(r, 0.0);

        let p = Plane::new(Vec3::UP, 5.0);
        let v = Vec3::new(9967217.0, 0.0, -5928.0);
        let r = p.signed_distance_to(v);

        assert_approx_eq!(r, -5.0);
    }

    #[test]
    fn side() {
        let p = Plane::new(Vec3::UP, 0.0);
        let v = Vec3::new(0.0, 5.0, 0.0);
        let r = p.side(v, 0.0001);

        assert_eq!(r, Side::Outside);

        let p = Plane::new(Vec3::UP, 0.0);
        let v = Vec3::new(0.0, -5.0, 0.0);
        let r = p.side(v, 0.0001);

        assert_eq!(r, Side::Inside);

        let p = Plane::new(Vec3::UP, 0.0);
        let v = Vec3::new(0.0, 0.0, 0.0);
        let r = p.side(v, 0.000001);

        assert_eq!(r, Side::On);

        let p = Plane::new(Vec3::UP, 5.0);
        let v = Vec3::new(0.0, 5.0, 0.0);
        let r = p.side(v, 0.0001);

        assert_eq!(r, Side::On);

        let p = Plane::new(Vec3::UP, 5.0);
        let v = Vec3::new(0.0, -5.0, 0.0);
        let r = p.side(v, 0.0001);

        assert_eq!(r, Side::Inside);

        let p = Plane::new(Vec3::UP, 5.0);
        let v = Vec3::new(0.0, 0.0, 0.0);
        let r = p.side(v, 0.000001);

        assert_eq!(r, Side::Inside);
    }

    #[test]
    fn project() {
        let p = Plane::new(Vec3::UP, 0.0);
        let v = Vec3::new(0.0, 5.0, 0.0);
        let r = p.project(v);

        assert_approx_eq!(r, Vec3::ZERO);

        let p = Plane::new(Vec3::UP, 0.0);
        let v = Vec3::new(59182.0, 5.0, -25901.0);
        let r = p.project(v);

        assert_approx_eq!(r, Vec3::new(59182.0, 0.0, -25901.0));
    }

    // #[test]
    // fn into_plane() {
    //     let tp = Triplane::X_INTERSECTION;
    //     let p: Plane = tp.into();

    //     assert_approx_eq!(p, Plane::X_INTERSECTION);

    //     let tp = Triplane::Y_INTERSECTION;
    //     let p: Plane = tp.into();

    //     assert_approx_eq!(p, Plane::Y_INTERSECTION);

    //     let tp = Triplane::Z_INTERSECTION;
    //     let p: Plane = tp.into();

    //     assert_approx_eq!(p, Plane::Z_INTERSECTION);

    //     let tp = Triplane::new(
    //         Vec3::new(1.0, 0.0, 0.0),
    //         Vec3::new(1.0, 1.0, 0.0),
    //         Vec3::new(1.0, 0.0, -1.0),
    //     );
    //     let p: Plane = tp.into();

    //     assert_approx_eq!(p, Plane::new(Vec3::RIGHT, 1.0));
    // }
}
