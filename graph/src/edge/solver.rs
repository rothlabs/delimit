use std::hash::Hash;

use crate::{base, AddStem, Edge, FromReactor, FromUnit, React};
use crate::{node, Reactor};

use super::edge;

pub struct Solver<U, T, L, S>(Edge<node::Solver<U, T, L, S>>);

impl<U, T, L, S> FromUnit for Solver<U, T, L, S> {
    type Unit = U;
    fn new(unit: U) -> Self {
        Self(edge::Edge::new(unit))
    }
}

impl<U, T, L, S> FromReactor for Solver<U, T, L, S> 
where
    U: React,
{
    fn from_reactor(&self, reactor: Reactor) -> Self {
        Self(self.0.from_reactor(reactor))
    }
}

impl<U, T, L, S> super::Solve for Solver<U, T, L, S>
where
    U: base::Solve<Task = T, Load = L>,
    T: Clone + Eq + PartialEq + Hash,
    L: Clone,
{
    type Stem = node::Solver<U, T, L, S>;
    fn solve(&self, task: U::Task) -> U::Load {
        self.0.solve(task)
    }
}

impl<U, T, L, S> AddStem for Solver<U, T, L, S>
where
    U: AddStem<Stem = S> + React + 'static,
    T: 'static,
    L: 'static,
    S: FromReactor + 'static,
{
    type Stem = U::Stem;
    fn add_stem(&mut self, stem: U::Stem) {
        self.0.add_stem(stem);
    }
}