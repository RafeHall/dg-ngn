use crate::{matrix::MatrixRxC, vector::VecN};

pub fn gauss_seidel<const ITERATIONS: usize, const N: usize>(
    matrix: MatrixRxC<N, N>,
    v: VecN<N>,
    i: VecN<N>,
) -> VecN<N> {
    let mut x = i;
    for _ in 0..ITERATIONS {
        let mut x_new = VecN::ZERO;
        for i in 0..N {
            let mut s1 = 0.0;
            for j in 0..i {
                s1 += matrix[i][j] * x_new[j];
            }
            let mut s2 = 0.0;
            for j in (i + 1)..N {
                s2 += matrix[i][j] * x[j];
            }
            x_new[i] = (v[i] - s1 - s2) / matrix[i][i];
        }
        x = x_new;
    }
    x
}

#[cfg(test)]
mod tests {
    use crate::{matrix::MatrixRxC, vector::VecN};

    use super::gauss_seidel;

    #[test]
    fn test() {
        let m = MatrixRxC::<4, 4>::new([
            [10.0, -1.0, 2.0, 0.0],
            [-1.0, 11.0, -1.0, 3.0],
            [2.0, -1.0, 10.0, -1.0],
            [0.0, 3.0, -1.0, 8.0],
        ]);

        println!("{}", m);

        let r = gauss_seidel::<1024, 4>(
            m,
            VecN::<4>::new([6.0, 25.0, -11.0, 15.0]),
            VecN::<4>::ZERO,
        );

        println!("{}", r);
    }
}
