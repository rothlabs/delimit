use std::{collections::HashMap, sync::{Arc, RwLock}};

use serde::Serialize;

use crate::{Compute, Id, Node, Read, Snap, Swap, Write};

pub struct Edge<U, G> {
    swap: Arc<RwLock<Swap<U, G>>>,  
    meta: Meta,
}

impl<U: Compute<G> + Clone + Serialize, G> Edge<U, G> {
    pub fn new(snap: &Snap, unit: U) -> Self {
        Self {
            swap: Arc::new(RwLock::new(Swap::new(snap, unit))),
            meta: Meta::new(),
        }
    }
    // pub fn read(&self) -> Read<Node<U>> {
    //     Read::new(self.swap.read().expect("the lock should not be poisoned"))
    // }
    // // use to write to node unit, use write_meta for writing to edge meta
    // pub fn write(&self) -> Write<Node<U>> {
    //     // let node = self.read();
    //     // if node.unit_strong_count() > 1 {
    //     //     let u_clone = node.read().clone();
    //     //     drop(node);
    //     //     self.write().unit = Arc::new(RwLock::new(u_clone));
    //     // } else {
    //     //     drop(node);
    //     // }
    //     Write::new(self.swap.write().expect("the lock should not be poisoned"))
    // }
    // // do not use to write to node unit, another node could be pointing to the same unit!
    // pub fn write_meta(&self) -> Write<Node<U>> {
    //     Write::new(self.swap.write().expect("the lock should not be poisoned"))
    // }
}

// impl<G> Edge<String, G> {
//     pub fn str(unit: &str) -> Self {
//         Self::new(unit.to_owned())
//     }
// }

impl<U, G> Clone for Edge<U, G> {
    fn clone(&self) -> Self {
        Self {
            swap: self.swap.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<U, G> Serialize for Edge<U, G> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer) //.read().expect("swap should not be poisoned").serialize(serializer)
    }
}

#[derive(Clone, Serialize)]
struct Meta {
    id: Id, 
}

impl Meta {
    fn new() -> Self {
        Self {
            id: Id::new(),
        }
    }
}
