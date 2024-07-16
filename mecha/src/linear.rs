pub use vector::Vector;

mod vector;
mod matrix;

pub trait Dot {
    type Num;
    fn dot(&self, rhs: &Self) -> Self::Num;
}

pub trait Zip {
    type Item;
    const ROWS: usize;
    fn zip<F: Fn(Self::Item, Self::Item) -> Self::Item>(&self, rhs: &Self, op: F) -> Self;
}