use anyhow::anyhow;
pub use leaf::*;

use super::*;
#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock};
#[cfg(feature = "oneThread")]
use std::{cell::RefCell, rc::Rc};
use std::{
    fmt,
    hash::{Hash, Hasher}, marker::PhantomData,
};

mod leaf;
#[cfg(test)]
mod tests;

/// `Link` to `Tray`.
pub type Leaf<T> = Link<edge::Leaf<T>, T>;

/// `Link` to domain-specific unit.
/// The unit type is intact. For type-erased unit, use `Ploy` instead.
pub type Node<U> = Link<edge::Node<U>, ()>;

/// `Link` to `Edge`, pointing to `Cusp`, containing work unit.
/// Unit fields often contain `Link`, creating a graph pattern.
// #[derive(Debug)]
pub struct Link<E, T> {
    #[cfg(not(feature = "oneThread"))]
    edge: Arc<RwLock<E>>,
    #[cfg(feature = "oneThread")]
    edge: Rc<RefCell<E>>,
    path: Option<Path>,
    rank: Option<u64>,
    out: PhantomData<T>
}

impl<E, T> fmt::Debug for Link<E, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Path: {:?}", self.path))
    }
}

impl<E, T> Link<E, T> {
    pub fn pathed(&self, path: Path) -> Self {
        Self {
            edge: self.edge.clone(),
            path: Some(path),
            rank: self.rank,
            out: PhantomData::default(),
        }
    }
    pub fn path(&self) -> Option<&Path> {
        self.path.as_ref()
    }
    pub fn rank(&self) -> Option<u64> {
        self.rank
    }
}

impl<E, T> Link<E, T>
where
    Self: Solve<Out = T>,
    T: 'static + Payload
{
    pub fn main(&self) -> Result<Hub<T>> {
        match self.solve(Task::Main)? {
            Gain::Hub(hub) => Ok(hub),
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

impl<E, T> Hash for Link<E, T>
where
    Self: Solve<Out = T>,
    T: 'static + Payload
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

impl<E, T> Serialize for Link<E, T>
where
    Self: Solve<Out = T>,
    T: 'static + Payload
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
        }
    }
}

impl<E, T> Link<E, T>
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
            out: PhantomData::default()
        }
    }
}

impl<E, T> Link<E, T>
where
    E: Make,
{
    pub fn make<F: FnOnce(&Back) -> Result<E::Unit>>(make: F) -> Result<Self> {
        let (edge, rank) = E::make(make)?;
        Ok(Self {
            path: None,
            rank,
            #[cfg(not(feature = "oneThread"))]
            edge: Arc::new(RwLock::new(edge)),
            #[cfg(feature = "oneThread")]
            edge: Rc::new(RefCell::new(edge)),
            out: PhantomData::default(),
        })
    }
}

impl<E, T> Link<E, T>
where
    E: 'static + Make + Engage,
{
    pub fn make_ploy<F: FnOnce(&Back) -> Result<E::Unit>>(make: F) -> Result<Ploy<E::Out>> {
        let (edge, rank) = E::make(make)?;
        Ok(Link {
            path: None,
            rank,
            #[cfg(not(feature = "oneThread"))]
            edge: Arc::new(RwLock::new(Box::new(edge) as Box<dyn Engage<Out = E::Out>>)),
            #[cfg(feature = "oneThread")]
            edge: Rc::new(RefCell::new(Box::new(edge) as Box<dyn Engage<Out = E::Out>>)),
            out: PhantomData::default()
        })
    }
}

impl<E, T> Link<E, T>
where
    E: ToPloy<ToPloyOut = T>,
{
    /// Copy the link with unit type erased.  
    pub fn ploy(&self) -> Result<Ploy<T>> {
        read_part(&self.edge, |edge| Ploy {
            edge: edge.ploy(),
            path: self.path.clone(),
            rank: self.rank,
            out: PhantomData::default(),
        })
    }
}

impl<E, T> Link<E, T>
where
    E: 'static + FromSnap + Engage,
{
    pub fn make_ploy_from_snap(snap: Snap<E::Unit>) -> Ploy<E::Out> {
        let (edge, rank) = E::from_snap(snap);
        Link {
            path: None,
            rank,
            #[cfg(not(feature = "oneThread"))]
            edge: Arc::new(RwLock::new(Box::new(edge) as Box<dyn Engage<Out = E::Out>>)),
            #[cfg(feature = "oneThread")]
            edge: Rc::new(RefCell::new(Box::new(edge) as Box<dyn Engage<Out = E::Out>>)),
            out: PhantomData::default(),
        }
    }
}

