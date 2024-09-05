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
