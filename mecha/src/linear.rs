pub use vector::VectorX;

mod vector;
mod matrix;

pub trait Next {
    type Next;
    fn next(&self) -> Option<&Self::Next>;
}

pub trait ZipX {
    type Item;
    fn zip<F: Fn(Self::Item, Self::Item) -> Self::Item>(&self, rhs: &Self, op: F) -> Self;
}

// pub trait IsTrue<const B: bool> {}

// impl IsTrue<true> for () {}

// pub struct Dim<const R: usize>;

// pub trait Dot {
//     type Num;
//     fn dot(&self, rhs: &Self) -> Self::Num;
// }