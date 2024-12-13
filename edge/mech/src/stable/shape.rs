use super::*;

// pub mod operation;

pub enum Kind<T, D: ArrayLength> {
    Point(Grc<Vector<T, D>>),
    Operation(Grc<Operation<T, D>>),
}

pub struct Operation<T, D: ArrayLength> {
    // pub stems: Vec<Grc<Shape<T, D>>>,
    pub kind: operation::Kind<T, D>,
}