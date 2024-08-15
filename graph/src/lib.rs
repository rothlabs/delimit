pub use adapt::{adapt_ok, no_adapter, Adapt, AdaptInner, Memo, Post};
pub use apex::{Apex, EngageApexes};
pub use bay::Bay;
pub use cusp::Cusp;
pub use edge::Edge;
pub use lake::Lake;
pub use link::{Leaf, Link, Node, Ploy, ToLeaf};
pub use meta::{random, Id, Key, Path, ToId};
pub use react::{
    AddRoot, Back, Backed, BackedPloy, DoAddRoot, DoReact, DoRebut, DoUpdate, React, Rebut, Ring,
    Root, ToPipedPloy, ToPloy, Update,
};
pub use serial::{DeserializeApex, ToHash, ToSerial};
pub use solve::{empty_apexes, no_solver, DoSolve, Gain, IntoGain, Solve, Task};
pub use tray::Tray;
pub use write::{
    Pack, WriteTray, WriteTrayOut, WriteTrayWork, WriteUnit, WriteUnitOut, WriteUnitWork,
};

use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
#[cfg(feature = "oneThread")]
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};
use std::{
    error,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

pub mod adapt;
pub mod apex;
pub mod lake;
pub mod react;
pub mod serial;
pub mod solve;

mod bay;
mod cusp;
mod edge;
mod link;
mod meta;
mod tray;
mod work;
mod write;

#[cfg(not(feature = "oneThread"))]
/// Graph Error
pub type Error = Box<dyn error::Error + Send + Sync>;
#[cfg(feature = "oneThread")]
/// Graph Error
pub type Error = Box<dyn error::Error>;

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
) -> O {
    read(part.read().expect(NO_POISON))
}

#[cfg(feature = "oneThread")]
fn read_part<P: ?Sized, O, F: FnOnce(Ref<P>) -> O>(part: &Rc<RefCell<P>>, read: F) -> O {
    read(part.borrow())
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

/// General engagement of Ploy with erased unit type.
pub trait Engage: Solve + AdaptInner + BackedPloy + AddRoot + Update + Debug {}

#[cfg(not(feature = "oneThread"))]
type PloyEdge = Arc<RwLock<Box<dyn Engage>>>;
#[cfg(feature = "oneThread")]
type PloyEdge = Rc<RefCell<Box<dyn Engage>>>;

dyn_clone::clone_trait_object!(Trade);
/// Trade a apex for another.
/// The implmentation should return the same semantic apex with different graph qualities.
pub trait Trade: DynClone + Debug {
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
    T: 'static + Adapt + Solve + SendSync,
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
    /// Move into `Apex`.
    fn apex(self) -> Apex;
}

impl<T> IntoApex for T
where
    T: 'static + IntoNode + Solve + Adapt + Debug + SendSync,
{
    fn apex(self) -> Apex {
        self.node().ploy().into()
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
        self.ploy().into()
    }
}

pub trait DoRead {
    type Item;
    fn do_read(&self) -> &Self::Item;
}

pub trait Read {
    type Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T;
}

pub trait DoReadTray {
    fn do_read_tray(&self) -> tray::ResultRef;
}

pub trait ReadTray {
    fn read_tray<T, F: FnOnce(tray::ResultRef) -> T>(&self, read: F) -> T;
}

pub trait ToTray {
    type Tray;
    /// Clone the Tray out of the graph part.
    /// May cause stem apexes to generate the tray.
    fn tray(&self) -> Self::Tray;
}

pub trait FromItem {
    type Item;
    fn new(item: Self::Item) -> Self;
}

pub trait Make {
    type Unit;
    fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self;
}

pub trait MakeInner {
    type Unit;
    fn do_make<F: FnOnce(&Back) -> Self::Unit>(&mut self, make: F, back: &Back);
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
