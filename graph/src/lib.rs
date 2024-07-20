pub use edge::Edge;
pub use link::{Ace, Deuce, IntoAce, Link, Pipe, Plan, Ploy, ToAce, Trey, Agent};
pub use meta::Meta;
pub use node::Node;
pub use react::{
    DoAddRoot, Back, Backed, BackedPlan, BackedPloy, DoReact, React, DoRebut, Rebut, Ring, Root,
    AddRoot, ToPlan, ToPloy, DoUpdate, Update,
};
pub use role::Role;
pub use unit::{Gate, Repo, Serial, ToSerial};
pub use view::{ToViewsBuilder, View};
pub use write::{Pack, DoWrite, WriteWithBack, Write, WriteWithPack};

use serde::Serialize;

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
pub trait Produce<L>: Grant<Load = L> + BackedPloy<Load = L> + AddRoot + Update {}

/// Edge that solves a task. In addition, clone the edge with a new Back.
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
