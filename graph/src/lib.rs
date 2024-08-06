pub use adapt::{did_not_adapt, Adapt, AdaptInner, Gain, Post};
pub use adapt::{post, ToAlter};
pub use apex::Apex;
use dyn_clone::DynClone;
pub use edge::Edge;
pub use link::{Agent, Leaf, Link, Ploy, ToLeaf};
pub use load::Load;
pub use meta::{Id, Meta, ToMeta};
pub use node::{Node, RankDown, TradeNodes};
pub use react::{
    AddRoot, Back, Backed, BackedPloy, DoAddRoot, DoReact, DoRebut, DoUpdate, React, Rebut, Ring,
    Root, ToPipedPloy, ToPloy, Update,
};
pub use repo::Repo;
use serde::{Deserialize, Serialize};
pub use serial::{Serial, SerializeGraph, SerializeGraphInner};
pub use solve::{did_not_solve, DoSolve, IntoTray, Query, Solve, Task, ToQuery, Tray};
pub use write::{
    Pack, WriteLoad, WriteLoadOut, WriteLoadWork, WriteUnit, WriteUnitOut, WriteUnitWork,
};

#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
#[cfg(feature = "oneThread")]
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};
use std::{error, result};

pub mod adapt;
pub mod node;
pub mod react;
pub mod serial;
pub mod solve;

mod apex;
mod edge;
mod link;
mod load;
mod meta;
mod repo;
mod work;
mod write;

pub const SAVE: &str = "save";
pub const LOAD: &str = "load";
pub const FIND: &str = "find";

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
pub trait Engage:
    Solve + AdaptInner + BackedPloy + AddRoot + Update + SerializeGraph + std::fmt::Debug
{
}

#[cfg(not(feature = "oneThread"))]
type PloyEdge = Arc<RwLock<Box<dyn Engage>>>;
#[cfg(feature = "oneThread")]
type PloyEdge = Rc<RefCell<Box<dyn Engage>>>;

dyn_clone::clone_trait_object!(DeserializeNode);
pub trait DeserializeNode: DynClone + std::fmt::Debug + SendSync {
    fn deserialize(&self, string: &str) -> result::Result<Node, Error>;
}

dyn_clone::clone_trait_object!(Trade);
pub trait Trade: DynClone + std::fmt::Debug {
    fn trade(&self, node: &Node) -> Node;
}

#[derive(Clone, Debug)]
struct BackTrader {
    back: Back,
}

impl BackTrader {
    fn new(back: &Back) -> Box<Self> {
        Box::new(Self { back: back.clone() })
    }
}

impl Trade for BackTrader {
    fn trade(&self, node: &Node) -> Node {
        node.backed(&self.back)
    }
}

pub trait ToAgent
where
    Self: Sized,
{
    fn agent(&self) -> Agent<Self>;
}

impl<T> ToAgent for T
where
    T: 'static + Adapt + Solve + Clone + SendSync,
{
    fn agent(&self) -> Agent<Self> {
        Agent::make(|back| {
            let mut unit = self.clone();
            let trade = BackTrader::new(back);
            unit.adapt(Post::Trade(trade))
                .expect("unit must handle Post::Trade");
            unit
        })
        //Agent::make(|back| self.make(back))
    }
}

pub trait ToNode {
    /// Place inside a Node.
    fn node(&self) -> Node;
}

impl<T> ToNode for T
where
    T: 'static + ToAgent + Solve + Adapt + Serialize + std::fmt::Debug + SendSync,
{
    fn node(&self) -> Node {
        self.agent().ploy().into()
    }
}

impl<T> ToNode for Agent<T>
where
    T: 'static + Solve + Adapt + Serialize + std::fmt::Debug + SendSync,
{
    fn node(&self) -> Node {
        self.ploy().into()
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

pub trait MakeInner {
    type Unit;
    fn do_make<F: FnOnce(&Back) -> Self::Unit>(&mut self, make: F, back: &Back);
}

pub trait Clear {
    fn clear(&mut self);
}

// pub trait Make {
//     fn make(&self, back: &Back) -> Self;
// }
