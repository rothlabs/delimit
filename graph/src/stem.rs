use std::{
    hash::Hash,
    sync::{Arc, RwLock},
};

use serde::Serialize;

use crate::{Meta, Node, Root, Solve};

const NO_POISON: &str = "the lock should not be poisoned";
const LOAD: &str = "there should be a goal";

pub struct Stem<U, T, L> {
    pub node: Arc<RwLock<Node<U, T, L>>>,
    pub meta: Meta,
}

impl<U, T, L> Stem<U, T, L>
where
    U: Clone + Solve<T, L>,
    T: Clone + Eq + PartialEq + Hash,
    L: Clone,
{
    pub fn new(unit: U) -> Self {
        Self {
            node: Arc::new(RwLock::new(Node::new(unit))),
            meta: Meta::new(),
        }
    }
    pub fn solve(&self, task: T) -> L { // TODO: rename to load?
        let mut node = self.node.write().expect(NO_POISON);
        node.solve(task)
    }
    pub fn unit(&self) -> U {
        let node = self.node.read().expect(NO_POISON);
        node.unit.clone()
    }
    pub fn read<F: FnOnce(&U)>(&self, read: F) {
        let node = self.node.read().expect(NO_POISON);
        read(&node.unit); 
    }
    pub fn write<F: FnOnce(&mut U)>(&self, write: F) {
        let mut node = self.node.write().expect(NO_POISON);
        write(&mut node.unit);
    }
    pub fn root(&self) -> Root<U, T, L> {
        Root {
            node: Arc::downgrade(&self.node),
            meta: self.meta.clone(),
        }
    }
}

impl<U, T, L> Clone for Stem<U, T, L> {
    fn clone(&self) -> Self {
        Self {
            node: self.node.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<U, T, L> Serialize for Stem<U, T, L> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}

// impl Flatten for String {
//     fn flatten(&self, flat: &mut Flat) { // , state: &mut Hasher
//         flat.units.in
//     }
// }

// impl<U, A, G> PartialEq for Base<U, A, G> {
//     fn eq(&self, rhs: &Base<U, A, G>) -> bool {
//         self.meta.node.id == rhs.meta.node.id
//     }
// }

//clone_trait_object!(Root);
// pub trait Root: { //DynClone {
//     fn clear_work(&mut self);
// }

// impl<U, T, G> Root for Base<U, T, G> {
//     fn clear_work(&mut self) {
//         self.work.clear();
//         for root in self.roots.iter() {
//             if let Some(root) = root.upgrade() {
//                 if let Ok(root) = &mut root.write() {
//                     root.clear_work();
//                 }
//             } // TODO: collect indices of dropped roots to remove from vec (do the same for poisoned?)
//         }
//     }
// }

// pub trait Stem {}
