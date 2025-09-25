use lalgebra_scalar::*;

#[derive(Debug)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Self(vec![vec![T::zero()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut rows = Vec::new();
        for _i in 0..row {
            let mut column = Vec::new();
            for _j in 0..col {
                column.push(T::zero());
            }
            rows.push(column);
        }
        Matrix(rows)
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut rows = Vec::new();
        for i in 0..n {
            let mut column = Vec::new();
            for j in 0..n {
                if i == j {
                    column.push(T::one());
                } else {
                    column.push(T::zero());
                }
            }

            rows.push(column);
        }
        Matrix(rows)
    }
}
