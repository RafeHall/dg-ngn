use crate::{plane::Plane, projection::Projection, transform::Transform3D, vector::Vec3};

pub struct Frustum {
    right: Plane,
    left: Plane,

    top: Plane,
    bottom: Plane,

    far: Plane,
    near: Plane,
}

impl Frustum {
    pub fn new(projection: Projection, view: Transform3D) -> Self {
        match projection {
            // TODO: handle view origin
            Projection::Orthographic { offset, size, near, far } => {
                Self {
                    right: Plane::new(view.x, size.x / 2.0 + offset.x),
                    left: Plane::new(view.x.neg(), size.x / 2.0 - offset.x),

                    top: Plane::new(view.y, size.y / 2.0 + offset.y),
                    bottom: Plane::new(view.y.neg(), size.y / 2.0 - offset.y),

                    far: Plane::new(view.z, far),
                    near: Plane::new(view.z, near),
                }
            },
            Projection::Perspective { fov_y, aspect, near, far } => {
                Self {
                    ..todo!()
                }
            },
        }

    }
}
