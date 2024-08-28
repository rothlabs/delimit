use super::*;

impl Apex {
    pub fn set(&self, index: usize, apex: impl Into<Apex>) -> adapt::Result {
        match self {
            Self::Ploy(ploy) => ploy.adapt(Post::SetAt(index, apex.into())),
            _ => Err(apex::Error::NotPloy)?,
        }
    }
    pub fn insert(&self, key: impl Into<Key>, apex: impl Into<Apex>) -> adapt::Result {
        match self {
            Self::Ploy(ploy) => ploy.adapt(Post::Insert(key.into(), apex.into())),
            _ => Err(apex::Error::NotPloy)?,
        }
    }
    pub fn extend(&self, apexes: Map) -> adapt::Result {
        match self {
            Self::Ploy(ploy) => ploy.adapt(Post::Extend(apexes)),
            _ => Err(apex::Error::NotPloy)?,
        }
    }
    pub fn write_string<T, F: FnOnce(&mut String) -> T>(&self, write: F) -> Result<T, crate::AnyError> {
        if let Self::Leaf(leaf) = self {
            leaf.write(|tray| match tray {
                Tray::String(string) => write(string),
                _ => write(&mut "".into()),
            })
        } else {
            Err("Not a leaf.")?
        }
    }
}
