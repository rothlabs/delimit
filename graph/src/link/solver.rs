use std::hash::Hash;

use derivative::Derivative;
use serde::Serialize;

use crate::{base, edge, AddStem, FromReactor, FromUnit, Link, React, Reactor, Solve, Work};

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

impl<U, W> Solve for Solver<U, W>
where
    edge::Solver<U, W>: Solve<Task = W::Task, Load = W::Load>,
    W: Work,
{
    type Load = W::Load;
    type Task = W::Task;
    fn solve(&self, task: Self::Task) -> Self::Load {
        self.0.solve(task)
    }
}

impl<U, W> AddStem for Solver<U, W>
where
    edge::Solver<U, W>: AddStem, 
{
    type Stem = <edge::Solver<U, W> as AddStem>::Stem;
    fn add_stem(&mut self, stem: Self::Stem) {
        self.0.add_stem(stem);
    }
}

impl<U, W> Serialize for Solver<U, W> {
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
