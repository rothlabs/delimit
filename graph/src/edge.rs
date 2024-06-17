use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::{Solve, Id, Node, Read, Snap, Swap, Write};

pub type LeafStr = Edge<String, (), ()>;

// TODO: type params: Unit, Args, Gain (U, A, G)
pub struct Edge<U, A, G> {
    swap: Arc<RwLock<Swap<U, A, G>>>,  
    meta: Meta,
}

impl<U: Solve<A, G> + Clone + Serialize, A, G> Edge<U, A, G> { 
    pub fn new(snap: &Snap, unit: U) -> Self {
        Self {
            swap: Arc::new(RwLock::new(Swap::new(snap, unit))),
            meta: Meta::new(),
        }
    }
    pub fn solve(&self, arg: A) -> Option<G> { // , func: &dyn Fn(&U) -> G
        let node = self.swap.read().expect("the lock should not be poisoned"); //.read(snap);
        node.get().unit.solve(arg)
        //func(&node.read().unit)
        //self.swap.read().expect("the lock should not be poisoned").read(snap)
        //Read::new(self.swap.read().expect("the lock should not be poisoned").read(snap))
    }
    pub fn unit(&self) -> &U {
        let node = self.swap.read().expect("the lock should not be poisoned"); //.read(snap);
        &node.unit
        //self.swap.read().expect("the lock should not be poisoned").read(snap)
        //Read::new(self.swap.read().expect("the lock should not be poisoned").read(snap))
    }
    // pub fn read(&self, func: &dyn Fn(&U) -> G) -> G {
    //     let node = self.swap.read().expect("the lock should not be poisoned"); //.read(snap);
    //     func(&node.read().unit)
    //     //self.swap.read().expect("the lock should not be poisoned").read(snap)
    //     //Read::new(self.swap.read().expect("the lock should not be poisoned").read(snap))
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
