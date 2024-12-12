use super::*;

mod shape;

pub struct Shape<T, const D: usize> {
    pub bounds: Vec<Grc<Shape<T, D>>>,
    pub kind: shape::Kind<T, D>,
}

pub struct Vector<T, const D: usize> {
    pub id: u32,
    pub data: [T; D]
}

