mod draw_calls;

use std::time::Duration;

use dg_math::{aabb::AABB, color::Color, frustrum::Frustrum, matrix::Matrix2x2, plane::Plane, rect::Rect, rotor::Rotor, scalar, transform::{Transform2D, Transform3D}, vector::{Vec2, Vec3}, Scalar};
use dg_structures::Id;
use draw_calls::DrawCalls;

#[derive(Debug, Clone, Copy)]
pub enum Lifetime {
    Frame,
    Duration(Duration),
}

trait ImmediateDraw2DImpl {
    fn begin(&mut self, id: Id);
    fn end(&mut self, id: Id);

    fn draw_points(&mut self, points: Vec<(Vec2, Scalar)>);
    fn draw_lines(&mut self, lines: Vec<[Vec2; 2]>);
    fn draw_triangles(&mut self, vertices: Vec<[Vec2; 3]>);
}

trait ImmediateDraw3DImpl {
    fn begin(&mut self, id: Id);
    fn end(&mut self, id: Id);

    fn draw_points(&mut self, points: Vec<Vec3>);
    fn draw_lines(&mut self, lines: Vec<[Vec3; 2]>);
    fn draw_triangles(&mut self, vertices: Vec<[Vec3; 3]>);
}

pub struct ImmediateDraw2D {
    transform: Transform2D,
    implementation: Box<dyn ImmediateDraw2DImpl>,

    points: DrawCalls<Vec<(Vec2, Scalar)>>,
    lines: DrawCalls<Vec<[Vec2; 2]>>,
    triangles: DrawCalls<Vec<[Vec2; 3]>>,
}


impl ImmediateDraw2D {
    pub fn set_transform(&mut self, transform: Transform2D) {
        self.transform = transform;
    }

    pub fn set_translation(&mut self, translation: Vec2) {
        self.transform.origin = translation;
    }

    pub fn set_rotation(&mut self, rotation: Scalar) {
        self.transform.set_rotation(rotation);
    }

    pub fn set_scale(&mut self, scale: Vec2) {
        self.transform.set_scale(scale);
    }

    pub fn point(&mut self, point: Vec2, size: Scalar, color: Color, lifetime: Lifetime) {
        self.add_points(vec![(point, size)], color, lifetime);
    }

    pub fn line(&mut self, a: Vec2, b: Vec2, color: Color, lifetime: Lifetime) {
        self.add_lines(vec![[a, b]], color, lifetime);
    }

    pub fn circle(&mut self, center: Vec2, radius: Scalar, color: Color, lifetime: Lifetime) {
        const STEPS: usize = 16;

        let mut lines = vec![[Vec2::ZERO; 2]; STEPS];

        for i in 0..STEPS {
            let t0 = scalar::consts::PI * 2.0 * STEPS as Scalar / i as Scalar;
            let t1 = scalar::consts::PI * 2.0 * (STEPS + 1) as Scalar / i as Scalar;

            lines[i] = [
                Vec2::new(t0.cos(), t0.sin()) * radius + center,
                Vec2::new(t1.cos(), t1.sin()) * radius + center,
            ];
        }

        self.add_lines(lines, color, lifetime);
    }

    pub fn arc(&mut self, center: Vec2, radius: Scalar, start: Scalar, end: Scalar, color: Color, lifetime: Lifetime) {
        const STEPS: usize = 16;

        let mut lines = vec![[Vec2::ZERO; 2]; STEPS];

        let difference = end - start;

        for i in 0..STEPS {
            let t0 = start + difference * STEPS as Scalar / i as Scalar;
            let t1 = start + difference * (STEPS + 1) as Scalar / i as Scalar;

            lines[i] = [
                Vec2::new(t0.cos(), t0.sin()) * radius + center,
                Vec2::new(t1.cos(), t1.sin()) * radius + center,
            ];
        }

        self.add_lines(lines, color, lifetime);
    }

