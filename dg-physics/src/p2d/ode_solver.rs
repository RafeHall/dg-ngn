use std::{
    iter::Sum,
    ops::{Add, Mul},
};

use dg_math::Scalar;

use super::state::State;

pub trait OdeSolver {
    fn solve(initial: &State, delta: Scalar) -> State;
}

macro_rules! euler {
    ($initial:expr, $change:expr, $delta:expr) => {
        ($initial) + ($change) * ($delta)
    };
}

#[derive(Default)]
pub struct EulerOdeSolver;

impl OdeSolver for EulerOdeSolver {
    fn solve(initial: &State, delta: Scalar) -> State {
        let mut state = initial.clone();

        state.point_masses.iter_mut().for_each(|(_, value)| {
            value.pos = euler!(value.pos, value.pos_vel, delta);
            value.angle = euler!(value.angle, value.angle_vel, delta);
            value.pos_vel = euler!(value.pos_vel, value.pos_accel, delta);
            value.angle_vel = euler!(value.angle_vel, value.angle_accel, delta);
        });

        state
    }
}

#[derive(Default)]
pub struct SemiEulerOdeSolver;

impl OdeSolver for SemiEulerOdeSolver {
    fn solve(initial: &State, delta: Scalar) -> State {
        let mut state = initial.clone();

        state.point_masses.iter_mut().for_each(|(_, value)| {
            value.pos_vel = euler!(value.pos_vel, value.pos_accel, delta);
            value.angle_vel = euler!(value.angle_vel, value.angle_accel, delta);
            value.pos = euler!(value.pos, value.pos_vel, delta);
            value.angle = euler!(value.angle, value.angle_vel, delta);
        });

        state
    }
}

#[derive(Clone)]
pub struct RungeKutta4OdeSolver;

impl RungeKutta4OdeSolver {
    fn solve_rk4<T: Add<T, Output = T> + Mul<Scalar, Output = T> + Sum + Clone + Copy>(
        initial: T,
        change: T,
        delta: Scalar,
    ) -> T {
        let k1 = change;
        let k2 = change + k1 * 0.5 * delta;
        let k3 = change + k2 * 0.5 * delta;
        let k4 = change + k3 * delta;

        initial + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (delta / 6.0)
    }
}

impl OdeSolver for RungeKutta4OdeSolver {
    fn solve(initial: &State, delta: Scalar) -> State {
        let mut state = initial.clone();

        state.point_masses.iter_mut().for_each(|(_, value)| {
            value.pos = Self::solve_rk4(value.pos, value.pos_vel, delta);
            value.pos_vel = Self::solve_rk4(value.pos_vel, value.pos_accel, delta);
            value.angle = Self::solve_rk4(value.angle, value.angle_vel, delta);
            value.angle_vel = Self::solve_rk4(value.angle_vel, value.angle_accel, delta);
        });

        state
    }
}
