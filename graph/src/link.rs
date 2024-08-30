use anyhow::anyhow;
pub use leaf::*;

use super::*;
#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock};
#[cfg(feature = "oneThread")]
use std::{cell::RefCell, rc::Rc};
use std::{
    fmt,
    hash::{Hash, Hasher},
};

mod leaf;
#[cfg(test)]
mod tests;

/// `Link` to `Tray`.
pub type Leaf = Link<edge::Leaf>;

/// `Link` to domain-specific unit.
/// The unit type is intact. For type-erased unit, use `Ploy` instead.
pub type Node<U> = Link<edge::Node<U>>;

/// `Link` to `Edge`, pointing to `Cusp`, containing work unit.
/// Unit fields often contain `Link`, creating a graph pattern.
// #[derive(Debug)]
pub struct Link<E> {
    #[cfg(not(feature = "oneThread"))]
    edge: Arc<RwLock<E>>,
    #[cfg(feature = "oneThread")]
    edge: Rc<RefCell<E>>,
    path: Option<Path>,
    rank: Option<usize>,
}

impl<E> fmt::Debug for Link<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Path: {:?}", self.path))
    }
}

impl<E> Link<E> {
    pub fn pathed(&self, path: Path) -> Self {
        Self {
            edge: self.edge.clone(),
            path: Some(path),
            rank: self.rank,
        }
    }
    pub fn path(&self) -> Option<&Path> {
        self.path.as_ref()
    }
    pub fn rank(&self) -> Option<usize> {
        self.rank
    }
    pub fn ranked(&self, rank: usize) -> Self {
        Self {
            edge: self.edge.clone(),
            path: self.path.clone(),
            rank: Some(rank),
        }
    }
}

impl<E> Link<E>
where
    Self: Solve,
{
    pub fn main(&self) -> Result<Apex> {
        match self.solve(Task::Main)? {
            Gain::Apex(apex) => Ok(apex),
            _ => Err(anyhow!("Wrong return type for Task::Main."))?,
        }
    }
    pub fn act(&self) -> Result<()> {
        match self.solve(Task::None) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

impl<E> Hash for Link<E>
where
    Self: Solve,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Ok(Gain::U64(digest)) = self.solve(Task::Hash) {
            digest.hash(state)
        } else {
            // TODO: Remove when sure that this won't be a problem
            panic!("failed to hash link")
        }
    }
}

impl<E> Serialize for Link<E>
where
    Self: Solve,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(path) = &self.path {
            path.serialize(serializer)
        } else if let Ok(Gain::U64(hash)) = self.solve(Task::Hash) {
            Path::Hash(hash).serialize(serializer)
        } else {
            // TODO: Remove when sure that this won't be a problem
            panic!("failed to serialize link")
            // self.path.serialize(serializer)
        }
    }
}

impl<E> Link<E>
where
    E: FromItem,
{
    pub fn new(unit: E::Item) -> Self {
        let edge = E::new(unit);
        Self {
            path: None,
            rank: None,
            #[cfg(not(feature = "oneThread"))]
            edge: Arc::new(RwLock::new(edge)),
            #[cfg(feature = "oneThread")]
            edge: Rc::new(RefCell::new(edge)),
        }
    }
}

impl<E> Link<E>
where
    E: Make,
{
    pub fn make<F: FnOnce(&Back) -> E::Unit>(make: F) -> Self {
        let edge = E::make(make);
        Self {
            path: None,
            rank: None,
            #[cfg(not(feature = "oneThread"))]
            edge: Arc::new(RwLock::new(edge)),
            #[cfg(feature = "oneThread")]
            edge: Rc::new(RefCell::new(edge)),
        }
    }
}

impl<E> Link<E>
where
    E: 'static + Make + Engage,
{
    pub fn make_ploy<F: FnOnce(&Back) -> E::Unit>(make: F) -> Ploy {
        let edge = E::make(make);
        Link {
            path: None,
            rank: None,
            #[cfg(not(feature = "oneThread"))]
            edge: Arc::new(RwLock::new(Box::new(edge) as Box<dyn Engage>)),
            #[cfg(feature = "oneThread")]
            edge: Rc::new(RefCell::new(Box::new(edge) as Box<dyn Engage>)),
        }
    }
}

impl<E> Link<E>
where
    E: ToPloy,
{
    /// Copy the link with unit type erased.  
    pub fn ploy(&self) -> Result<Ploy> {
        read_part(&self.edge, |edge| Ploy {
            edge: edge.ploy(),
            path: self.path.clone(),
            rank: self.rank,
        })
    }
}

