use super::*;
use std::{collections::HashSet, hash::Hash};

#[cfg(not(feature = "oneThread"))]
type WeakPointer<T> = std::sync::Weak<parking_lot::RwLock<T>>;
#[cfg(feature = "oneThread")]
type WeakPointer<T> = std::rc::Weak<std::cell::RefCell<T>>;

pub trait Rebut {
    /// Invalidate the tray. Call only after write during rebut phase.
    fn rebut(&self) -> Result<Ring>;
    fn clear_roots(&self) -> Result<()>;
}

pub trait RebutMut {
    /// Invalidatd the tray at cusp level. Call only after write during rebut phase.
    fn rebut(&mut self) -> Result<Ring>;
    fn clear_roots(&mut self) -> Result<()>;
}

pub trait React {
    /// Cause the unit to react. Call only on graph roots returned from the rebut phase.
    fn react(&self) -> GraphFuture<Result<()>>;
}

pub trait ReactMut {
    /// Cause the unit to react. Call only on graph roots returned from the rebut phase.
    fn react(&mut self) -> GraphFuture<Result<()>> {
        Box::pin(async move { Ok(()) })
    }
}

pub trait AddRoot {
    /// Add a root to a cusp `Ring` of roots. Must be called after reading contents
    /// so that the cusp will react if contents change.
    fn add_root(&mut self, root: &Option<Root>);
}

pub trait Backed {
    /// Make a copy of the link that includes the provided cusp `&Back` on the edge.
    /// Must be called to include `&Back` in the rebut phase.
    fn backed(&self, back: &Back) -> Result<Self>
    where
        Self: Sized;
}

pub trait BackedMid {
    // type Cusp;
    /// Make a copy of the link that includes the provided cusp `&Back` on the edge.
    /// Must be called to include `&Back` in the rebut phase.
    fn backed(&self, back: &Back) -> Pointer<Self>;
}

/// For edge that Rebuts a Ring and reacts.
pub trait Update: Rebut + React + SendSync {}
impl<T> Update for T where T: Rebut + React + SendSync {}
// impl<T> Update for Box<T> where T: Rebut + React + SendSync {}

/// For cusp to rebut a ring and react if the root of the rebut phase.
pub trait UpdateMut: RebutMut + ReactMut + SendSync {}
impl<T> UpdateMut for T where T: RebutMut + ReactMut + SendSync {}

/// Weakly point to a root edge, the inverse of Link.
/// A Cusp holds a Ring of Roots.
#[derive(Clone, Debug)]
pub struct Root {
    pub edge: WeakPointer<dyn Update>,
    pub id: Id,
}

impl Root {
    pub fn rebut(&self) -> Result<Ring> {
        if let Some(edge) = self.edge.upgrade() {
            read_part(&edge, |edge| edge.rebut())?
        } else {
            Ok(Ring::new())
        }
    }
    fn clear(&self) -> Result<()> {
        if let Some(edge) = self.edge.upgrade() {
            read_part(&edge, |edge| edge.clear_roots())??
        }
        Ok(())
    }
}

impl Eq for Root {}

impl PartialEq for Root {
    fn eq(&self, other: &Self) -> bool {
        #[cfg(not(feature = "oneThread"))]
        let eq = std::sync::Weak::ptr_eq(&self.edge, &other.edge); // && self.id == other.id
        #[cfg(feature = "oneThread")]
        let eq = std::rc::Weak::ptr_eq(&self.edge, &other.edge);
        eq
    }
}

impl Hash for Root {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl React for Root {
    fn react(&self) -> GraphFuture<Result<()>> {
        Box::pin(async move {
            if let Some(edge) = self.edge.upgrade() {
                read_part_async(&edge, |edge| async move { edge.react().await })?.await
            } else {
                Ok(())
            }
        })
    }
}

/// Weakly point to the back of a cusp as UpdateMid.
#[derive(Clone, Debug)]
pub struct Back {
    pub cusp: WeakPointer<dyn UpdateMut>,
    pub id: Id,
}

impl Back {
    pub fn rebut(&self) -> Result<Ring> {
        if let Some(cusp) = self.cusp.upgrade() {
            write_part(&cusp, |mut cusp| cusp.rebut())?
        } else {
            Ok(Ring::new())
        }
    }
    pub fn clear(&self) -> Result<()> {
        if let Some(cusp) = self.cusp.upgrade() {
            write_part(&cusp, |mut cusp| cusp.clear_roots())?
        } else {
            Ok(())
        }
    }
    pub async fn react(&self) -> Result<()> {
        if let Some(cusp) = self.cusp.upgrade() {
            write_part_async(&cusp, |mut cusp| async move { cusp.react().await })?.await
        } else {
            Ok(())
        }
    }
}

impl Deal for Back {
    fn one(&mut self, _: &str, view: View) -> Result<()> {
        let apex = view.backed(self)?;
        view.set(apex)?;
        Ok(())
    }
    fn vec(&mut self, _: &str, view: ViewVec) -> Result<()> {
        for view in view.views() {
            let apex = view.backed(self)?;
            view.set(apex)?;
        }
        Ok(())
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
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
    pub async fn react(&self) -> Result<()> {
        for root in &self.roots {
            root.react().await?;
        }
        Ok(())
    }
    pub fn extend(&mut self, ring: Ring) {
        self.roots.extend(ring.roots);
    }
    pub fn add_root(&mut self, root: Root) {
        self.roots.insert(root);
    }
    pub fn rebut(&mut self) -> Result<Ring> {
        let mut out = Ring::new();
        for root in &self.roots {
            let ring = root.rebut()?;
            if ring.roots.is_empty() {
                out.roots.insert(root.clone());
            } else {
                out.roots.extend(ring.roots);
            }
        }
        // self.roots.clear();
        Ok(out)
    }
    pub fn root_rebut(&mut self) -> Result<Ring> {
        let mut ring = Ring::new();
        for root in &self.roots {
            ring.roots.extend(root.rebut()?.roots);
        }
        self.clear()?;
        // self.roots.clear();
        Ok(ring)
    }
    pub fn clear(&mut self) -> Result<()> {
        for root in &self.roots {
            root.clear()?;
        }
        self.roots.clear();
        Ok(())
    }
}
