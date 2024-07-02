use crate::*;

pub use leaf::{Leaf, ToLeaf};
pub use solver::Solver;
pub use unit_solver::UnitSolver;

#[cfg(test)]
mod tests;

mod leaf;
mod solver;
mod unit_solver;

pub trait Stemmer {
    type Unit;
    fn stemmer<T: WithReactor, F: FnOnce(&mut Self::Unit, T)>(&self, stem: &T, add_stem: F);
}