pub use adapt::{Adapt, AdaptMut};
pub use apex::Apex;
pub use base::{Vf32, Vf64};
pub use bay::Bay;
pub use cusp::Cusp;
pub use deal::Deal;
use derive_builder::UninitializedFieldError;
pub use edge::Edge;
pub use hub::{DealItem, Hub, SolveDown};
pub use lake::{Lake, Serial};
pub use link::{IntoLeaf, Leaf, Link, Node, ToLeaf};
pub use map::Map;
pub use meta::{upper_all, Id, Import, Key, Path, WORLD_ALL};
pub use ploy::{Based, Engage, Ploy, PloyEdge, ToPloy};
pub use react::{
    AddRoot,
    Back,
    Backed,
    BackedMid,
    React,
    ReactMut,
    Rebut,
    RebutMut,
    Ring,
    Root,
    Update,
    UpdateMut,
    // ClearRoots, ClearRootsMut,
};
pub use serial::{DeserializeUnit, ToHash, ToSerial, UnitHasher};
pub use snap::{IntoSnapWithImport, IntoSnapWithImports, Snap};
pub use solve::{
    reckon_ok, solve_ok, Act, Gain, IntoGain, Solve, SolveMut, Task, SolveMid,
};
pub use tray::Tray;
pub use view::View;
pub use view_vec::ViewVec;
pub use write::{Pack, Post, WriteBase, WriteBaseOut, WriteUnit, WriteUnitOut, WriteUnitWork};

use aim::*;
use async_trait::async_trait;
#[cfg(not(feature = "oneThread"))]
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use scope::*;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "oneThread"))]
use std::sync::Arc;
// use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
#[cfg(feature = "oneThread")]
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};
use std::{
    collections::HashMap,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};
use thiserror::Error;

pub mod adapt;
pub mod hub;
pub mod lake;
pub mod react;
pub mod serial;
pub mod snap;
pub mod solve;
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
mod ploy;
mod scope;
#[cfg(test)]
mod tests;
mod tray;
mod view;
mod view_vec;

