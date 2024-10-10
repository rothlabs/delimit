pub use adapt::{AdaptEdge, AdaptMut};
pub use anyhow::anyhow;
pub use anyhow::Error as anyError;
pub use apex::{Apex, DealItem, Poll, View, ViewVec};
pub use base::HashGraph;
pub use bay::Bay;
pub use cusp::Cusp;
pub use deal::Deal;
pub use edge::Edge;
pub use hub::{Hub, SolveDown, ToPloyHub, ToWingHub};
pub use lake::{Lake, Serial};
pub use link::{IntoLeaf, Leaf, Link, Node, ToLeaf};
pub use map::Map;
pub use meta::{upper_all, Id, Import, Key, Path, WORLD_ALL};
pub use paste::paste;
pub use ploy::{Based, Employ, Employed, Engage, Ploy, PloyEdge, Wing, WingEdge};
pub use react::{
    AddRoot, Back, Backed, BackedMid, React, ReactMut, Rebut, RebutMut, Ring, Root, Update,
    UpdateMut,
};
pub use serial::{DeserializeUnit, Digest, ToSerial, UnitHasher};
pub use snap::{IntoSnapWithImport, IntoSnapWithImports, Snap};
pub use solve::{reckon_ok, solve_ok, Act, Gain, IntoGain, Solve, SolveMut, Task};
pub use thiserror;
pub use thiserror::Error as ThisError;
pub use tray::Tray;
// pub use view::{View, ViewVec};
pub use write::{Pack, WriteBase, WriteBaseOut, WriteUnit, WriteUnitOut, WriteUnitWork};

use aim::*;
use derive_builder::UninitializedFieldError;
use scope::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Debug,
    future::Future,
    hash::{DefaultHasher, Hash, Hasher},
    pin::Pin,
};
use thiserror::Error;

#[cfg(not(feature = "oneThread"))]
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
#[cfg(not(feature = "oneThread"))]
use std::sync::Arc;
#[cfg(feature = "oneThread")]
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

#[macro_export]
macro_rules! make_func {
    ($Unit:ident) => {
        pub fn make(self) -> graph::Result<$Unit> {
            match self.build() {
                Ok(value) => Ok(value),
                Err(err) => Err(anyhow!(err.to_string()))?,
            }
        }
    };
}

#[macro_export]
macro_rules! node_and_apex {
    ($Unit:ident) => {
        make_func!($Unit);
        pub fn node(self) -> graph::Result<Node<$Unit>> {
            self.make()?.node()
        }
        pub fn apex(self) -> graph::Result<Apex> {
            Ok(self.hub()?.into())
        }
    };
}

#[macro_export]
macro_rules! build_methods {
    ($Unit:ident $Base:ty) => {
        impl WingOnly for $Unit {}
        impl paste! {[<$Unit "Builder">]} {
            node_and_apex!($Unit);
            pub fn hub(self) -> graph::Result<Hub<$Base>> {
                self.make()?.hub()
            }
        }
    };
}

