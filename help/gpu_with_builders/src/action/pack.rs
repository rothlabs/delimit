use super::*;

pub mod pass;
pub mod unit;

#[derive(Debug)]
pub struct Action {
    pub id: u32,
    pub stems: Vec<Grc<Action>>,
    pub kind: Kind,
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

#[derive(Debug)]
pub enum Kind {
    Leaf,
    Compute(pass::Compute),
    Render(pass::Render),
    // Other,
}

// impl Action {
//     pub fn render_pass(&self) -> bool {
//         if let Kind::Pass(pass) = &self.kind {
//             if let pass::Kind::Render(_) = &pass.kind {
//                 return true;
//             }
//         }
//         false
//     }
// }

// #[derive(Debug)]
// pub struct Pass {
//     pub binds: Vec<Bind>,
//     pub kind: pass::Kind,
// }

// #[derive(Debug, Default)]
// pub enum Action {
//     #[default]
//     Leaf,
//     Pack(Pack),
// }

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