// #[cfg(feature = "oneThread")]
// const IMMEDIATE_ACCESS: &str = "Item should be immediately accessible after creation.";

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
    Aim(#[from] aim::Error),
    #[error("no back: {0}")]
    NoBack(String),
    #[error(transparent)]
    Tray(#[from] tray::Error),
    #[error(transparent)]
    Adapt(#[from] adapt::Error),
    #[error(transparent)]
    Solve(#[from] solve::Error),
    #[error(transparent)]
    Hub(#[from] hub::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Uninit(#[from] UninitializedFieldError),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

// pub trait ErrorToJsValue {
//     fn js_err(&self)
// }

// impl From<JsValue> for Error {
//     fn from(value: JsValue) -> Self {
//         Error::Any(anyhow!("crap"))
//     }
// }

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

pub trait Unit: Solve + SendSync + Debug {}
impl<T> Unit for T
where
    T: Solve + SendSync + Debug,
    T::Base: Payload,
{
}

pub trait Payload: Default + Clone + Hash + Serialize + Debug + SendSync {}
impl<T> Payload for T where T: Default + Clone + Hash + Serialize + Debug + SendSync {}

#[cfg(not(feature = "oneThread"))]
pub type Pointer<T> = Arc<RwLock<T>>;
#[cfg(feature = "oneThread")]
pub type Pointer<T> = Rc<RefCell<T>>;

#[cfg(not(feature = "oneThread"))]
pub fn edge_pointer<T>(edge: T) -> Arc<RwLock<T>>
where
    T: 'static + Update + SetRoot,
{
    let edge = Arc::new(RwLock::new(edge));
    let update = edge.clone() as Arc<RwLock<dyn Update>>;
    let root = Root {
        edge: Arc::downgrade(&update),
        id: rand::random(),
    };
    edge.write().set_root(root); 
    edge
}

#[cfg(feature = "oneThread")]
pub fn edge_pointer<T>(edge: T) -> Rc<RefCell<T>>
where
    T: 'static + Update + SetRoot,
{
    let edge = Rc::new(RefCell::new(edge));
    let update = edge.clone() as Rc<RefCell<dyn Update>>;
    let root = Root {
        edge: Rc::downgrade(&update),
        id: rand::random(),
    };
    edge.borrow_mut().set_root(root); 
    edge
}


#[cfg(not(feature = "oneThread"))]
fn read_part<P, F, O>(part: &Arc<RwLock<P>>, read: F) -> Result<O>
where
    P: ?Sized,
    F: FnOnce(RwLockReadGuard<P>) -> O,
{
    Ok(read(part.read()))
}

#[cfg(feature = "oneThread")]
fn read_part<P, F, O>(part: &Rc<RefCell<P>>, read: F) -> Result<O>
where
    P: ?Sized,
    F: FnOnce(Ref<P>) -> O,
{
    match part.try_borrow() {
        Ok(part) => Ok(read(part)),
        Err(err) => Err(Error::Read(err.to_string())),
    }
}

#[cfg(not(feature = "oneThread"))]
fn read_part_async<'a, P, F, O>(part: &'a Arc<RwLock<P>>, read: F) -> Result<O>
where
    P: ?Sized,
    F: FnOnce(RwLockReadGuard<'a, P>) -> O,
    O: std::future::Future,
{
    Ok(read(part.read()))
}

#[cfg(feature = "oneThread")]
fn read_part_async<'a, F, P, O>(part: &'a Rc<RefCell<P>>, read: F) -> Result<O>
where
    P: ?Sized,
    F: FnOnce(Ref<'a, P>) -> O,
    O: std::future::Future,
{
    match part.try_borrow() {
        Ok(part) => Ok(read(part)),
        Err(err) => Err(Error::Read(err.to_string())),
    }
}

#[cfg(not(feature = "oneThread"))]
fn write_part<P, F, O>(part: &Arc<RwLock<P>>, write: F) -> Result<O>
where
    P: ?Sized,
    F: FnOnce(RwLockWriteGuard<P>) -> O,
{
    Ok(write(part.write()))
}

#[cfg(feature = "oneThread")]
fn write_part<P, F, O>(part: &Rc<RefCell<P>>, write: F) -> Result<O>
where
    P: ?Sized,
    F: FnOnce(RefMut<P>) -> O,
{
    match part.try_borrow_mut() {
        Ok(part) => Ok(write(part)),
        Err(err) => Err(Error::Write(err.to_string())),
    }
}

#[cfg(not(feature = "oneThread"))]
fn write_part_async<'a, P, F, O>(part: &'a Arc<RwLock<P>>, write: F) -> Result<O>
where
    P: ?Sized,
    F: FnOnce(RwLockWriteGuard<'a, P>) -> O,
    O: std::future::Future,
{
    Ok(write(part.write()))
}

#[cfg(feature = "oneThread")]
fn write_part_async<'a, F, P, O>(part: &'a Rc<RefCell<P>>, write: F) -> Result<O>
where
    P: ?Sized,
    F: FnOnce(RefMut<'a, P>) -> O,
    O: std::future::Future,
{
    match part.try_borrow_mut() {
        Ok(part) => Ok(write(part)),
        Err(err) => Err(Error::Write(err.to_string())),
    }
}

pub trait IntoNode
where
    Self: Unit + Sized,
{
    fn node(self) -> Result<Node<Self>>;
}

impl<T> IntoNode for T
where
    T: 'static + Unit,
{
    fn node(self) -> Result<Node<Self>> {
        Node::from_unit(self)
        // Node::make(|back| {
        //     self.adapt(&mut back.clone())?;
        //     Ok(self)
        // })
    }
}

pub trait IntoPloy
where
    Self: Solve + Sized,
{
    fn ploy(self) -> Result<Ploy<Self::Base>>;
}

impl<T> IntoPloy for T
where
    T: 'static + Unit,
{
    fn ploy(self) -> Result<Ploy<T::Base>> {
        Node::ploy_from_unit(self)
        // Node::make_ploy(|back| {
        //     self.adapt(&mut back.clone())
        //         .expect("To move into Ploy, unit must Adapt with Post::Trade.");
        //     Ok(self)
        // })
    }
}

pub trait IntoHub {
    type Base: Payload;
    /// Move into `Hub`
    fn hub(self) -> Result<Hub<Self::Base>>;
}

impl<T> IntoHub for T
where
    T: IntoPloy,
    T::Base: Payload,
{
    type Base = T::Base;
    fn hub(self) -> Result<Hub<Self::Base>> {
        Ok(self.ploy()?.into())
    }
}

pub trait ToItem {
    type Item;
    fn item(&self) -> &Self::Item;
}

pub trait BaseMut<T> {
    fn base(&mut self) -> &mut T;
}

pub trait Read {
    type Item;
    /// Read the Unit or Payload of the graph part.
    fn read<T, F>(&self, reader: F) -> Result<T>
    where
        F: FnOnce(&Self::Item) -> T;
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
pub trait ReadDown<T> {
    async fn read<O, F: FnOnce(&T) -> O + SendSync>(&self, read: F) -> Result<O>;
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
pub trait BaseDown<T> {
    async fn base(&self) -> Result<T>;
}

pub trait FromItem {
    type Item;
    fn new(item: Self::Item) -> Self;
}

pub trait SetRoot {
    fn set_root(&mut self, root: Root);
}


pub trait FromSnap {
    type Unit;
    fn from_snap(unit: Snap<Self::Unit>) -> Result<(Option<u64>, Pointer<Self>)>;
}

pub trait WorkFromSnap {
    type Unit;
    fn from_snap(unit: Snap<Self::Unit>) -> (Option<u64>, Self);
}

pub trait Clear {
    fn clear(&mut self);
}