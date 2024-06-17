use std::collections::HashMap;

use serde::Serialize;

use crate::{Node, Snap};

#[derive(Serialize)]
pub struct Swap<U, A, G> {
    node: HashMap<Snap, Node<U, A, G>>,
    snap: Snap,
}

impl<U, A, G> Swap<U, A, G> {
    pub fn new(snap: &Snap, unit: U) -> Self {
        let mut node = HashMap::new();
        node.insert(snap.clone(), Node::new(unit));
        Self {
            node,
            snap: snap.clone(),
        }
    }
    pub fn get(&self) -> &Node<U, A, G> {
        self.node.get(&self.snap).expect("there should be a node at this snap")
    }
}

// impl<U, G> Serialize for Swap<U, G> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         self.swap.serialize(serializer) //.read().expect("swap should not be poisoned").serialize(serializer)
//     }
// }