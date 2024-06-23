use std::hash::Hash;

use crate::node::{self, Reactor};
use crate::{Edge, New};

use super::edge;

pub struct Solver<U, T, L>(Edge<Reactor, node::Solver<U, T, L>>);

impl<U, T, L> New for Solver<U, T, L> {
    type Unit = U;
    fn new(unit: Self::Unit) -> Self {
        Self(edge::Edge::new(unit))
    }
}

impl<U, T, L> super::Solve for Solver<U, T, L>
where
    U: node::Solve<Task = T, Load = L>,
    T: Clone + Eq + PartialEq + Hash,
    L: Clone,
{
    type Stem = node::Solver<U, T, L>;
    fn solve(&self, task: <Self::Stem as node::Solve>::Task) -> <Self::Stem as node::Solve>::Load {
        self.0.solve(task)
    }
}
