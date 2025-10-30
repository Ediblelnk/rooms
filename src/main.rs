use rooms::Matrix;

pub fn main() {
    let m1: Matrix<2, 2, _> = Matrix::new(vec![1, 2, 3, 4]);
    let m2: Matrix<2, 2, _> = Matrix::new(vec![5, 6, 7, 8]);
    let m3 = m1 | m2;
    println!("Result of BitOr:\n{}", m3);
    println!("Element at (1,1): {}", m3[(1,1)]);

    let m4: Matrix<2, 3, _> = Matrix::new(vec![1, 2, 3, 4, 5, 6]);
    let m5: Matrix<3, 4, _> = Matrix::new(vec![7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]);
    let m6 = m4 * m5;
    println!("Result of Multiplication:\n{}", m6);

    println!("{}", !0);
}