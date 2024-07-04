pub mod edge;
pub mod link;
pub mod meta;
pub mod node;
pub mod react;
pub mod read;
pub mod repo;
pub mod work;
pub mod write;
pub mod unit;

pub use link::{IntoLeaf, Leaf, Stemmer, ToLeaf, Solver, UnitSolver, UnitTasker};
pub use meta::Meta;
pub use react::{
    AddReactor, React, Reactor, Reactors, SolverWithReactor, TaskerWithReactor, ToReactor,
    WithReactor,
};
pub use read::{Read, Reader, Solve, SolveTask};
pub use repo::Repo;
pub use work::Work;
pub use write::{SolveMut, SolveTaskMut, Write, WriteWithReactor, Writer, WriterWithReactor, WriterPack};
pub use unit::Gate;

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

pub enum SolveLay<L> {
    Bare(L),
    Leaf(Leaf<L>),
    Solver(Solver<L>),
}

impl<L: Clone> SolveLay<L> {
    pub fn solve(&self) -> L {
        match self {
            SolveLay::Bare(unit) => unit.clone(),
            SolveLay::Leaf(leaf) => leaf.solve(),
            SolveLay::Solver(solver) => solver.solve(),
        }
    }
}

const NO_POISON: &str = "the lock should not be poisoned";


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