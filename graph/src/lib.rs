pub use edge::Edge;
pub use link::{Ace, Agent, Deuce, IntoAce, Link, Pipe, Plan, Ploy, ToAce, Trey};
pub use meta::Meta;
pub use node::Node;
pub use react::{
    AddRoot, Back, Backed, BackedPlan, BackedPloy, DoAddRoot, DoReact, DoRebut, DoUpdate, React,
    Rebut, Ring, Root, ToPlan, ToPloy, Update,
};
pub use role::Role;
pub use unit::{Gate, Repo, Serial, ToSerial};
pub use view::{ToViewsMutator, View, ViewsBuilder};
pub use write::{DoWrite, Pack, Write, WriteWithBack, WriteWithPack};

use serde::Serialize;
#[cfg(not(feature="oneThread"))]
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
#[cfg(feature="oneThread")]
use std::{cell::{RefCell, Ref, RefMut}, rc::Rc};

pub mod role;
pub mod view;
// pub mod pipe;

mod edge;
mod link;
mod meta;
mod node;
mod react;
mod unit;
mod work;
mod write;

const NO_POISON: &str = "the lock should not be poisoned";

#[cfg(not(feature="oneThread"))]
pub trait Threading: Send + Sync {}
#[cfg(not(feature="oneThread"))]
impl<T: Send + Sync> Threading for T {}

#[cfg(feature="oneThread")] 
pub trait Threading {}
#[cfg(feature="oneThread")] 
impl<T> Threading for T {}



#[cfg(not(feature="oneThread"))]
fn read_part<E: ?Sized, O, F: FnOnce(RwLockReadGuard<E>) -> O>(edge: &Arc<RwLock<E>>, read: F) -> O {
    read(edge.read().expect(NO_POISON))
}

#[cfg(feature="oneThread")] 
fn read_part<E: ?Sized, O, F: FnOnce(Ref<E>) -> O>(edge: &Rc<RefCell<E>>, read: F) -> O {
    read(edge.borrow())
}

#[cfg(not(feature="oneThread"))]
fn write_part<E: ?Sized, O, F: FnOnce(RwLockWriteGuard<E>) -> O>(edge: &Arc<RwLock<E>>, write: F) -> O {
    write(edge.write().expect(NO_POISON))
}

#[cfg(feature="oneThread")] 
fn write_part<E: ?Sized, O, F: FnOnce(RefMut<E>) -> O>(edge: &Rc<RefCell<E>>, write: F) -> O {
    write(edge.borrow_mut())
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

pub trait ReaderByTask {
    type Task;
    type Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, task: Self::Task, read: F) -> T;
}

/// impl for units that do not act upon external systems
pub trait Grant {
    type Load;
    fn grant(&self) -> Self::Load;
}

pub trait DoGrant {
    type Load;
    fn do_grant(&mut self, back: &Back) -> Self::Load;
}

/// impl for units that act upon external systems
pub trait Act {
    type Load;
    fn act(&self) -> Self::Load;
}

pub trait DoAct {
    type Load;
    fn do_act(&mut self, back: &Back) -> Self::Load;
}

pub trait Solve {
    type Task;
    type Load;
    fn solve(&self, task: Self::Task) -> Self::Load;
}

pub trait DoSolve {
    type Task;
    type Load;
    fn do_solve(&mut self, task: Self::Task) -> Self::Load;
}

/// Edge that grants a load. In addition, clone the edge with a new back,
#[cfg(not(feature="oneThread"))]
pub trait Produce<L>: Grant<Load = L> + BackedPloy<Load = L> + AddRoot + Update + Send + Sync {}
#[cfg(feature="oneThread")] 
pub trait Produce<L>: Grant<Load = L> + BackedPloy<Load = L> + AddRoot + Update {}

/// Edge that solves a task. In addition, clone the edge with a new Back.
#[cfg(not(feature="oneThread"))]
pub trait Convert<T, L>:
    Solve<Task = T, Load = L> + BackedPlan<Task = T, Load = L> + AddRoot + Update + Send + Sync
{
}
#[cfg(feature="oneThread")] 
pub trait Convert<T, L>:
    Solve<Task = T, Load = L> + BackedPlan<Task = T, Load = L> + AddRoot + Update
{
}

pub trait ToLoad {
    type Load;
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
    fn from_ace(ace: Ace<Self::Load>) -> Self;
}

pub trait IntoView {
    type Item;
    fn into_view(item: Self::Item) -> Self;
}

pub trait AddAce {
    type Load;
    fn add_ace(&mut self, ace: Ace<Self::Load>);
}

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
