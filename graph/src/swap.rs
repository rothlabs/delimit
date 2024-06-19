use std::{collections::HashMap, hash::Hash};

use serde::Serialize;

use crate::{node::Node, Base, Snap, Solve};

const NO_POISON: &str = "the lock should not be poisoned";
const SNAP_KEY: &str = "there should be a snap key and node value";

#[derive(Serialize)]
pub struct Swap<U, A, G> {
    mode: Mode<U, A, G>,
}

impl<U: Clone + Solve<T, G>, T: Clone + Eq + PartialEq + Hash, G: Clone> Swap<U, T, G> {
    pub fn new(unit: U) -> Self {
        Self {
            mode: Mode::Mono(Node::new(unit)),
        }
    }
    pub fn unit(&self) -> U {
        match &self.mode {
            Mode::Mono(node) => node.base.read().expect(NO_POISON).unit.clone(),
            Mode::Mult(mult) => {
                let base = &mult.map.get(&mult.now).expect(SNAP_KEY).base;
                base.read().expect(NO_POISON).unit.clone()
            },
        }
    }
    pub fn solve(&self, task: T) -> G {
        match &self.mode {
            Mode::Mono(node) => node.base.write().expect(NO_POISON).solve(task),
            Mode::Mult(mult) => {
                let base = &mult.map.get(&mult.now).expect(SNAP_KEY).base;
                base.write().expect(NO_POISON).solve(task)
            },
        }
    }
    pub fn read<F: FnOnce(&Base<U, T, G>)>(&self, read: F) {
        match &self.mode {
            Mode::Mono(node) => read(&node.base.read().expect(NO_POISON)),
            Mode::Mult(mult) => {
                let base = &mult.map.get(&mult.now).expect(SNAP_KEY).base;
                read(&base.read().expect(NO_POISON));
            },
        }
    }
    pub fn write<F: FnOnce(&mut Base<U, T, G>)>(&mut self, write: F) {
        match &mut self.mode {
            Mode::Mono(node) => write(&mut node.base.write().expect(NO_POISON)),
            Mode::Mult(mult) => {
                let base = &mult.map.get_mut(&mult.now).expect(SNAP_KEY).base;
                write(&mut base.write().expect(NO_POISON));
            },
        }
    }
}

#[derive(Serialize)]
enum Mode<U, A, G> {
    Mult(Mult<U, A, G>),
    Mono(Node<U, A, G>),
}

#[derive(Serialize)]
struct Mult<U, A, G> {
    map: HashMap<Snap, Node<U, A, G>>,
    now: Snap,
}


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
