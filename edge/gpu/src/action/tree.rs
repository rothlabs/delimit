use super::*;

pub mod pass;

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
}
