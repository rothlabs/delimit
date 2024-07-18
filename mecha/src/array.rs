use crate::proto;

use graph::*;

pub use add::Add;

#[cfg(test)]
mod tests;

mod add;

pub mod view {
    use super::*;
    /// Matrix Link for super graphs
    pub type Plan<P, T> = graph::View<role::Plan<P, T, Role>, Stem>;
}

/// Matrix Load for super graphs
pub type Role = role::Plan<Part, Task, Load>;

type Link<U> = Trey<U, Task, Load>;
type Stem = graph::view::end::Plan<Part, Task, Bare>;
type Load = Ace<Bare>;

type Vector = proto::Array<f64, 1>;
type Array = proto::Array<f64, 3>;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Task {
    Array,
    GpuRun,
}

#[derive(Clone)]
pub enum Bare {
    Array(Array),
    GpuRun,
}

impl Bare {
    fn array(self) -> Array {
        if let Self::Array(mat) = self {
            mat
        } else {
            panic!("not array")
        }
    }
    fn array_ref(&self) -> &Array {
        if let Self::Array(mat) = self {
            mat
        } else {
            panic!("not array")
        }
    }
}

#[derive(Clone)]
pub enum Part {
    Add(Link<Add>),
}
