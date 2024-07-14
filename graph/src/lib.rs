use serde::Serialize;

pub use bare::BareSole;
pub use edge::Edge;
pub use link::{IntoSole, Link, Pair, Plan, Ploy, Sole, ToSole, Trey};
pub use meta::Meta;
pub use node::Node;
pub use react::{
    AddRoot, Back, Backed, ConvertWithBack, Cycle, ProduceWithBack, React, Reactor, Rebut, Rebuter,
    Ring, Root, Update, Updater,
};
pub use read::{Grant, Read, Reader, Solve};
pub use role::Role;
pub use unit::{Gate, Repo, Serial, ToSerial};
pub use view::{AddStr, AddView, ToViewsBuilder, View};
pub use write::{Grantor, Pack, Solver, Write, WriteWithRoot, Writer, WriterWithPack};

pub mod bare;
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

pub struct Hold<L, V> {
    pub link: L,
    pub view: V,
}

pub trait ToLoad {
    type Load;
    fn load(&self) -> Self::Load;
}

/// Edge that grants a load. In addition, clone the edge with a new back,
pub trait Produce<L>: Grant<Load = L> + ProduceWithBack<Load = L> {}

/// Edge that solves a task. In addition, clone the edge with a new Back.
pub trait Convert<T, L>: Solve<Task = T, Load = L> + ConvertWithBack<Task = T, Load = L> {}

pub trait Clear {
    fn clear(&mut self);
}

pub trait FromItem {
    type Item;
    fn new(unit: Self::Item) -> Self;
}

pub trait FromSole {
    type Load;
    fn from_sole(sole: Sole<Self::Load>) -> Self;
}

pub trait FromString {
    fn from_string(string: &str) -> Self;
}

pub trait IntoView {
    type Item;
    fn into_view(load: Self::Item) -> Self;
}

impl<A, L> Serialize for Role<A, L>
where
    A: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.actual.serialize(serializer)
    }
}

// pub trait AddStem {
//     type Unit;
//     fn add_stem<T, F: FnOnce(&mut Self::Unit, T)>(&mut self, stem: T, add_stem: F);
// }

// pub trait Memory {
//     type Task;
//     type Load;
//     fn add(&mut self, task: Self::Task, load: Self::Load);
//     fn get(&self, task: &Self::Task) -> Option<&Self::Load>;
// }
