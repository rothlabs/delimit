use std::hash::Hash;

use derivative::Derivative;
use serde::Serialize;

use crate::{edge, node, Link, New};

#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Solver<U, T, L>(Link<edge::Solver<U, T, L>>);

impl<U, T, L> New for Solver<U, T, L> {
    type Unit = U;
    fn new(unit: U) -> Self {
        Self(Link::new(unit))
    }
}

impl<U, T, L> super::Solve for Solver<U, T, L>
where
    U: node::Solve<Task = T, Load = L>,
    T: Clone + Eq + PartialEq + Hash,
    L: Clone,
{
    type Edge = edge::Solver<U, T, L>;
    fn solve(&self, task: U::Task) -> U::Load {
        self.0.solve(task)
    }
}

impl<U, T, L> Serialize for Solver<U, T, L> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

// impl<U, T, L> Clone for Solver<U, T, L> {
//     fn clone(&self) -> Self {
//         Self(self.0.clone())
//     }
// }

// impl<U, T, L> Clone for Solver<U, T, L> {
//     fn clone(&self) -> Self {
//         Self {
//             edge: self.edge.clone(),
//             meta: self.meta.clone(),
//         }
//     }
// }

// impl<U, T, L> Serialize for Solver<U, T, L> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         self.meta.serialize(serializer)
//     }
// }

// let edge = self.edge.read().expect(NO_POISON);
// edge.solve(task)
