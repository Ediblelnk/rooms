use rooms::linear_algebra::Matrix;
use rooms::number_theory::Rational as Q;
// use rooms::logic::*;

pub fn main() {
    let m1: Matrix<_> = Matrix::new(2, 2, vec![1, 2, 3, 4]);
    let m2: Matrix<_> = Matrix::new(2, 2, vec![5, 6, 7, 8]);
    let m3 = m1 | m2;
    println!("Result of BitOr:\n{}", m3);
    println!("Element at (1,1): {}", m3[(1,1)]);

    let m4: Matrix<_> = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
    let m5: Matrix<_> = Matrix::new(3, 4, vec![7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]);
    let m6 = m4 * m5.clone();
    println!("Result of Multiplication:\n{}", m6);

    println!("Normal\n{}", m5.clone());
    println!("Transpose\n{}", m5.clone().transpose());
    println!("Submatrix (removing row 0, col 1):\n{}", m5.submatrix(0,1));
    println!("General Submatrix (removing rows 0,1, cols 1,2):\n{}", m5.general_submatrix(vec![0,1], vec![1,2]));

    println!("Rows (1,2):\n{}", m5.rows(vec![1, 2]));
    println!("Cols (1,2):\n{}", m5.cols(vec![1, 2]));

    println!("Identity:\n{}", Matrix::<i32>::identity(3));

    let m7: Matrix<Q> = Matrix::new(6, 6, vec![
        1.into(), 8.into(), 3.into(), 9.into(), 3.into(), 1.into(),
        (-1).into(), 3.into(), 6.into(), 3.into(), (-9).into(), 3.into(),
        8.into(), (-9).into(), 7.into(), (-3).into(), 2.into(), 3.into(),
        0.into(), 3.into(), (-2).into(), 3.into(), (-2).into(), 1.into(),
        1.into(), 1.into(), 3.into(), (-6).into(), (-3).into(), 5.into(),
        5.into(), 2.into(), (-4).into(), (-1).into(), (-8).into(), 1.into()
    ]);
    println!("m7:\n{}", m7);
    println!("m7 inverse:\n{}", m7.inverse());
}
