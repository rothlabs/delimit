use serde::Serialize;

use crate::{AddReactor, FromUnit, Reactor};

use super::{Read, Write};

pub struct Leaf<U> {
    unit: U,
    reactors: Vec<Reactor>,
}

impl<U> FromUnit for Leaf<U> {
    type Unit = U;
    fn new(unit: Self::Unit) -> Self {
        Self {
            unit,
            reactors: vec![],
        }
    }
}

impl<U> AddReactor for Leaf<U> {
    fn add_reactor(&mut self, reactor: &Reactor) {
        self.reactors.push(reactor.clone());
    }
}

impl<U> Read for Leaf<U> {
    type Unit = U;
    fn read(&self) -> &Self::Unit {
        &self.unit
    }
}

impl<U> Write for Leaf<U> {
    type Unit = U;
    fn write<F: FnOnce(&mut U)>(&mut self, write: F) {
        write(&mut self.unit);
        for reactor in &self.reactors {
            reactor.react();
        }
    }
}

impl<U: Serialize> Serialize for Leaf<U> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.unit.serialize(serializer)
    }
}