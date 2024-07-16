use std::{ops::*, slice::SliceIndex};

#[derive(Clone)]
pub struct Vector<T>(Vec<T>);

impl<T> Vector<T> 
where 
    T: Copy + Default
{
    pub fn new(len: usize) -> Self {
        Self(vec![T::default(); len])
    }
    pub fn zip<F: Fn(T, T) -> T>(&self, rhs: &Self, op: F) -> Self {
        let mut vector = self.clone();
        for r in 0..vector.len() {
            vector[r] = op(self[r], rhs[r]);
        }
        vector
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn vec(&self) -> &Vec<T> {
        &self.0
    } 
}

impl<T> Vector<T> 
where 
    T: Copy + Default + Mul<T, Output = T> + Add<T, Output = T>
{
    pub fn dot(&self, rhs: &Self) -> T {
        (self * rhs).0.iter().fold(T::default(), |a, u| a + *u)
    } 
}

impl<T> From<Vec<T>> for Vector<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T: Copy> From<&Vec<T>> for Vector<T> {
    fn from(value: &Vec<T>) -> Self {
        Self(value.to_vec())
    }
}

impl<T> Add<&Vector<T>> for &Vector<T>
where
    T: Copy + Default + Add<T, Output = T>,
{
    type Output = Vector<T>;
    fn add(self, rhs: &Vector<T>) -> Vector<T> {
        self.zip(rhs, |l, r| l + r)
    }
}

impl<T> Sub<&Vector<T>> for &Vector<T>
where
    T: Copy + Default + Sub<T, Output = T>,
{
    type Output = Vector<T>;
    fn sub(self, rhs: &Vector<T>) -> Vector<T> {
        self.zip(rhs, |l, r| l - r)
    }
}

impl<T> Mul<&Vector<T>> for &Vector<T>
where
    T: Copy + Default + Mul<T, Output = T>,
{
    type Output = Vector<T>;
    fn mul(self, rhs: &Vector<T>) -> Vector<T> {
        self.zip(rhs, |l, r| l * r)
    }
}

impl<T, Idx> Index<Idx> for Vector<T> 
where
    Idx: SliceIndex<[T], Output = T>,
{
    type Output = T;
    fn index(&self, index: Idx) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T, Idx> IndexMut<Idx> for Vector<T> 
where
    Idx: SliceIndex<[T], Output = T>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}