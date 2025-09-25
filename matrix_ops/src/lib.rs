// use matrix::Matrix;
use lalgebra_scalar::*;
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T:  Add<Output = T> + std::fmt::Debug + Copy> Add for Matrix<T> {
    type Output = Option<Self>;
    fn add(self, other: Self) -> Self::Output {
        // hna check rows + columns
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }
        let mut rows: Vec<Vec<T>> = Vec::new();
        for i in 0..self.0.len() {
            let mut columns: Vec<T> = Vec::new();
            for j in 0..self.0[0].len() {
                let result = self.0[i][j] + other.0[i][j];
                columns.push(result);
            }
            rows.push(columns);
        }

        Some(Matrix(rows))
    }
}

impl<T: Sub<Output = T> + std::fmt::Debug + Copy> Sub for Matrix<T> {
    type Output = Option<Self>;
    fn sub(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }
        let mut rows = Vec::new();
        for i in 0..self.0.len() {
            let mut columns = Vec::new();
            for j in 0..self.0[0].len() {
                let result: T = self.0[i][j] - other.0[i][j];
                columns.push(result);
            }
            rows.push(columns);
        }

        Some(Matrix(rows))
    }
}
