use derive_node::Digest;
use serde::Serialize;

use super::*;

#[derive(Builder, Debug, Digest, Serialize)]
#[builder(pattern = "owned")]
pub struct Shape {
    rule: Rule,
    frame: Hub<graph::Buffer>,
    controls: Vec<Control>,
    // plots: Node<gpu::Dispatcher>,
}

impl Solve for Shape {
    type Base = ();
}

impl Adapt for Shape {}

#[derive(Debug, Digest, Serialize)]
pub enum Control {
    Shape(Hub<Node<Shape>>),
    Buffer(Hub<graph::Buffer>),
}

#[derive(Debug, Digest, Serialize)]
pub enum Rule {
    NURBS(Hub<u8>)
}