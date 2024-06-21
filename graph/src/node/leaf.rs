use crate::link::{React, Reactor};

use super::{Read, Write};

pub struct Leaf<U> {
    unit: U,
    reactors: Vec<Reactor>,
}

impl<U> Read for Leaf<U> {
    type Unit = U;
    fn read(&self) -> &Self::Unit {
        &self.unit
    }
}

impl<U> Write for Leaf<U> {
    type Unit = U;
    fn write(&mut self) -> &mut Self::Unit {
        for reactor in self.reactors.iter() {
            reactor.react(());
        }
        &mut self.unit
    }
}
