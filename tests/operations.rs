use rooms::Matrix;

#[test]
fn add() {
    let mut m1: Matrix<2, 2, _> = Matrix::new(vec![1, 2, 3, 4]);
    let m2: Matrix<2, 2, _> = Matrix::new(vec![5, 6, 7, 8]);
    let result = m1.clone() + m2.clone();
    let expected: Matrix<2, 2, _> = Matrix::new(vec![6, 8, 10, 12]);
    assert_eq!(result, expected);
    m1 += m2;
    assert_eq!(m1, expected);
}

#[test]
fn sub() {
    let mut m1: Matrix<2, 2, _> = Matrix::new(vec![1, 2, 3, 4]);
    let m2: Matrix<2, 2, _> = Matrix::new(vec![5, 6, 7, 8]);
    let result = m1.clone() - m2.clone();
    let expected: Matrix<2, 2, _> = Matrix::new(vec![-4, -4, -4, -4]);
    assert_eq!(result, expected);
    m1 -= m2;
    assert_eq!(m1, expected);
}