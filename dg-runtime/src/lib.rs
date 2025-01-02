pub struct Input;
pub struct InputEvent;

pub trait BasicRunner {
    fn input_event(input: &Input, event: &InputEvent);
    fn update_event(delta: f32);
    fn render_event(delta: f32);
}