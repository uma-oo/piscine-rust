use lalgebra_scalar::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Copy + Clone> Matrix<T> {
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

impl<T: Copy + std::fmt::Debug> Mul for Matrix<T> {
    type Output = Option<Self>;

    fn mul(self, other: Self) -> Self::Output {
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }

        // kula row dyal self for l column dyal other

        for i in 0..self.number_of_rows() {
            for j in 
        }


    }
}
