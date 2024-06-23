use std::hash::Hash;

use derivative::Derivative;
use serde::Serialize;

use crate::{edge, node, Link, FromUnit};

#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Solver<U, T, L, S>(Link<edge::Solver<U, T, L, S>>);

impl<U, T, L, S> FromUnit for Solver<U, T, L, S> {
    type Unit = U;
    fn new(unit: U) -> Self {
        Self(Link::new(unit))
    }
}

impl<U, T, L, S> super::Solve for Solver<U, T, L, S>
where
    U: node::Solve<Task = T, Load = L>,
    T: Clone + Eq + PartialEq + Hash,
    L: Clone,
{
    type Edge = edge::Solver<U, T, L, S>;
    fn solve(&self, task: U::Task) -> U::Load {
        self.0.solve(task)
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
