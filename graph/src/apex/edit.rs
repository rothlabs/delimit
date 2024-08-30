use super::*;

impl Apex {
    pub fn set(&self, index: usize, apex: impl Into<Apex>) -> adapt::Result {
        match self {
            Self::Ploy(ploy) => ploy.adapt(Post::SetAt(index, apex.into())),
            _ => Err(apex::Error::NotPloy)?,
        }
    }
    pub fn insert(&self, aim: impl Into<Aim<'static>>, apex: impl Into<Apex>) -> adapt::Result {
        match self {
            Self::Ploy(ploy) => ploy.adapt(Post::Insert(aim.into(), apex.into())),
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