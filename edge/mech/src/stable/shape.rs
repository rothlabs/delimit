use super::*;

pub mod operation;

pub enum Kind<T, const D: usize> {
    Point(Grc<Vector<T, D>>),
    Operation(Grc<Operation<T, D>>),
}

pub struct Operation<T, const D: usize> {
    pub stems: Vec<Grc<Shape<T, D>>>,
    pub kind: operation::Kind<T, D>,
}