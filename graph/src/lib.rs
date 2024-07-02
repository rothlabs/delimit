pub mod edge;
pub mod link;
pub mod meta;
pub mod node;
pub mod react;
pub mod read;
pub mod repo;
pub mod work;
pub mod write;

pub use link::{Leaf, Stemmer, ToLeaf, IntoLeaf, UnitSolver, UnitTasker};
pub use meta::Meta;
pub use react::{AddReactor, React, Reactor, Reactors, SolverWithReactor, TaskerWithReactor, ToReactor, WithReactor};
pub use read::{CloneUnit, Read, Reader, Solve, SolveTask};
pub use repo::Repo;
pub use work::Work;
pub use write::{SolveMut, SolveTaskMut, Write, WriteWithReactor, Writer, WriterWithReactor};

pub trait SolveShare<L>:
    Solve<Load = L> + SolverWithReactor<Load = L>
{
}

pub trait SolveTaskShare<T, L>:
    SolveTask<Task = T, Load = L> + TaskerWithReactor<Task = T, Load = L>
{
}

pub trait ToSolver {
    type Load;
    fn to_solver(&self) -> link::Solver<Self::Load>;
}

pub trait ToTasker {
    type Task;
    type Load;
    fn to_tasker(&self) -> link::Tasker<Self::Task, Self::Load>;
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

/// Make a string. ToLeaf comes for free.
pub trait GraphString {
    fn string(&self) -> String;
}

pub trait Memory {
    type Task: Clone;
    type Load: Clone;
    fn add(&mut self, task: Self::Task, load: Self::Load);
    fn get(&self, task: &Self::Task) -> Option<&Self::Load>;
}

const NO_POISON: &str = "the lock should not be poisoned";
