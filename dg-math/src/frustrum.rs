use crate::{matrix::Matrix4x4, vector::Vec3};

pub struct Frustrum {
    view_matrix: Matrix4x4,
    inv_view_matrix: Matrix4x4,

    perspective_matrix: Matrix4x4,
    inv_perspective_matrix: Matrix4x4,
}

impl Frustrum {
    pub fn new(view_matrix: Matrix4x4, perspective_matrix: Matrix4x4) -> Self {
        Self {
            inv_view_matrix: view_matrix.inverse(),
            inv_perspective_matrix: perspective_matrix.inverse(),

            view_matrix,
            perspective_matrix,
        }
    }

    pub fn new_from_inverse(inv_view_matrix: Matrix4x4, inv_perspective_matrix: Matrix4x4) -> Self {
        Self {
            view_matrix: inv_view_matrix.inverse(),
            perspective_matrix: inv_perspective_matrix.inverse(),

            inv_view_matrix,
            inv_perspective_matrix,
        }
    }

    pub fn get_corners(&self) -> [Vec3; 8] {
        let mut corners = [
            Vec3::new(-1.0, -1.0, 1.0),
            Vec3::new(1.0, -1.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(-1.0, 1.0, 1.0),
            Vec3::new(-1.0, -1.0, -1.0),
            Vec3::new(1.0, -1.0, -1.0),
            Vec3::new(1.0, 1.0, -1.0),
            Vec3::new(-1.0, 1.0, -1.0),
        ];

        // let m = self.inv_perspective_matrix * self.inv_view_matrix;

        for _corner in &mut corners {

        }

        corners
    }
}
