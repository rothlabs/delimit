use super::*;

#[derive(Clone)]
pub enum Asset<L> {
    Bare(L),
    Ploy(Ploy<L>),
}

impl<L> Default for Asset<L> 
where 
    L: Default
{
    fn default() -> Self {
        Self::Bare(L::default())
    }
}

impl<L> From<L> for Asset<L> {
    fn from(value: L) -> Self {
        Self::Bare(value)
    }
}

impl<L> From<Ploy<L>> for Asset<L> {
    fn from(value: Ploy<L>) -> Self {
        Self::Ploy(value)
    }
}

impl<L> ToLoad for Asset<L> 
where 
    L: 'static + ToLoad
{
    type Load = L::Load;
    fn load(&self) -> Self::Load {
        match self {
            Self::Bare(bare) => bare.load(),
            Self::Ploy(ploy) => ploy.grant().load()
        }
    }
}

impl<L> Read for Asset<L> 
where 
    L: 'static + Read
{
    type Item = L::Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        match self {
            Self::Bare(bare) => bare.read(read),
            Self::Ploy(ploy) => ploy.grant().read(read)
        }
    }
}

impl<L> Backed for Asset<L> 
where 
    L: Clone
{
    fn backed(&self, back: &Back) -> Self {
        match self {
            Self::Bare(bare) => Self::Bare(bare.clone()),
            Self::Ploy(ploy) => Self::Ploy(ploy.backed(back)),
        }
    }
}