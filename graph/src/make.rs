use crate::*;

// pub trait Leafy {}

pub trait ToString {
    fn string(&self) -> String;
}

pub trait ToLeaf<T> {
    fn leaf(&self) -> Leaf<T>;
}

pub trait IntoLeaf<T> {
    fn into_leaf(self) -> Leaf<T>;
}

impl<T> IntoLeaf<T> for T {
    fn into_leaf(self) -> Leaf<T> {
        Leaf::from_unit(self)
    }
}

impl<T: ToString> ToLeaf<String> for T {
    fn leaf(&self) -> Leaf<String> {
        self.string().into_leaf()
    }
}