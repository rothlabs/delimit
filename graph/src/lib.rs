pub use adapt::{adapt_ok, no_adapter, Adapt, AdaptInner, Memo, Post};
pub use apex::Apex;
pub use bay::Bay;
pub use edge::Edge;
pub use lake::Lake;
pub use link::{Agent, Leaf, Link, Ploy, ToLeaf};
pub use load::Load;
pub use meta::{random, Id, Key, Path, ToId};
pub use node::{Node, EngageNodes};
pub use react::{
    AddRoot, Back, Backed, BackedPloy, DoAddRoot, DoReact, DoRebut, DoUpdate, React, Rebut, Ring,
    Root, ToPipedPloy, ToPloy, Update,
};
pub use serial::{DeserializeNode, ToHash, ToSerial};
pub use solve::{empty_nodes, no_solver, DoSolve, IntoTray, Solve, Task, Tray};
pub use write::{
    Pack, WriteLoad, WriteLoadOut, WriteLoadWork, WriteUnit, WriteUnitOut, WriteUnitWork,
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
pub mod lake;
pub mod node;
pub mod react;
pub mod serial;
pub mod solve;

mod apex;
mod bay;
mod edge;
mod link;
mod load;
mod meta;
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
/// Trade a node for another.
/// The implmentation should return the same semantic node with different graph qualities.
pub trait Trade: DynClone + Debug {
    /// Trade a node for another.
    fn trade(&self, node: &Node) -> Node;
}

pub trait IntoAgent
where
    Self: Sized,
{
    fn agent(self) -> Agent<Self>;
}

impl<T> IntoAgent for T
where
    T: 'static + Adapt + Solve + SendSync,
{
    fn agent(mut self) -> Agent<Self> {
        Agent::make(|back| {
            self.adapt(Post::Trade(back))
                .expect("To move into Agent, unit must Adapt with Post::Trade.");
            self
        })
    }
}

pub trait IntoNode {
    /// Move into `Node`.
    fn node(self) -> Node;
}

impl<T> IntoNode for T
where
    T: 'static + IntoAgent + Solve + Adapt + Debug + SendSync,
{
    fn node(self) -> Node {
        self.agent().ploy().into()
    }
}

impl IntoNode for Leaf {
    fn node(self) -> Node {
        self.into()
    }
}

pub trait ToNode {
    /// Place inside a Node.
    fn node(&self) -> Node;
}

impl<T> ToNode for Agent<T>
where
    T: 'static + Solve + Adapt + Debug + SendSync,
{
    fn node(&self) -> Node {
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

// impl<T> ToNode for T
// where
//     T: 'static + ToAgent + Solve + Adapt + Debug + SendSync,
// {
//     fn node(&self) -> Node {
//         self.agent().ploy().into()
//     }
// }
