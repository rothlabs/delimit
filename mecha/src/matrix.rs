use crate::proto;

use graph::*;

pub use extrude::Extrude;

mod extrude;

pub mod view {
    use super::*;
    /// Point-frame input plan for super graphs 
    pub type Plan<P, T> = graph::View<role::Plan<P, T, Role>, Stem>;
}

/// Point-frame output for super graphs
pub type Role = role::Plan<Part, Task, Load>; 

type Link<U> = Trey<U, Task, Load>;
type Stem = graph::view::ace::Plan<Part, Task, Load>;

type Vector = proto::Vector<f64>;
type Matrix = proto::Matrix<f64>;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Task {
    Matrix,
    GpuMap,    
}

#[derive(Clone)]
pub enum Load {
    Matrix(Matrix),
    GpuMap,
} 

#[derive(Clone)]
pub enum Part {
    Extrude(Link<Extrude>)
}