pub mod edge;
pub mod link;
pub mod meta;
pub mod node;
pub mod react;
pub mod read;
pub mod repo;
pub mod work;
pub mod write;

pub use link::{Leaf, UnitSolver, Stemmer, ToLeaf};
pub use meta::Meta;
pub use react::{AddReactor, React, Reactor, Reactors, SolverWithReactor, ToReactor, WithReactor};
pub use read::{CloneUnit, Read, Reader, Solve};
pub use repo::Repo;
pub use work::Work;
pub use write::{SolveMut, Write, Writer, WriteWithReactor, WriterWithReactor};

pub trait ToSolver {
    type Task;
    type Load;
    fn to_solver(&self) -> link::Solver<Self::Task, Self::Load>;
}

pub trait SolveReact<T, L>: Solve<Task = T, Load = L> + SolverWithReactor<Task = T, Load = L> {}

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
