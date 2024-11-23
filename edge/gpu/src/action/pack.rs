use super::*;

pub mod unit;

#[derive(Clone, Debug)]
pub struct Action {
    pub id: u32,
    pub stems: Vec<Grc<Action>>,
    pub kind: Kind
}

impl Default for Action {
    fn default() -> Self {
        Self {
            id: rand::random(),
            stems: vec![],
            kind: Kind::Leaf,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Kind {
    Leaf,
    Dispatch(Dispatch),
    Draw(Draw),
}

impl Kind {
    pub fn pass(&self) -> Option<Pass> {
        match self {
            Self::Leaf => None,
            Self::Dispatch(_) => Some(Pass::Compute),
            Self::Draw(_) => Some(Pass::Render),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Dispatch {
    pipe: Grc<ComputePipeline>,
    binds: Vec<Bind>,
    size: u32,
}

#[derive(Clone, Debug)]
pub struct Draw {
    pipe: Grc<RenderPipeline>,
    binds: Vec<Bind>,
    vertex: Vertex,
    pub vertices: Range<u32>,
    pub instances: Range<u32>,
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
