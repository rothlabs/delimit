use crate::proto;

use graph::*;

pub use extrude::Extrude;

mod extrude;

pub mod view {
    use super::*;
    /// Matrix Link for super graphs
    pub type Plan<P, T> = graph::View<role::Plan<P, T, Role>, Stem>;
}

/// Matrix Load for super graphs
pub type Role = role::Plan<Part, Task, Load>;

type Link<U> = Trey<U, Task, Load>;
type Stem = graph::view::ace::Plan<Part, Task, Load>;

type Vector = proto::Vector<f64>;
type Matrix = proto::Matrix<f64>;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Task {
    Matrix,
    GpuRun,
}

#[derive(Clone)]
pub enum Load {
    Matrix(Matrix),
    GpuRun,
}

impl Load {
    fn matrix(&self) -> &Matrix {
        if let Self::Matrix(mat) = self {
            mat
        } else {
            panic!("not a matrix")
        }
    }
}

#[derive(Clone)]
pub enum Part {
    Extrude(Link<Extrude>),
}
