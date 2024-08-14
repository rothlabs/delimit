use super::*;
#[cfg(not(feature = "oneThread"))]
use std::sync::{RwLock, Weak};
#[cfg(feature = "oneThread")]
use std::{cell::RefCell, rc::Weak};
use std::{collections::HashSet, hash::Hash};

pub type Result = std::result::Result<(), Error>;

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
    fn react(&self, id: &Id) -> react::Result;
}

pub trait DoReact {
    /// Cause the unit to react. Call only on graph roots returned from the rebut phase.
    fn do_react(&mut self, id: &Id) -> react::Result;
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

pub trait ToPloy {
    /// Copy with unit type erased.  
    fn ploy(&self) -> PloyEdge;
}

pub trait ToPipedPloy {
    /// Copy with unit type erased.  
    fn ploy(&self) -> PloyEdge;
}

pub trait BackedPloy {
    fn backed_ploy(&self, back: &Back) -> PloyEdge;
}

/// For edge that Rebuts a Ring and reacts.
pub trait Update: Rebut + React + ToId + SendSync {}

/// For apex to rebut a ring and react if the root of the rebut phase.
pub trait DoUpdate: DoRebut + DoReact + ToId + SendSync {}

/// Weakly point to a root edge, the inverse of Link.
/// A Apex holds a Ring of Roots.
#[derive(Clone, Debug)]
pub struct Root {
    #[cfg(not(feature = "oneThread"))]
    pub edge: Weak<RwLock<dyn Update>>,
    #[cfg(feature = "oneThread")]
    pub edge: Weak<RefCell<dyn Update>>,
    pub id: Id,
}

impl Root {
    pub fn rebut(&self) -> Ring {
        if let Some(edge) = self.edge.upgrade() {
            read_part(&edge, |edge| edge.rebut())
        } else {
            Ring::new()
        }
    }
    pub fn react(&self, id: &Id) -> react::Result {
        if let Some(edge) = self.edge.upgrade() {
            read_part(&edge, |edge| edge.react(id))
        } else {
            Ok(())
        }
    }
}

impl Eq for Root {}

impl PartialEq for Root {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.edge, &other.edge) // && self.id == other.id
    }
}

impl Hash for Root {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

/// Weakly point to the back of a apex as DoUpdate.
#[derive(Clone, Debug)]
pub struct Back {
    #[cfg(not(feature = "oneThread"))]
    pub apex: Weak<RwLock<dyn DoUpdate>>,
    #[cfg(feature = "oneThread")]
    pub apex: Weak<RefCell<dyn DoUpdate>>,
    pub id: Id,
}

impl Back {
    #[cfg(not(feature = "oneThread"))]
    pub fn new(apex: Weak<RwLock<dyn DoUpdate>>, id: Id) -> Self {
        Self { apex, id }
    }
    #[cfg(feature = "oneThread")]
    pub fn new(apex: Weak<RefCell<dyn DoUpdate>>, id: Id) -> Self {
        Self { apex, id }
    }
    pub fn rebut(&self) -> Ring {
        if let Some(apex) = self.apex.upgrade() {
            write_part(&apex, |mut apex| apex.do_rebut())
        } else {
            Ring::new()
        }
    }
    pub fn react(&self, id: &Id) -> react::Result {
        if let Some(apex) = self.apex.upgrade() {
            write_part(&apex, |mut apex| apex.do_react(id))
        } else {
            Ok(())
        }
    }
}

/// Points to many root edges, each pointing to back of a apex.
#[derive(Default, Debug)]
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
    pub fn rebut_roots(&mut self) -> Vec<Root> {
        let mut ring = Ring::new();
        for root in &self.roots {
            ring.roots.extend(root.rebut().roots);
        }
        self.roots.clear();
        Vec::from_iter(ring.roots)
    }
}
