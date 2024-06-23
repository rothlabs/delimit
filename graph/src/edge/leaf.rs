use std::sync::{Arc, RwLock};

use crate::node::{self, Reactor};
use crate::{Edge, FromRoot, FromUnit, Meta};

use super::{CloneUnit, Read, Write};

pub struct Leaf<U>(Edge<Reactor, node::Leaf<U>>);

impl<U> FromUnit for Leaf<U> {
    type Unit = U;
    fn new(unit: U) -> Self {
        Self(Edge {
            root: None,
            stem: Arc::new(RwLock::new(node::Leaf::new(unit))),
            meta: Meta::new(),
        })
    }
}

impl<U> FromRoot for Leaf<U> {
    type Root = Reactor;
    fn from_root(&self, root: &Arc<RwLock<Self::Root>>) -> Self {
        Self(self.0.from_root(root))
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
