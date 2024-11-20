use super::*;

pub mod unit;

#[derive(Clone, Debug)]
pub enum Action {
    Leaf(u64),
    Dispatch(Dispatch),
    Draw(Draw),
}

#[derive(Clone, Debug)]
pub struct Dispatch {
    stems: Vec<Grc<Action>>,
    pipe: Grc<ComputePipeline>,
    bind: Vec<Binding>,
    size: u32,
}

#[derive(Clone, Debug)]
pub struct Draw {
    stems: Vec<Grc<Action>>,
    pipe: Grc<RenderPipeline>,
    bind: Vec<Binding>,
    vertex: Vertex,
    vertices: Range<u32>,
    instances: Range<u32>,
}



