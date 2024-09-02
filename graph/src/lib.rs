pub use adapt::{adapt_ok, Adapt, AdaptMid, AdaptOut, Memo, Post};
pub use apex::{wrong_tray, Apex, EngageApexes};
pub use bay::Bay;
pub use cusp::Cusp;
pub use edge::Edge;
pub use lake::{Lake, SerialNode};
pub use link::{Leaf, Link, Node, ToLeaf};
pub use map::{Map, Fit};
pub use meta::{upper_all, Id, Import, Key, Path, ToId, WORLD_ALL};
pub use ploy::{BackedPloy, Engage, Ploy, PloyPointer, ToPloy};
pub use react::{
    AddRoot, AddRootMut, Back, Backed, React, ReactMut, Rebut, RebutMut, Ring, Root, TryBacked,
    Update, UpdateMut,
};
pub use serial::{DeserializeUnit, ToHash, ToSerial, UnitHasher};
pub use snap::{IntoSnapWithImport, IntoSnapWithImports, Snap};
pub use solve::{solve_ok, Act, Gain, IntoGain, Solve, SolveMut, Task};
pub use tray::Tray;
pub use write::{Pack, WriteTray, WriteTrayOut, WriteUnit, WriteUnitOut, WriteUnitWork};

use aim::*;
use scope::*;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
#[cfg(feature = "oneThread")]
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};
use std::{
    collections::{hash_map::Iter, HashMap},
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};
use thiserror::Error;

pub mod adapt;
pub mod apex;
pub mod lake;
pub mod react;
pub mod serial;
pub mod snap;
pub mod solve;
pub mod work;
pub mod write;

mod aim;
mod bay;
mod cusp;
mod edge;
mod link;
mod map;
mod meta;
mod ploy;
mod scope;
#[cfg(test)]
mod tests;
mod tray;

const IMMEDIATE_ACCESS: &str = "Item should be immediately accessible after creation.";

/// Graph Result
pub type Result<T> = std::result::Result<T, Error>;

/// Graph Error
#[derive(Error, Debug)]
pub enum Error {
    #[error("read graph part failed ({0})")]
    Read(String),
    #[error("write graph part failed ({0})")]
    Write(String),
    #[error(transparent)]
    Tray(#[from] tray::Error),
    #[error(transparent)]
    Adapt(#[from] adapt::Error),
    #[error(transparent)]
    Solve(#[from] solve::Error),
    #[error(transparent)]
    Apex(#[from] apex::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

#[cfg(not(feature = "oneThread"))]
const NO_POISON: &str = "The lock should not be poisoned.";

#[cfg(not(feature = "oneThread"))]
pub trait SendSync: Send + Sync {}
#[cfg(not(feature = "oneThread"))]
impl<T: Send + Sync> SendSync for T {}

#[cfg(feature = "oneThread")]
pub trait SendSync {}
#[cfg(feature = "oneThread")]
impl<T> SendSync for T {}

#[cfg(not(feature = "oneThread"))]
fn read_part<P: ?Sized, O, F: FnOnce(RwLockReadGuard<P>) -> O>(
    part: &Arc<RwLock<P>>,
    read: F,
) -> Result<O> {
    match part.read() {
        Ok(part) => Ok(read(part)),
        Err(err) => Err(Error::Read(err.to_string())),
    }
}

#[cfg(feature = "oneThread")]
fn read_part<P: ?Sized, O, F: FnOnce(Ref<P>) -> O>(part: &Rc<RefCell<P>>, read: F) -> Result<O> {
    match part.try_borrow() {
        Ok(part) => Ok(read(part)),
        Err(err) => Err(Error::Read(err.to_string())),
    }
}

#[cfg(not(feature = "oneThread"))]
fn write_part<P: ?Sized, O, F: FnOnce(RwLockWriteGuard<P>) -> O>(
    part: &Arc<RwLock<P>>,
    write: F,
) -> Result<O> {
    match part.write() {
        Ok(part) => Ok(write(part)),
        Err(err) => Err(Error::Read(err.to_string())),
    }
}

#[cfg(feature = "oneThread")]
fn write_part<P: ?Sized, O, F: FnOnce(RefMut<P>) -> O>(
    part: &Rc<RefCell<P>>,
    write: F,
) -> Result<O> {
    match part.try_borrow_mut() {
        Ok(part) => Ok(write(part)),
        Err(err) => Err(Error::Read(err.to_string())),
    }
}

/// Trade a apex for another.
/// The implmentation should return the same semantic apex with different graph qualities.
pub trait Trade: Debug {
    /// Trade a apex for another.
    fn trade(&self, apex: &Apex) -> Apex;
}

pub trait IntoNode
where
    Self: Sized,
{
    fn node(self) -> Node<Self>;
}

impl<T> IntoNode for T
where
    T: 'static + Adapt + Solve + SendSync + Debug,
{
    fn node(mut self) -> Node<Self> {
        Node::make(|back| {
            self.adapt(Post::Trade(back))
                .expect("To move into Node, unit must Adapt with Post::Trade.");
            self
        })
    }
}

pub trait IntoPloy
where
    Self: Sized,
{
    fn ploy(self) -> Ploy;
}

impl<T> IntoPloy for T
where
    T: 'static + Adapt + Solve + SendSync + Debug,
{
    fn ploy(mut self) -> Ploy {
        Node::make_ploy(|back| {
            self.adapt(Post::Trade(back))
                .expect("To move into Ploy, unit must Adapt with Post::Trade.");
            self
        })
    }
}

pub trait IntoApex {
    /// Move into `Apex`
    fn apex(self) -> Apex;
}

impl<T> IntoApex for T
where
    T: 'static + IntoPloy + Solve + Adapt + Debug + SendSync,
{
    fn apex(self) -> Apex {
        self.ploy().into()
    }
}

impl IntoApex for Leaf {
    fn apex(self) -> Apex {
        self.into()
    }
}

pub trait ToApex {
    /// Place inside a Apex.
    fn apex(&self) -> Result<Apex>;
}

impl<T> ToApex for Node<T>
where
    T: 'static + Solve + Adapt + Debug + SendSync,
{
    fn apex(&self) -> Result<Apex> {
        Ok(Apex::from(self.ploy()?))
    }
}

pub trait ToItem {
    type Item;
    fn item(&self) -> &Self::Item;
}

pub trait MutTray {
    fn tray(&mut self) -> &mut Tray;
}

pub trait Read {
    type Item;
    /// Read the payload of the graph part.
    fn read<T, F>(&self, reader: F) -> Result<T>
    where
        F: FnOnce(&Self::Item) -> T;
}

pub trait ReadGraph<Item, T> {
    fn read(self, item: Item) -> T;
}

impl<Item, T, F: FnOnce(Item) -> T> ReadGraph<Item, T> for F {
    fn read(self, item: Item) -> T {
        self(item)
    }
}

pub trait FromItem {
    type Item;
    fn new(item: Self::Item) -> Self;
}

pub trait Make {
    type Unit;
    fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self;
}

pub trait MakeMid {
    type Unit;
    fn make<F: FnOnce(&Back) -> Self::Unit>(&mut self, make: F, back: &Back);
}

pub trait FromSnap {
    type Unit;
    fn from_snap(snap: Snap<Self::Unit>) -> Self;
}

pub trait WithSnap {
    type Unit;
    fn with_snap(&mut self, snap: Snap<Self::Unit>, back: &Back);
}

pub trait Clear {
    fn clear(&mut self);
}
