use serde::Serialize;

use crate::link::{React, Reactor};

use super::{New, Read, Write};

#[derive(Clone)]
pub struct Leaf<U> {
    unit: U,
    reactors: Vec<Reactor>,
}

impl<U: Clone> Leaf<U> {
    fn set(&mut self, unit: U) {
        self.unit = unit;
        for reactor in self.reactors.iter() {
            reactor.react();
        }
    }
}

impl<U> New for Leaf<U> {
    type Unit = U;
    fn new(unit: Self::Unit) -> Self {
        Self {
            unit, 
            reactors: vec![],
        }
    }
}

impl<U: Clone> Read for Leaf<U> {
    type Unit = U;
    fn read(&self) -> &Self::Unit {
        &self.unit
    }
}

impl<U: Clone> Write for Leaf<U> {
    type Unit = U;
    fn write(&mut self) -> &mut Self::Unit {
        &mut self.unit
    }
}

impl<U: Serialize> Serialize for Leaf<U> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.unit.serialize(serializer)
    }
}