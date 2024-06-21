use std::{
    any::Any, sync::{Arc, RwLock, Weak}
};

use serde::Serialize;

use crate::{Meta, Node};

mod main;

pub use main::Main;

const NO_POISON: &str = "the lock should not be poisoned";
const ROOT: &str = "there should be a root";

pub trait Edge {
    type R: Node;
    type S: Node;
    fn new(unit: <Self::S as Node>::U) -> Self;
    fn solve(&self, task: <Self::S as Node>::T) -> <Self::S as Node>::L;
    fn unit(&self) -> <Self::S as Node>::U;
    fn read<F: FnOnce(&<Self::S as Node>::U)>(&self, read: F);
    fn write<F: FnOnce(&mut <Self::S as Node>::U) -> <Self::R as Node>::V>(&self, write: F);
    fn root(&mut self, stem: &Arc<RwLock<Self::R>>);
}

// //#[derive(Clone)]
// pub struct BoxAny(pub Box<dyn Any>);

// impl Clone for BoxAny {
//     fn clone(&self) -> Self {
//         Self(
//             Box::new(self.0)
//         )
//     }
// }



// impl Clone for dyn Edge<R = dyn Node<U = dyn Any, T = dyn Any, L = dyn Any, V = dyn Any>, S = dyn Node<U = dyn Any, T = dyn Any, L = dyn Any, V = dyn Any>> {
//     fn clone(&self) -> Self {
//         Self {
//             stem: self.stem.clone(),
//             root: self.root.clone(),
//             meta: self.meta.clone(),
//         }
//     }
// }


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
