use super::*;

#[derive(Clone, Debug)]
pub enum Post {
    /// Trade a node for another. The implmentation should update graph info and return the same node semantically.
    Trade(Box<dyn Trade>),
    Import,
    Insert(Node),
    Extend(Vec<Node>),
    Remove(usize),
}

impl Backed for Post {
    fn backed(&self, back: &Back) -> Self {
        match self {
            Post::Insert(nodes) => Post::Insert(nodes.backed(back)),
            _ => self.clone(),
        }
    }
}