use crate::*;

pub trait ToString {
    fn string(&self) -> String;
}

impl<T: ToString> ToLeaf<String> for T {
    fn leaf(&self) -> Leaf<String> {
        self.string().into_leaf()
    }
}

pub trait ToLeaf<T> {
    fn leaf(&self) -> Leaf<T>;
}

impl ToLeaf<String> for str {
    fn leaf(&self) -> Leaf<String> {
        Leaf::from_unit(self.to_owned())
    }
}

pub trait IntoLeaf<T> {
    fn into_leaf(self) -> Leaf<T>;
}

impl<T> IntoLeaf<T> for T {
    fn into_leaf(self) -> Leaf<T> {
        Leaf::from_unit(self)
    }
}
