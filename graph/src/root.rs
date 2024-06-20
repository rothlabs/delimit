use std::{
    any::Any, hash::Hash, sync::{RwLock, Weak}
};

use serde::Serialize;

use crate::{Meta, Node, SolveReact, Edge};

const GOAL: &str = "there should be a goal";

pub struct Root<U, T, L> {
    pub node: Weak<RwLock<Node<U, T, L>>>,
    pub meta: Meta, // TODO: remove meta?
}

impl<U, T, L> Root<U, T, L> 
where
    U: Clone + SolveReact<T, L>,
    T: Clone + Eq + PartialEq + Hash,
    L: Clone,
{
    pub fn react(&self) { 
        // let mut node = self.node.write().expect(NO_POISON);
        // node.solve(task)
    }
}

impl<U, T, L> Clone for Root<U, T, L> {
    fn clone(&self) -> Self {
        Self {
            node: self.node.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<U, T, L> Serialize for Root<U, T, L> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}

#[derive(Clone, Serialize)]
pub struct AnyRoot(pub Root<Box<dyn Any>, Box<dyn Any>, Box<dyn Any>>);
