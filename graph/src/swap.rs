use std::{collections::HashMap, hash::Hash};

use serde::Serialize;

use crate::{node::Node, Base, Snap, Solve};

const SNAP_KEY: &str = "there should be a snap key and node value";

#[derive(Serialize)]
pub struct Swap<U, A, G> {
    mode: Mode<U, A, G>,
}

impl<U: Solve<T, G>, T: Clone + Eq + PartialEq + Hash, G: Clone> Swap<U, T, G> {
    pub fn new(unit: U) -> Self {
        Self {
            mode: Mode::Node(Node::new(unit)),
        }
    }
    pub fn now(&self) -> &Base<U, T, G> { // pub fn read<F: FnOnce(&U)>(&self, read: F) {
        match &self.mode {
            Mode::Node(node) => node.0.read(),
            Mode::Nodes(nodes) => nodes.map.get(&nodes.now).expect(SNAP_KEY),
        }
    }
    pub fn now_mut(&mut self) -> &mut Base<U, T, G> {
        match &mut self.mode {
            Mode::Node(node) => node,
            Mode::Nodes(nodes) => nodes.map.get_mut(&nodes.now).expect(SNAP_KEY),
        }
    }
}

#[derive(Serialize)]
enum Mode<U, A, G> {
    Nodes(Nodes<U, A, G>),
    Node(Node<U, A, G>),
}

#[derive(Serialize)]
struct Nodes<U, A, G> {
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
