pub use apex::Apex;
pub use edge::Edge;
pub use link::{Ace, Agent, Deuce, Envoy, IntoAce, Link, Pipe, Plan, Ploy, ToAce, Trey};
pub use meta::{Meta, ToMeta};
pub use node::{Node, RankDown};
pub use react::{
    AddRoot, Back, Backed, BackedPlan, BackedPloy, DoAddRoot, DoReact, DoRebut, DoUpdate, React,
    Rebut, Ring, Root, ToPipedPloy, ToPlan, ToPloy, Update,
};
pub use repo::Repo;
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

pub mod node;
pub mod react;

mod apex;
mod edge;
mod edit;
mod link;
mod meta;
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

pub trait ToDeuce
where
    Self: Grant + Sized,
{
    fn link(&self) -> Deuce<Self>;
}

// impl<U> MakeLink for U
// where
//     U: 'static + ToLink + Grant + SendSync,
//     U::Load: SendSync
// {
//     fn link(&self) -> Deuce<Self> {
//         Deuce::make(|back| self.to_link(back))
//     }
// }

// pub trait ToLink {
//     fn to_link(&self, back: &Back) -> Self;
// }

impl<U> ToDeuce for U
where
    U: 'static + Backed + Grant + SendSync,
    U::Load: SendSync,
{
    fn link(&self) -> Deuce<Self> {
        Deuce::make(|back| self.backed(back))
    }
}

pub trait ToNode<L> {
    fn node(&self) -> Node<L>;
}

impl<U, L> ToNode<L> for U
where
    U: 'static + ToDeuce + Grant<Load = Node<L>> + SendSync,
    L: 'static + Clone + Default + SendSync,
{
    fn node(&self) -> Node<L> {
        self.link().ploy().into()
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

pub trait ReadByTask {
    type Task;
    type Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, task: Self::Task, read: F) -> T;
}

pub trait Grant {
    type Load;
    /// For units to grant a load and NOT act upon external systems
    fn grant(&self) -> Self::Load;
}

pub trait DoGrant {
    type Load;
    /// For graph internals to handle grant calls
    fn do_grant(&mut self, back: &Back) -> Self::Load;
}

pub trait Solve {
    type Task;
    type Load;
    /// Unit solves a task and does NOT act on externals
    fn solve(&self, task: Self::Task) -> Self::Load;
}

pub trait DoSolve {
    type Task;
    type Load;
    /// For graph internals to handle solve calls
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
