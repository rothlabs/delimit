pub use leaf::{IntoLeaf, ToLeaf};

use super::*;
#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock};
#[cfg(feature = "oneThread")]
use std::{cell::RefCell, rc::Rc};

mod leaf;
#[cfg(test)]
mod tests;

/// Link to a load. The most simple graph part.
pub type Leaf = Link<edge::Leaf>;

/// Link to a unit that grants a load.
pub type Agent<U> = Link<edge::Agent<U>>;

/// Link that grants a load.
pub type Ploy = Link<Box<dyn Engage>>;

/// Link to an edge that leads to a apex that contains a unit.
/// Units hold links as source of input used to compute output.
#[derive(Debug)]
pub struct Link<E> {
    #[cfg(not(feature = "oneThread"))]
    edge: Arc<RwLock<E>>,
    #[cfg(feature = "oneThread")]
    edge: Rc<RefCell<E>>,
    /// TODO: rename to Path
    path: Path,
    rank: Option<usize>,
}

impl<E> Serialize for Link<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.path.serialize(serializer)
    }
}

impl<E> Link<E> {
    pub fn path(&self) -> Path {
        self.path.clone()
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
    pub fn main(&self) -> node::Result {
        match self.solve(Task::Main)? {
            Tray::Node(node) => Ok(node),
            _ => Err("not Tray::Node".into()),
        }
    }
    // pub fn serial(&self) -> serial::Result {
    //     match self.solve(Task::Serial)? {
    //         Tray::String(string) => Ok(string),
    //         _ => Err("not Tray::String".into()),
    //     }
    // }
}

impl<E> Link<E>
where
    E: FromItem,
{
    pub fn new(unit: E::Item) -> Self {
        let edge = E::new(unit);
        Self {
            path: Path::None,
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
            path: Path::None,
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
    E: ToLoad,
{
    pub fn load(&self) -> E::Load {
        read_part(&self.edge, |edge| edge.load())
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

impl<E> Link<E>
where
    E: ToPloy,
{
    /// Copy the link with unit type erased.  
    pub fn ploy(&self) -> Ploy {
        read_part(&self.edge, |edge| Ploy {
            edge: edge.ploy(),
            path: self.path.clone(),
            rank: self.rank,
        })
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
        Arc::<RwLock<E>>::ptr_eq(&self.edge, &other.edge) && self.path == other.path
    }
    #[cfg(feature = "oneThread")]
    fn eq(&self, other: &Self) -> bool {
        Rc::<RefCell<E>>::ptr_eq(&self.edge, &other.edge) && self.path == other.path
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

/// TODO: make reader that does not add a root to the apex.
/// This will allow readers to inspect without rebuting in the future.
impl<E> Read for Link<E>
where
    E: 'static + Read + Update + AddRoot,
{
    type Item = E::Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        read_part(&self.edge, |edge| {
            let out = edge.read(read);
            edge.add_root(self.as_root(edge.id()));
            out
        })
    }
}

impl<E> ReadLoad for Link<E>
where
    E: 'static + ReadLoad + Update + AddRoot,
{
    fn read_load<T, F: FnOnce(load::ResultRef) -> T>(&self, read: F) -> T {
        read_part(&self.edge, |edge| {
            let out = edge.read_load(read);
            edge.add_root(self.as_root(edge.id()));
            out
        })
    }
}

impl<E> WriteLoad for Link<E>
where
    E: WriteLoad,
{
    type Item = E::Item;
    fn write<T, F: FnOnce(&mut Self::Item) -> T>(&self, write: F) -> write::Result<T> {
        read_part(&self.edge, |edge| edge.write(write))
    }
}

impl<E> WriteUnit for Link<E>
where
    E: WriteUnit,
{
    type Unit = E::Unit;
    fn write<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(&self, write: F) -> write::Result<T> {
        read_part(&self.edge, |edge| edge.write(write))
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
        })
    }
}

impl<E> AdaptInner for Link<E>
where
    E: AdaptInner,
{
    fn adapt(&self, post: Post) -> adapt::Result {
        read_part(&self.edge, |edge| edge.adapt(post))
    }
}

impl Backed for Ploy {
    fn backed(&self, back: &Back) -> Self {
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