impl<E> Link<E>
where
    E: 'static + FromSnap + Engage,
{
    pub fn make_ploy_from_snap(snap: Snap<E::Unit>) -> Ploy {
        let edge = E::from_snap(snap);
        Link {
            path: None,
            rank: None,
            #[cfg(not(feature = "oneThread"))]
            edge: Arc::new(RwLock::new(Box::new(edge) as Box<dyn Engage>)),
            #[cfg(feature = "oneThread")]
            edge: Rc::new(RefCell::new(Box::new(edge) as Box<dyn Engage>)),
        }
    }
}

impl<E> Link<E>
where
    E: Read<Payload = Tray>,
{
    pub fn tray(&self) -> Result<Tray> {
        read_part(&self.edge, |edge| edge.read(|tray| tray.clone()))?
    }
}

impl<E> Link<E>
where
    E: 'static + Update,
{
    #[cfg(not(feature = "oneThread"))]
    pub fn as_root(&self, id: Id) -> Root {
        let edge = self.edge.clone() as Arc<RwLock<dyn Update>>;
        Root {
            edge: Arc::downgrade(&edge),
            id,
        }
    }
    #[cfg(feature = "oneThread")]
    pub fn as_root(&self, id: Id) -> Root {
        let edge = self.edge.clone() as Rc<RefCell<dyn Update>>;
        Root {
            edge: Rc::downgrade(&edge),
            id,
        }
    }
}

impl<E> Clone for Link<E> {
    fn clone(&self) -> Self {
        Self {
            edge: self.edge.clone(),
            path: self.path.clone(),
            rank: self.rank,
        }
    }
}

impl<E> PartialEq for Link<E> {
    #[cfg(not(feature = "oneThread"))]
    fn eq(&self, other: &Self) -> bool {
        Arc::<RwLock<E>>::ptr_eq(&self.edge, &other.edge)
            && self.path == other.path
            && self.rank == other.rank
    }
    #[cfg(feature = "oneThread")]
    fn eq(&self, other: &Self) -> bool {
        Rc::<RefCell<E>>::ptr_eq(&self.edge, &other.edge)
            && self.path == other.path
            && self.rank == other.rank
    }
}

/// TODO: make method to make new link with cloned edge without Back!
impl<E> Backed for Link<E>
where
    E: Backed,
{
    #[cfg(not(feature = "oneThread"))]
    fn backed(&self, back: &Back) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: Arc::new(RwLock::new(edge.backed(back))),
            path: self.path.clone(),
            rank: self.rank,
        }
    }
    #[cfg(feature = "oneThread")]
    fn backed(&self, back: &Back) -> Self {
        let edge = self.edge.borrow();
        Self {
            edge: Rc::new(RefCell::new(edge.backed(back))),
            path: self.path.clone(),
            rank: self.rank,
        }
    }
}

/// TODO: make reader that does not add a root to the cusp.
/// This will allow readers to inspect without rebuting in the future.
impl<E> Link<E> 
where
    E: 'static + Read + Update + AddRoot,
{
    /// Read payload of Link.
    pub fn read<T, F: FnOnce(&E::Payload) -> T>(&self, read: F) -> Result<T> {
        read_part(&self.edge, |edge| {
            let out = edge.read(read);
            edge.add_root(self.as_root(edge.id()));
            out
        })?
    }
}

impl<E> WriteTray for Link<E>
where
    E: WriteTray,
{
    fn write<T, F: FnOnce(&mut Tray) -> T>(&self, write: F) -> Result<T> {
        read_part(&self.edge, |edge| edge.write(write))?
    }
}

impl<E> WriteUnit for Link<E>
where
    E: WriteUnit,
{
    type Unit = E::Unit;
    fn write<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(
        &self,
        write: F,
    ) -> Result<T> {
        read_part(&self.edge, |edge| edge.write(write))?
    }
}

impl<E> Solve for Link<E>
where
    E: 'static + Solve + AddRoot + Update,
{
    fn solve(&self, task: Task) -> solve::Result {
        read_part(&self.edge, |edge| {
            let result = edge.solve(task);
            edge.add_root(self.as_root(edge.id()));
            result
        })?
    }
}

impl<E> AdaptMid for Link<E>
where
    E: AdaptMid,
{
    fn adapt(&self, post: Post) -> adapt::Result {
        read_part(&self.edge, |edge| edge.adapt(post))?
    }
}

impl TryBacked for Ploy {
    type Out = Ploy;
    fn backed(&self, back: &Back) -> Result<Self::Out> {
        read_part(&self.edge, |edge| Self {
            edge: edge.backed_ploy(back),
            path: self.path.clone(),
            rank: self.rank,
        })
    }
}

impl<T> Backed for Vec<T>
where
    T: Backed,
{
    fn backed(&self, back: &Back) -> Self {
        self.iter().map(|link| link.backed(back)).collect()
    }
}
