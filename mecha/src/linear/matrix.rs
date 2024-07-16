use std::ops::*;

use super::*;

pub struct Matrix<V, const C: usize>(pub [V; C]);

impl<V, const C: usize> Default for Matrix<V, C>
where
    V: Copy + Default,
{
    fn default() -> Self {
        Self([V::default(); C])
    } 
}

impl<V, const C: usize> Matrix<V, C>
where
    V: Copy + Default,
{
    fn zip<F: Fn(V, V) -> V>(&self, rhs: &Self, op: F) -> Self {
        let mut matrix = Matrix::default();
        for c in 0..matrix.0.len() {
            matrix.0[c] = op(self.0[c], rhs.0[c]);
        }
        matrix
    }
}

// impl<V, const C: usize> Matrix<V, C>
// where
//     V: Copy + Default + Zip,
// {
//     fn transpose(&self) -> Matrix::<Vector<V::Item, C>, { <Vector<V::Item, V::ROWS> as Zip>::ROWS }> {
//         let mut matrix = Matrix::default();
//         // for c in 0..matrix.0.len() {
//         //     matrix.0[c] = op(self.0[c], rhs.0[c]);
//         // }
//         matrix
//     }
// }

impl<V, const C: usize> Add<&Matrix<V, C>> for &Matrix<V, C>
where
    V: Copy + Default + Add<V, Output = V>,
{
    type Output = Matrix<V, C>;
    fn add(self, rhs: &Matrix<V, C>) -> Matrix<V, C> {
        self.zip(rhs, |l, r| l + r)
    }
}

impl<V, const C: usize> Sub<&Matrix<V, C>> for &Matrix<V, C>
where
    V: Copy + Default + Sub<V, Output = V>,
{
    type Output = Matrix<V, C>;
    fn sub(self, rhs: &Matrix<V, C>) -> Matrix<V, C> {
        self.zip(rhs, |l, r| l - r)
    }
}

// impl<V, const C: usize> Mul<&Matrix<V, C>> for &Matrix<V, C>
// where
//     V: Copy + Default + Dot,
// {
//     type Output = Matrix<V, C>;
//     fn mul(self, rhs: &Matrix<V, C>) -> Matrix<V, C> {
//         self.zip(rhs, |l, r| l - r)
//     }
// }

// pub type Matrix<V, const C: usize> = Vector<V, C>;







// impl<V, const C: usize> Zip for Matrix<V, C>
// where
//     V: Copy + Default + Zip,
// {
//     type Num = V::Num;
//     fn zip<F: Fn(V::Num, V::Num) -> V::Num>(&self, rhs: &Self, op: F) -> Self {
//         let mut matrix = Matrix::default();
//         for c in 0..matrix.0.len() {
//             matrix.0[c] = self.0[c].zip(&rhs.0[c], &op);
//         }
//         matrix
//     }
// }

// impl<V, const C: usize> Add<&Matrix<V, C>> for &Matrix<V, C>
// where
//     V: Copy + Default + Zip,
//     V::Num: Add<V::Num, Output = V::Num>,
// {
//     type Output = Matrix<V, C>;
//     fn add(self, rhs: &Matrix<V, C>) -> Matrix<V, C> {
//         self.zip(rhs, |l, r| l + r)
//     }
// }





// use itertools::izip;

// impl<V, const C: usize> Matrix<V, C>
// where
//     V: Default + Copy
// {
//     fn zip<F: Fn(V, V) -> V>(&self, rhs: &Self, op: F) -> Self {
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
