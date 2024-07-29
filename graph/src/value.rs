use super::*;

/// This could be renamed to leaf because only a leaf value should ever 
/// be placed here. It could also be replaced with Leaf<L> = Box<dyn ReadLoadBacked<L>>
/// and impl ToLoad, Read, and backed for L
#[derive(Clone)]
pub enum Value<L> {
    Meta(Meta),
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
    L: 'static + Clone + Default
{
    type Load = L;
    // TODO: load should take a link with repo traits
    fn load(&self) -> Self::Load {
        match self {
            // TODO: should attempt to lookup from repo
            Self::Meta(_) => L::default(),
            Self::Bare(bare) => bare.clone(),
            Self::Ace(ace) => ace.load(),
            Self::Ploy(ploy) => ploy.grant().load()
        }
    }
}

impl<L> Read for Value<L> 
where 
    L: 'static + SendSync + Default
{
    type Item = L;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        match self {
            Self::Meta(_) => read(&L::default()),
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
            Self::Meta(meta) => Self::Meta(meta.clone()),
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

impl<L> From<&Value<L>> for Value<L> 
where 
    L: Clone
{
    fn from(value: &Value<L>) -> Self {
        value.clone()
    }
}

// impl<L> From<&Ploy<Ploy<Ace<L>>>> for Value<L> 
// where 
//     L: 'static + Clone + Update + SendSync
// {
//     fn from(value: &Ploy<Ploy<Ace<L>>>) -> Self {
//         let wow = Pipe::new(value.clone());
//         let val = wow.grant();
//         let huh = wow.ploy();
//         Self::Ploy(Pipe::new(value.clone()))
//     }
// }



// impl<L> From<&Vec<Value<L>>> for Value<L> 
// where 
//     L: Clone
// {
//     fn from(value: &Vec<Value<L>>) -> Self {
//         value.clone()
//     }
// }

// impl From<&str> for &Value<String> {
//     fn from(value: &str) -> Self {
//         Self::Bare(value.to_owned())
//     }
// }