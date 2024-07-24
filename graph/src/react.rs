use crate::*;

use std::{collections::HashSet, hash::Hash};

#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock, Weak};
#[cfg(feature = "oneThread")]
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::Meta;

pub trait Rebut {
    fn rebut(&self) -> Ring;
}

pub trait DoRebut {
    fn do_rebut(&mut self) -> Ring;
}

pub type ReactResult = Result<(), String>;

pub trait React {
    fn react(&self, meta: &Meta) -> ReactResult;
}

pub trait DoReact {
    fn do_react(&mut self, meta: &Meta) -> ReactResult;
}

pub trait AddRoot {
    fn add_root(&self, root: Root);
}

pub trait DoAddRoot {
    fn do_add_root(&mut self, root: Root);
}

pub trait Backed {
    fn backed(&self, back: &Back) -> Self;
}

#[cfg(not(feature = "oneThread"))]
type PloyEdge<L> = Arc<RwLock<Box<dyn Produce<L>>>>;
#[cfg(feature = "oneThread")]
type PloyEdge<L> = Rc<RefCell<Box<dyn Produce<L>>>>;

pub trait ToPloy {
    type Load;
    fn ploy(&self) -> PloyEdge<Self::Load>;
}

pub trait BackedPloy {
    type Load;
    fn backed_ploy(&self, back: &Back) -> PloyEdge<Self::Load>;
}

#[cfg(not(feature = "oneThread"))]
type PlanEdge<T, L> = Arc<RwLock<Box<dyn Convert<T, L>>>>;
#[cfg(feature = "oneThread")]
type PlanEdge<T, L> = Rc<RefCell<Box<dyn Convert<T, L>>>>;

pub trait ToPlan {
    type Task;
    type Load;
    fn plan(&self) -> PlanEdge<Self::Task, Self::Load>;
}

pub trait BackedPlan {
    type Task;
    type Load;
    fn backed_plan(&self, back: &Back) -> PlanEdge<Self::Task, Self::Load>;
}

/// For edge that Rebuts a Ring and reacts.
pub trait Update: Rebut + React + SendSync {}

/// For node that mutably Rebuts a Ring and reacts.
pub trait DoUpdate: DoRebut + DoReact + SendSync {}

/// Weakly point to a root edge, the inverse of Link.
/// Should meta be removed?
#[derive(Clone)]
pub struct Root {
    #[cfg(not(feature = "oneThread"))]
    pub edge: Weak<RwLock<dyn Update>>, //  + Send + Sync
    #[cfg(feature = "oneThread")]
    pub edge: Weak<RefCell<dyn Update>>,
    pub meta: Meta,
}

impl Root {
    pub fn rebut(&self) -> Ring {
        if let Some(edge) = self.edge.upgrade() {
            read_part(&edge, |edge| edge.rebut())
        } else {
            Ring::new()
        }
    }
    pub fn react(&self, meta: &Meta) -> ReactResult {
        if let Some(edge) = self.edge.upgrade() {
            read_part(&edge, |edge| edge.react(meta))
        } else {
            Ok(())
        }
    }
}

impl Eq for Root {}

impl PartialEq for Root {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.edge, &other.edge) && self.meta.id == other.meta.id
    }
}

impl Hash for Root {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.meta.id.hash(state);
    }
}

/// Weakly point to the back of a node as Update.
#[derive(Clone)]
pub struct Back {
    #[cfg(not(feature = "oneThread"))]
    pub node: Weak<RwLock<dyn DoUpdate>>,
    #[cfg(feature = "oneThread")]
    pub node: Weak<RefCell<dyn DoUpdate>>,
}

impl Back {
    #[cfg(not(feature = "oneThread"))]
    pub fn new(node: Weak<RwLock<dyn DoUpdate>>) -> Self {
        Self { node }
    }
    #[cfg(feature = "oneThread")]
    pub fn new(node: Weak<RefCell<dyn DoUpdate>>) -> Self {
        Self { node }
    }
    pub fn rebut(&self) -> Ring {
        if let Some(node) = self.node.upgrade() {
            write_part(&node, |mut node| node.do_rebut())
        } else {
            Ring::new()
        }
    }
    pub fn react(&self, meta: &Meta) -> ReactResult {
        if let Some(node) = self.node.upgrade() {
            write_part(&node, |mut node| node.do_react(meta))
        } else {
            Ok(())
        }
    }
}

/// Points to many root edges, each pointing to back of a node.
#[derive(Default)]
pub struct Ring {
    roots: HashSet<Root>,
}

impl Ring {
    // TODO: make method to remove reactors with dropped edges
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_root(&mut self, root: Root) {
        self.roots.insert(root);
    }
    pub fn rebut(&mut self) -> Ring {
        let mut result = Ring::new();
        for root in &self.roots {
            let ring = root.rebut();
            if ring.roots.is_empty() {
                result.roots.insert(root.clone());
            } else {
                result.roots.extend(ring.roots);
            }
        }
        self.roots.clear();
        result
    }
    pub fn rebut_roots(&mut self, meta: &Meta) -> (Vec<Root>, Meta) {
        let mut ring = Ring::new();
        for root in &self.roots {
            ring.roots.extend(root.rebut().roots);
        }
        self.roots.clear();
        (Vec::from_iter(ring.roots), meta.clone())
    }
}

// pub fn rebut_this(&mut self) -> Ring {
//     let mut result = Ring::new();
//     for root in &self.roots {
//         let ring = root.rebut();
//         if ring.roots.is_empty() {
//             result.roots.insert(root.clone());
//         } else {
//             result.roots.extend(ring.roots);
//         }
//     }
//     self.roots.clear();
//     result
// }
// pub fn cycle(&mut self, meta: &Meta) {
//     let mut ring = Ring::new();
//     for root in &self.roots {
//         ring.roots.extend(root.rebut().roots);
//     }
//     //let ring = self.rebut_this();
//     for root in &ring.roots {
//         root.react(meta);
//     }
//     self.roots.clear();
// }

// #[cfg(not(feature="oneThread"))]
// pub trait Update: Rebut + React + Send + Sync {}
// #[cfg(feature="oneThread")]
// pub trait Update: Rebut + React {}

// #[cfg(not(feature="oneThread"))]
// pub trait DoUpdate: DoRebut + DoReact + Send + Sync {}
// #[cfg(feature="oneThread")]
// pub trait DoUpdate: DoRebut + DoReact {}
