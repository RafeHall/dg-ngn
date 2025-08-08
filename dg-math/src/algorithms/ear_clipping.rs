use crate::vector::Vec2;

fn is_regular(polygon: &[Vec2]) -> bool {
    todo!()
}

pub fn ear_clipping(polygon: &[Vec2]) -> Option<Vec<[usize; 3]>> {
    if !is_regular(polygon) {
        return None;
    }

    let indices = vec![];

    todo!();

    Some(indices)
}