use crate::{
    rotor::Rotor, vector::{Vec2, Vec3}, Scalar
};

// Effectively a column major 3x2 matrix in memory
#[derive(Debug, Clone)]
pub struct Transform2D {
    pub x: Vec2,
    pub y: Vec2,
    pub origin: Vec2,
}

impl Transform2D {
    pub const IDENTITY: Transform2D = Transform2D::new(Vec2::ZERO, Vec2::RIGHT, Vec2::UP);

    pub const fn new(origin: Vec2, x: Vec2, y: Vec2) -> Self {
        Self { origin, x, y }
    }

    pub fn get_origin(&self) -> Vec2 {
        self.origin
    }

    pub fn set_origin(&mut self, origin: Vec2) {
        self.origin = origin;
    }

    pub fn get_rotation(&self) -> Scalar {
        self.x.angle()
    }

    // TODO: Ensure this is correct...
    pub fn set_rotation(&mut self, rotation: Scalar) {
        let scale = self.get_scale();

        let a = self.x.angle_to(self.y);

        self.x = Vec2::new(rotation.cos(), rotation.sin());
        self.y = Vec2::new((rotation + a).cos(), (rotation + a).sin());

        self.set_scale(scale);
    }

    pub fn get_skew(&self) -> Scalar {
        self.x.angle_to(self.y) + std::f64::consts::PI as Scalar / 2.0
    }

    pub fn set_skew(&mut self, mut skew: Scalar) {
        let s = self.get_scale().y;
        let r = self.get_rotation();

        skew -= std::f64::consts::PI as Scalar / 2.0;

        self.y = Vec2::new((r + skew).cos(), (r + skew).sin()) * s;
    }

    pub fn get_scale(&self) -> Vec2 {
        let d = self.determinant().signum();

        Vec2::new(self.x.length(), self.y.length() * d)
    }

    pub fn set_scale(&mut self, scale: Vec2) {
        self.x = self.x.normalized() * scale.x;
        self.y = self.y.normalized() * scale.y;
    }

    pub fn determinant(&self) -> Scalar {
        self.y.cross(self.x)
    }
}

// Effectively a column major 4x3 matrix in memory
#[derive(Debug, Clone)]
pub struct Transform3D {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
    pub origin: Vec3,
}

impl Transform3D {
    pub const IDENTITY: Transform3D =
        Transform3D::new(Vec3::ZERO, Vec3::RIGHT, Vec3::UP, Vec3::FORWARD);

    pub const fn new(origin: Vec3, x: Vec3, y: Vec3, z: Vec3) -> Self {
        Self { origin, x, y, z }
    }

    pub fn get_origin(&self) -> Vec3 {
        self.origin
    }

    pub fn set_origin(&mut self, origin: Vec3) {
        self.origin = origin;
    }

    pub fn get_rotation(&self) -> Rotor {
        todo!()
    }

    pub fn set_rotation(&mut self, rotation: Rotor) {
        todo!()
    }

    pub fn get_scale(&self) -> Vec3 {
        todo!()
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use crate::scalar;

    use super::Transform2D;

    #[test]
    fn test_skew() {
        let mut t = Transform2D::IDENTITY;
        t.set_skew(scalar::consts::PI / 4.0);

        let s = t.get_skew();

        println!("{:#?}", s);

        println!("{:#?}", t);

        panic!();
    }
}