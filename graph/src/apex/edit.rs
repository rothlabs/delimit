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
}