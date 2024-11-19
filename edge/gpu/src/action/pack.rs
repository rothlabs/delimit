use super::*;

mod unit;

#[derive(Clone, Debug)]
pub enum Action {
    Dispatch(Dispatch),
    Leaf(u64),
}

#[derive(Clone, Debug)]
pub struct Dispatch {
    stems: Vec<Grc<Action>>,
    pipe: Grc<ComputePipeline>,
    bind: Vec<Binding>,
    size: u32,
}

