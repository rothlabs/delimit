use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::{Node, Read, Write};

// #[derive(Clone)]
pub struct Edge<U> {
    node: Arc<RwLock<Node<U>>>,
}

impl<U> Edge<U> {
    pub fn new(unit: U) -> Self {
        Self {
            node: Arc::new(RwLock::new(Node::new(unit))),
        }
    }
    pub fn read(&self) -> Read<Node<U>> {
        Read::new(
            self.node.read().expect("the lock should not be poisoned")
        )
    }
    pub fn write(&self) -> Write<Node<U>> {
        Write::new(
            self.node.write().expect("the lock should not be poisoned")
        )
    }
}

impl Edge<String> {
    pub fn str(unit: &str) -> Self {
        Self::new(unit.to_owned())
    }
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
