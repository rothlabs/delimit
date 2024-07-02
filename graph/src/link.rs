use crate::*;

pub use leaf::{Leaf, ToLeaf};
pub use solver::Solver;
pub use tasker::Tasker;
pub use unit_solver::UnitSolver;
pub use unit_tasker::UnitTasker;

#[cfg(test)]
mod tests;

mod leaf;
mod solver;
mod tasker;
mod unit_solver;
mod unit_tasker;

pub trait Stemmer {
    type Unit;
    fn stemmer<T: WithReactor, F: FnOnce(&mut Self::Unit, T)>(&self, stem: &T, add_stem: F);
}
