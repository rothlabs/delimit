use std::ops::*;

use super::*;

#[derive(Default)]
struct End;

impl Next for End {
    type Next = ();
    fn next(&self) -> Option<&Self::Next> {
        None
    }
}

#[derive(Default)]
struct Vector<T, N> {
    here: T,
    next: N,
}

impl<T, N> Next for Vector<T, N> {
    type Next = N;
    fn next(&self) -> Option<&Self::Next> {
        Some(&self.next)
    }
}

impl<T, N> Vector<T, N> where 
    N: Next,
    <N as Next>::Next: Next,
{
    fn walk(&self) {
        let wow = self.next();
        let wow2 = wow.unwrap().next();
        let wow3 = wow2.unwrap().next();
    }
}

// impl<T, N> Zip for Vector<T, N> 
// where 
//     N: Zip<Item = T>
// {
//     type Item = T;
//     type Next = N;
//     fn zip<F: Fn(Self::Item, Self::Item, usize)>(&self, rhs: &Self::Next, index: usize, op: F) {
//         op(self.here, rhs.);
//     }
// }



#[derive(Clone)]
pub struct VectorX<T, const R: usize>(pub [T; R]);

impl<T, const R: usize> Default for VectorX<T, R>
where
    T: Copy + Default,
{
    fn default() -> Self {
        Self([T::default(); R])
    }
}

impl<T, const R: usize> VectorX<T, R>
where
    T: Copy + Default + Mul<T, Output = T> + Add<T, Output = T>,
{
    pub fn dot(&self, rhs: &Self) -> T {
        (self * rhs).0.iter().fold(T::default(), |a, u| a + *u)
    } 
}

impl<T, const R: usize> ZipX for VectorX<T, R>
where
    T: Copy + Default,
{
    type Item = T;
    //const ROWS: usize = R;
    fn zip<F: Fn(T, T) -> T>(&self, rhs: &Self, op: F) -> Self {
        let mut vector = VectorX::default();
        for r in 0..vector.0.len() {
            vector.0[r] = op(self.0[r], rhs.0[r]);
        }
        vector
    }
}

impl<T, const R: usize> Add<&VectorX<T, R>> for &VectorX<T, R>
where
    T: Copy + Default + Add<T, Output = T>,
{
    type Output = VectorX<T, R>;
    fn add(self, rhs: &VectorX<T, R>) -> VectorX<T, R> {
        self.zip(rhs, |l, r| l + r)
    }
}

impl<T, const R: usize> Sub<&VectorX<T, R>> for &VectorX<T, R>
where
    T: Copy + Default + Sub<T, Output = T>,
{
    type Output = VectorX<T, R>;
    fn sub(self, rhs: &VectorX<T, R>) -> VectorX<T, R> {
        self.zip(rhs, |l, r| l - r)
    }
}

impl<T, const R: usize> Mul<&VectorX<T, R>> for &VectorX<T, R>
where
    T: Copy + Default + Mul<T, Output = T>,
{
    type Output = VectorX<T, R>;
    fn mul(self, rhs: &VectorX<T, R>) -> VectorX<T, R> {
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