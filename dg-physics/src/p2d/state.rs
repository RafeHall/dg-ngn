use dg_math::{vector::Vec2, Scalar};
use slotmap::DenseSlotMap;

slotmap::new_key_type! { pub struct PointMassKey; }

#[derive(Debug, Clone)]
pub struct State {
    pub(crate) point_masses: DenseSlotMap<PointMassKey, PointMass>,
}

impl State {
    pub fn new() -> Self {
        Self {
            point_masses: DenseSlotMap::with_key(),
        }
    }

    pub fn add(&mut self, point_mass: PointMass) -> PointMassKey {
        self.point_masses.insert(point_mass)
    }

    pub fn get(&self, key: PointMassKey) -> &PointMass {
        &self.point_masses[key]
    }

    pub fn get_mut(&mut self, key: PointMassKey) -> &mut PointMass {
        &mut self.point_masses[key]
    }

    pub fn apply_force(&mut self, force: Vec2, key: PointMassKey) {
        self.point_masses[key].pos_accel += force;
    }
}

#[derive(Debug, Clone)]
pub struct PointMass {
    pub angle_accel: Scalar,
    pub angle_vel: Scalar,
    pub angle: Scalar,

    pub pos_accel: Vec2,
    pub pos_vel: Vec2,
    pub pos: Vec2,

    pub mass: Scalar,

    pub force: Vec2,
}

impl Default for PointMass {
    fn default() -> Self {
        Self {
            angle_accel: Default::default(),
            angle_vel: Default::default(),
            angle: Default::default(),

            pos_accel: Default::default(),
            pos_vel: Default::default(),
            pos: Default::default(),

            mass: 1.0,

            force: Default::default(),
        }
    }
}
