use std::time::Duration;

use crate::{input::InputState, App, Runner, Time};

pub fn run_headless(mut runner: impl Runner) {
    let mut time = Time::new(Duration::from_secs_f64(1.0 / 50.0));
    let mut app;

    loop {
        let (time_step, fixed_steps) = time.step();

        app = App {
            exit: false,
            time: time_step,
            input: InputState {},
        };

        
        if fixed_steps > 0 {
            runner.fixed_update(&mut app);
            if app.exit {
                break;
            }
        }

        runner.update(&mut app);
        if app.exit {
            break;
        }

        runner.render(&mut app);
        if app.exit {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::run_headless;
    use crate::{App, Runner};

    #[test]
    fn simple_run() {
        #[derive(Default)]
        struct TestRunner {
            message: String,
            i: usize,
        }

        impl Runner for TestRunner {
            fn fixed_update(&mut self, app: &mut App) {
                self.i += 1;
                if self.i > 25 {
                    app.exit = true;
                }
            }

            fn update(&mut self, app: &mut App) {
                self.message = format!("update: {}", app.time.delta.as_secs_f32());
            }

            fn render(&mut self, app: &mut App) {
                self.message = format!("render: {} / {}", app.time.delta.as_secs_f32(), app.time.render_interpolation);
            }
        }

        run_headless(TestRunner::default());
    }
}
