use super::*;

pub enum Kind<T, D: ArrayLength> {
    Axis(Grc<Axis<T, D>>)
}

pub struct Axis<T, D: ArrayLength> {
    pub vector: Grc<Vector<T, D>>,
    pub kind: axis::Kind<T>
}

mod axis {
    pub enum Kind<T> {
        Extrude,
        Revolve(T)
    }
}