use std::{hash::Hash, sync::{Arc, RwLock}};

use serde::Serialize;

use crate::{Id, Solve, Stem, Roll};

const NO_POISON: &str = "the lock should not be poisoned";
//const GOAL: &str = "there should be a goal";

pub struct Edge<U, T, G> {
    roll: Arc<RwLock<Roll<U, T, G>>>,
    meta: Meta,
}

impl<U: Clone + Serialize + Solve<T, G>, T: Clone + Eq + PartialEq + Hash, G: Clone> Edge<U, T, G> {
    pub fn new(unit: U) -> Self {
        // TODO: read edges and set self as root for stems
        // for stem in unit.stems().iter() {

        // }
        Self {
            roll: Arc::new(RwLock::new(Roll::new(unit))),
            meta: Meta::new(),
        }
    }
    pub fn unit(&self) -> U {
        let roll = self.roll.read().expect(NO_POISON);
        let base = roll.read();
        base.unit.clone()
    }
    pub fn solve(&self, task: T) -> G { // TODO: rename to goal?
        let roll = self.roll.read().expect(NO_POISON);
        let mut base = roll.write();
        base.solve(task)
    }
    pub fn read<F: FnOnce(&U)>(&self, read: F) {
        let roll = self.roll.read().expect(NO_POISON);
        read(&roll.read().unit); 
    }
    pub fn write<F: FnOnce(&mut U)>(&self, write: F) {
        let roll = self.roll.read().expect(NO_POISON);
        write(&mut roll.write().unit);
        // TODO: read edges and set self as root for stems
    }
}

impl<U, T, G> Clone for Edge<U, T, G> {
    fn clone(&self) -> Self {
        Self {
            roll: self.roll.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<U, T, G> Serialize for Edge<U, T, G> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}

impl<U, T, G> Stem for Edge<U, T, G> {}

#[derive(Clone, Serialize)]
struct Meta {
    id: Id,
}

impl Meta {
    fn new() -> Self {
        Self { id: Id::new() }
    }
}
