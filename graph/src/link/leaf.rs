use crate::*;

pub trait ToLeaf {
    /// Clone to Leaf.
    fn leaf(&self) -> Leaf;
}

impl ToLeaf for str {
    /// Clone to Leaf.
    fn leaf(&self) -> Leaf {
        Leaf::new(Load::String(self.into()))
    }
}
