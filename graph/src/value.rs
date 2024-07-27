use super::*;

#[derive(Clone)]
pub enum Value<L> {
    Bare(L),
    Ace(Ace<L>),
    Ploy(Ploy<Ace<L>>),
}

impl<L> Default for Value<L> 
where 
    L: Default
{
    fn default() -> Self {
        Self::Bare(L::default())
    }
}

impl<L> ToLoad for Value<L> 
where 
    L: 'static + Clone
{
    type Load = L;
    fn load(&self) -> Self::Load {
        match self {
            Self::Bare(bare) => bare.clone(),
            Self::Ace(ace) => ace.load(),
            Self::Ploy(ploy) => ploy.grant().load()
        }
    }
}

impl<L> Read for Value<L> 
where 
    L: 'static + SendSync
{
    type Item = L;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        match self {
            Self::Bare(bare) => read(bare),
            Self::Ace(ace) => ace.read(read),
            Self::Ploy(ploy) => ploy.grant().read(read)
        }
    }
}

impl<L> Backed for Value<L> 
where 
    L: Clone
{
    fn backed(&self, back: &Back) -> Self {
        match self {
            Self::Bare(bare) => Self::Bare(bare.clone()),
            Self::Ace(ace) => Self::Ace(ace.backed(back)),
            Self::Ploy(ploy) => Self::Ploy(ploy.backed(back)),
        }
    }
}

impl<L> From<L> for Value<L> {
    fn from(value: L) -> Self {
        Self::Bare(value)
    }
}

impl<L> From<Ace<L>> for Value<L> {
    fn from(value: Ace<L>) -> Self {
        Self::Ace(value)
    }
}

impl<L> From<Ploy<Ace<L>>> for Value<L> {
    fn from(value: Ploy<Ace<L>>) -> Self {
        Self::Ploy(value)
    }
}

impl<L> From<&L> for Value<L> 
where 
    L: Clone
{
    fn from(value: &L) -> Self {
        Self::Bare(value.clone())
    }
}

impl<L> From<&Ace<L>> for Value<L> {
    fn from(value: &Ace<L>) -> Self {
        Self::Ace(value.clone())
    }
}

impl<L> From<&Ploy<Ace<L>>> for Value<L> {
    fn from(value: &Ploy<Ace<L>>) -> Self {
        Self::Ploy(value.clone())
    }
}

impl From<&str> for Value<String> {
    fn from(value: &str) -> Self {
        Self::Bare(value.to_owned())
    }
}