use std::collections::HashMap;

use serde::Serialize;

use crate::{Node, Snap};

#[derive(Serialize)]
pub struct Swap<U, A, G> {
    mode: Mode<U, A, G>,
    // node: HashMap<Snap, Node<U, A, G>>,
    // snap: Snap,
}

impl<U, A, G> Swap<U, A, G> {
    pub fn new(unit: U) -> Self {
        Self {
            mode: Mode::Node(Node::new(unit))
        }
    }
    // pub fn new(snap: &Snap, unit: U) -> Self {
    //     let mut node = HashMap::new();
    //     node.insert(snap.clone(), Node::new(unit));
    //     Self {
    //         node,
    //         snap: snap.clone(),
    //     }
    // }
    pub fn node(&self) -> &Node<U, A, G> {
        match &self.mode {
            Mode::Node(node) => node,
            Mode::Nodes(nodes) => nodes.map.get(&nodes.now).expect("there should be a node at this snap"),
        }
        //self.node.get(&self.snap).expect("there should be a node at this snap")
    }
    pub fn get_mut(&mut self) -> &mut Node<U, A, G> {
        let wow = match &mut self.mode {
            Mode::Node(node) => node,
            Mode::Nodes(nodes) => nodes.map.get_mut(&nodes.now).expect("there should be a node at this snap"),
        };
        wow 
        //self.node.get(&self.snap).expect("there should be a node at this snap")
    }
    // pub fn snap(&self) -> &Snap {
    //     &self.snap
    // }
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

// impl<U, G> Serialize for Swap<U, G> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         self.swap.serialize(serializer) //.read().expect("swap should not be poisoned").serialize(serializer)
//     }
// }