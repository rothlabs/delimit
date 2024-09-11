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

impl IntoLeaf<Vf32> for Vec<f32> {
    fn leaf(self) -> Leaf<Vf32> {
        Leaf::new(Vf32(self))
    }
}
