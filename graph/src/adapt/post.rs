use super::*;

#[derive(Clone, Debug)]
pub enum Post<'a> {
    /// Trade a apex for another. The implmentation should update graph info and return the same apex semantically.
    Trade(&'a dyn Trade),
    Import,
    Insert(Key, Apex),
    Extend(Map),
    SetAt(usize, Apex),
    Remove(usize),
    Paths(Vec<Path>),
}

impl Backed for Post<'_> {
    fn backed(&self, back: &Back) -> Self {
        match self {
            Post::Insert(key, apex) => Post::Insert(key.clone(), apex.backed(back)),
            Post::Extend(map) => Post::Extend(map.trade(back)),
            Post::SetAt(index, apex) => Post::SetAt(*index, apex.backed(back)),
            _ => self.clone(),
        }
    }
}
