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

pub type Result = std::result::Result<(), String>;

pub trait Rebut {
    /// Invalidate the load. Call only after write during rebut phase.
    fn rebut(&self) -> Ring;
}

pub trait DoRebut {
    /// Invalidatd the load at apex level. Call only after write during rebut phase.
    fn do_rebut(&mut self) -> Ring;
}

pub trait React {
    /// Cause the unit to react. Call only on graph roots returned from the rebut phase.
    fn react(&self, meta: &Meta) -> react::Result;
}

pub trait DoReact {
    /// Cause the unit to react. Call only on graph roots returned from the rebut phase.
    fn do_react(&mut self, meta: &Meta) -> react::Result;
}

pub trait AddRoot {
    /// Add a root to a apex `Ring` of roots. Must be called after reading contents
    /// so that the apex will react if contents change.
    fn add_root(&self, root: Root);
}

pub trait DoAddRoot {
    /// Add a root to a apex `Ring` of roots. Must be called after reading contents
    /// so that the apex will react if contents change.
    fn do_add_root(&mut self, root: Root);
}

pub trait Backed {
    /// Make a copy of the link that includes the provided apex `&Back` on the edge.
    /// Must be called to include `&Back` in the rebut phase.
    fn backed(&self, back: &Back) -> Self;
}

#[cfg(not(feature = "oneThread"))]
type PloyEdge<L> = Arc<RwLock<Box<dyn Produce<L>>>>;
#[cfg(feature = "oneThread")]
type PloyEdge<L> = Rc<RefCell<Box<dyn Produce<L>>>>;

pub trait ToPloy {
    type Load;
    /// Copy with unit type erased.  
    fn ploy(&self) -> PloyEdge<Self::Load>;
}

pub trait ToPipedPloy {
    type Load;
    /// Copy with unit type erased.  
    fn ploy(&self) -> PloyEdge<Self::Load>;
}

pub trait BackedPloy {
    type Load;
    fn backed_ploy(&self, back: &Back) -> PloyEdge<Self::Load>;
}

/// For edge that Rebuts a Ring and reacts.
pub trait Update: Rebut + React + SendSync {}

/// For apex that mutably Rebuts a Ring and reacts.
pub trait DoUpdate: DoRebut + DoReact + SendSync {}

/// Weakly point to a root edge, the inverse of Link.
/// A Apex holds a Ring of Roots.
#[derive(Clone)]
pub struct Root {
    #[cfg(not(feature = "oneThread"))]
    pub edge: Weak<RwLock<dyn Update>>,
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
    pub fn react(&self, meta: &Meta) -> react::Result {
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

/// Weakly point to the back of a apex as DoUpdate.
#[derive(Clone)]
pub struct Back {
    #[cfg(not(feature = "oneThread"))]
    pub apex: Weak<RwLock<dyn DoUpdate>>,
    #[cfg(feature = "oneThread")]
    pub apex: Weak<RefCell<dyn DoUpdate>>,
}

impl Back {
    #[cfg(not(feature = "oneThread"))]
    pub fn new(apex: Weak<RwLock<dyn DoUpdate>>) -> Self {
        Self { apex }
    }
    #[cfg(feature = "oneThread")]
    pub fn new(apex: Weak<RefCell<dyn DoUpdate>>) -> Self {
        Self { apex }
    }
    pub fn rebut(&self) -> Ring {
        if let Some(apex) = self.apex.upgrade() {
            write_part(&apex, |mut apex| apex.do_rebut())
        } else {
            Ring::new()
        }
    }
    pub fn react(&self, meta: &Meta) -> react::Result {
        if let Some(apex) = self.apex.upgrade() {
            write_part(&apex, |mut apex| apex.do_react(meta))
        } else {
            Ok(())
        }
    }
}

/// Points to many root edges, each pointing to back of a apex.
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
