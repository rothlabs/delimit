use super::*;

#[derive(Clone, Debug)]
pub enum Post<'a> {
    /// Trade a apex for another. The implmentation should update graph info and return the same apex semantically.
    Trade(&'a dyn Trade),
    Import,
    Insert(Apex),
    Extend(Vec<Apex>),
    Remove(usize),
}

impl Backed for Post<'_> {
    fn backed(&self, back: &Back) -> Self {
        match self {
            Post::Insert(apexes) => Post::Insert(apexes.backed(back)),
            _ => self.clone(),
        }
    }
}