    pub fn rect(&mut self, rect: Rect, color: Color, lifetime: Lifetime) {
        let corners = rect.get_corners();

        let lines = vec![
            [corners[0], corners[1]],
            [corners[1], corners[2]],
            [corners[2], corners[3]],
            [corners[3], corners[0]],
        ];

        self.add_lines(lines, color, lifetime);
    }
    
    pub fn rounded_rect(&mut self, rect: Rect, color: Color, radius: Scalar, lifetime: Lifetime) {
        let outward = radius.is_sign_positive();
        let radius = radius.abs();

        let corners = rect.get_corners();

        let lines = vec![
            [corners[0] + Vec2::RIGHT * radius, corners[1] + Vec2::LEFT * radius],
            [corners[1] + Vec2::DOWN * radius, corners[2] + Vec2::UP * radius],
            [corners[2] + Vec2::LEFT * radius, corners[3] + Vec2::RIGHT * radius],
            [corners[3] + Vec2::UP * radius, corners[0] + Vec2::DOWN * radius],
        ];

        let corners = rect.shrink(radius).get_corners();

        for i in 0..4 {
            let center = corners[i];
            let start = if outward {
                scalar::consts::PI + scalar::consts::FRAC_PI_2 * i as Scalar
            } else {
                scalar::consts::FRAC_PI_2 * i as Scalar
            };

            let end = start + scalar::consts::FRAC_PI_2;

            self.arc(center, radius, start, end, color, lifetime);
        }

        self.add_lines(lines, color, lifetime);
    }

    pub fn arrow(&mut self, from: Vec2, to: Vec2, size: Scalar, color: Color, lifetime: Lifetime) {        
        let right = from.direction_to(to);
        let m = Matrix2x2::new_from_vecs(right, right.right());

        let a = m.transform(Vec2::new(-1.0, 1.0).normalized() * size);
        let b = m.transform(Vec2::new(-1.0, -1.0).normalized() * size);

        let lines = vec![
            [from, to],
            [to, to + a],
            [to, to + b],
            [to + a, to + b],
        ];

        self.add_lines(lines, color, lifetime);
    }

    pub fn cross(&mut self, center: Vec2, right: Vec2, color: Color, lifetime: Lifetime) {
        let up = right.right();

        self.line(center - right, center + right, color, lifetime);
        self.line(center - up, center + up, color, lifetime);
    }

    pub fn axis(&mut self, center: Vec2, size: Scalar, color_horizontal: Color, color_vertical: Color, lifetime: Lifetime) {
        self.line(center, center + Vec2::new(size, 0.0), color_horizontal, lifetime);
        self.line(center, center + Vec2::new(0.0, size), color_vertical, lifetime);
    }

    pub fn grid(&mut self, center: Vec2, extents: Vec2, x_steps: u32, y_steps: u32, color: Color, lifetime: Lifetime) {
        let mut lines = Vec::new();

        for x in 0..=x_steps {
            let x_pos = x as Scalar / x_steps as Scalar * extents.x;

            lines.push([Vec2::new(x_pos, -extents.y) + center, Vec2::new(x_pos, extents.y) + center]);
            lines.push([Vec2::new(-x_pos, -extents.y) + center, Vec2::new(-x_pos, extents.y) + center]);
        }

        for y in 0..=y_steps {
            let y_pos = y as Scalar / y_steps as Scalar * extents.y;

            lines.push([Vec2::new(-extents.x, y_pos) + center, Vec2::new(extents.x, y_pos) + center]);
            lines.push([Vec2::new(-extents.x, -y_pos) + center, Vec2::new(extents.x, -y_pos) + center]);
        }

        self.add_lines(lines, color, lifetime);
    }
    
    pub fn polyline() {
        todo!()
    }

    pub fn polygon() {
        todo!()
    }

    fn add_points(&mut self, points: Vec<(Vec2, Scalar)>, color: Color, lifetime: Lifetime) {
        match lifetime {
            Lifetime::Frame => {
                self.implementation.draw_points(points);
            },
            Lifetime::Duration(duration) => {
                self.points.insert(duration, points);
            },
        }
    }

