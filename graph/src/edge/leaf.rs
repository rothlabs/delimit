use std::sync::{Arc, RwLock};

use crate::node::{self, Reactor};
use crate::{Edge, Meta, New};

use super::{CloneUnit, Read, SetRoot, Write};

pub struct Leaf<U>(Edge<Reactor, node::Leaf<U>>);

impl<U> New for Leaf<U> {
    type Unit = U;
    fn new(unit: U) -> Self {
        Self(Edge {
            root: None,
            stem: Arc::new(RwLock::new(node::Leaf::new(unit))),
            meta: Meta::new(),
        })
    }
}

impl<U> SetRoot for Leaf<U> {
    type Node = Reactor;
    fn set_root(&mut self, node: &Arc<RwLock<Self::Node>>) {
        self.0.set_root(node);
    }
}

impl<U> Read for Leaf<U> {
    type Stem = node::Leaf<U>;
    fn read<F: FnOnce(&<Self::Stem as node::Read>::Unit)>(&self, read: F) {
        self.0.read(read);
    }
}

impl<U> Write for Leaf<U> {
    type Stem = node::Leaf<U>;
    fn write<F: FnOnce(&mut <Self::Stem as node::Write>::Unit)>(&self, read: F) {
        self.0.write(read);
    }
}

impl<U: Clone> CloneUnit for Leaf<U> {
    type Stem = node::Leaf<U>;
    fn unit(&self) -> <Self::Stem as crate::node::Read>::Unit {
        self.0.unit()
    }
}
