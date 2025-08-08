use crate::{plane::Plane, projection::Projection, global_transform::GlobalTransform3D, vector::Vec3, Side};

#[derive(Debug, Clone)]
pub struct Frustum {
    right: Plane,
    left: Plane,

    top: Plane,
    bottom: Plane,

    far: Plane,
    near: Plane,
}

impl Frustum {
    pub fn new(projection: Projection, view: GlobalTransform3D) -> Self {
        match projection {
            Projection::Orthographic {
                offset,
                size,
                near,
                far,
            } => Self {
                right: Plane::new(view.x.neg(), size.x / 2.0 - offset.x).translate(view.origin),
                left: Plane::new(view.x, size.x / 2.0 + offset.x).translate(view.origin),

                top: Plane::new(view.y.neg(), size.y / 2.0 - offset.y).translate(view.origin),
                bottom: Plane::new(view.y, size.y / 2.0 + offset.y).translate(view.origin),

                far: Plane::new(view.z, far).translate(view.origin),
                near: Plane::new(view.z.neg(), -near).translate(view.origin),
            },
            Projection::Perspective {
                fov_y,
                aspect,
                near,
                far,
            } => {
                let _ = ();

                Self {
                    right: todo!(),
                    left: todo!(),

                    top: todo!(),
                    bottom: todo!(),

                    far: Plane::new(view.z, far).translate(view.origin),
                    near: Plane::new(view.z.neg(), -near).translate(view.origin),
                }
            }
        }
    }

    pub fn point_inside(&self, point: Vec3) -> bool {
        [
            self.right,
            self.left,
            self.top,
            self.bottom,
            self.far,
            self.near,
        ]
        .iter()
        .all(|plane| plane.side(point, 0.0001) == Side::Inside)
    }
}

#[cfg(test)]
mod tests {
    use crate::{projection::Projection, global_transform::GlobalTransform3D, vector::{Vec2, Vec3}};

    use super::Frustum;

    #[test]
    fn point_inside_orthographic() {
        {
            let f = Frustum::new(
                Projection::Orthographic {
                    offset: Vec2::ZERO,
                    size: Vec2::new(16.0, 16.0),
                    near: -8.0,
                    far: 8.0,
                },
                GlobalTransform3D::IDENTITY,
            );
            let p = Vec3::ZERO;
            let r = f.point_inside(p);
    
            assert!(r, "point {:?} should be inside frustum {:#?}", p, f);

            let p = Vec3::new(17.0, 0.0, 0.0);
            let r = f.point_inside(p);
    
            assert!(!r, "point {:?} shouldn't be inside frustum {:#?}", p, f);
        }

        {
            let mut t = GlobalTransform3D::IDENTITY;
            t.origin = Vec3::new(0.0, 0.0, -8.0);

            let f = Frustum::new(
                Projection::Orthographic {
                    offset: Vec2::ZERO,
                    size: Vec2::new(16.0, 16.0),
                    near: -8.0,
                    far: 8.0,
                },
                t,
            );
            let p = Vec3::ZERO;
            let r = f.point_inside(p);
    
            assert!(!r, "point shouldn't be inside frustum");
        }

        {
            let mut t = GlobalTransform3D::IDENTITY;
            t.origin = Vec3::new(7.99, 7.99, -7.99);

            let f = Frustum::new(
                Projection::Orthographic {
                    offset: Vec2::ZERO,
                    size: Vec2::new(16.0, 16.0),
                    near: -8.0,
                    far: 8.0,
                },
                t,
            );
            let p = Vec3::ZERO;
            let r = f.point_inside(p);
    
            assert!(r, "point should be inside frustum");
        }

        {
            let mut t = GlobalTransform3D::IDENTITY;
            t.origin = Vec3::new(0.0, 0.0, -16.0);

            let f = Frustum::new(
                Projection::Orthographic {
                    offset: Vec2::ZERO,
                    size: Vec2::new(16.0, 16.0),
                    near: -8.0,
                    far: 8.0,
                },
                t,
            );

            let p = Vec3::ZERO;
            let r = f.point_inside(p);
    
            assert!(!r, "point shouldn't be inside frustum");

            let p = Vec3::new(0.0, 0.0, -16.0);
            let r = f.point_inside(p);
    
            assert!(r, "point should be inside frustum");
        }

    }
}
