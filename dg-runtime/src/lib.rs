pub mod headless;
pub mod input;

use std::time::{Duration, Instant};

use dg_math::Scalar;
use input::{InputEvent, InputState};

pub struct Time {
    startup_time: Instant,
    last_update: Instant,
    fixed_step: Duration,
    accumulation: Duration,
}

impl Time {
    pub fn new(fixed_step: Duration) -> Self {
        Self {
            startup_time: Instant::now(),
            last_update: Instant::now(),
            fixed_step,
            accumulation: Duration::ZERO,
        }
    }

    pub fn step(&mut self) -> (TimeStep, usize) {
        let now = Instant::now();
        let delta = now - self.last_update;
        self.last_update = now;
        self.accumulation += delta;

        let mut fixed_steps = 0;
        while self.accumulation >= self.fixed_step {
            fixed_steps += 1;
            self.accumulation -= self.fixed_step;
        }

        // NOTE: fixed_steps will only be > 1 when the game is running at < 50 FPS causing
        // slowdown, so as it stands I am going to compensate by multiplying
        // time.fixed_step by the amount of fixed steps and call fixed_update once as calling it multiple
        // times to compensate could cause cascading slowdown...
        let fixed_delta = self.fixed_step * fixed_steps as u32;

        let t = (self.accumulation.as_secs_f64() / self.fixed_step.as_secs_f64()) as Scalar;

        (
            TimeStep {
                total_elapsed: now - self.startup_time,
                render_interpolation: t,
                delta,
                fixed_delta,
            },
            fixed_steps,
        )
    }
}

pub struct TimeStep {
    pub total_elapsed: Duration,
    pub delta: Duration,
    pub fixed_delta: Duration,
    pub render_interpolation: Scalar,
}

pub struct App {
    pub time: TimeStep,
    pub input: InputState,
    pub exit: bool,
}

#[allow(unused_variables)]
pub trait Runner {
    fn fixed_update(&mut self, app: &mut App) {}
    fn update(&mut self, app: &mut App) {}
    fn render(&mut self, app: &mut App) {}
    fn input(&mut self, event: InputEvent, app: &mut App) {}
}
