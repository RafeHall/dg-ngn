use crate::{
    rotor::Rotor,
    vector::{Vec2, Vec3},
    Scalar,
};

// Effectively a column major 3x2 matrix in memory
#[derive(Debug, Clone)]
pub struct Transform2D {
    pub x: Vec2,
    pub y: Vec2,
    pub origin: Vec2,
}

impl Transform2D {
    // TODO: Figure out if Vec2::UP is correct here or if Vec2::DOWN is proper and should be reflected (-y) in code
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

        self.y = Vec2::new((r + skew).cos(), (r + skew).sin()).mul_scalar(s);
    }

    pub fn get_scale(&self) -> Vec2 {
        let d = self.determinant().signum();

        Vec2::new(self.x.length(), self.y.length() * d)
    }

    pub fn set_scale(&mut self, scale: Vec2) {
        self.x = self.x.normalized().mul_scalar(scale.x);
        self.y = self.y.normalized().mul_scalar(scale.y);
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

    pub fn get_euler(&self) -> Vec3 {
        todo!()
    }

    pub fn set_euler(&mut self, euler: Vec3) {
        todo!()
    }

    pub fn get_rotor(&self) -> Rotor {
        todo!()
    }

    pub fn set_rotor(&mut self, rotor: Rotor) {
        todo!()
    }

    pub fn get_scale(&self) -> Vec3 {
        Vec3::new(self.x.length(), self.y.length(), self.z.length())
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.x = self.x.normalized().mul_scalar(scale.x);
        self.y = self.y.normalized().mul_scalar(scale.y);
        self.z = self.z.normalized().mul_scalar(scale.z);
    }

    pub fn determinant(&self) -> Scalar {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vec2;

    use super::Transform2D;

    #[test]
    fn scale_2d() {
        let mut t = Transform2D::new(Vec2::ZERO, Vec2::RIGHT, Vec2::UP);
        t.set_scale(Vec2::new(2.0, 3.0));

        let scale = t.get_scale();
        println!("{:?}", scale);
        
        let rotation = t.get_rotation();
        println!("{}", rotation);

        panic!();
    }

    #[test]
    fn rotation_2d() {
        let mut t = Transform2D::new(Vec2::ZERO, Vec2::RIGHT, Vec2::UP);
        let r = t.get_rotation();

        println!("{}", r);

        t.x = Vec2::DOWN;
        t.y = Vec2::RIGHT;

        let r = t.get_rotation();
        println!("{}", r);

        t.x = Vec2::LEFT;
        t.y = Vec2::DOWN;

        let r = t.get_rotation();
        println!("{}", r);

        t.x = Vec2::UP;
        t.y = Vec2::LEFT;

        let r = t.get_rotation();
        println!("{}", r);

        panic!();
    }

    #[test]
    fn scale_3d() {
        panic!()
    }
}
