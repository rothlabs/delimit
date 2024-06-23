use std::sync::{Arc, RwLock};

pub trait FromUnit {
    type Unit;
    fn new(unit: Self::Unit) -> Self;
}

pub trait FromRoot {
    type Root;
    fn from_root(&self, root: &Arc<RwLock<Self::Root>>) -> Self;
}
