use super::*;

#[derive(Clone, Debug)]
pub enum Post<'a> {
    /// Trade a node for another. The implmentation should update graph info and return the same node semantically.
    Trade(&'a dyn Trade),
    Import,
    Insert(Node),
    Extend(Vec<Node>),
    Remove(usize),
}

impl Backed for Post<'_> {
    fn backed(&self, back: &Back) -> Self {
        match self {
            Post::Insert(nodes) => Post::Insert(nodes.backed(back)),
            _ => self.clone(),
        }
    }
}
