use serde::Serialize;

pub use edge::Edge;
pub use link::{IntoSole, Link, Trey, Pair, Sole, Solver, Tasker, ToSole};
pub use meta::Meta;
pub use node::Node;
pub use react::{
    AddRoot, Cycle, Event, EventMut, EventReact, EventReactMut, React, ReactMut, Ring, Root,
    RootEdge, SolverWithRoot, TaskerWithRoot, ToRootEdge, WithRoot,
};
pub use read::{Read, Reader, Grant, SolveTask};
pub use unit::{Gate, Repo, Serial, ToSerial};
pub use view::{
    AddStr, AddToLoadViews, AddToViews, LoadView, SoleView, ToLoadViewsBuilder, ToViewsBuilder,
    View,
};
pub use write::{Pack, Grantor, SolveTaskMut, Write, WriteWithRoot, Writer, WriterWithPack};
pub use role::{SolveRole, TaskRole, IntoRole};

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

pub trait Formula<L>: Grant<Load = L> + SolverWithRoot<Load = L> {}

pub trait TaskShare<T, L>:
    SolveTask<Task = T, Load = L> + TaskerWithRoot<Task = T, Load = L>
{
}

pub trait ToSolver {
    type Load;
    fn solver(&self) -> Solver<Self::Load>;
}

pub trait ToTasker {
    type Task;
    type Load;
    fn tasker(&self) -> Tasker<Self::Task, Self::Load>;
}

pub trait Clear {
    fn clear(&mut self);
}

pub trait FromItem {
    type Item;
    fn new(unit: Self::Item) -> Self;
}

impl<L, E> Serialize for SolveRole<L, E>
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







// pub trait FromLoad {
//     type Load;
//     fn from_load(unit: Self::Load) -> Self;
// }

// pub trait SolveWithRoot {
//     type Load;
//     fn solve_with_root(&self, root: &Root) -> Self::Load;
// }

// pub trait Stemmer {
//     type Unit;
//     fn stemmer<T: WithReactor, F: FnOnce(&mut Self::Unit, T)>(&self, stem: &T, add_stem: F);
// }

// impl<L: 'static> SolveLay<L> {
//     pub fn read<F: FnOnce(&L)>(&self, read: F) {
//         match self {
//             SolveLay::Bare(unit) => read(unit),
//             SolveLay::Leaf(leaf) => leaf.reader(read),
//             SolveLay::Solver(_) => panic!("cannot read link::Solver<U>"),
//         };
//     }
// }

// /// Make a string. ToLeaf comes for free.
// pub trait GraphString {
//     fn string(&self) -> String;
// }