    fn add_lines(&mut self, lines: Vec<[Vec2; 2]>, color: Color, lifetime: Lifetime) {
        match lifetime {
            Lifetime::Frame => {
                self.implementation.draw_lines(lines);
            },
            Lifetime::Duration(duration) => {
                self.lines.insert(duration, lines);
            },
        }
    }

    fn add_triangles(&mut self, triangles: Vec<[Vec2; 3]>, color: Color, lifetime: Lifetime) {
        match lifetime {
            Lifetime::Frame => {
                self.implementation.draw_triangles(triangles);
            },
            Lifetime::Duration(duration) => {
                self.triangles.insert(duration, triangles);
            },
        }
    }
}

pub struct ImmediateDraw3D {
    transform: Transform3D,
    implementation: Box<dyn ImmediateDraw3DImpl>,
}

impl ImmediateDraw3D {
    pub fn set_transform(&mut self, transform: Transform3D) {
        self.transform = transform;
    }

    pub fn set_translation(&mut self, translation: Vec3) {
        self.transform.origin = translation;
    }

    pub fn set_rotation(&mut self, rotation: Rotor) {
        self.transform.set_rotation(rotation);
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.transform.set_scale(scale);
    }

    pub fn point(&mut self, point: Vec3, size: Scalar, color: Color, lifetime: Lifetime) {

    }   

    pub fn line(&mut self, a: Vec3, b: Vec3, color: Color, lifetime: Lifetime) {

    }

    pub fn circle(&mut self, center: Vec3, radius: Scalar, normal: Vec3, lifetime: Lifetime) {
        
    }
    
    pub fn sphere(&mut self, center: Vec3, radius: Scalar, lifetime: Lifetime) {

    }
    
    pub fn cylinder(&mut self, center: Vec3, height: Scalar, radius: Scalar, direction: Vec3, lifetime: Lifetime) {

    }

    pub fn rect(&mut self, center: Vec3, normal: Vec3, size: Vec2, lifetime: Lifetime) {

    }

    pub fn rounded_rect(&mut self, center: Vec3, normal: Vec3, size: Vec2, radius: Scalar, lifetime: Lifetime) {

    }
    
    pub fn aabb(&mut self, aabb: AABB, color: Color, lifetime: Lifetime) {

    }

    pub fn frustrum(&mut self, frustrum: Frustrum, color: Color, lifetime: Lifetime) {
        let corners = frustrum.get_corners();

        let mut lines = Vec::new();

        lines.push([corners[0], corners[1]]);
        lines.push([corners[1], corners[2]]);
        lines.push([corners[2], corners[3]]);
        lines.push([corners[3], corners[0]]);

        lines.push([corners[0], corners[4]]);
        lines.push([corners[1], corners[5]]);
        lines.push([corners[2], corners[6]]);
        lines.push([corners[3], corners[7]]);

        lines.push([corners[4], corners[5]]);
        lines.push([corners[5], corners[6]]);
        lines.push([corners[6], corners[7]]);
        lines.push([corners[7], corners[4]]);

        self.add_lines(lines, color, lifetime);
    }
    
    pub fn plane(&mut self, plane: Plane, size: Scalar, normal_size: Scalar, color: Color, lifetime: Lifetime) {

    }
    
    pub fn cone(&mut self, lifetime: Lifetime) {

    }
    
    pub fn cross(&mut self, lifetime: Lifetime) {

    }

    pub fn axis(&mut self, lifetime: Lifetime) {

    }

    pub fn grid(&mut self, lifetime: Lifetime) {

    }

    pub fn arrow(&mut self, lifetime: Lifetime) {

    }

    fn add_points(&mut self, points: Vec<Vec3>, color: Color, lifetime: Lifetime) {}
    fn add_lines(&mut self, lines: Vec<[Vec3; 2]>, color: Color, lifetime: Lifetime) {}
    fn add_triangles(&mut self, triangles: Vec<[Vec3; 3]>, color: Color, lifetime: Lifetime) {}
}