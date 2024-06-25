use std::hash::Hash;

use derivative::Derivative;
use serde::Serialize;

use crate::{base, edge, AddStem, FromReactor, FromUnit, Link, React, Reactor};

#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Solver<U, W>(Link<edge::Solver<U, W>>);

impl<U, W> FromUnit for Solver<U, W>
where
    U: FromUnit,
    W: Default
{
    type Unit = U::Unit;
    fn from_unit(unit: Self::Unit) -> Self {
        Self(Link::from_unit(unit))
    }
}

impl<U, W> FromReactor for Solver<U, W> 
where 
    U: React,
{
    fn from_reactor(&self, root: Reactor) -> Self {
        Self(self.0.from_reactor(root))
    }
}

impl<U, T, L, S> super::Solve for Solver<U, T, L, S>
where
    U: base::Solve<Task = T, Load = L>,
    T: Clone + Eq + PartialEq + Hash,
    L: Clone,
{
    type Edge = edge::Solver<U, T, L, S>;
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
    fn add_stem(&mut self, stem: Self::Stem) {
        self.0.add_stem(stem);
    }
}

impl<U, T, L, St> Serialize for Solver<U, T, L, St> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

// impl<U, T, L, S> Clone for Solver<U, T, L, S> {
//     fn clone(&self) -> Self {
//         Self(self.0.clone())
//     }
// }

// impl<U, T, L, S> Clone for Solver<U, T, L, S> {
//     fn clone(&self) -> Self {
//         Self {
//             edge: self.edge.clone(),
//             meta: self.meta.clone(),
//         }
//     }
// }

// impl<U, T, L, S> Serialize for Solver<U, T, L, S> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         self.meta.serialize(serializer)
//     }
// }

// let edge = self.edge.read().expect(NO_POISON);
// edge.solve(task)
