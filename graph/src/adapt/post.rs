use super::*;

#[derive(Clone, Debug)]
pub enum Post<'a> {
    /// Trade a apex for another. The implmentation should update graph info and return the same apex semantically.
    Trade(&'a dyn Trade),
    Insert(Aim<'a>, Apex),
    Extend(Map),
    SetAt(usize, Apex),
    // Replace(),
    // Remove(usize),
}

impl<'a> Backed for Post<'a> {
    fn backed(&self, back: &Back) -> Self {
        match self {
            Post::Insert(key, apex) => Post::Insert(key.clone(), apex.backed(back)),
            Post::Extend(map) => Post::Extend(map.trade(back)),
            Post::SetAt(index, apex) => Post::SetAt(*index, apex.backed(back)),
            _ => self.clone(),
        }
    }
}