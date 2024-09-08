use super::*;
#[cfg(not(feature = "oneThread"))]
use std::sync::{RwLock, Weak};
#[cfg(feature = "oneThread")]
use std::{cell::RefCell, rc::Weak};
use std::{collections::HashSet, hash::Hash};

pub type Result = std::result::Result<(), crate::Error>;

pub trait Rebut {
    /// Invalidate the tray. Call only after write during rebut phase.
    fn rebut(&self) -> crate::Result<Ring>;
}

pub trait RebutMut {
    /// Invalidatd the tray at cusp level. Call only after write during rebut phase.
    fn rebut(&mut self) -> crate::Result<Ring>;
}

pub trait React {
    /// Cause the unit to react. Call only on graph roots returned from the rebut phase.
    fn react(&self, id: &Id) -> react::Result;
}

pub trait ReactMut {
    /// Cause the unit to react. Call only on graph roots returned from the rebut phase.
    fn react(&mut self, id: &Id) -> react::Result;
}

pub trait AddRoot {
    /// Add a root to a cusp `Ring` of roots. Must be called after reading contents
    /// so that the cusp will react if contents change.
    fn add_root(&self, root: Root) -> crate::Result<()>;
}

pub trait AddRootMut {
    /// Add a root to a cusp `Ring` of roots. Must be called after reading contents
    /// so that the cusp will react if contents change.
    fn add_root(&mut self, root: Root);
}

pub trait Backed {
    /// Make a copy of the link that includes the provided cusp `&Back` on the edge.
    /// Must be called to include `&Back` in the rebut phase.
    fn backed(&self, back: &Back) -> crate::Result<Self> where Self: Sized;
}

pub trait BackedMid {
    /// Make a copy of the link that includes the provided cusp `&Back` on the edge.
    /// Must be called to include `&Back` in the rebut phase.
    fn backed(&self, back: &Back) -> Self;
}

/// For edge that Rebuts a Ring and reacts.
pub trait Update: Rebut + React + ToId + SendSync {}
impl<T> Update for T where T: Rebut + React + ToId + SendSync {}

/// For cusp to rebut a ring and react if the root of the rebut phase.
pub trait UpdateMut: RebutMut + ReactMut + ToId + SendSync {}
impl<T> UpdateMut for T where T: RebutMut + ReactMut + ToId + SendSync {}

/// Weakly point to a root edge, the inverse of Link.
/// A Cusp holds a Ring of Roots.
#[derive(Clone, Debug)]
pub struct Root {
    #[cfg(not(feature = "oneThread"))]
    pub edge: Weak<RwLock<dyn Update>>,
    #[cfg(feature = "oneThread")]
    pub edge: Weak<RefCell<dyn Update>>,
    pub id: Id,
}

impl Root {
    pub fn rebut(&self) -> crate::Result<Ring> {
        if let Some(edge) = self.edge.upgrade() {
            read_part(&edge, |edge| edge.rebut())?
        } else {
            Ok(Ring::new())
        }
    }
    pub fn react(&self, id: &Id) -> react::Result {
        if let Some(edge) = self.edge.upgrade() {
            read_part(&edge, |edge| edge.react(id))?
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

/// Weakly point to the back of a cusp as UpdateMid.
#[derive(Clone, Debug)]
pub struct Back {
    #[cfg(not(feature = "oneThread"))]
    pub cusp: Weak<RwLock<dyn UpdateMut>>,
    #[cfg(feature = "oneThread")]
    pub cusp: Weak<RefCell<dyn UpdateMut>>,
    pub id: Id,
}

impl Back {
    #[cfg(not(feature = "oneThread"))]
    pub fn new(cusp: Weak<RwLock<dyn UpdateMut>>, id: Id) -> Self {
        Self { cusp, id }
    }
    #[cfg(feature = "oneThread")]
    pub fn new(cusp: Weak<RefCell<dyn UpdateMut>>, id: Id) -> Self {
        Self { cusp, id }
    }
    pub fn rebut(&self) -> crate::Result<Ring> {
        if let Some(cusp) = self.cusp.upgrade() {
            write_part(&cusp, |mut cusp| cusp.rebut())?
        } else {
            Ok(Ring::new())
        }
    }
    pub fn react(&self, id: &Id) -> react::Result {
        if let Some(cusp) = self.cusp.upgrade() {
            write_part(&cusp, |mut cusp| cusp.react(id))?
        } else {
            Ok(())
        }
    }
}

impl Deal for Back {
    fn one(&mut self, _: &str, view: View) -> crate::Result<()> {
        let apex = view.apex();
        view.set(apex.backed(self)?)?;
        Ok(())
    }
    fn vec(&mut self, _: &str, view: ViewVec) -> crate::Result<()> {
        // TODO: make ViewVec::back to replace this
        for view in view.views() {
            let apex = view.apex();
            view.set(apex.backed(self)?)?;
        }
        Ok(())
    }
    fn map(&mut self, map: &mut Map) -> crate::Result<()> {
        *map = map.backed(self)?;
        Ok(())
    }
}

/// Points to many root edges, each pointing to back of a cusp.
#[derive(Default, Debug)]
pub struct Ring {
    roots: HashSet<Root>,
}

impl Ring {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_root(&mut self, root: Root) {
        self.roots.insert(root);
    }
    pub fn rebut(&mut self) -> crate::Result<Ring> {
        let mut result = Ring::new();
        for root in &self.roots {
            let ring = root.rebut()?;
            if ring.roots.is_empty() {
                result.roots.insert(root.clone());
            } else {
                result.roots.extend(ring.roots);
            }
        }
        self.roots.clear();
        Ok(result)
    }
    pub fn rebut_roots(&mut self) -> crate::Result<Vec<Root>> {
        let mut ring = Ring::new();
        for root in &self.roots {
            ring.roots.extend(root.rebut()?.roots);
        }
        self.roots.clear();
        Ok(Vec::from_iter(ring.roots))
    }
}
