use super::*;

pub mod unit;

mod pass;

#[derive(Debug, Default)]
pub enum Action {
    #[default]
    Leaf,
    Pack(Pack),
}

#[derive(Debug)]
pub struct Pack {
    pub id: u32,
    pub stems: Vec<Grc<Action>>,
    pub kind: Kind
}

impl Default for Pack {
    fn default() -> Self {
        Self {
            id: rand::random(),
            stems: vec![],
            kind: Kind::Other,
        }
    }
}

#[derive(Debug)]
pub enum Kind {
    Pass(pass::Pass),
    Other,
}


// impl Kind {
//     pub fn pass(&self) -> Option<Pass> {
//         match self {
//             Self::Leaf => None,
//             Self::Dispatch(_) => Some(Pass::Compute),
//             Self::Draw(_) => Some(Pass::Render),
//         }
//     }
// }


// #[derive(Clone, Debug)]
// pub struct Draw2 {
//     stems: HashMap<*const Action, Grc<Action>>,
//     pipe: Grc<RenderPipeline>,
//     binds: Vec<Bind>,
//     vertex: Vertex,
//     vertices: Range<u32>,
//     instances: Range<u32>,
// }
