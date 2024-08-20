use super::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Post<'a> {
    /// Trade a apex for another. The implmentation should update graph info and return the same apex semantically.
    Trade(&'a dyn Trade),
    Import,
    Insert(Key, Apex),
    Extend(HashMap<Key, Apex>),
    SetAt(usize, Apex),
    Remove(usize),
    Paths(Vec<Path>),
}

impl Backed for Post<'_> {
    fn backed(&self, back: &Back) -> Self {
        match self {
            Post::Insert(key, apex) => Post::Insert(key.clone(), apex.backed(back)),
            Post::Extend(map) => {
                let mut backed = HashMap::new();
                for (key, apex) in map.iter() {
                    backed.insert(key.clone(), apex.backed(back));
                }
                Post::Extend(backed)
            }
            Post::SetAt(index, apex) => Post::SetAt(*index, apex.backed(back)),
            _ => self.clone(),
        }
    }
}
