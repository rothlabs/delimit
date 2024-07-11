use serde::Serialize;

pub use edge::Edge;
pub use link::{IntoSole, Link, Pair, Sole, Solver, ToSole};
pub use meta::Meta;
pub use node::Node;
pub use react::{
    AddRoot, Cycle, Event, EventMut, EventReact, EventReactMut, React, ReactMut, Root, RootEdge,
    Ring, SolverWithRoot, TaskerWithRoot, ToRootEdge, WithRoot,
};
pub use read::{Read, Reader, Solve, SolveTask};
pub use unit::{Repo, Gate, Serial, ToSerial};
pub use view::{
    AddStr, AddToBaseViews, AddToViews, BaseView, SoleView, ToBaseViewsBuilder, ToViewsBuilder,
    View,
};
pub use write::{Pack, SolveMut, SolveTaskMut, Write, WriteWithRoot, Writer, WriterWithPack};

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

const NO_POISON: &str = "the lock should not be poisoned";

pub struct Hold<L, V> {
    pub link: L,
    pub view: V,
}

pub trait ToLoad {
    type Load;
    fn load(&self) -> Self::Load;
}

pub trait SolveShare<L>: Solve<Load = L> + SolverWithRoot<Load = L> {}

pub trait SolveTaskShare<T, L>:
    SolveTask<Task = T, Load = L> + TaskerWithRoot<Task = T, Load = L>
{
}

pub trait ToSolver {
    type Load;
    fn solver(&self) -> Solver<Self::Load>;
}

// pub trait ToTasker {
//     type Task;
//     type Load;
//     fn tasker(&self) -> link::Tasker<Self::Task, Self::Load>;
// }

pub trait AddStem {
    type Unit;
    fn add_stem<T, F: FnOnce(&mut Self::Unit, T)>(&mut self, stem: T, add_stem: F);
}

pub trait Clear {
    fn clear(&mut self);
}

pub trait FromItem {
    type Item;
    fn new(unit: Self::Item) -> Self;
}

pub trait Memory {
    type Task: Clone;
    type Load: Clone;
    fn add(&mut self, task: Self::Task, load: Self::Load);
    fn get(&self, task: &Self::Task) -> Option<&Self::Load>;
}

pub trait IntoRole {
    type Load;
    fn into_role(load: Self::Load) -> Self;
}

pub struct Role<L, E> {
    pub exact: E,
    pub solver: Solver<L>,
}

impl<L, E> Clone for Role<L, E>
where
    E: Clone,
{
    fn clone(&self) -> Self {
        Self {
            exact: self.exact.clone(),
            solver: self.solver.clone(),
        }
    }
}

impl<L, E> Solve for Role<L, E> {
    type Load = L;
    fn solve(&self) -> Self::Load {
        self.solver.solve()
    }
}

impl<L, E> WithRoot for Role<L, E>
where
    E: Clone,
{
    type Root = Root;
    fn with_root(&self, root: &Self::Root) -> Self {
        Self {
            exact: self.exact.clone(),
            solver: self.solver.with_reactor(root),
        }
    }
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
