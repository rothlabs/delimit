use crate::*;

/// Clone to Ace link.
pub trait ToLeaf {
    fn leaf(&self) -> Leaf;
}

impl ToLeaf for Load {
    fn leaf(&self) -> Leaf {
        Leaf::new(self.clone())
    }
}

impl ToLeaf for str {
    fn leaf(&self) -> Leaf {
        Leaf::new(Load::String(self.into()))
    }
}

pub trait IntoLeaf {
    fn leaf(self) -> Leaf;
}

impl<T> IntoLeaf for T
where
    T: IntoLoad,
{
    fn leaf(self) -> Leaf {
        Leaf::new(self.into_load())
    }
}
