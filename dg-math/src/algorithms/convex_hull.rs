use crate::vector::{Vec2, Vec3};


// pub fn convex_hull_2d(points: &[Vec2]) -> Vec<usize> {
//     if points.len() < 3 {
//         return vec![];
//     }

//     let hull = vec![];

    

//     hull
// }

pub fn convex_hull_2d(points: &[Vec2]) -> Vec<usize> {
    use crate::{scalar, Scalar};

    if points.len() < 3 {
        return vec![];
    }

    todo!();

    let left_most_index = points.iter().enumerate().min_by(|(_, a), (_, b)| a.x.total_cmp(&b.x)).unwrap().0;
    let mut start = points[left_most_index];

    println!("{}", left_most_index);

    let mut hull = vec![left_most_index];
    loop {
        let mut best_fit: usize = 0;
        let mut best_angle: Scalar = scalar::INFINITY;

        for i in 0..points.len() {
            let end = points[i];
            let angle = start.angle_to(end);

            println!("{}", angle);

            if i == *hull.last().unwrap() {
                continue;
            }

            if angle < best_angle {
                best_fit = i;
                best_angle = angle;
            }
        }

        if best_fit == left_most_index {
            break;
        }

        hull.push(best_fit);
        start = points[best_fit];

        break;
    }

    hull
}

pub fn convex_hull_3d(points: &[Vec3]) -> Vec<usize> {
    if points.len() < 4 {
        return vec![];
    }

    let hull = vec![];

    

    hull
}

#[cfg(test)]
mod tests {
    use crate::vector::Vec2;

    #[test]
    fn convex_hull_2d() {
        let result = super::convex_hull_2d(&[
            Vec2::new(0.825, 2.18),
            Vec2::new(0.39, 0.91),
            Vec2::new(1.42, -1.3),
            Vec2::new(-0.62, -1.03),
            Vec2::new(2.24, 1.406),
            Vec2::new(-0.095, 3.336),
            Vec2::new(-0.876, 1.91),
            Vec2::new(0.29, -0.12),
            Vec2::new(4.88, 2.045),
        ]);

        println!("{:#?}", result);
    }
}