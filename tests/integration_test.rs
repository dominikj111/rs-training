// This file is kept as a placeholder for any future integration tests, it expects to have src/lib.rs

use rs_training::common::matrix::Matrix;

#[test]
fn matrix_module_is_exposed() {
    let matrix = Matrix::new(2, 3, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    assert_eq!(matrix.rows, 2);
    assert_eq!(matrix.cols, 3);
    assert_eq!(matrix.data, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
}
