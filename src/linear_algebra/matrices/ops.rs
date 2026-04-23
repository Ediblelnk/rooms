use super::Matrix;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index,
    IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Sub, SubAssign,
};

impl<T> Add for Matrix<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn add(self, other: Matrix<T>) -> Matrix<T> {
        self._assert_same_size(&other);

        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a + b)
            .collect();
        Matrix::new(self.rows, self.cols, data)
    }
}

impl<T> AddAssign for Matrix<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, other: Matrix<T>) {
        self._assert_same_size(&other);

        for i in 0..self.data.len() {
            self.data[i] += other.data[i];
        }
    }
}

impl<T> BitAnd for Matrix<T>
where
    T: BitAnd<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn bitand(self, other: Matrix<T>) -> Matrix<T> {
        self._assert_same_size(&other);

        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a & b)
            .collect();
        Matrix::new(self.rows, self.cols, data)
    }
}

impl<T> BitAndAssign for Matrix<T>
where
    T: BitAndAssign + Copy,
{
    fn bitand_assign(&mut self, other: Matrix<T>) {
        self._assert_same_size(&other);

        for i in 0..self.data.len() {
            self.data[i] &= other.data[i];
        }
    }
}

impl<T> BitOr for Matrix<T>
where
    T: BitOr<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn bitor(self, other: Matrix<T>) -> Matrix<T> {
        self._assert_same_size(&other);

        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a | b)
            .collect();
        Matrix::new(self.rows, self.cols, data)
    }
}

impl<T> BitOrAssign for Matrix<T>
where
    T: BitOrAssign + Copy,
{
    fn bitor_assign(&mut self, other: Matrix<T>) {
        self._assert_same_size(&other);

        for i in 0..self.data.len() {
            self.data[i] |= other.data[i];
        }
    }
}

impl<T> BitXor for Matrix<T>
where
    T: BitXor<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn bitxor(self, other: Matrix<T>) -> Matrix<T> {
        self._assert_same_size(&other);

        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a ^ b)
            .collect();
        Matrix::new(self.rows, self.cols, data)
    }
}

impl<T> BitXorAssign for Matrix<T>
where
    T: BitXorAssign + Copy,
{
    fn bitxor_assign(&mut self, other: Matrix<T>) {
        self._assert_same_size(&other);

        for i in 0..self.data.len() {
            self.data[i] ^= other.data[i];
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (r, c) = index;
        &self.data[r * self.cols + c]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (r, c) = index;
        &mut self.data[r * self.cols + c]
    }
}

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Mul<Output = T> + Add<Output = T> + Default + Copy + AddAssign,
{
    type Output = Matrix<T>;

    fn mul(self, other: Matrix<T>) -> Matrix<T> {
        assert_eq!(
            self.cols, other.rows,
            "Expected left columns to match right rows: {} != {}",
            self.cols, other.rows
        );

        let mut result_data = vec![T::default(); self.rows * other.cols];
        for r in 0..self.rows {
            for c in 0..other.cols {
                let mut sum = T::default();
                for k in 0..self.cols {
                    sum += self.data[r * self.cols + k] * other.data[k * other.cols + c];
                }
                result_data[r * other.cols + c] = sum;
            }
        }
        Matrix::new(self.rows, other.cols, result_data)
    }
}

impl<T> MulAssign<Matrix<T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default + Copy + AddAssign,
{
    fn mul_assign(&mut self, other: Matrix<T>) {
        let result = self.clone() * other;
        *self = result;
    }
}

impl<T> Mul<T> for Matrix<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn mul(self, scalar: T) -> Matrix<T> {
        let data = self.data.iter().map(|&a| a * scalar).collect();
        Matrix::new(self.rows, self.cols, data)
    }
}

impl<T> MulAssign<T> for Matrix<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, scalar: T) {
        for i in 0..self.data.len() {
            self.data[i] *= scalar;
        }
    }
}

impl<T> Neg for Matrix<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn neg(self) -> Matrix<T> {
        let data = self.data.iter().map(|&a| -a).collect();
        Matrix::new(self.rows, self.cols, data)
    }
}

impl<T> Not for Matrix<T>
where
    T: Not<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn not(self) -> Matrix<T> {
        let data = self.data.iter().map(|&a| !a).collect();
        Matrix::new(self.rows, self.cols, data)
    }
}

impl<T> Rem<T> for Matrix<T>
where
    T: Rem<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn rem(self, scalar: T) -> Matrix<T> {
        let data = self.data.iter().map(|&a| a % scalar).collect();
        Matrix::new(self.rows, self.cols, data)
    }
}

impl<T> RemAssign<T> for Matrix<T>
where
    T: RemAssign + Copy,
{
    fn rem_assign(&mut self, scalar: T) {
        for i in 0..self.data.len() {
            self.data[i] %= scalar;
        }
    }
}

impl<T> Sub for Matrix<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn sub(self, other: Matrix<T>) -> Matrix<T> {
        self._assert_same_size(&other);

        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a - b)
            .collect();
        Matrix::new(self.rows, self.cols, data)
    }
}

impl<T> SubAssign for Matrix<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, other: Matrix<T>) {
        for i in 0..self.data.len() {
            self.data[i] -= other.data[i];
        }
    }
}
