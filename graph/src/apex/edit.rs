use super::*;
use std::collections::HashMap;

impl Apex {
    pub fn insert(&self, key: impl Into<Key>, apex: impl Into<Apex>) -> adapt::Result {
        match self {
            Self::Ploy(ploy) => ploy.adapt(Post::Insert(key.into(), apex.into())),
            _ => Err("No ploy.")?,
        }
    }
    pub fn extend(&self, apexes: HashMap<Key, Apex>) -> adapt::Result {
        match self {
            Self::Ploy(ploy) => ploy.adapt(Post::Extend(apexes)),
            _ => Err("No ploy.")?,
        }
    }
    pub fn set_at(&self, index: usize, apex: impl Into<Apex>) -> adapt::Result {
        match self {
            Self::Ploy(ploy) => ploy.adapt(Post::SetAt(index, apex.into())),
            _ => Err("No ploy.")?,
        }
    }

    pub fn write_string<T, F: FnOnce(&mut String) -> T>(&self, write: F) -> Result<T, Error> {
        eprintln!("trying to write leaf string");
        if let Self::Leaf(leaf) = self {
            eprintln!("it is a leaf");
            leaf.write(|tray| match tray {
                Tray::String(string) => write(string),
                _ => write(&mut "".into()),
            })
        } else {
            Err("Not a leaf.")?
        }
    }
}
