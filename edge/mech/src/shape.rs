use derive_node::Digest;
use serde::Serialize;

use super::*;

#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
pub struct Shape {
    rule: Rule,
    frame: Hub<Buffer>,
    control: Control,
    // plots: Node<gpu::Dispatcher>,
}

// impl Solve for Shape {
//     type Base = ();
// }

// impl Adapt for Shape {}

#[derive(Clone, Debug)]
pub enum Control {
    Shapes(Vec<Hub<Shape>>),
    Buffer(Hub<Buffer>),
    // Vector(Hub<Vec<f64>>),
}

#[derive(Clone, Digest, Serialize, Debug)]
pub enum Rule {
    NURBS(Hub<u8>)
}