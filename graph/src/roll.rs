use std::{collections::HashMap, hash::Hash, sync::{RwLockReadGuard, RwLockWriteGuard}};

use serde::Serialize;

use crate::{node::Node, Base, Snap, Solve};

const NO_POISON: &str = "the lock should not be poisoned";
const SNAP_KEY: &str = "there should be a snap key and node value";

#[derive(Serialize)]
pub enum Roll<U, T, G> {
    Node(Node<U, T, G>),
    Mult(Mult<U, T, G>),
}

impl<U: Clone + Solve<T, G>, T: Clone + Eq + PartialEq + Hash, G: Clone> Roll<U, T, G> {
    pub fn new(unit: U) -> Self {
        Self::Node(Node::new(unit))
    }
    pub fn read(&self) -> RwLockReadGuard<'_, Base<U, T, G>> {
        match self {
            Roll::Node(node) => node.base.read().expect(NO_POISON),
            Roll::Mult(mult) => mult.read(),
        }
    }
    pub fn write(&self) -> RwLockWriteGuard<'_, Base<U, T, G>> {
        match self {
            Roll::Node(node) => node.base.write().expect(NO_POISON),
            Roll::Mult(mult) => mult.write(),
        }
    }
}

#[derive(Serialize)]
pub struct Mult<U, T, G> {
    map: HashMap<Snap, Node<U, T, G>>,
    now: Snap,
}

impl<U, T, G> Mult<U, T, G> {
    fn read(&self) -> RwLockReadGuard<'_, Base<U, T, G>> {
        let base = &self.map.get(&self.now).expect(SNAP_KEY).base;
        base.read().expect(NO_POISON)
    }
    fn write(&self) -> RwLockWriteGuard<'_, Base<U, T, G>> {
        let base = &self.map.get(&self.now).expect(SNAP_KEY).base;
        base.write().expect(NO_POISON)
    }
}


// match &mut self.mode {
//     Mode::Mono(node) => write(&mut node.base.write().expect(NO_POISON)),
//     Mode::Mult(mult) => {
//         let base = &mult.map.get_mut(&mult.now).expect(SNAP_KEY).base;
//         write(&mut base.write().expect(NO_POISON));
//     },
// }

        // match &self.mode {
        //     Mode::Mono(node) => read(&node.base.read().expect(NO_POISON)),
        //     Mode::Mult(mult) => {
        //         let base = &mult.map.get(&mult.now).expect(SNAP_KEY).base;
        //         read(&base.read().expect(NO_POISON));
        //     },
        // }


// pub fn from_snap_unit(snap: &Snap, unit: U) -> Self {
//     let mut map = HashMap::new();
//     map.insert(snap.clone(), Node::new(unit));
//     Self {
//         mode: Mode::Nodes(Nodes {
//             map,
//             now: snap.clone(),
//         }),
//     }
// }


// pub fn now(&self) -> RwLockReadGuard<'_, Base<U, T, G>> {  
//     match &self.mode {
//         Mode::Mono(node) => node.0.read().expect(NO_POISON),
//         Mode::Snaps(nodes) => {
//             let arc = nodes.map.get(&nodes.now).expect(SNAP_KEY).0;
//             let wow = arc.read().expect(NO_POISON);
//             wow
//         },
//     }
// }
