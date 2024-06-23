use serde::Serialize;

use crate::{
    link::{React, Reactor},
    New,
};

use super::{Read, Write};

pub struct Leaf<U> {
    unit: U,
    reactors: Vec<Reactor>,
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
        for reactor in self.reactors.iter() {
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

// impl<U> Leaf<U> {
//     fn set(&mut self, unit: U) {
//         self.unit = unit;
//         for reactor in self.reactors.iter() {
//             reactor.react();
//         }
//     }
// }
