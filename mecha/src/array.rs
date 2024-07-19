use super::*;
use add::*;
use graph::*;

pub mod view {
    use super::*;
    /// Array Link for super graphs
    pub type Plan<P, T, N> = graph::View<role::Plan<P, T, Role<N>>, Stem<N>>;
}

mod add;
#[cfg(test)]
mod tests;

/// Matrix Load for super graphs
pub type Role<N> = role::Ploy<Part<N>, Load<N>>;

#[derive(Clone)]
pub enum Bare<N> {
    Array(Array3<N>),
    Buffer,
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

#[derive(Clone)]
pub enum Part<N> {
    Add(Link<Add<N>, N>),
}

type Load<N> = Ace<Bare<N>>;
type Link<U, N> = Deuce<U, Load<N>>;
type Stem<N> = graph::view::end::Ploy<Part<N>, Bare<N>>;

// impl<N> Clone for Bare<N>
// where
//     N: Copy,
// {
//     fn clone(&self) -> Self {
//         match self {
//             Self::Array(array) => Self::Array(array.clone()),
//             Self::GpuRun => Self::GpuRun,
//         }
//     }
// }

// impl<N> Clone for Part<N> {
//     fn clone(&self) -> Self {
//         match self {
//             Self::Add(link) => Self::Add(link.clone()),
//         }
//     }
// }
