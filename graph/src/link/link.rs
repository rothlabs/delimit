use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::*;

pub struct Link<E> {
    pub edge: Arc<RwLock<E>>,
    pub meta: Meta,
}

impl<E> FromUnit for Link<E>
where
    E: FromUnit,
{
    type Unit = E::Unit;
    fn from_unit(unit: E::Unit) -> Self {
        Self {
            edge: Arc::new(RwLock::new(E::from_unit(unit))),
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

impl<E> ReadWith for Link<E>
where
    E: ReadWith,
{
    type Unit = E::Unit;
    fn read<F: FnOnce(&Self::Unit)>(&self, read: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.read(read);
    }
}

impl<E> WriteInner for Link<E>
where
    E: WriteInner,
{
    type Unit = E::Unit;
    fn write<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.write(write);
    }
}

impl<E> CloneUnit for Link<E>
where
    E: CloneUnit,
{
    type Unit = E::Unit;
    fn unit(&self) -> Self::Unit {
        let edge = self.edge.read().expect(NO_POISON);
        edge.unit()
    }
}

impl<E> Solve for Link<E>
where
    E: Solve,
{
    type Load = E::Load;
    type Task = E::Task;
    fn solve(&self, task: Self::Task) -> Self::Load {
        let edge = self.edge.read().expect(NO_POISON);
        edge.solve(task)
    }
}

impl<E> AddStem for Link<E>
where
    E: AddStem, // + React + 'static,
{
    type Stem = <E as AddStem>::Stem;
    fn add_stem(&mut self, stem: Self::Stem) {
        //let reactor = Reactor::new(&self);
        // let link = stem.from_reactor(reactor);
        // let mut stem = self.stem.write().expect(NO_POISON);
        // stem.add_stem(link);
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
