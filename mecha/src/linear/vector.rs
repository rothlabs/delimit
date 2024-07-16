use std::ops::*;

use super::*;

#[derive(Clone)]
pub struct Vector<T, const R: usize>(pub [T; R]);

impl<T, const R: usize> Default for Vector<T, R>
where
    T: Copy + Default,
{
    fn default() -> Self {
        Self([T::default(); R])
    }
}

impl<T, const R: usize> Vector<T, R>
where
    T: Copy + Default + Mul<T, Output = T> + Add<T, Output = T>,
{
    pub fn dot(&self, rhs: &Self) -> T {
        (self * rhs).0.iter().fold(T::default(), |a, u| a + *u)
    } 
}

impl<T, const R: usize> Zip for Vector<T, R>
where
    T: Copy + Default,
{
    type Item = T;
    const ROWS: usize = R;
    fn zip<F: Fn(T, T) -> T>(&self, rhs: &Self, op: F) -> Self {
        let mut vector = Vector::default();
        for r in 0..vector.0.len() {
            vector.0[r] = op(self.0[r], rhs.0[r]);
        }
        vector
    }
}

impl<T, const R: usize> Add<&Vector<T, R>> for &Vector<T, R>
where
    T: Copy + Default + Add<T, Output = T>,
{
    type Output = Vector<T, R>;
    fn add(self, rhs: &Vector<T, R>) -> Vector<T, R> {
        self.zip(rhs, |l, r| l + r)
    }
}

impl<T, const R: usize> Sub<&Vector<T, R>> for &Vector<T, R>
where
    T: Copy + Default + Sub<T, Output = T>,
{
    type Output = Vector<T, R>;
    fn sub(self, rhs: &Vector<T, R>) -> Vector<T, R> {
        self.zip(rhs, |l, r| l - r)
    }
}

impl<T, const R: usize> Mul<&Vector<T, R>> for &Vector<T, R>
where
    T: Copy + Default + Mul<T, Output = T>,
{
    type Output = Vector<T, R>;
    fn mul(self, rhs: &Vector<T, R>) -> Vector<T, R> {
        self.zip(rhs, |l, r| l * r)
    }
}


// use itertools::izip;

// impl<T, const R: usize> Vector<T, R>
// where
//     T: Default + Copy
// {
//     fn zip<F: Fn(T, T) -> T>(&self, rhs: &Self, op: F) -> Self {
//         let mut vector = Vector::default();
//         for (lhs, rhs, col) in izip!(&self.0, &rhs.0, &mut vector.0) {
//             for (lhs, rhs, unit) in izip!(lhs, rhs, col) {
//                 *unit = op(lhs.clone(), rhs.clone());
//             }
//         }
//         vector
//     }
// }

// impl<T, const R: usize, Idx> Index<Idx> for Vector<T, R> 
// where
//     Idx: SliceIndex<[T], Output = T>,
// {
//     type Output = T;
//     fn index(&self, index: Idx) -> &Self::Output {
//         self.0.index(index)
//     }
// }