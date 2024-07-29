pub use edge::Edge;
pub use link::{Ace, Agent, Deuce, Envoy, IntoAce, Link, Pipe, Plan, Ploy, ToAce, Trey};
pub use meta::{Meta, ToMeta};
pub use apex::Apex;
pub use react::{
    AddRoot, Back, Backed, BackedPlan, BackedPloy, DoAddRoot, DoReact, DoRebut, DoUpdate, React,
    Rebut, Ring, Root, ToPipedPloy, ToPlan, ToPloy, Update,
};
pub use repo::Repo;
pub use node::{RankDown, Node};
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

pub mod react;
pub mod node;

mod edge;
mod edit;
mod link;
mod meta;
mod apex;
mod repo;
mod work;
mod write;

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
#[cfg(not(feature = "oneThread"))]
pub trait Produce<L>: Grant<Load = L> + BackedPloy<Load = L> + AddRoot + Update {}
/// Edge that grants a load. It can also clone the edge with a new back.
#[cfg(feature = "oneThread")]
pub trait Produce<L>: Grant<Load = L> + BackedPloy<Load = L> + AddRoot + Update {}

/// Edge that solves a task. It can also clone the edge with a new Back.
#[cfg(not(feature = "oneThread"))]
pub trait Convert<T, L>:
    Solve<Task = T, Load = L> + BackedPlan<Task = T, Load = L> + AddRoot + Update
{
}
#[cfg(feature = "oneThread")]
pub trait Convert<T, L>:
    Solve<Task = T, Load = L> + BackedPlan<Task = T, Load = L> + AddRoot + Update
{
}

#[derive(Clone)]
pub struct Hold<L, R> {
    pub link: L,
    pub role: R,
}

pub trait DoRead {
    type Item;
    fn do_read(&self) -> &Self::Item;
}

pub trait Read {
    type Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T;
}

pub trait ReadByTask {
    type Task;
    type Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, task: Self::Task, read: F) -> T;
}

// pub trait Give<T> {
//     fn give(&self) -> Sel;
// }

/// For units to grant a load and NOT act upon external systems
pub trait Grant {
    type Load;
    fn grant(&self) -> Self::Load;
}

/// For graph internals to handle grant calls
pub trait DoGrant {
    type Load;
    fn do_grant(&mut self, back: &Back) -> Self::Load;
}

pub trait Solve {
    type Task;
    type Load;
    fn solve(&self, task: Self::Task) -> Self::Load;
}

/// For graph internals to handle solve calls
pub trait DoSolve {
    type Task;
    type Load;
    fn do_solve(&mut self, task: Self::Task) -> Self::Load;
}

pub trait Act {
    type Load;
    /// Unit will act upon externals and provide a load
    fn act(&self) -> Self::Load;
}

pub trait DoAct {
    type Load;
    /// For graph internals to handle act calls
    fn do_act(&mut self, back: &Back) -> Self::Load;
}

/// For units to act upon externals and provide a load by task
pub trait Serve {
    type Task;
    type Load;
    fn serve(&self, task: Self::Task) -> Self::Load;
}

/// For graph internals to handle serve calls
pub trait DoServe {
    type Task;
    type Load;
    fn do_serve(&mut self, task: Self::Task) -> Self::Load;
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