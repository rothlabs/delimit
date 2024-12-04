pub use anyhow::anyhow;
pub use apex::{Apex, DealItem};
pub use base::Digest;
pub use bay::Bay;
pub use deal::Deal;
pub use hub::{Hub, SolveDown, ToGateHub, ToPloyHub};
pub use lake::{Lake, Serial};
pub use link::{IntoLeaf, Leaf, Link, Node, ToLeaf};
pub use map::Map;
pub use meta::{upper_all, Id, Import, Key, Path, WORLD_ALL};
pub use node::{acted, Act, Adapt, Solve};
pub use node_derive;
pub use react::{Back, Backed};
pub use serial::{DeserializeUnit, ToSerial, UnitHasher};
pub use snap::{IntoSnapWithImport, IntoSnapWithImports};
pub use thiserror;
pub use write::{WriteBase, WriteUnit};

use aim::*;
use apex::{View, ViewVec};
use derive_builder::UninitializedFieldError;
use edge::{gate, ploy};
use link::{Gate, Ploy};
use react::{AddRoot, ReactMut, Rebut, RebutMut, Ring, Root, Update, UpdateMut};
use scope::*;
use serde::{Deserialize, Serialize};
use snap::Snap;
use std::{
    collections::HashMap,
    fmt::Debug,
    future::Future,
    hash::{DefaultHasher, Hash, Hasher},
    pin::Pin,
};
use tray::Tray;
use write::{Pack, WriteBaseOut, WriteUnitOut, WriteUnitWork};

#[cfg(not(feature = "oneThread"))]
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
#[cfg(not(feature = "oneThread"))]
use std::sync::Arc;
#[cfg(feature = "oneThread")]
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub mod hub;
pub mod lake;
pub mod node;
pub mod react;
pub mod serial;
pub mod snap;
pub mod work;
pub mod write;

mod aim;
mod apex;
mod base;
mod bay;
mod cusp;
mod deal;
mod edge;
mod link;
mod map;
mod meta;
mod scope;
#[cfg(test)]
mod tests;
mod tray;

/// Graph Result
pub type Result<T> = std::result::Result<T, Error>;

