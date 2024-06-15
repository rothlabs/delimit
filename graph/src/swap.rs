use std::{collections::HashMap, sync::Arc};

use serde::Serialize;

use crate::{Node, Snap};

#[derive(Serialize)]
pub struct Swap<U, G> {
    node: HashMap<Snap, Node<U, G>>,
    snap: Snap,
}

impl<U, G> Swap<U, G> {
    pub fn new(snap: &Snap, unit: U) -> Self {
        let mut node = HashMap::new();
        node.insert(snap.clone(), Node::new(unit));
        Self {
            node,
            snap: snap.clone(),
        }
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