impl<E, T> Link<E, T>
where
    E: Read<Item = T>,
    T: Payload,
{
    pub fn tray(&self) -> Result<Tray<T>> {
        read_part(&self.edge, |edge| edge.read(|tray| Tray::Item(tray.clone())))?
    }
}

impl<E, T> Link<E, T>
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

impl<E, T> Clone for Link<E, T> {
    fn clone(&self) -> Self {
        Self {
            edge: self.edge.clone(),
            path: self.path.clone(),
            rank: self.rank,
            out: self.out.clone(),
        }
    }
}

impl<E, T> PartialEq for Link<E, T> {
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

impl<E, T> TryBacked for Link<E, T>
where
    E: Backed,
{
    type Out = Self;
    #[cfg(not(feature = "oneThread"))]
    fn backed(&self, back: &Back) -> Result<Self> {
        let edge = match self.edge.read() {
            Ok(edge) => edge,
            Err(err) => return Err(crate::Error::Read(err.to_string()))?,
        };
        Ok(Self {
            edge: Arc::new(RwLock::new(edge.backed(back))),
            path: self.path.clone(),
            rank: self.rank,
            out: self.out.clone()
        })
    }
    #[cfg(feature = "oneThread")]
    fn backed(&self, back: &Back) -> Result<Self> {
        let edge = match self.edge.try_borrow() {
            Ok(edge) => edge,
            Err(err) => return Err(crate::Error::Read(err.to_string()))?,
        };
        // let edge = self.edge.borrow();
        Ok(Self {
            edge: Rc::new(RefCell::new(edge.backed(back))),
            path: self.path.clone(),
            rank: self.rank,
        })
    }
}

impl<E, T> Link<E, T>
where
    E: 'static + Read + Update + AddRoot,
{
    /// Read payload of Link.
    pub fn read<O, F: FnOnce(&E::Item) -> O>(&self, read: F) -> Result<O> {
        read_part(&self.edge, |edge| {
            let out = edge.read(read);
            edge.add_root(self.as_root(edge.id()))?;
            out
        })?
    }
}

impl<E, T> WriteTray<T> for Link<E, T>
where
    E: WriteTray<T>,
{
    fn write<O, F: FnOnce(&mut T) -> O>(&self, write: F) -> Result<O> {
        read_part(&self.edge, |edge| edge.write(write))?
    }
}

impl<E, T> WriteUnit for Link<E, T>
where
    E: WriteUnit,
{
    type Unit = E::Unit;
    fn write<O, F: FnOnce(&mut Pack<Self::Unit>) -> O>(&self, write: F) -> Result<O> {
        read_part(&self.edge, |edge| edge.write(write))?
    }
}

impl<E, T> Solve for Link<E, T>
where
    //T: 'static + Debug + SendSync, // Hash + Serialize + 
    E: 'static + Solve<Out = T> + AddRoot + Update,
    T: Payload
{
    type Out = T;
    fn solve(&self, task: Task) -> Result<Gain<Self::Out>> {
        read_part(&self.edge, |edge| {
            let result = edge.solve(task);
            edge.add_root(self.as_root(edge.id()))?;
            result
        })?
    }
}

impl<E, T> AdaptMid for Link<E, T>
where
    E: 'static + AdaptMid + AddRoot + Update,
{
    fn adapt(&self, deal: &mut dyn Deal) -> Result<()> {
        read_part(&self.edge, |edge| {
            let result = edge.adapt(deal);
            if deal.read() {
                edge.add_root(self.as_root(edge.id()))?;
            }
            result
        })?
    }
}

impl<T> TryBacked for Ploy<T> 
where 
    T: Payload
{
    type Out = Ploy<T>;
    fn backed(&self, back: &Back) -> Result<Self::Out> {
        read_part(&self.edge, |edge| Self {
            edge: edge.backed_ploy(back),
            path: self.path.clone(),
            rank: self.rank,
            out: self.out.clone(),
        })
    }
}

impl<T> TryBacked for Vec<T>
where
    T: TryBacked<Out = T>,
{
    type Out = Self;
    fn backed(&self, back: &Back) -> Result<Self> {
        self.iter().map(|link| link.backed(back)).collect()
    }
}

// pub fn ranked(&self) -> Self {
//     let rank = if let Ok(Gain::U64(rank)) = self.solve(Task::Rank) {
//         Some(rank)
//     } else {
//         None
//     };
//     Self {
//         edge: self.edge.clone(),
//         path: self.path.clone(),
//         rank,
//     }
// }
