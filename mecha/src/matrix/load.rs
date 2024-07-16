use std::ops;

use graph::Ace;

// pub type Vector<T, const R: usize> = Matrix<T, R, 1>;

pub type Load<const R: usize, const C: usize> = Ace<Matrix<f64, R, C>>;

pub struct Matrix<T, const R: usize, const C: usize>(pub [[T; R]; C]);

impl<T, const R: usize, const C: usize> Default for Matrix<T, R, C>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self([[T::default(); R]; C])
    }
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Default + Copy,
{
    fn zip<F: Fn(T, T) -> T>(&self, rhs: &Self, op: F) -> Self {
        let mut matrix = Matrix::default();
        for i in 0..matrix.0.len() {
            for j in 0..matrix.0[i].len() {
                matrix.0[i][j] = op(self.0[i][j], rhs.0[i][j]);
            }
        }
        matrix
    }
}

impl<T, const R: usize, const C: usize> ops::Add<&Matrix<T, R, C>> for &Matrix<T, R, C>
where
    T: Default + Copy + ops::Add<T, Output = T>,
{
    type Output = Matrix<T, R, C>;
    fn add(self, rhs: &Matrix<T, R, C>) -> Matrix<T, R, C> {
        self.zip(rhs, |l, r| l + r)
    }
}

impl<T, const R: usize, const C: usize> ops::Sub<&Matrix<T, R, C>> for &Matrix<T, R, C>
where
    T: Default + Copy + ops::Sub<T, Output = T>,
{
    type Output = Matrix<T, R, C>;
    fn sub(self, rhs: &Matrix<T, R, C>) -> Matrix<T, R, C> {
        self.zip(rhs, |l, r| l - r)
    }
}

// use itertools::izip;

// impl<T, const R: usize, const C: usize> Matrix<T, R, C>
// where
//     T: Default + Copy
// {
//     fn zip<F: Fn(T, T) -> T>(&self, rhs: &Self, op: F) -> Self {
//         let mut matrix = Matrix::default();
//         for (lhs, rhs, col) in izip!(&self.0, &rhs.0, &mut matrix.0) {
//             for (lhs, rhs, unit) in izip!(lhs, rhs, col) {
//                 *unit = op(lhs.clone(), rhs.clone());
//             }
//         }
//         matrix
//     }
// }

// trait Zip {
//     type Num;
//     fn zip<F: Fn(Self::Num, Self::Num) -> Self::Num>(&self, rhs: &Self, op: F) -> Self;
// }
