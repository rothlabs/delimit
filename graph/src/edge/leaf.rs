use std::sync::{Arc, RwLock};

use crate::*;

pub struct Leaf<U>(Edge<node::Leaf<U>>);

impl<U> FromUnit for Leaf<U> {
    type Unit = U;
    fn from_unit(unit: U) -> Self {
        Self(Edge {
            root: None,
            stem: Arc::new(RwLock::new(node::Leaf::from_unit(unit))),
            meta: Meta::new(),
        })
    }
}

impl<U> WithReactor for Leaf<U> {
    fn with_reactor<T: ToReactor>(&self, item: T) -> Self {
        Self(self.0.with_reactor(item))
    }
}

impl<U> Reader for Leaf<U> {
    type Unit = U;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F) {
        self.0.reader(read);
    }
}

impl<U> Writer for Leaf<U> {
    type Unit = U;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, read: F) {
        self.0.writer(read);
    }
}

impl<U: Clone> CloneUnit for Leaf<U> {
    type Unit = U;
    fn unit(&self) -> Self::Unit {
        self.0.unit()
    }
}
