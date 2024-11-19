use super::*;

pub enum Node {
    Dispatch(Dispatch),
    Leaf(u64),
}

pub struct Dispatch {
    stems: Vec<Grc<Node>>,
    pipe: Grc<ComputePipeline>,
    bind: Vec<(u32, Grc<BindGroup>, Vec<u32>)>,
    size: u32,
}
