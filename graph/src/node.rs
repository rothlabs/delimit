use std::{cell::RefCell, collections::HashMap, hash::Hash, sync::{Arc, RwLock, Weak}};

use serde::Serialize;
// use dyn_clone::{clone_trait_object, DynClone};
// /use erased_serde::{serialize_trait_object, Serialize as DynSerialize};

use crate::{Id, Solve};

const GOAL: &str = "there should be a goal";

//pub trait Task: Clone + Eq + PartialEq + Hash  {}
//impl Task for () {}

// Multiple Nodes can point to the same Unit.
// Pointers to Unit should be serialized as hash digest of Unit.
// Each Unit should be serialized once along side their hash digest.

pub struct Node<U, T, G>(pub Arc<RwLock<Base<U, T, G>>>);

impl<U: Solve<T, G>, T: Clone + Eq + PartialEq + Hash, G: Clone> Node<U, T, G> {
    pub fn new(unit: U) -> Self {
        Self(Arc::new(RwLock::new(Base::new(unit))))
    }
}

#[derive(Clone, Serialize)]
pub struct Base<U, T, G> {
    pub unit: U,
    pub work: HashMap<T, G>,
    pub meta: Meta,
    #[serde(skip)]
    pub roots: Vec<Weak<RwLock<dyn Root>>>,
}

impl<U: Solve<T, G>, T: Clone + Eq + PartialEq + Hash, G: Clone> Base<U, T, G> {
    pub fn new(unit: U) -> Self {
        Self {
            unit,
            work: HashMap::new(),
            meta: Meta::new(),
            roots: vec![],
        }
    }
    pub fn solve(&mut self, task: T) -> G {
        if let Some(goal) = self.work.get(&task) {
            goal.clone()
        } else {
            let goal = self.unit.solve(task.clone()).expect(GOAL);
            self.work.insert(task, goal.clone());
            goal
        }
    }
    // pub fn flatten(&self, flat: &mut Flat) { // , state: &mut Hasher
    //     let unit = serde_json::to_string(&self.read()).unwrap();
    // }
    // TODO: pub fn duplicate(&self) -> Node<U>  // clone and set new ID
}

// impl Flatten for String {
//     fn flatten(&self, flat: &mut Flat) { // , state: &mut Hasher
//         flat.units.in
//     }
// }

impl<U, A, G> PartialEq for Base<U, A, G> {
    fn eq(&self, rhs: &Base<U, A, G>) -> bool {
        self.meta.node.id == rhs.meta.node.id
    }
}

//clone_trait_object!(Root);
pub trait Root: { //DynClone {
    fn clear_work(&mut self);
}

impl<U, T, G> Root for Base<U, T, G> {
    fn clear_work(&mut self) {
        self.work.clear();
        for root in self.roots.iter() {
            if let Some(root) = root.upgrade() { 
                if let Ok(root) = &mut root.write() {
                    root.clear_work();
                }
            } // TODO: collect indices of dropped roots to remove from vec (do the same for poisoned?)
        }
    }
}

pub trait Stem {
    
}


#[derive(Clone, Serialize)]
pub struct Meta {
    pub node: meta::Node,
    //snap: meta::Snap,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            node: meta::Node { id: Id::new() },
            //snap: meta::Snap{}
        }
    }
}

mod meta {
    use std::sync::Weak;

    use serde::Serialize;

    use crate::Id;

    #[derive(Clone, Serialize)]
    pub struct Node {
        pub id: Id,
    }

    #[derive(Clone)]
    pub struct Snap(Weak<crate::Snap>);
}
