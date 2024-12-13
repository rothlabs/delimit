use super::*;

pub enum Kind<T, const D: usize> {
    Axis(Axis<T, D>)
}

pub struct Axis<T, const D: usize> {
    pub vector: Grc<Vector<T, D>>,
    pub kind: axis::Kind<T>
}

mod axis {
    pub enum Kind<T> {
        Extrude,
        Revolve(T)
    }
}