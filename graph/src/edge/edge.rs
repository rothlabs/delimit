use std::{
    any::Any,
    sync::{Arc, RwLock, Weak},
};

use serde::Serialize;

use crate::{node, Meta, NO_POISON};

use super::{AsUnit, Read, Write}; 

pub struct Edge<R, S> {
    pub root: Option<Weak<RwLock<R>>>,
    pub stem: Arc<RwLock<S>>,
    pub meta: Meta,
}

impl<R, S> Read for Edge<R, S>
where
    S: node::Read,
{
    type Stem = S;
    fn read<F: FnOnce(&S::Unit)>(&self, read: F) {
        let stem = self.stem.read().expect(NO_POISON);
        read(&stem.read());
    }
}

impl<R, S> Write for Edge<R, S>
where
    S: node::Write,
{
    type Root = R;
    type Stem = S;
    fn write<F: FnOnce(&mut S::Unit)>(&self, write: F) {
        let mut stem = self.stem.write().expect(NO_POISON);
        write(&mut stem.write());
    }
}

impl<R, S> AsUnit for Edge<R, S>
where
    S: node::Read,
    S::Unit: Clone,
{
    type Stem = S;
    fn unit(&self) -> S::Unit {
        let stem = self.stem.read().expect(NO_POISON);
        stem.read().clone()
    }
}

// fn write<F: FnOnce(&mut S::Unit) -> R::Vary>(&self, write: F) {
//     let mut stem = self.stem.write().expect(NO_POISON);
//     let variance = write(&mut stem.unit_mut());
//     if let Some(weak) = &self.root {
//         let arc = weak.upgrade().expect(ROOT);
//         let mut root = arc.write().expect(NO_POISON);
//         root.react(variance);
//     }
// }






// impl<R, S> Edge for Main<R, S>
// where
//     R: React,
//     S: UnitRef + Solve,
// {
//     type R = R;
//     type S = S;
//     fn new(unit: S::Unit) -> Self {
//         Self {
//             stem: Arc::new(RwLock::new(S::new(unit))),
//             root: None,
//             meta: Meta::new(),
//         }
//     }
//     fn solve(&self, task: S::Task) -> S::Load { // TODO: rename to load?
//         let mut node = self.stem.write().expect(NO_POISON);
//         node.solve(task)
//     }
//     fn unit(&self) -> S::Unit {
//         let node = self.stem.read().expect(NO_POISON);
//         node.unit().clone()
//     }
//     fn read<F: FnOnce(&S::Unit)>(&self, read: F) {
//         let node = self.stem.read().expect(NO_POISON);
//         read(&node.unit());
//     }
//     fn write<F: FnOnce(&mut S::Unit) -> R::Vary>(&self, write: F) {
//         let mut stem = self.stem.write().expect(NO_POISON);
//         let variance = write(&mut stem.unit_mut());
//         if let Some(weak) = &self.root {
//             let arc = weak.upgrade().expect(ROOT);
//             let mut root = arc.write().expect(NO_POISON);
//             root.react(variance);
//         }
//     }
//     fn root(&mut self, stem: &Arc<RwLock<R>>) {
//         self.root = Some(Arc::downgrade(stem));
//     }
// }

impl<R, S> Clone for Edge<R, S> {
    fn clone(&self) -> Self {
        Self {
            stem: self.stem.clone(),
            root: self.root.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<R, St> Serialize for Edge<R, St> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}
