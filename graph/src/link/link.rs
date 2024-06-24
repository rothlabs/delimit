use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::{base, edge, node, AddStem, FromReactor, FromUnit, Meta, Reactor, NO_POISON};

use super::{CloneUnit, Read, Solve, Write};

pub struct Link<E> {
    edge: Arc<RwLock<E>>,
    meta: Meta,
}

impl<E> FromUnit for Link<E>
where
    E: FromUnit,
{
    type Unit = E::Unit;
    fn new(unit: E::Unit) -> Self {
        Self {
            edge: Arc::new(RwLock::new(E::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<E> FromReactor for Link<E>
where
    E: FromReactor,
{
    fn from_reactor(&self, reactor: Reactor) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: Arc::new(RwLock::new(edge.from_reactor(reactor))),
            meta: self.meta.clone(),
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

impl<E> CloneUnit for Link<E>
where
    E: edge::CloneUnit,
{
    type Edge = E;
    fn unit(&self) -> <E::Stem as node::Read>::Unit {
        let edge = self.edge.read().expect(NO_POISON);
        edge.unit()
    }
}

impl<E> Solve for Link<E>
where
    E: edge::Solve,
{
    type Edge = E;
    fn solve(&self, task: <E::Stem as base::Solve>::Task) -> <E::Stem as base::Solve>::Load {
        let edge = self.edge.read().expect(NO_POISON);
        edge.solve(task)
    }
}

impl<E> AddStem for Link<E>
where
    E: AddStem,
{
    type Stem = <E as AddStem>::Stem;
    fn add_stem(&mut self, stem: Self::Stem) {
        let mut edge = self.edge.write().expect(NO_POISON);
        edge.add_stem(stem);
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
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}
