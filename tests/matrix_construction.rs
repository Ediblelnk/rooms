use rooms::linear_algebra::Matrix;

#[test]
fn new() {
    let m: Matrix<_> = Matrix::new(2, 3,vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(*m.as_vec(), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
#[should_panic]
fn new_incorrect_size() {
    let _m: Matrix<_> = Matrix::new(2,2,vec![1, 2, 3]); // This should panic
}

#[test]
#[should_panic]
fn new_zero_size() {
    let _m: Matrix<isize> = Matrix::new(0,0,Vec::new());
}

#[test]
fn display() {
    let m: Matrix<_> = Matrix::new(2,2,vec![1, 2, 3, 4]);
    let output = format!("{}", m);
    let expected = "1 2 \n3 4 \n";
    assert_eq!(output, expected);
}
