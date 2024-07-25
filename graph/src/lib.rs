pub use edge::Edge;
pub use link::{Ace, Agent, Asset, Deuce, Envoy, IntoAce, Link, Pipe, Plan, Ploy, ToAce, Trey};
pub use meta::{Meta, ToMeta};
pub use node::Node;
pub use react::{
    AddRoot, Back, Backed, BackedPlan, BackedPloy, DoAddRoot, DoReact, DoRebut, DoUpdate, React,
    Rebut, Ring, Root, ToPlan, ToPloy, Update,
};
pub use role::Role;
pub use unit::{Gate, Repo, Serial, ToSerial};
pub use view::{AddBase, ToViewsMutator, Value, View, ViewsBuilder};
pub use write::{
    Pack, WriteLoad, WriteLoadOut, WriteLoadWork, WriteUnit, WriteUnitOut, WriteUnitWork,
};

use serde::Serialize;
#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
#[cfg(feature = "oneThread")]
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub mod react;
pub mod role;
pub mod view;

mod edge;
mod link;
mod meta;
mod node;
mod unit;
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

/// For units to provide a load by task and NOT act upon externals
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
    /// Clone a Load (payload) out of the graph part
    /// The graph part might perform calculations to generate the load.
    fn load(&self) -> Self::Load;
}

pub trait ToLoadByTask {
    type Task;
    type Load;
    fn load(&self, task: Self::Task) -> Self::Load;
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

pub trait FromAce {
    type Load;
    fn from_ace(ace: &Ace<Self::Load>) -> Self;
}

pub trait IntoView {
    type Item;
    fn into_view(item: Self::Item) -> Self;
}

pub trait AddAce {
    type Load;
    fn add_ace(&mut self, ace: Ace<Self::Load>);
}

// pub trait AddBacked<T> {
//     fn add_backed(&mut self, item: T) -> &mut Self;
// }

pub trait AddStr {
    fn add_str(&mut self, str: &'static str);
}

pub trait Clear {
    fn clear(&mut self);
}

// pub trait UsePloy {
//     type Load;
//     fn use_ploy<T: Grant<Load = Self::Load>>(&mut self, item: &T);
// }

// pub trait Memory {
//     type Task;
//     type Load;
//     fn add(&mut self, task: Self::Task, load: Self::Load);
//     fn get(&self, task: &Self::Task) -> Option<&Self::Load>;
// }

// pub trait AddStem {
//     type Unit;
//     fn add_stem<T, F: FnOnce(&mut Self::Unit, T)>(&mut self, stem: T, add_stem: F);
// }
