pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<Vec<f64>>) -> Self {
        assert_eq!(rows, data.len());
        assert_eq!(cols, data[0].len());
        Matrix { rows, cols, data }
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let data = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(row_self, row_other)| {
                row_self.iter().zip(row_other).map(|(a, b)| a + b).collect()
            })
            .collect();
        Matrix::new(self.rows, self.cols, data)
    }

    pub fn subtract(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let data = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(row_self, row_other)| {
                row_self.iter().zip(row_other).map(|(a, b)| a - b).collect()
            })
            .collect();
        Matrix::new(self.rows, self.cols, data)
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);
        let mut data = vec![vec![0.0; other.cols]; self.rows];
        for (i, row) in data.iter_mut().enumerate().take(self.rows) {
            for (j, cell) in row.iter_mut().enumerate().take(other.cols) {
                for k in 0..self.cols {
                    *cell += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Matrix::new(self.rows, other.cols, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_addition() {
        let m1 = Matrix::new(2, 2, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let m2 = Matrix::new(2, 2, vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
        let result = m1.add(&m2);
        assert_eq!(result.data, vec![vec![6.0, 8.0], vec![10.0, 12.0]]);
    }

    #[test]
    fn test_matrix_subtraction() {
        let m1 = Matrix::new(2, 2, vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
        let m2 = Matrix::new(2, 2, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let result = m1.subtract(&m2);
        assert_eq!(result.data, vec![vec![4.0, 4.0], vec![4.0, 4.0]]);
    }

    #[test]
    fn test_matrix_multiplication() {
        let m1 = Matrix::new(2, 3, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
        let m2 = Matrix::new(
            3,
            2,
            vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]],
        );
        let result = m1.multiply(&m2);
        assert_eq!(result.data, vec![vec![58.0, 64.0], vec![139.0, 154.0]]);
    }
}
