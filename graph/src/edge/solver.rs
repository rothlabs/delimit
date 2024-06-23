use std::hash::Hash;

use crate::node::{self, Reactor};
use crate::{Edge, New};

use super::edge;

pub struct Solver<U, T, L, S>(Edge<Reactor, node::Solver<U, T, L, S>>);

impl<U, T, L, S> New for Solver<U, T, L, S> {
    type Unit = U;
    fn new(unit: U) -> Self {
        Self(edge::Edge::new(unit))
    }
}

impl<U, T, L, S> super::Solve for Solver<U, T, L, S>
where
    U: node::Solve<Task = T, Load = L>,
    T: Clone + Eq + PartialEq + Hash,
    L: Clone,
{
    type Stem = node::Solver<U, T, L, S>;
    fn solve(&self, task: U::Task) -> U::Load {
        self.0.solve(task)
    }
}

impl<U, T, L, S> super::AddLink for Solver<U, T, L, S> {
    type Unit = U;
    type Stem = S;
    fn add_link<F: FnOnce(&mut Self::Unit, &mut Self::Stem)>(
        &mut self,
        stem: &mut Self::Stem,
        add: F,
    ) {
        self.0.stem.write();
        //add(&mut self.0.stem.write(), stem);
    }
}
