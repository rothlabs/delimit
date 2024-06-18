use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::{Solve, Id, Node, Read, Snap, Swap, Write};

pub type LeafStr = Edge<String, (), ()>;

#[derive(Clone, Serialize)]
pub struct UnitStr {
    pub at: String,
    pub snap: Snap,
}

// TODO: type params: Unit, Args, Gain (U, A, G)
pub struct Edge<U, A, G> {
    swap: Arc<RwLock<Swap<U, A, G>>>,  
    meta: Meta,
}

impl<U: Solve<A, G> + Clone + Serialize, A, G: Clone> Edge<U, A, G> { 
    pub fn new(snap: &Snap, unit: U) -> Self {
        Self {
            swap: Arc::new(RwLock::new(Swap::new(snap, unit))),
            meta: Meta::new(),
        }
    }
    pub fn solve(&self, task: A) -> G { 
        let swap = self.swap.read().expect("the lock should not be poisoned");
        swap.node().unit.solve(task).expect("there should be a goal").clone()
    }
    pub fn read<F: FnOnce(&U)>(&self, read: F) { 
        let swap = self.swap.read().expect("the lock should not be poisoned"); 
        read(&swap.node().unit);
    }
    pub fn snap(&self) -> Snap { 
        let swap = self.swap.read().expect("the lock should not be poisoned"); 
        swap.snap().clone()
    }
}

impl<U, A, G> Clone for Edge<U, A, G> {
    fn clone(&self) -> Self {
        Self {
            swap: self.swap.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<U, A, G> Serialize for Edge<U, A, G> {
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

