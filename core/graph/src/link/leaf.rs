use crate::*;

pub trait ToLeaf<T> {
    /// Clone to new Leaf.
    fn leaf(&self) -> Leaf<T>;
}

impl ToLeaf<String> for str {
    /// Clone to Leaf.
    fn leaf(&self) -> Leaf<String> {
        Leaf::new(self.to_owned())
        // Leaf::new(Tray::String(self.into()))
    }
}

pub trait IntoLeaf<T> {
    /// Move into Leaf.
    fn leaf(self) -> Leaf<T>;
}

impl IntoLeaf<Vec<f32>> for Vec<f32> {
    fn leaf(self) -> Leaf<Vec<f32>> {
        Leaf::new(self)
    }
}

impl IntoLeaf<i32> for i32 {
    fn leaf(self) -> Leaf<i32> {
        Leaf::new(self)
    }
}
