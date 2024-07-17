use std::ops::*;

#[derive(Clone)]
pub struct Matrix<T>{
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> 
where 
    T: Copy + Default
{
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {rows, cols, data: vec![T::default(); rows * cols]}
    }
    pub fn rows_data(rows: usize, data: Vec<T>) -> Self {
        Self {
            rows,
            cols: data.len() / rows,
            data,
        }
    }
    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.data[self.rows * col + row]
    }
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.data[self.rows * col + row]
    }
    pub fn transpose(&self) -> Self {
        let mut out = Self::new(self.cols, self.rows);
        for r in 0..self.rows {
            for c in 0..self.cols {
                *out.get_mut(c, r) = self.get(r, c).clone();
            }
        }
        out
    }
    pub fn vec(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T> Mul<&Matrix<T>> for &Matrix<T>
where
    T: Copy + Default + AddAssign<T> + Mul<T, Output = T>,
{
    type Output = Matrix<T>;
    fn mul(self, rhs: &Matrix<T>) -> Matrix<T> {
        let mut out = Matrix::new(self.rows, rhs.cols);
        for lr in 0..self.rows {
            for lc in 0..self.cols {
                for rc in 0..rhs.cols {
                    *out.get_mut(lr, rc) += *self.get(lr, lc) * *rhs.get(lc, rc);
                }
            }
        }
        out
    }
}

// impl<T> From<(usize, Vec<T>)> for Matrix<T> 
// where 
//     T: Copy + Default
// {
//     fn from(data: (usize, Vec<T>)) -> Self {
//         Self {
//             rows: data.0,
//             cols: data.1.len() / data.0,
//             data: data.1
//         }
//     }
// }

// impl<T> Mul<&Matrix<T>> for &Matrix<T>
// where
//     T: Copy + Default + Mul<T, Output = T> + Add<T, Output = T>,
// {
//     type Output = Matrix<T>;
//     fn mul(self, rhs: &Matrix<T>) -> Matrix<T> {
//         //let mut matrix = Matrix::new(self[0].len(), rhs.len());
//         let mut matrix = Matrix::new(0, 0);
//         let transpose = self.transpose();
//         for rv in &rhs.0 {
//             let vector = Vector::from(transpose.0.iter().map(|lv| lv.dot(rv)).collect::<Vec<T>>());
//             matrix.0.push(vector);
//         // for c in 0..transpose.len() {
//         //     matrix[c] = &transpose[c] * &rhs[c];
//         // }
//         }
//         matrix
//     }
// }

// impl<T, Idx> Index<Idx> for Matrix<T> 
// where
//     Idx: SliceIndex<[T], Output = T>,
// {
//     type Output = [T];
//     fn index(&self, index: Idx) -> &Self::Output {
//         //self.data.index(index)
//         &self.data[index..2]
//     }
// }

// impl<T, Idx> IndexMut<Idx> for Matrix<T> 
// where
//     Idx: SliceIndex<[T], Output = T>,
// {
//     fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
//         self.data.index_mut(index)
//     }
// }



// impl<V, const C: usize> Matrix<V, C>
// where
//     V: Copy + Default,
// {
//     fn zip<F: Fn(V, V) -> V>(&self, rhs: &Self, op: F) -> Self {
//         let mut matrix = Matrix::default();
//         for c in 0..matrix.0.len() {
//             matrix.0[c] = op(self.0[c], rhs.0[c]);
//         }
//         matrix
//     }
// }

// // impl<V, const C: usize> Matrix<V, C>
// // where
// //     V: Copy + Default + Zip,
// // {
// //     fn transpose(&self) -> Matrix::<Vector<V::Item, C>, { <Vector<V::Item, V::ROWS> as Zip>::ROWS }> {
// //         let mut matrix = Matrix::default();
// //         // for c in 0..matrix.0.len() {
// //         //     matrix.0[c] = op(self.0[c], rhs.0[c]);
// //         // }
// //         matrix
// //     }
// // }

// impl<V, const C: usize> Add<&Matrix<V, C>> for &Matrix<V, C>
// where
//     V: Copy + Default + Add<V, Output = V>,
// {
//     type Output = Matrix<V, C>;
//     fn add(self, rhs: &Matrix<V, C>) -> Matrix<V, C> {
//         self.zip(rhs, |l, r| l + r)
//     }
// }

// impl<V, const C: usize> Sub<&Matrix<V, C>> for &Matrix<V, C>
// where
//     V: Copy + Default + Sub<V, Output = V>,
// {
//     type Output = Matrix<V, C>;
//     fn sub(self, rhs: &Matrix<V, C>) -> Matrix<V, C> {
//         self.zip(rhs, |l, r| l - r)
//     }
// }

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



// impl<T> From<(usize, Vec<T>)> for Matrix<T> 
// where 
//     T: Copy + Default
// {
//     fn from(data: (usize, Vec<T>)) -> Self {
//         let mut matrix = Matrix::new(data.0, data.1.len() / data.0);
//         for c in 0..matrix.len() {
//             matrix[c] = 
//         }
//         matrix
//     }
// }