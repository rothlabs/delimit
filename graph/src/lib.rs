use serde::Serialize;

pub use edge::Edge;
pub use link::{Ace, Deuce, IntoAce, Link, Plan, Ploy, ToAce, Trey};
pub use meta::Meta;
pub use node::Node;
pub use react::{
    AddRoot, Back, Backed, PlanWithBack, PloyWithBack, React, Reactor, Rebut, Rebuter, Ring, Root,
    Update, Updater,
};
pub use read::{Grant, Read, Reader, Solve};
pub use role::Role;
pub use unit::{Gate, Repo, Serial, ToSerial};
pub use view::{AceView, ToViewsBuilder, View};
pub use write::{Grantor, Pack, Solver, Write, WriteWithBack, Writer, WriterWithPack};

pub mod edge;
pub mod link;
pub mod meta;
pub mod node;
pub mod react;
pub mod read;
pub mod role;
pub mod unit;
pub mod view;
pub mod work;
pub mod write;

const NO_POISON: &str = "the lock should not be poisoned";

pub struct Hold<L, R> {
    pub link: L,
    pub role: R,
}

/// Edge that grants a load. In addition, clone the edge with a new back,
pub trait Produce<L>: Grant<Load = L> + PloyWithBack<Load = L> {}

/// Edge that solves a task. In addition, clone the edge with a new Back.
pub trait Convert<T, L>: Solve<Task = T, Load = L> + PlanWithBack<Task = T, Load = L> {}

pub trait ToLoad {
    type Load;
    fn load(&self) -> Self::Load;
}

pub trait UsePloy {
    type Load;
    fn use_ploy<T: Grant<Load = Self::Load>>(&mut self, item: &T);
}

pub trait FromItem {
    type Item;
    fn new(item: Self::Item) -> Self;
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
