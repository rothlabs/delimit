pub use apex::Apex;
pub use edge::Edge;
pub use edit::Field;
pub use link::{Leaf, Agent, Link, Ploy, ToAce};
pub use load::Load;
pub use meta::{Meta, ToMeta};
pub use node::{Node, RankDown};
pub use react::{
    AddRoot, Back, Backed, BackedPloy, DoAddRoot, DoReact, DoRebut, DoUpdate, React, Rebut, Ring,
    Root, ToPipedPloy, ToPloy, Update,
};
pub use repo::Repo;
pub use solve::{DoSolve, IntoTray, Query, Solve, ToQuery, Tray};
pub use write::{
    Pack, WriteLoad, WriteLoadOut, WriteLoadWork, WriteUnit, WriteUnitOut, WriteUnitWork,
};

use std::error;
#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
#[cfg(feature = "oneThread")]
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub mod node;
pub mod react;
pub mod solve;

mod apex;
mod edge;
mod edit;
mod link;
mod load;
mod meta;
mod repo;
mod work;
mod write;

//trait GraphError: error::Error + SendSync {}

#[cfg(not(feature = "oneThread"))]
pub type Error = Box<dyn error::Error + Send + Sync>;
#[cfg(feature = "oneThread")]
pub type Error = Box<dyn error::Error>;

#[cfg(not(feature = "oneThread"))]
const NO_POISON: &str = "the lock should not be poisoned";

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

/// Edge that grants a load. It can also clone the edge with a new back.
pub trait Engage: Solve + BackedPloy + AddRoot + Update {}

#[cfg(not(feature = "oneThread"))]
type PloyEdge = Arc<RwLock<Box<dyn Engage>>>;
#[cfg(feature = "oneThread")]
type PloyEdge = Rc<RefCell<Box<dyn Engage>>>;

pub trait ToAgent
where
    Self: Solve + Sized,
{
    fn agent(&self) -> Agent<Self>;
}

impl<U> ToAgent for U
where
    U: 'static + Backed + Solve + SendSync,
{
    fn agent(&self) -> Agent<Self> {
        Agent::make(|back| self.backed(back))
    }
}

pub trait ToNode {
    fn node(&self) -> Node;
}

impl<U> ToNode for U
where
    U: 'static + ToAgent + Solve + SendSync,
{
    fn node(&self) -> Node {
        self.agent().ploy().into()
    }
}

impl ToNode for Leaf {
    fn node(&self) -> Node {
        self.into()
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

pub trait DoReadLoad {
    fn do_read_load(&self) -> load::ResultRef;
}

pub trait ReadLoad {
    fn read_load<T, F: FnOnce(load::ResultRef) -> T>(&self, read: F) -> T;
}

pub trait ToLoad {
    type Load;
    /// Clone the Load out of the graph part.
    /// May cause stem nodes to generate the load.
    fn load(&self) -> Self::Load;
}

pub trait FromItem {
    type Item;
    fn new(item: Self::Item) -> Self;
}

pub trait Make {
    type Unit;
    fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self;
}

pub trait DoMake {
    type Unit;
    fn do_make<F: FnOnce(&Back) -> Self::Unit>(&mut self, make: F, back: &Back);
}

pub trait Clear {
    fn clear(&mut self);
}
