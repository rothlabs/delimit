use crate::linear;

use graph::*;

pub use extrude::Extrude;
use serde::Serialize;

mod extrude;

pub mod view {
    use super::*;
    /// Point-frame input plan for super graphs 
    pub type Plan<P> = graph::View<role::Ploy<P, Role>, Stem>;
}

/// Point-frame output for super graphs
pub type Role = role::Plan<Part, Task, Load>; 

type Link<U> = Trey<U, Task, Load>;
type Stem = graph::view::ace::Plan<Part, Task, Load>;

type Vector = linear::Vector<f64>;
type Matrix = linear::Matrix<f64>;

#[derive(Clone)]
pub enum Task {
    Matrix,
    GpuMap,    
}

#[derive(Clone)]
enum Load {
    Matrix(Matrix),
    GpuMap,
} 

#[derive(Clone, Serialize)]
pub enum Part {

}

// enum Load<T, const R: usize, const C: usize> {
//     Scalar(T),
//     Matrix(Matrix<T, R, C>),
// }