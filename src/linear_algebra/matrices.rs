use crate::properties::Identity;
use std::{
    fmt::Display,
    ops::{AddAssign, Div, Mul, Sub, SubAssign},
};

mod ops;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert_eq!(data.len(), rows * cols);
        assert_ne!(rows == 0 || cols == 0, true);
        Matrix { rows, cols, data }
    }

    pub fn defaults(rows: usize, cols: usize) -> Self
    where
        T: Default + Clone,
    {
        Matrix {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }

    pub fn identity(n: usize) -> Self
    where
        T: Default + Identity + Clone,
    {
        let mut data = vec![T::default(); n * n];
        for i in 0..n {
            data[i * n + i] = T::identity();
        }
        Matrix {
            rows: n,
            cols: n,
            data,
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    pub fn as_vec(&self) -> &Vec<T> {
        &self.data
    }

    fn _assert_same_size(&self, other: &Matrix<T>) {
        assert_eq!(
            self.rows, other.rows,
            "Expected same number of rows: {} != {}",
            self.rows, other.rows
        );
        assert_eq!(
            self.cols, other.cols,
            "Expected same number of columns: {} != {}",
            self.cols, other.cols
        );
    }

    fn _assert_in_range(&self, row: usize, col: usize) {
        assert!(
            row < self.rows,
            "Row index out of bounds: {} >= {}",
            row,
            self.rows
        );
        assert!(
            col < self.cols,
            "Column index out of bounds: {} >= {}",
            col,
            self.cols
        );
    }

    fn _assert_square(&self) {
        assert!(
            self.is_square(),
            "Expected square matrix: {} != {}",
            self.rows,
            self.cols
        );
    }
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn row(&self, r: usize) -> Matrix<T> {
        assert!(
            r < self.rows,
            "Row index out of bounds: {} >= {}",
            r,
            self.rows
        );

        Matrix::new(
            1,
            self.cols,
            (0..self.cols)
                .map(|c| self.data[r * self.cols + c].clone())
                .collect(),
        )
    }

    pub fn row_mut(&mut self, r: usize) -> &mut [T] {
        assert!(
            r < self.rows,
            "Row index out of bounds: {} >= {}",
            r,
            self.rows
        );

        let start = r * self.cols;
        &mut self.data[start..start + self.cols]
    }

    pub fn row_slice(&self, r: usize) -> &[T] {
        let start = r * self.cols;
        &self.data[start..start + self.cols]
    }

    pub fn rows(&self, rows: Vec<usize>) -> Matrix<T> {
        assert!(
            rows.iter().all(|r| *r < self.rows),
            "Row index out of bounds: {:?} >= {}",
            rows,
            self.rows
        );

        Matrix::new(
            rows.len(),
            self.cols,
            rows.into_iter()
                .flat_map(|r| {
                    self.data[r * self.cols..(r + 1) * self.cols]
                        .iter()
                        .cloned()
                })
                .collect(),
        )
    }

    pub fn col(&self, c: usize) -> Matrix<T> {
        assert!(
            c < self.cols,
            "Column index out of bounds: {} >= {}",
            c,
            self.cols
        );

        Matrix::new(
            self.rows,
            1,
            (0..self.rows)
                .map(|r| self.data[r * self.cols + c].clone())
                .collect(),
        )
    }

    pub fn cols(&self, cols: Vec<usize>) -> Matrix<T> {
        assert!(
            cols.iter().all(|c| *c < self.cols),
            "Column index out of bounds: {:?} >= {}",
            cols,
            self.cols
        );
        let mut data = vec![];
        for r in 0..self.rows {
            for c in 0..self.cols {
                if !cols.contains(&c) {
                    continue;
                }
                data.push(self.data[r * self.cols + c].clone());
            }
        }
        Matrix::new(self.rows, cols.len(), data)
    }
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn transpose(&self) -> Matrix<T> {
        let mut transposed_data = vec![];
        for c in 0..self.cols {
            for r in 0..self.rows {
                transposed_data.push(self.data[r * self.cols + c].clone());
            }
        }
        Matrix::new(self.cols, self.rows, transposed_data)
    }

    /**
     * Returns a submatrix of the matrix, excluding the specified row and column.
     */
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<T> {
        self._assert_in_range(row, col);

        let mut data = vec![];
        for r in 0..self.rows {
            for c in 0..self.cols {
                if r == row || c == col {
                    continue;
                }
                data.push(self.data[r * self.cols + c].clone());
            }
        }
        Matrix::new(self.rows - 1, self.cols - 1, data)
    }

    /**
     * Removes the specified rows and columns from the matrix.
     */
    pub fn general_submatrix(&self, rows: Vec<usize>, cols: Vec<usize>) -> Matrix<T> {
        let mut data = vec![];
        for r in 0..self.rows {
            for c in 0..self.cols {
                if rows.contains(&r) || cols.contains(&c) {
                    continue;
                }
                data.push(self.data[r * self.cols + c].clone());
            }
        }
        Matrix::new(self.rows - rows.len(), self.cols - cols.len(), data)
    }
}

impl<T> Matrix<T>
where
    T: Div<Output = T>
        + Clone
        + Copy
        + Default
        + Mul<Output = T>
        + Identity
        + SubAssign
        + AddAssign
        + Sub<Output = T>,
{
    pub fn inverse(&self) -> Matrix<T> {
        self._assert_square();
        let n = self.rows;
        let (l, u) = self.lu_decomposition();
        let mut a_inv = Matrix::defaults(n, n);
        for col in 0..n {
            let y = l._forward_substitution(col);
            let x = u._backward_substitution(&y);
            for i in 0..n {
                a_inv[(i, col)] = x[i];
            }
        }

        a_inv
    }

    pub fn lu_decomposition(&self) -> (Matrix<T>, Matrix<T>) {
        self._assert_square();

        let mut l = Matrix::identity(self.rows);
        let mut u = self.clone();

        for k in 0..(self.rows - 1) {
            for i in k + 1..self.rows {
                let factor = u[(i, k)] / u[(k, k)];
                l[(i, k)] = factor.clone();
                for j in k..self.cols {
                    let temp = factor.clone() * u[(k, j)];
                    u[(i, j)] -= temp;
                }
            }
        }

        (l, u)
    }

    fn _forward_substitution(&self, col: usize) -> Vec<T> {
        self._assert_square();
        let n = self.rows;

        let mut y = vec![T::default(); n];
        y[col] = T::identity();

        for i in col + 1..n {
            let mut sum = T::default();
            for j in col..i {
                sum += self[(i, j)] * y[j];
            }
            y[i] = T::default() - sum;
        }

        y
    }

    fn _backward_substitution(&self, y: &[T]) -> Vec<T> {
        self._assert_square();
        let n = self.rows;

        let mut x = vec![T::default(); n];

        for i in (0..n).rev() {
            let mut sum = T::default();
            for j in i + 1..n {
                sum += self[(i, j)] * x[j];
            }
            x[i] = (y[i] - sum) / self[(i, i)];
        }

        x
    }
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.cols {
                write!(f, "{:>3} ", self.data[r * self.cols + c])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
