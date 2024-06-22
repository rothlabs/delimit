use std::{hash::Hash, sync::{Arc, RwLock}};

use serde::Serialize;

use crate::{edge::{self, New}, node, Meta, NO_POISON};

use crate::edge::Solve;

pub struct Solver<U, T, L>{
    edge: Arc<RwLock<edge::Solver<U, T, L>>>,
    meta: Meta,
}

impl<U, T, L> super::New for Solver<U, T, L>
// where
//     E: edge::New,
{
    type Unit = U;
    fn new(unit: U) -> Self {
        Self {
            edge: Arc::new(RwLock::new(edge::Solver::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<U, T, L> super::Solve for Solver<U, T, L>  
where
    U: node::Solve<Task = T, Load = L>,
    T: Clone + Eq + PartialEq + Hash,
    L: Clone,
{
    type Edge = edge::Solver<U, T, L>;
    fn solve(&self, task: <<Self::Edge as edge::Solve>::Stem as node::Solve>::Task) -> <<Self::Edge as edge::Solve>::Stem as node::Solve>::Load {
        let edge = self.edge.read().expect(NO_POISON);
        edge.solve(task)
    }
}

impl<U, T, L> Clone for Solver<U, T, L> {
    fn clone(&self) -> Self {
        Self {
            edge: self.edge.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<U, T, L> Serialize for Solver<U, T, L> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}

