use super::*;

pub mod unit;

#[derive(Clone, Debug)]
pub enum Action {
    Leaf(u64),
    Dispatch(Dispatch),
    Draw(Draw),
}

impl Action {
    pub fn stems(&self) -> Option<&Vec<Grc<Action>>> {
        match self {
            Self::Leaf(_) => None,
            Self::Dispatch(x) => Some(&x.stems),
            Self::Draw(x) => Some(&x.stems),
        }
    }
    pub fn pass(&self) -> Option<Pass> {
        match self {
            Self::Leaf(_) => None,
            Self::Dispatch(_) => Some(Pass::Compute),
            Self::Draw(_) => Some(Pass::Render),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Dispatch {
    stems: Vec<Grc<Action>>,
    pipe: Grc<ComputePipeline>,
    binds: Vec<Bind>,
    size: u32,
}

#[derive(Clone, Debug)]
pub struct Draw {
    stems: Vec<Grc<Action>>,
    pipe: Grc<RenderPipeline>,
    binds: Vec<Bind>,
    vertex: Vertex,
    vertices: Range<u32>,
    instances: Range<u32>,
}

// #[derive(Clone, Debug)]
// pub struct Draw2 {
//     stems: HashMap<*const Action, Grc<Action>>,
//     pipe: Grc<RenderPipeline>,
//     binds: Vec<Bind>,
//     vertex: Vertex,
//     vertices: Range<u32>,
//     instances: Range<u32>,
// }
