use super::*;

#[allow(dead_code)]
#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
pub struct Shape {
    table: Hub<Table>,
    dimension: Hub<u8>,
    rule: Rule,
    control: Control,
    instance: Option<Instance>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Rule {
    Nurbs(Hub<u8>),
    Extrude,
    Revolve,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Control {
    Shape(Vec<Shape>),
    Table(Hub<Table>),
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Instance {
    table: Hub<Table>,
    layout: Layout,
    instance: Option<Box<Instance>>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Layout {
    Free,
    Grid,
    Radial,
}

