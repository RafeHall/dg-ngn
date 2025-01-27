use crate::{matrix::Matrix4x4, vector::Vec4};


fn row_reduction(mut matrix: Matrix4x4) -> Matrix4x4 {
    const ROWS: usize = 3;
    const COLUMNS: usize = 4;

    for row in 0..ROWS {
        for i in (row+1)..ROWS {
            // zero out pivot row index
            let denominator = matrix[row][row];
            let numerator = matrix[i][row];
            let fraction = numerator / denominator;

            for j in 0..COLUMNS {
                matrix[i][j] -= matrix[row][j] * fraction;
            }
        }
    }

    matrix
}

// fn back_substitution(mut matrix: Matrix4x4) -> Matrix4x4 {
//     const ROWS: usize = 3;
//     const COLUMNS: usize = 4;

//     // let mut solution = Vec4::ZERO;
//     for row in (0..ROWS).rev() {
//         println!("pivot row: {}", row);

//         for i in 0..row {
//             println!("operating row: {}", i);


//             let denominator = matrix[row][row];
//             let numerator = matrix[i][row];
//             let fraction = numerator / denominator;
    
//             // println!("{} / {} = {}", numerator, denominator, fraction);

//             for j in (row..COLUMNS).rev() {
//                 println!("{}, {}", i, j);
                
//                 matrix[i][j] -= matrix[row][j] * fraction;
//             }
            
//             // println!("{}", matrix);
//         }
//         matrix[row][COLUMNS-1] /= matrix[row][row];
//         matrix[row][row] = 1.0;
//     }

//     matrix
// }


fn back_substitution(mut matrix: Matrix4x4) -> Vec4 {
    const ROWS: usize = 3;
    const COLUMNS: usize = 4;

    let mut solution = Vec4::ZERO;
    for i in (0..ROWS).rev() {
        let mut sum = 0.0;
        for j in ((i+1)..ROWS).rev() {
            sum += solution[j] * matrix[i][j];

        }
        solution[i] = (matrix[i][COLUMNS - 1] - sum) / matrix[i][i];
    }

    solution
}


// pub fn gaussian_elimination_4x4(mut matrix: Matrix4x4) -> Matrix4x4 {
//     matrix = row_reduction(matrix);
//     matrix = back_substitution(matrix);

//     matrix
// }

#[cfg(test)]
mod tests {
    use crate::{algorithms::gaussian_elimination::{back_substitution, row_reduction}, matrix::Matrix4x4, vector::Vec4};


    #[test]
    fn basic_test() {
        let m = Matrix4x4::new_rows(
            Vec4::new(2.0, 1.0, -1.0, 8.0),
            Vec4::new(-3.0, -1.0, 2.0, -11.0),
            Vec4::new(-2.0, 1.0, 2.0, -3.0),
            Vec4::new(0.0, 0.0, 0.0, 0.0),
        );

        let m = row_reduction(m);
        println!("{}", m);

        let r = Matrix4x4::new_rows(
            Vec4::new(2.0, 1.0, -1.0, 8.0),
            Vec4::new(0.0, 0.5, 0.5, 1.0),
            Vec4::new(0.0, 0.0, -1.0, 1.0),
            Vec4::new(0.0, 0.0, 0.0, 0.0),
        );
        // assert_eq!(m, r);

        let m = back_substitution(m);
        println!("{}", m);

        // let s = Vec4::new(2.0, 3.0, -1.0, 0.0);

        // assert_eq!(m, s);

        // let m_g = Matrix4x4::new_rows(
        //     Vec4::new(2.0, 1.0, -1.0, 8.0),
        //     Vec4::new(-3.0, -1.0, 2.0, -11.0),
        //     Vec4::new(-2.0, 1.0, 2.0, -3.0),
        //     Vec4::new(0.0, 0.0, 0.0, 0.0),
        // );

        // let m_g = gaussian_elimination_4x4(m_g);

        // assert_eq!(m, m_g);

        // assert_eq!(result[0][0] as i32, 1);
        // assert_eq!(result[1][1] as i32, 1);
        // assert_eq!(result[2][2] as i32, 1);

        // assert_eq!(result[3][0] as i32, 2);
        // assert_eq!(result[3][1] as i32, 3);
        // assert_eq!(result[3][2] as i32, -1);
    }
}