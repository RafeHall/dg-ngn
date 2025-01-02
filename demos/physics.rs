use std::time::{Duration, Instant};

use dg_math::vector::Vec2 as DgVec2;
use dg_math::Scalar;
use dg_physics::p2d::{
    ode_solver::{OdeSolver, RungeKutta4OdeSolver},
    state::{PointMass, State},
};
use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

fn window_conf() -> Conf {
    Conf {
        window_title: "Physics Demo".into(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        sample_count: 8,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = State::new();
    // type Solver = SemiEulerOdeSolver2D;
    type Solver = RungeKutta4OdeSolver;

    let object = state.add(PointMass {
        pos: DgVec2::new(WINDOW_WIDTH as Scalar / 2.0, WINDOW_HEIGHT as Scalar / 3.0),
        pos_vel: DgVec2::new(64.0, -64.0),
        ..Default::default()
    });

    let mut paused: bool = true;

    let physics_step = Duration::from_secs_f64(1.0 / 64.0);
    let physics_steps: u32 = 10;
    let physics_delta = (physics_step / physics_steps).as_secs_f64() as Scalar;
    let mut physics_time: Duration = Duration::ZERO;

    let mut last = Instant::now();
    loop {
        let now = Instant::now();
        let dt: Duration = now - last;
        if !paused {
            physics_time += dt;
        } else if is_key_pressed(KeyCode::Right) {
            physics_time += physics_step;
        }
        last = now;

        clear_background(Color::from_rgba(0, 25, 65, 255));
        draw_text("Physics Demo", 16.0, 16.0 + 16.0, 32.0, WHITE);

        let o = state.get(object);
        let pos = o.pos;
        let angle = o.angle;
        let r = pos + DgVec2::new(angle.cos(), angle.sin()) * 16.0;

        draw_circle_lines(pos.x as f32, pos.y as f32, 16.0, 1.0, RED);
        draw_line(pos.x as f32, pos.y as f32, r.x as f32, r.y as f32, 1.0, RED);

        let vel = o.pos_vel;
        let vel_dir = vel.normalized();
        let a = pos + vel_dir * 16.0;
        let b = pos + vel_dir * 16.0 + vel;
        draw_line(a.x as f32, a.y as f32, b.x as f32, b.y as f32, 1.0, GREEN);

        while physics_time >= physics_step {
            for _ in 0..physics_steps {
                state.apply_force(DgVec2::DOWN * 1.0, object);
                state = Solver::solve(&mut state, physics_delta);
            }
            physics_time -= physics_step;
        }

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        next_frame().await;
    }
}
