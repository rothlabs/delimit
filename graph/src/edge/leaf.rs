use std::sync::{Arc, RwLock};

use crate::node::{self, Reactor};
use crate::{Edge, Meta, New};

use super::{Read, ToUnit, Write};

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

impl<U: Clone> Read for Leaf<U>
// where
//     node::Leaf<U>: node::Read
{
    type Stem = node::Leaf<U>;
    fn read<F: FnOnce(&<Self::Stem as node::Read>::Unit)>(&self, read: F) {
        self.0.read(read);
    }
}

impl<U: Clone> Write for Leaf<U>
// where
//     node::Leaf<U>: node::Write
{
    type Stem = node::Leaf<U>;
    fn write<F: FnOnce(&mut <Self::Stem as node::Write>::Unit)>(&self, read: F) {
        self.0.write(read);
    }
}

impl<U: Clone> ToUnit for Leaf<U> {
    type Stem = node::Leaf<U>;
    fn unit(&self) -> <Self::Stem as crate::node::Read>::Unit {
        self.0.unit()
    }
}
