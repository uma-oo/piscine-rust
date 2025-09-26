use lalgebra_scalar::*;
use std::ops::Mul;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Copy + Clone + Mul<Output=T> > Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut column: Vec<T> = Vec::new();
        for i in 0..self.number_of_rows() {
            column.push(self.0[i][n]);
        }
        column
    }
}

impl<T: Clone + Copy + std::fmt::Debug + Mul<Output=T> + lalgebra_scalar::Scalar<Item=T> + std::ops::AddAssign > Mul for Matrix<T> {
    type Output = Option<Self>;

    fn mul(self, other: Self) -> Self::Output {
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }

        // kula row dyal self for l column dyal other
        let mut rows: Vec<Vec<T>> = Vec::new();
        for i in 0..self.number_of_rows() {
            let mut vect_rows = Vec::new();
            for j in 0..other.number_of_cols() {
                let mut result = T::zero();
                let row = self.row(i);
                let column = other.col(i);
                for k in 0..row.len() {
                    result += row[k]*column[k];
                }
                vect_rows.push(result);
            }
            rows.push(vect_rows);
        }

        Some(Matrix(rows))
    }
}

// for i in 0..self.number_of_rows() {
//     let vect_rows = Vec::new();
//     let result = T::zero();
//     let row = self.row(i);
//     let column = other.column(i);
//     for j in 0..row.len() {
//         result += row[j] * column[j];
//     }
//     vect_rows.push(result);
// }
