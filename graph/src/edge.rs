use std::{hash::Hash, sync::{Arc, RwLock}};

use serde::Serialize;

use crate::{Id, Solve, Stem, Swap};

const NO_POISON: &str = "the lock should not be poisoned";
//const GOAL: &str = "there should be a goal";

pub struct Edge<U, T, G> {
    swap: Arc<RwLock<Swap<U, T, G>>>,
    meta: Meta,
}

impl<U: Clone + Serialize + Solve<T, G>, T: Clone + Eq + PartialEq + Hash, G: Clone> Edge<U, T, G> {
    pub fn new(unit: U) -> Self {
        // TODO: read edges and set self as root for stems
        // for stem in unit.stems().iter() {

        // }
        Self {
            swap: Arc::new(RwLock::new(Swap::new(unit))),
            meta: Meta::new(),
        }
    }
    pub fn solve(&self, task: T) -> G {
        let mut swap = self.swap.write().expect(NO_POISON);
        swap.now_mut().solve(task)
    }
    pub fn read<F: FnOnce(&U)>(&self, read: F) {
        let swap = self.swap.read().expect(NO_POISON);
        read(&swap.now().unit);
    }
    pub fn write<F: FnOnce(&mut U)>(&self, write: F) {
        let mut swap = self.swap.write().expect(NO_POISON);
        write(&mut swap.now_mut().unit);
        // TODO: read edges and set self as root for stems
    }
    pub fn unit(&self) -> U {
        let swap = self.swap.read().expect(NO_POISON);
        swap.now().unit.clone()
    }
}

impl<U, T, G> Clone for Edge<U, T, G> {
    fn clone(&self) -> Self {
        Self {
            swap: self.swap.clone(),
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

impl<U, T, G> Stem for Edge<U, T, G> {
    
}

#[derive(Clone, Serialize)]
struct Meta {
    id: Id,
}

impl Meta {
    fn new() -> Self {
        Self { id: Id::new() }
    }
}
