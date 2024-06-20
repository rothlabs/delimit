use std::{
    any::Any, hash::Hash, sync::{Arc, RwLock, Weak}
};

use serde::Serialize;

use crate::{Meta, Node, SolveReact};

const NO_POISON: &str = "the lock should not be poisoned";
const LOAD: &str = "there should be a goal";
const ROOT: &str = "there should be a root";

pub struct Edge<SU, ST, SL, SV, RU, RT, RL, RV> {
    pub stem: Arc<RwLock<Node<SU, ST, SL, SV>>>,
    pub root: Option<Weak<RwLock<Node<RU, RT, RL, RV>>>>,
    pub meta: Meta,
}

impl<SU, ST, SL, SV, RU, RT, RL, RV> Edge<SU, ST, SL, SV, RU, RT, RL, RV>
where
    SU: Clone + SolveReact<ST, SL, SV>,
    ST: Clone + Eq + PartialEq + Hash,
    SL: Clone,
    RU: Clone + SolveReact<RT, RL, RV>,
    RT: Clone + Eq + PartialEq + Hash,
    RL: Clone,
{
    pub fn new(unit: SU) -> Self {
        Self {
            stem: Arc::new(RwLock::new(Node::new(unit))),
            root: None,
            meta: Meta::new(),
        }
    }
    pub fn solve(&self, task: ST) -> SL { // TODO: rename to load?
        let mut node = self.stem.write().expect(NO_POISON);
        node.solve(task)
    }
    pub fn unit(&self) -> SU {
        let node = self.stem.read().expect(NO_POISON);
        node.unit.clone()
    }
    pub fn read<F: FnOnce(&SU)>(&self, read: F) {
        let node = self.stem.read().expect(NO_POISON);
        read(&node.unit); 
    }
    pub fn write<F: FnOnce(&mut SU) -> RV>(&self, write: F) {
        let mut stem = self.stem.write().expect(NO_POISON);
        let variance = write(&mut stem.unit);
        if let Some(weak) = &self.root {
            let arc = weak.upgrade().expect(ROOT);
            let mut root = arc.write().expect(NO_POISON);
            root.react(variance);
        }
    }
    pub fn root(&mut self, stem: &Arc<RwLock<Node<RU, RT, RL, RV>>>) {
        self.root = Some(Arc::downgrade(stem));
    }
}

impl<SU, ST, SL, SV, RU, RT, RL, RV> Clone for Edge<SU, ST, SL, SV, RU, RT, RL, RV> {
    fn clone(&self) -> Self {
        Self {
            stem: self.stem.clone(),
            root: self.root.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<SU, ST, SL, SV, RU, RT, RL, RV>  Serialize for Edge<SU, ST, SL, SV, RU, RT, RL, RV>  {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}

//#[derive(Clone)]
pub struct BoxAny(pub Box<dyn Any>);



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
