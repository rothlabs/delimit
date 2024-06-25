use std::sync::{Arc, RwLock};

use crate::{node, Read, Write, Reactor};
use crate::{Edge, FromReactor, FromUnit, Meta};

use super::CloneUnit;

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

impl<U> FromReactor for Leaf<U> {
    fn from_reactor(&self, root: Reactor) -> Self {
        Self(self.0.from_reactor(root))
    }
}

impl<U> super::Read for Leaf<U> {
    type Unit = <node::Leaf<U> as Read>::Unit;
    fn read<F: FnOnce(&Self::Unit)>(&self, read: F) {
        self.0.read(read);
    }
}

impl<U> super::Write for Leaf<U> {
    type Unit = <node::Leaf<U> as Read>::Unit;
    fn write<F: FnOnce(&mut Self::Unit)>(&self, read: F) {
        self.0.write(read);
    }
}

impl<U: Clone> CloneUnit for Leaf<U> {
    type Unit = <node::Leaf<U> as Read>::Unit;
    fn unit(&self) -> Self::Unit {
        self.0.unit()
    }
}
