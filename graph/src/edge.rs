use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::{Solve, Id, Swap};

// TODO: type params: Unit, Args, Gain (U, A, G)
pub struct Edge<U, A, G> {
    swap: Arc<RwLock<Swap<U, A, G>>>,  
    meta: Meta,
}

impl<U: Solve<A, G> + Clone + Serialize, A, G: Clone> Edge<U, A, G> { 
    pub fn new(unit: U) -> Self {
        Self {
            swap: Arc::new(RwLock::new(Swap::new(unit))),
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
    pub fn write<F: FnOnce(&mut U)>(&self, write: F) { 
        let mut swap = self.swap.write().expect("the lock should not be poisoned"); 
        write(&mut swap.get_mut().unit);
    }
    // pub fn snap(&self) -> Snap { 
    //     let swap = self.swap.read().expect("the lock should not be poisoned"); 
    //     swap.snap().clone()
    // }
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

