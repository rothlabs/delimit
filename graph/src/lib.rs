use serde::Serialize;

pub use edge::Edge;
pub use link::{IntoSole, Link, Trey, Pair, Sole, Ploy, Plan, ToSole};
pub use meta::Meta;
pub use node::Node;
pub use react::{
    AddRoot, Cycle, Rebut, Rebuter, Update, Updater, React, Reactor, Ring, Back,
    Root, ProduceWithBack, ConvertWithBack, WithRoot,
};
pub use read::{Read, Reader, Grant, Solve};
pub use unit::{Gate, Repo, Serial, ToSerial};
pub use view::{
    AddStr, AddToLoadViews, AddToViews, PloyView, SoleView, ToLoadViewsBuilder, ToViewsBuilder,
    View,
};
pub use write::{Pack, Grantor, Solver, Write, WriteWithRoot, Writer, WriterWithPack};
pub use role::{Role, IntoRole};

pub mod edge;
pub mod link;
pub mod meta;
pub mod node;
pub mod react;
pub mod read;
pub mod unit;
pub mod view;
pub mod work;
pub mod write;
pub mod role;

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

impl<L, E> Serialize for Role<L, E>
where
    E: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.exact.serialize(serializer)
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