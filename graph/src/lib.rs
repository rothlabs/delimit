pub mod edge;
pub mod link;
pub mod meta;
pub mod node;
pub mod react;
pub mod read;
pub mod repo;
pub mod unit;
pub mod view;
pub mod work;
pub mod write;

pub use link::{IntoLeaf, Leaf, Solver, Stemmer, ToLeaf, UnitSolver, UnitTasker};
pub use meta::Meta;
pub use react::{
    AddReactor, React, Reactor, Reactors, SolverWithReactor, TaskerWithReactor, ToReactor,
    WithReactor,
};
pub use read::{Read, Reader, Solve, SolveTask};
pub use repo::Repo;
pub use unit::Gate;
pub use view::{AddStr, AddToLeafView, LeafView};
pub use work::Work;
pub use write::{
    SolveMut, SolveTaskMut, Write, WriteWithReactor, Writer, WriterPack, WriterWithPack,
};

const NO_POISON: &str = "the lock should not be poisoned";

pub trait SolveShare<L>: Solve<Load = L> + SolverWithReactor<Load = L> {}

pub trait SolveTaskShare<T, L>:
    SolveTask<Task = T, Load = L> + TaskerWithReactor<Task = T, Load = L>
{
}

pub trait ToSolver {
    type Load;
    fn solver(&self) -> link::Solver<Self::Load>;
}

pub trait ToTasker {
    type Task;
    type Load;
    fn tasker(&self) -> link::Tasker<Self::Task, Self::Load>;
}

pub trait AddStem {
    type Unit;
    fn add_stem<T, F: FnOnce(&mut Self::Unit, T)>(&mut self, stem: T, add_stem: F);
}

pub trait Clear {
    fn clear(&mut self);
}

pub trait FromUnit {
    type Unit;
    fn new(unit: Self::Unit) -> Self;
}

pub trait Memory {
    type Task: Clone;
    type Load: Clone;
    fn add(&mut self, task: Self::Task, load: Self::Load);
    fn get(&self, task: &Self::Task) -> Option<&Self::Load>;
}

#[derive(Clone)]
pub struct Role<L, E> {
    pub exact: E,
    pub solver: Solver<L>,
}

impl<L, E> WithReactor for Role<L, E>
where
    E: Clone,
{
    fn with_reactor(&self, reactor: &Reactor) -> Self {
        Self {
            exact: self.exact.clone(),
            solver: self.solver.with_reactor(reactor),
        }
    }
}

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
