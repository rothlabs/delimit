use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::{edge, node, Meta, NO_POISON};

use super::{New, Read, Solve, Write};

pub struct Link<E> {
    edge: Arc<RwLock<E>>,
    meta: Meta,
}

impl<E> New for Link<E>
where
    E: edge::New,
{
    type Unit = E::Unit;
    fn new(unit: E::Unit) -> Self {
        Self {
            edge: Arc::new(RwLock::new(E::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<E> Read for Link<E>
where
    E: edge::Read,
{
    type Edge = E;
    fn read<F: FnOnce(&<E::Stem as node::Read>::Unit)>(&self, read: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.read(read);
    }
}

impl<E> Write for Link<E>
where
    E: edge::Write,
{
    type Edge = E;
    fn write<F: FnOnce(&mut <E::Stem as node::Write>::Unit)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.write(write);
    }
}

impl<E> Solve for Link<E> 
where
    E: edge::Solve,
{
    type Edge = E;
    fn solve(&self, task: <E::Stem as node::Solve>::Task) -> <E::Stem as node::Solve>::Load {
        let edge = self.edge.read().expect(NO_POISON);
        edge.solve(task)
    }
}

impl<E> Clone for Link<E> {
    fn clone(&self) -> Self {
        Self {
            edge: self.edge.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<E> Serialize for Link<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.meta.serialize(serializer)
    }
    
}
