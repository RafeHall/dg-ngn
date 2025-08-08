use crate::{
    matrix::Matrix4x4,
    vector::{Vec2, Vec4},
    Scalar,
};

#[derive(Debug)]
pub enum Projection {
    Orthographic {
        offset: Vec2,
        size: Vec2,
        near: Scalar,
        far: Scalar,
    },
    Perspective {
        fov_y: Scalar,
        aspect: Scalar,
        near: Scalar,
        far: Scalar,
    },
}

impl Projection {
    pub fn matrix(&self) -> Matrix4x4 {
        match self {
            Projection::Orthographic {
                offset,
                size,
                near,
                far,
            } => {
                let d = far - near;
                Matrix4x4::new_rows(
                    Vec4::new(2.0 / size.x, 0.0, 0.0, -offset.x / size.x),
                    Vec4::new(0.0, 2.0 / size.y, 0.0, -offset.y / size.y),
                    Vec4::new(0.0, 0.0, -1.0 / d, -near / d),
                    Vec4::new(0.0, 0.0, 0.0, 1.0),
                )
            }
            Projection::Perspective {
                fov_y,
                aspect,
                near,
                far,
            } => {
                todo!();
                let f = 1.0 / (fov_y * 0.5).tan();
                let _d = far - near;
                Matrix4x4::new_rows(
                    Vec4::new(f / aspect, 0.0, 0.0, 0.0),
                    Vec4::new(0.0, -f, 0.0, 0.0),
                    Vec4::new(0.0, 0.0, -1.0, 0.0),
                    Vec4::new(0.0, 0.0, 0.0, 0.0),
                )
            }
        }
    }
}

impl Default for Projection {
    fn default() -> Self {
        Self::Orthographic {
            offset: Vec2::ZERO,
            size: Vec2::new(1280.0, 720.0),
            near: -100.0,
            far: 100.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{scalar, vector::{Vec2, Vec3, Vec4}};

    use super::Projection;

    #[test]
    fn matrix_orthographic() {
        let projection = Projection::Orthographic {
            offset: Vec2::ZERO,
            size: Vec2::new(2.0, 2.0),
            near: -1.0,
            far: 1.0,
        };
        let matrix = projection.matrix();
        println!("{:#?}", matrix);

        let point = Vec4::new(1.0, 1.0, -1.0, 1.0);
        let result = matrix.xform_vec4(point);
        println!("{:?}", result);

        let point = Vec4::new(-1.0, -1.0, 0.0, 1.0);
        let result = matrix.xform_vec4(point);
        println!("{:?}", result);

        let point = Vec4::new(-1.0, -1.0, 1.0, 1.0);
        let result = matrix.xform_vec4(point);
        println!("{:?}", result);
    }

    #[test]
    fn matrix_perspective() {
        let projection = Projection::Perspective {
            fov_y: scalar::consts::FRAC_PI_2,
            aspect: 16.0 / 9.0,
            near: 0.0,
            far: 1.0,
        };
        let matrix = projection.matrix();
        println!("{:#?}", matrix);

        let point = Vec3::new(1.0, 1.0, -0.5);
        let result = matrix.xform_vec3(point);
        println!("{:?}", result);

        let point = point.extend(1.0);
        let result = matrix.xform_vec4(point);
        println!("{:?}", result);
    }
}
