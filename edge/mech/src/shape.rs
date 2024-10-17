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
}

#[derive(Clone, Debug)]
pub enum Rule {
    NURBS(Hub<u8>)
}