use std::{
    hash::Hash,
    sync::{RwLock, Weak},
};

use serde::Serialize;

use crate::{Meta, Node, Solve, Stem};

const GOAL: &str = "there should be a goal";

pub struct Root<U, T, L> {
    pub node: Weak<RwLock<Node<U, T, L>>>,
    pub meta: Meta,
}

impl<U: Solve<T, L>, T: Clone + Eq + PartialEq + Hash, L: Clone> Root<U, T, L> {
    // pub fn new(stem: Stem<U, T, L>) -> Self {

    //     Self {
    //         node: Weak::new(RwLock::new(Node::new(unit))),
    //         meta: Meta::new(),
    //     }
    // }
}

impl<U, T, L> Serialize for Root<U, T, L> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}
