// use rooms::linear_algebra::Matrix;
use rooms::number_theory::primes::pi;

pub fn main() {
    // #[rustfmt::skip]
    // let data = vec![
    //     1, 8, 3, 9, 3, 1,
    //     (-1), 3, 6, 3, (-9), 3,
    //     8, (-9), 7, (-3), 2, 3,
    //     0, 3, (-2), 3, (-2), 1,
    //     1, 1, 3, (-6), (-3), 5,
    //     5, 2, (-4), (-1), (-8), 1];
    // let m7: Matrix<Q> = Matrix::new(6, 6, data.into_iter().map(|x| x.into()).collect());
    // println!("m7:\n{}", m7);
    // println!("m7 inverse:\n{}", m7.inverse());

    // println!("m7 * m7:\n{}", m7.clone() * m7.clone());

    // println!("m7 - m7:\n{}", m7.clone() - Matrix::identity(6));

    println!("{}", pi(100));
}
