use serde::Serialize;

use crate::*;

pub struct Leaf<U> {
    unit: U,
    reactors: Reactors,
}

impl<U> FromUnit for Leaf<U> {
    type Unit = U;
    fn new(unit: Self::Unit) -> Self {
        Self {
            unit,
            reactors: Reactors::default(),
        }
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
        self.reactors.cycle();
    }
}

impl<U> AddReactor for Leaf<U> {
    fn add_reactor(&mut self, reactor: &Reactor) {
        self.reactors.add(reactor);
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
