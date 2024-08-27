pub use leaf::*;

use super::*;
use std::hash::{Hash, Hasher};
#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock};
#[cfg(feature = "oneThread")]
use std::{cell::RefCell, rc::Rc};

mod leaf;
#[cfg(test)]
mod tests;

/// `Link` to `Tray`.
pub type Leaf = Link<edge::Leaf>;

/// `Link` to domain-specific unit.
/// The unit type is intact. For type-erased unit, use `Ploy` instead.
pub type Node<U> = Link<edge::Node<U>>;

/// `Link` to domain-specific unit.
/// The unit type is erased. To keep unit type intact, use `Node` instead.
pub type Ploy = Link<Box<dyn Engage>>;

/// `Link` to `Edge`, pointing to `Cusp`, containing work unit.
/// Unit fields often contain `Link`, creating a graph pattern.
#[derive(Debug)]
pub struct Link<E> {
    #[cfg(not(feature = "oneThread"))]
    edge: Arc<RwLock<E>>,
    #[cfg(feature = "oneThread")]
    edge: Rc<RefCell<E>>,
    path: Option<Path>,
    rank: Option<usize>,
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
    pub fn main(&self) -> Result<Apex, Error> {
        match self.solve(Task::Main)? {
            Gain::Apex(apex) => Ok(apex),
            _ => Err("Wrong return type for Task::Main.")?,
        }
    }
}

impl<E> Hash for Link<E>
where
    Self: Solve,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Ok(Gain::U64(digest)) = self.solve(Task::Digest) {
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
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(path) = &self.path {
            path.serialize(serializer)
        } else if let Ok(Gain::U64(hash)) = self.solve(Task::Digest) {
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
    E: Make2,
{
    pub fn make2(unit: E::Unit, imports: &Vec<Import>) -> Self {
        let edge = E::make2(unit, imports);
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
    E: ToTray,
{
    pub fn tray(&self) -> E::Tray {
        read_part(&self.edge, |edge| edge.tray())
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
        Arc::<RwLock<E>>::ptr_eq(&self.edge, &other.edge) && self.path == other.path && self.rank == other.rank
    }
    #[cfg(feature = "oneThread")]
    fn eq(&self, other: &Self) -> bool {
        Rc::<RefCell<E>>::ptr_eq(&self.edge, &other.edge) && self.path == other.path && self.rank == other.rank
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

impl<E> ReadTray for Link<E>
where
    E: 'static + ReadTray + Update + AddRoot,
{
    fn read_tray<T, F: FnOnce(tray::ResultRef) -> T>(&self, read: F) -> T {
        read_part(&self.edge, |edge| {
            let out = edge.read_tray(read);
            edge.add_root(self.as_root(edge.id()));
            out
        })
    }
}

impl<E> WriteTray for Link<E>
where
    E: WriteTray,
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

// impl<U, E> From<Snap<U>> for Link<E> 
// where 
//     // U: Default + Solve,
//     Snap<U>: Into<E>,
// {
//     fn from(snap: Snap<U>) -> Self {
//         Self {
//             path: None,
//             rank: None,
//             #[cfg(not(feature = "oneThread"))]
//             edge: Arc::new(RwLock::new(snap.into())),
//             #[cfg(feature = "oneThread")]
//             edge: Rc::new(RefCell::new(snap.into())),
//         }
//     }
// }
