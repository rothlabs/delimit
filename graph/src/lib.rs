pub use adapt::{adapt_ok, no_adapter, Adapt, AdaptMid, AdaptOut, Memo, Post};
pub use apex::{wrong_tray, Apex, EngageApexes};
pub use bay::Bay;
pub use cusp::Cusp;
pub use edge::Edge;
pub use lake::{Lake, SerialNode};
pub use link::{Leaf, Link, Node, ToLeaf};
pub use map::Map;
pub use meta::{upper_all, Id, Import, Key, Path, ToId, WORLD_ALL};
pub use ploy::{BackedPloy, Engage, Ploy, PloyPointer, ToPloy};
pub use react::{
    AddRoot, AddRootMut, Back, Backed, React, ReactMut, Rebut, RebutMut, Ring, Root, TryBacked,
    Update, UpdateMid,
};
pub use serial::{DeserializeUnit, ToHash, ToSerial, UnitHasher};
pub use snap::{IntoSnapWithImport, IntoSnapWithImports, Snap};
pub use solve::{no_solver, solve_ok, DoSolve, Gain, IntoGain, Solve, Task};
pub use tray::Tray;
pub use write::{
    Pack, WriteTray, WriteTrayOut, WriteTrayWork, WriteUnit, WriteUnitOut, WriteUnitWork,
};

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
    result,
};
use thiserror::Error;

pub mod adapt;
pub mod apex;
pub mod lake;
pub mod react;
pub mod serial;
pub mod solve;
pub mod work;

mod bay;
mod cusp;
mod edge;
mod link;
mod map;
mod meta;
mod ploy;
mod scope;
mod snap;
mod tray;
mod write;

pub type GraphResult<T> = result::Result<T, Error>;

/// Graph Error
#[derive(Error, Debug)]
pub enum Error {
    #[error("read graph part failed ({0})")]
    Read(String),
    #[error("write graph part failed ({0})")]
    Write(String),
    #[error(transparent)]
    Adapt(#[from] adapt::Error),
    #[error(transparent)]
    Solve(#[from] solve::Error),
    #[error(transparent)]
    Apex(#[from] apex::Error),
    // #[error("failed to make node ({0})")]
    // Make(String),
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
fn read_part<P: ?Sized, O, F: FnOnce(GraphResult<RwLockReadGuard<P>>) -> GraphResult<O>>(
    part: &Arc<RwLock<P>>,
    read: F,
) -> GraphResult<O> {
    match part.read() {
        Ok(part) => read(Ok(part)),
        Err(err) => read(Err(Error::Read(err.to_string()))),
    }
}

#[cfg(feature = "oneThread")]
fn read_part<P: ?Sized, O, F: FnOnce(GraphResult<Ref<P>>) -> GraphResult<O>>(
    part: &Rc<RefCell<P>>,
    read: F,
) -> GraphResult<O> {
    match part.try_borrow() {
        Ok(part) => read(Ok(part)),
        Err(err) => read(Err(Error::Read(err.to_string()))),
    }
}

#[cfg(not(feature = "oneThread"))]
fn write_part<P: ?Sized, O, F: FnOnce(RwLockWriteGuard<P>) -> O>(
    part: &Arc<RwLock<P>>,
    write: F,
) -> O {
    write(part.write().expect(NO_POISON))
}

#[cfg(feature = "oneThread")]
fn write_part<P: ?Sized, O, F: FnOnce(RefMut<P>) -> O>(part: &Rc<RefCell<P>>, write: F) -> O {
    write(part.borrow_mut())
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

pub trait IntoApex {
    /// Move into `Apex`
    fn apex(self) -> Apex;
}

impl<T> IntoApex for T
where
    T: 'static + IntoNode + Solve + Adapt + Debug + SendSync,
{
    fn apex(self) -> Apex {
        self.node()
            .ploy()
            .expect("The freshly made node should not be poisoned.")
            .into()
    }
}

impl IntoApex for Leaf {
    fn apex(self) -> Apex {
        self.into()
    }
}

pub trait ToApex {
    /// Place inside a Apex.
    fn apex(&self) -> Apex;
}

impl<T> ToApex for Node<T>
where
    T: 'static + Solve + Adapt + Debug + SendSync,
{
    fn apex(&self) -> Apex {
        // TODO: remove unwrap
        self.ploy().unwrap().into()
    }
}

pub trait ToItem {
    type Item;
    fn item(&self) -> &Self::Item;
}

pub trait Read {
    type Item;
    /// Read the payload of the node.
    fn read<T, F: FnOnce(GraphResult<&Self::Item>) -> GraphResult<T>>(
        &self,
        read: F,
    ) -> GraphResult<T>;
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

// impl<T> ToApex for T
// where
//     T: 'static + ToNode + Solve + Adapt + Debug + SendSync,
// {
//     fn apex(&self) -> Apex {
//         self.node().ploy().into()
//     }
// }

// if let Some(apex) = self.0.get(key) {
//     Some(apex.pathed(key))
// } else {
//     None
// }
