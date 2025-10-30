use std::{
    fmt::Display,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index,
        IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Sub, SubAssign,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<const R: usize, const C: usize, T> {
    data: Vec<T>,
}

impl<const R: usize, const C: usize, T> Matrix<R, C, T> {
    pub fn new(data: Vec<T>) -> Self {
        assert_eq!(data.len(), R * C);
        assert_ne!(R == 0 || C == 0, true);
        Matrix { data }
    }

    pub fn rows(&self) -> usize {
        R
    }

    pub fn cols(&self) -> usize {
        C
    }

    pub fn as_vec(&self) -> &Vec<T> {
        &self.data
    }
}

impl<const R: usize, const C: usize, T> Display for Matrix<R, C, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..R {
            for c in 0..C {
                write!(f, "{} ", self.data[r * C + c])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const R: usize, const C: usize, T> Add for Matrix<R, C, T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Matrix<R, C, T>;

    fn add(self, other: Matrix<R, C, T>) -> Matrix<R, C, T> {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a + b)
            .collect();
        Matrix::new(data)
    }
}

impl<const R: usize, const C: usize, T> AddAssign for Matrix<R, C, T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, other: Matrix<R, C, T>) {
        for i in 0..self.data.len() {
            self.data[i] += other.data[i];
        }
    }
}

impl<const R: usize, const C: usize, T> BitAnd for Matrix<R, C, T>
where
    T: BitAnd<Output = T> + Copy,
{
    type Output = Matrix<R, C, T>;

    fn bitand(self, other: Matrix<R, C, T>) -> Matrix<R, C, T> {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a & b)
            .collect();
        Matrix::new(data)
    }
}

impl<const R: usize, const C: usize, T> BitAndAssign for Matrix<R, C, T>
where
    T: BitAndAssign + Copy,
{
    fn bitand_assign(&mut self, other: Matrix<R, C, T>) {
        for i in 0..self.data.len() {
            self.data[i] &= other.data[i];
        }
    }
}

impl<const R: usize, const C: usize, T> BitOr for Matrix<R, C, T>
where
    T: BitOr<Output = T> + Copy,
{
    type Output = Matrix<R, C, T>;

    fn bitor(self, other: Matrix<R, C, T>) -> Matrix<R, C, T> {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a | b)
            .collect();
        Matrix::new(data)
    }
}

impl<const R: usize, const C: usize, T> BitOrAssign for Matrix<R, C, T>
where
    T: BitOrAssign + Copy,
{
    fn bitor_assign(&mut self, other: Matrix<R, C, T>) {
        for i in 0..self.data.len() {
            self.data[i] |= other.data[i];
        }
    }
}

impl<const R: usize, const C: usize, T> BitXor for Matrix<R, C, T>
where
    T: BitXor<Output = T> + Copy,
{
    type Output = Matrix<R, C, T>;

    fn bitxor(self, other: Matrix<R, C, T>) -> Matrix<R, C, T> {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a ^ b)
            .collect();
        Matrix::new(data)
    }
}

impl<const R: usize, const C: usize, T> BitXorAssign for Matrix<R, C, T>
where
    T: BitXorAssign + Copy,
{
    fn bitxor_assign(&mut self, other: Matrix<R, C, T>) {
        for i in 0..self.data.len() {
            self.data[i] ^= other.data[i];
        }
    }
}

impl<const R: usize, const C: usize, T> Index<(usize, usize)> for Matrix<R, C, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (r, c) = index;
        &self.data[r * C + c]
    }
}

impl<const R: usize, const C: usize, T> IndexMut<(usize, usize)> for Matrix<R, C, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (r, c) = index;
        &mut self.data[r * C + c]
    }
}

impl<const R: usize, const C: usize, const D: usize, T> Mul<Matrix<C, D, T>> for Matrix<R, C, T>
where
    T: Mul<Output = T> + Add<Output = T> + Default + Copy + AddAssign,
{
    type Output = Matrix<R, D, T>;

    fn mul(self, other: Matrix<C, D, T>) -> Matrix<R, D, T> {
        let mut result_data = vec![T::default(); R * D];
        for r in 0..R {
            for c in 0..D {
                let mut sum = T::default();
                for k in 0..C {
                    sum += self.data[r * C + k] * other.data[k * D + c];
                }
                result_data[r * D + c] = sum;
            }
        }
        Matrix::new(result_data)
    }
}

impl<const R: usize, const C: usize, T> MulAssign<Matrix<C, C, T>> for Matrix<R, C, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default + Copy + AddAssign,
{
    fn mul_assign(&mut self, other: Matrix<C, C, T>) {
        let result = self.clone() * other;
        *self = result;
    }
}

impl<const R: usize, const C: usize, T> Mul<T> for Matrix<R, C, T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Matrix<R, C, T>;

    fn mul(self, scalar: T) -> Matrix<R, C, T> {
        let data = self.data.iter().map(|&a| a * scalar).collect();
        Matrix::new(data)
    }
}

impl<const R: usize, const C: usize, T> MulAssign<T> for Matrix<R, C, T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, scalar: T) {
        for i in 0..self.data.len() {
            self.data[i] *= scalar;
        }
    }
}

impl<const R: usize, const C: usize, T> Neg for Matrix<R, C, T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Matrix<R, C, T>;

    fn neg(self) -> Matrix<R, C, T> {
        let data = self.data.iter().map(|&a| -a).collect();
        Matrix::new(data)
    }
}

impl<const R: usize, const C: usize, T> Not for Matrix<R, C, T>
where
    T: Not<Output = T> + Copy,
{
    type Output = Matrix<R, C, T>;

    fn not(self) -> Matrix<R, C, T> {
        let data = self.data.iter().map(|&a| !a).collect();
        Matrix::new(data)
    }
}

impl<const R: usize, const C: usize, T> Rem<T> for Matrix<R, C, T>
where
    T: Rem<Output = T> + Copy,
{
    type Output = Matrix<R, C, T>;

    fn rem(self, scalar: T) -> Matrix<R, C, T> {
        let data = self.data.iter().map(|&a| a % scalar).collect();
        Matrix::new(data)
    }
}

impl<const R: usize, const C: usize, T> RemAssign<T> for Matrix<R, C, T>
where
    T: RemAssign + Copy,
{
    fn rem_assign(&mut self, scalar: T) {
        for i in 0..self.data.len() {
            self.data[i] %= scalar;
        }
    }
}

impl<const R: usize, const C: usize, T> Sub for Matrix<R, C, T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Matrix<R, C, T>;

    fn sub(self, other: Matrix<R, C, T>) -> Matrix<R, C, T> {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a - b)
            .collect();
        Matrix::new(data)
    }
}

impl<const R: usize, const C: usize, T> SubAssign for Matrix<R, C, T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, other: Matrix<R, C, T>) {
        for i in 0..self.data.len() {
            self.data[i] -= other.data[i];
        }
    }
}
