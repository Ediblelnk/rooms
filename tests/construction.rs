use rooms::Matrix;

#[test]
fn new() {
    let m: Matrix<2, 3, _> = Matrix::new(vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(m.rows(), 2);
    assert_eq!(m.cols(), 3);
    assert_eq!(*m.as_vec(), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
#[should_panic]
fn new_incorrect_size() {
    let _m: Matrix<2, 2, _> = Matrix::new(vec![1, 2, 3]); // This should panic
}

#[test]
#[should_panic]
fn new_zero_size() {
    let _m: Matrix<0,0, isize> = Matrix::new(Vec::new());
}

#[test]
fn display() {
    let m: Matrix<2, 2, _> = Matrix::new(vec![1, 2, 3, 4]);
    let output = format!("{}", m);
    let expected = "1 2 \n3 4 \n";
    assert_eq!(output, expected);
}
