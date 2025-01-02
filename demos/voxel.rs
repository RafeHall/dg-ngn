use dg_runtime::BasicRunner;

struct VoxelRunner {

}

impl BasicRunner for VoxelRunner {
    fn input_event(_input: &dg_runtime::Input, _event: &dg_runtime::InputEvent) {
        todo!()
    }

    fn update_event(_delta: f32) {
        todo!()
    }

    fn render_event(_delta: f32) {
        todo!()
    }
}

fn main() {
    let _runner = VoxelRunner {};
}