/// Graph Error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("read graph part failed ({0})")]
    Read(String),
    #[error("write graph part failed ({0})")]
    Write(String),
    #[error(transparent)]
    Aim(#[from] aim::Error),
    #[error("no back: {0}")]
    NoBack(String),
    #[error(transparent)]
    Tray(#[from] tray::Error),
    // #[error(transparent)]
    // Adapt(#[from] adapt::Error),
    #[error(transparent)]
    Hub(#[from] hub::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Uninit(#[from] UninitializedFieldError),
    #[error(transparent)]
    Generic(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

pub fn no_back(source: &str) -> Result<()> {
    Err(Error::NoBack(source.into()))
}

#[cfg(not(feature = "oneThread"))]
pub trait SendSync: Send + Sync {}
#[cfg(not(feature = "oneThread"))]
impl<T: Send + Sync> SendSync for T {}
#[cfg(feature = "oneThread")]
pub trait SendSync {}
#[cfg(feature = "oneThread")]
impl<T> SendSync for T {}

#[cfg(not(feature = "oneThread"))]
pub trait IsSend: Send {}
#[cfg(not(feature = "oneThread"))]
impl<T: Send> IsSend for T {}
#[cfg(feature = "oneThread")]
pub trait IsSend {}
#[cfg(feature = "oneThread")]
impl<T> IsSend for T {}

/// Graph reference counter
/// TODO: rename to Nrc for Node Reference Counter?
#[cfg(not(feature = "oneThread"))]
pub type Grc<T> = Arc<T>;
/// Graph reference counter
#[cfg(feature = "oneThread")]
pub type Grc<T> = Rc<T>;

#[cfg(not(feature = "oneThread"))]
pub type Cell<T> = RwLock<T>;
/// Graph reference counter
#[cfg(feature = "oneThread")]
pub type Cell<T> = RefCell<T>;

pub type Pointer<T> = Grc<Cell<T>>;

// #[cfg(not(feature = "oneThread"))]
// pub type Pointer<T> = Arc<RwLock<T>>;
// #[cfg(feature = "oneThread")]
// pub type Pointer<T> = Rc<RefCell<T>>;

#[cfg(not(feature = "oneThread"))]
type GraphFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
#[cfg(feature = "oneThread")]
type GraphFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

#[cfg(not(feature = "oneThread"))]
fn read_part<'a, P, F, O>(part: &'a Pointer<P>, read: F) -> Result<O>
where
    P: ?Sized,
    F: FnOnce(RwLockReadGuard<'a, P>) -> O,
{
    Ok(read(part.read()))
}

#[cfg(feature = "oneThread")]
fn read_part<'a, P, F, O>(part: &'a Pointer<P>, read: F) -> Result<O>
where
    P: ?Sized,
    F: FnOnce(Ref<'a, P>) -> O,
{
    match part.try_borrow() {
        Ok(part) => Ok(read(part)),
        Err(err) => Err(Error::Read(err.to_string())),
    }
}

#[cfg(not(feature = "oneThread"))]
fn try_write_part<'a, P, F, O>(part: &'a Pointer<P>, write: F) -> Result<O>
where
    P: ?Sized,
    F: FnOnce(RwLockWriteGuard<'a, P>) -> O,
{
    Ok(write(part.write()))
}

#[cfg(feature = "oneThread")]
fn try_write_part<'a, P, F, O>(part: &'a Pointer<P>, write: F) -> Result<O>
where
    P: ?Sized,
    F: FnOnce(RefMut<'a, P>) -> O,
{
    match part.try_borrow_mut() {
        Ok(part) => Ok(write(part)),
        Err(err) => Err(Error::Write(err.to_string())),
    }
}

#[cfg(not(feature = "oneThread"))]
fn write_part<'a, P, F, O>(part: &'a Pointer<P>, write: F) -> O
where
    P: ?Sized,
    F: FnOnce(RwLockWriteGuard<'a, P>) -> O,
{
    write(part.write())
}

#[cfg(feature = "oneThread")]
fn write_part<'a, P, F, O>(part: &'a Pointer<P>, write: F) -> O
where
    P: ?Sized,
    F: FnOnce(RefMut<'a, P>) -> O,
{
    write(part.borrow_mut())
}

pub trait Unit: Solve + Adapt + Debug + SendSync {}
impl<T> Unit for T where T: Solve + Adapt + Debug + SendSync {}

pub trait Gather: Clone + Debug + SendSync {}
impl<T> Gather for T where T: Clone + Debug + SendSync {}

pub trait Transmit: Gather + Digest + Serialize {}
impl<T> Transmit for T where T: Gather + Digest + Serialize {}

// pub trait PloyTag {}

pub trait GateTag {}

pub trait IntoNode
where
    Self: Unit + Sized,
{
    fn node(self) -> Node<Self>;
}

impl<T> IntoNode for T
where
    T: 'static + Unit,
    T::Base: Clone,
{
    fn node(self) -> Node<Self> {
        Node::from_unit(self)
    }
}

pub trait IntoPloy
where
    Self: Solve,
{
    fn ploy(self) -> Ploy<Self::Base>;
}

impl<T> IntoPloy for T
where
    T: 'static + Unit + Digest + Serialize,
    T::Base: Clone + Debug,
{
    fn ploy(self) -> Ploy<T::Base> {
        Node::ploy_from_unit(self)
    }
}

pub trait IntoGate
where
    Self: Solve,
{
    fn gate(self) -> Gate<Self::Base>;
}

impl<T> IntoGate for T
where
    T: 'static + Unit + GateTag,
    T::Base: Clone + Debug,
{
    fn gate(self) -> Gate<Self::Base> {
        Node::gate_from_unit(self)
    }
}

pub trait IntoGateHub {
    type Base;
    /// Move into `Hub`
    fn hub(self) -> Hub<Self::Base>;
}

impl<T: IntoGate> IntoGateHub for T {
    type Base = T::Base;
    fn hub(self) -> Hub<Self::Base> {
        self.gate().into()
    }
}

pub trait ToItem {
    type Item;
    fn item(&self) -> &Self::Item;
}

pub trait BaseMut {
    type Base;
    fn base(&mut self) -> &mut Self::Base;
}

pub trait WorkFromBase {
    type Base;
    fn from_base(base: Self::Base) -> Self;
}

pub trait WorkFromSnap {
    type Unit;
    fn from_snap(unit: Snap<Self::Unit>) -> (Option<u16>, Self);
}

pub trait Clear {
    fn clear(&mut self);
}

pub trait BackIt {
    fn back(&mut self, back: &Back);
}

impl<T: Backed> BackIt for T {
    fn back(&mut self, back: &Back) {
        *self = self.backed(back);
    }
}

pub trait Reckon {
    fn get_imports(&self) -> Result<Vec<Import>>;
    fn get_hash(&self) -> Result<u64>;
    fn get_serial(&self) -> Result<String>;
}

pub trait ReckonMut {
    fn get_imports(&self) -> Result<Vec<Import>>;
    fn get_hash(&mut self) -> Result<u64>;
    fn get_serial(&mut self) -> Result<String>;
}

pub trait Based<T> {
    fn base(&self) -> impl Future<Output = Result<Vec<T>>>;
}

pub trait BasedOption<T> {
    fn base(&self) -> impl Future<Output = Result<Option<T>>>;
}

pub trait Depend {
    fn depend(&self) -> impl Future<Output = Result<()>>;
}

pub trait ConcatVec<T> {
    fn with(&self, rhs: &[T]) -> Self;
}

impl<T: Clone> ConcatVec<T> for Vec<T> {
    fn with(&self, rhs: &[T]) -> Self {
        [self.clone(), rhs.to_vec()].concat()
    }
}