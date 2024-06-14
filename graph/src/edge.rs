use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::{Base, Node, Read, Write};


pub struct Edge<U, P> {
    node: Arc<RwLock<Node<U>>>,
    meta: Arc<RwLock<Option<P>>>,
}

impl<U: Clone + Serialize, P: Base> Edge<U, P> {
    pub fn new(unit: U) -> Self {
        Self {
            node: Arc::new(RwLock::new(Node::new(unit))),
            meta: Arc::new(RwLock::new(None)),
        }
    }
    pub fn read(&self) -> Read<Node<U>> {
        Read::new(
            self.node.read().expect("the lock should not be poisoned")
        )
    }
    // use to write to node unit, use write_meta for writing to edge meta 
    pub fn write(&self) -> Write<Node<U>> {
        let node = self.read();
        if node.unit_strong_count() > 1 {
            let u_clone = node.read().clone();
            drop(node);
            self.write().unit = Arc::new(RwLock::new(u_clone));
        } else {
            drop(node);
        }
        Write::new(
            self.node.write().expect("the lock should not be poisoned")
        )
    }
    // do not use to write to node unit, another node could be pointing to the same unit!
    pub fn write_meta(&self) -> Write<Node<U>> {
        Write::new(
            self.node.write().expect("the lock should not be poisoned")
        )
    }
}

// impl<U, R> Edge<U> {
//     pub fn compute(&self) -> R {
//         //self.read().read()
//     }
// }

impl Edge<String> {
    pub fn str(unit: &str) -> Self {
        Self::new(unit.to_owned())
    }
}

impl Base<Edge<String>> for Edge<String> {
    fn compute(&self) -> Edge<String> {
        self.clone()
    }
    // fn edges(&self) -> Vec<Box<dyn Base>> {
    //     vec![]
    // }
}

impl<U> Clone for Edge<U> {
    fn clone(&self) -> Self {
        Self {
            node: self.node.clone(),
        }
    }
}

impl<U> Serialize for Edge<U> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.node.serialize(serializer)
    }
}
