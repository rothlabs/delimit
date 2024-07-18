use super::*;
use add::*;
use graph::*;

pub mod view {
    use super::*;
    /// Matrix Link for super graphs
    pub type Plan<P, T, N> = graph::View<role::Plan<P, T, Role<N>>, Stem<N>>;
}

mod add;
#[cfg(test)]
mod tests;

/// Matrix Load for super graphs
pub type Role<N> = role::Plan<Part<N>, Task, Load<N>>;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Task {
    Array,
    GpuRun,
}

pub enum Bare<N> {
    Array(Array3<N>),
    GpuRun,
}

impl<N> Clone for Bare<N>
where
    N: Copy,
{
    fn clone(&self) -> Self {
        match self {
            Self::Array(array) => Self::Array(array.clone()),
            Self::GpuRun => Self::GpuRun,
        }
    }
}

impl<N> Bare<N> {
    pub fn array(self) -> Array3<N> {
        if let Self::Array(array) = self {
            array
        } else {
            panic!("not array")
        }
    }
    pub fn array_ref(&self) -> &Array3<N> {
        if let Self::Array(array) = self {
            array
        } else {
            panic!("not array")
        }
    }
}

pub enum Part<N> {
    Add(Link<Add<N>, N>),
}

impl<N> Clone for Part<N> {
    fn clone(&self) -> Self {
        match self {
            Self::Add(link) => Self::Add(link.clone()),
        }
    }
}

type Load<N> = Ace<Bare<N>>;
type Link<U, N> = Trey<U, Task, Load<N>>;
type Stem<N> = graph::view::end::Plan<Part<N>, Task, Bare<N>>;
