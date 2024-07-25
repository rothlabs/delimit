use std::ops;

use super::*;
use add::*;
use graph::*;

pub mod view {
    use super::*;
    /// Array Link for super graphs
    pub type Plan<P, T, N> = View<role::Plan<P, T, Role<N>>, Stem<N>>;
}

mod add;
#[cfg(test)]
mod tests;

/// Matrix Load for super graphs
pub type Role<N> = role::Ploy<Part<N>, Load<N>>;

pub type Stem<N> = graph::view::Ploy<Part<N>, Bare<N>>;

/// Units may grant in-memory arrays (Mem) or
/// they may grant gpu frame_buffer ref (Gpu)
#[derive(Clone)]
pub enum Bare<N> {
    Mem(Array3<N>),
    Gpu,
}

impl<N> Bare<N> {
    pub fn array(self) -> Array3<N> {
        if let Self::Mem(array) = self {
            array
        } else {
            panic!("not array")
        }
    }
    pub fn array_ref(&self) -> &Array3<N> {
        if let Self::Mem(array) = self {
            array
        } else {
            panic!("not array")
        }
    }
}

#[derive(Clone)]
pub enum Part<N: Number> {
    Add(Link<Add<N>>),
}

type Load<N> = Ace<Bare<N>>;
type Link<U> = Deuce<U>;

pub trait Number: Copy + Default + ops::Add<Self, Output = Self> + SendSync + 'static {}

impl<T> Number for T where T: Copy + Default + ops::Add<Self, Output = Self> + SendSync + 'static {}

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