#[macro_export]
macro_rules! Make {
    (
    $(#[$attr:meta])*
    $pub:vis
    struct $Unit:ident $tt:tt
    ) => {
        paste! {
            impl [<$Unit "Builder">] {
                make_func!($Unit);
            }
        }
    };
}

#[macro_export]
macro_rules! Unit {
    (
    $(#[$attr:meta])*
    $pub:vis
    struct $Unit:ident $tt:tt
    ) => {
        impl WingOnly for $Unit {}
        impl paste! {[<$Unit "Builder">]} {
            node_and_apex!($Unit);
            pub fn hub(self) -> graph::Result<Hub<()>> {
                self.make()?.hub()
            }
        }
    };
}

#[macro_export]
macro_rules! Vf32 {(
    $(#[$attr:meta])*
    $pub:vis
    struct $Unit:ident $tt:tt
) => {
        build_methods!($Unit Vec<f32>);
    };
}

#[macro_export]
macro_rules! Output {
    (
    $(#[$attr:meta])*
    $pub:vis
    struct $Unit:ident<T: Payload + Pod> $tt:tt
    ) => {
        impl<T: Payload + Pod> WingOnly for $Unit<T> {}
        impl<T> paste! {[<$Unit "Builder">]<T>}
        where
            T: Payload + Pod,
        {
            pub fn make(self) -> graph::Result<$Unit<T>> {
                match self.build() {
                    Ok(value) => Ok(value),
                    Err(err) => Err(anyhow!(err.to_string()))?,
                }
            }
            pub fn node(self) -> graph::Result<Node<$Unit<T>>> {
                self.make()?.node()
            }
            pub fn apex(self) -> graph::Result<Apex> {
                Ok(self.hub()?.into())
            }
            pub fn hub(self) -> graph::Result<Hub<()>> {
                self.make()?.hub()
            }
        }
    };
}

#[macro_export]
macro_rules! Input {
    (
    $(#[$attr:meta])*
    $pub:vis
    struct $Unit:ident<T> $tt:tt
    ) => {
        impl<T> WingOnly for $Unit<T> {}
        impl<T> paste! {[<$Unit "Builder">]<T>}
        where
            T: Payload + AnyBitPattern,
            Vec<T>: Payload,
        {
            pub fn make(self) -> graph::Result<$Unit<T>> {
                match self.build() {
                    Ok(value) => Ok(value),
                    Err(err) => Err(anyhow!(err.to_string()))?,
                }
            }
            pub fn node(self) -> graph::Result<Node<$Unit<T>>> {
                self.make()?.node()
            }
            pub fn hub(self) -> graph::Result<Hub<Vec<T>>> {
                self.make()?.hub()
            }
        }
    };
}

#[macro_export]
macro_rules! GraphHash {
    (
    $(#[$attr:meta])*
    $pub:vis
    struct $Unit:ident {
        $($field:ident: $typ:ty),* $(,)*
    }
    ) => {
        impl HashGraph for $Unit {
            fn hash_graph<H: std::hash::Hasher>(&self, state: &mut H) {
                $(
                    self.$field.hash_graph(state);
                 )*
            }
        }
    };
}

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
// mod view;

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
    Recieve(#[from] flume::RecvError),
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

pub trait Unit: Solve + SendSync + Debug {}
impl<T> Unit for T where T: Solve + SendSync + Debug {}

pub trait Payload: 'static + Default + Clone + HashGraph + Serialize + Debug + SendSync {}
impl<T> Payload for T where T: 'static + Default + Clone + HashGraph + Serialize + Debug + SendSync {}

/// Graph reference counter
#[cfg(not(feature = "oneThread"))]
pub type Grc<T> = Arc<T>;
/// Graph reference counter
#[cfg(feature = "oneThread")]
pub type Grc<T> = Rc<T>;

#[cfg(not(feature = "oneThread"))]
pub type Pointer<T> = Arc<RwLock<T>>;
#[cfg(feature = "oneThread")]
pub type Pointer<T> = Rc<RefCell<T>>;

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
fn write_part<'a, P, F, O>(part: &'a Pointer<P>, write: F) -> Result<O>
where
    P: ?Sized,
    F: FnOnce(RwLockWriteGuard<'a, P>) -> O,
{
    Ok(write(part.write()))
}

#[cfg(feature = "oneThread")]
fn write_part<'a, P, F, O>(part: &'a Pointer<P>, write: F) -> Result<O>
where
    P: ?Sized,
    F: FnOnce(RefMut<'a, P>) -> O,
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
    }
}

pub trait IntoPloy
where
    Self: Solve,
{
    fn ploy(self) -> Result<Ploy<Self::Base>>;
}

impl<T> IntoPloy for T
where
    T: 'static + Unit + HashGraph + Serialize,
{
    fn ploy(self) -> Result<Ploy<T::Base>> {
        Node::ploy_from_unit(self)
    }
}

pub trait IntoHub {
    type Base: Payload;
    /// Move into `Hub`
    fn hub(self) -> Result<Hub<Self::Base>>;
}

impl<T: IntoPloy> IntoHub for T {
    type Base = T::Base;
    fn hub(self) -> Result<Hub<Self::Base>> {
        Ok(self.ploy()?.into())
    }
}

pub trait IntoWing
where
    Self: Solve,
{
    fn wing(self) -> Result<Wing<Self::Base>>;
}

impl<T> IntoWing for T
where
    T: 'static + Unit + WingOnly,
{
    fn wing(self) -> Result<Wing<Self::Base>> {
        Node::wing_from_unit(self)
    }
}

pub trait IntoWingHub {
    type Base: Payload;
    /// Move into `Hub`
    fn hub(self) -> Result<Hub<Self::Base>>;
}

impl<T: IntoWing> IntoWingHub for T {
    type Base = T::Base;
    fn hub(self) -> Result<Hub<Self::Base>> {
        Ok(self.wing()?.into())
    }
}

pub trait WingOnly {}

pub trait ToItem {
    type Item;
    fn item(&self) -> &Self::Item;
}

pub trait BaseMut {
    type Base;
    fn base(&mut self) -> &mut Self::Base;
}

pub trait Read {
    type Item;
    /// Read the Unit or Payload of the graph part.
    fn read<T, F>(&self, reader: F) -> Result<T>
    where
        F: FnOnce(&Self::Item) -> T;
}

pub trait FromBase {
    type Base;
    fn from_base(base: Self::Base) -> Pointer<Self>;
}

pub trait WorkFromBase {
    type Base;
    fn from_base(base: Self::Base) -> Self;
}

pub trait SetRoot {
    fn set_root(&mut self, root: Root);
}

pub trait FromSnap {
    type Unit;
    fn from_snap(unit: Snap<Self::Unit>) -> Result<(Option<u16>, Pointer<Self>)>;
}

pub trait WorkFromSnap {
    type Unit;
    fn from_snap(unit: Snap<Self::Unit>) -> (Option<u16>, Self);
}

pub trait Clear {
    fn clear(&mut self);
}

pub trait BackIt {
    fn back(&mut self, back: &Back) -> Result<()>;
}

impl<T: Backed> BackIt for T {
    fn back(&mut self, back: &Back) -> Result<()> {
        *self = self.backed(back)?;
        Ok(())
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

// #[macro_export]
// macro_rules! Make {
//     (
//     $(#[$attr:meta])*
//     $pub:vis
//     struct $Unit:ident<$lt:lifetime> $tt:tt
//     ) => {
//         impl<'a> DemoBuilder<'a> {
//             make_func!($Unit);
//         }
//     };
// }

// ($Unit:ident<'_>) => {
//     impl paste! {[<$Unit "Builder">]<'static>} {
//         node_and_apex!($Unit);
//         pub fn hub(self) -> graph::Result<Hub<()>> {
//             Ok(self.node()?.hub())
//         }
//     }
// };

// #[macro_export]
// macro_rules! Make2 {
//     (
//     $(#[$attr:meta])*
//     $pub:vis
//     struct $Unit:ident $(<$($lt:tt),+>)? $tt:tt
//     ) => {
//         paste! {
//             impl $(<$($lt:tt),+>)? [<$Unit "Builder">] {
//                 make_func!($Unit);
//             }
//         }
//     };
// }

// pub trait ErrorToJsValue {
//     fn js_err(&self)
// }

// impl From<JsValue> for Error {
//     fn from(value: JsValue) -> Self {
//         Error::Any(anyhow!("crap"))
//     }
// }

// #[cfg(feature = "oneThread")]
// const IMMEDIATE_ACCESS: &str = "Item should be immediately accessible after creation.";

// #[cfg(not(feature = "oneThread"))]
// fn read_part_async<'a, P, F, O>(part: &'a Pointer<P>, read: F) -> Result<O>
// where
//     P: ?Sized,
//     F: FnOnce(RwLockReadGuard<'a, P>) -> O,
//     O: std::future::Future,
// {
//     Ok(read(part.read()))
// }

// #[cfg(feature = "oneThread")]
// fn read_part_async<'a, F, P, O>(part: &'a Pointer<P>, read: F) -> Result<O>
// where
//     P: ?Sized,
//     F: FnOnce(Ref<'a, P>) -> O,
//     O: std::future::Future,
// {
//     match part.try_borrow() {
//         Ok(part) => Ok(read(part)),
//         Err(err) => Err(Error::Read(err.to_string())),
//     }
// }

// #[cfg(not(feature = "oneThread"))]
// fn write_part_async<'a, P, F, O>(part: &'a Pointer<P>, write: F) -> Result<O>
// where
//     P: ?Sized,
//     F: FnOnce(RwLockWriteGuard<'a, P>) -> O,
//     O: std::future::Future,
// {
//     Ok(write(part.write()))
// }

// #[cfg(feature = "oneThread")]
// fn write_part_async<'a, F, P, O>(part: &'a Pointer<P>, write: F) -> Result<O>
// where
//     P: ?Sized,
//     F: FnOnce(RefMut<'a, P>) -> O,
//     O: std::future::Future,
// {
//     match part.try_borrow_mut() {
//         Ok(part) => Ok(write(part)),
//         Err(err) => Err(Error::Write(err.to_string())),
//     }
// }
