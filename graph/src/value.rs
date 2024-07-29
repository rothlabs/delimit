use super::*;

#[derive(Clone, Default, PartialEq)]
pub struct Value<L> {
    level: usize,
    next: Level<L>,
}

impl<L> Value<L> 
where 
    L: Default
{
    pub fn ace(load: L) -> Self {
        Self {
            level: 0,
            next: Level::Ace(Ace::new(load))
        }
    }
    pub fn bare(load: L) -> Self {
        Self {
            level: 0,
            next: Level::Bare(load)
        }
    }
}

impl<L> ToLoad for Value<L>
where
    L: 'static + Clone + Default,
{
    type Load = L;
    // TODO: load should take a link with repo traits
    fn load(&self) -> Self::Load {
        self.next.load()
    }
}

impl<L> Read for Value<L>
where
    L: 'static + SendSync + Default,
{
    type Item = L;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        self.next.read(read)
    }
}

impl<L> Backed for Value<L>
where
    L: Clone,
{
    fn backed(&self, back: &Back) -> Self {
        Self {
            level: self.level,
            next: self.next.backed(back),
        }
    }
}



/// This could be renamed to leaf because only a leaf value should ever
/// be placed here. It could also be replaced with Leaf<L> = Box<dyn ReadLoadBacked<L>>
/// and impl ToLoad, Read, and backed for L
#[derive(Clone, PartialEq)]
pub enum Level<L> {
    Meta(Meta),
    Bare(L),
    Ace(Ace<L>),
    Ploy(Ploy<Value<L>>),
}

// impl<L> Value<L> {
//     pub fn reduce(&self, level: usize) -> Self {

//     }
// }

impl<L> Default for Level<L>
where
    L: Default,
{
    fn default() -> Self {
        Self::Bare(L::default())
    }
}

impl<L> ToLoad for Level<L>
where
    L: 'static + Clone + Default,
{
    type Load = L;
    // TODO: load should take a link with repo traits
    fn load(&self) -> Self::Load {
        match self {
            // TODO: should attempt to lookup from repo
            Self::Meta(_) => L::default(),
            Self::Bare(bare) => bare.clone(),
            Self::Ace(ace) => ace.load(),
            Self::Ploy(ploy) => ploy.grant().load(),
        }
    }
}

impl<L> Read for Level<L>
where
    L: 'static + SendSync + Default,
{
    type Item = L;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        match self {
            Self::Meta(_) => read(&L::default()),
            Self::Bare(bare) => read(bare),
            Self::Ace(ace) => ace.read(read),
            Self::Ploy(ploy) => ploy.grant().read(read),
        }
    }
}

impl<L> Backed for Level<L>
where
    L: Clone,
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

// impl<L> From<L> for Value<L> {
//     fn from(value: L) -> Self {
//         Self::Bare(value)
//     }
// }

// impl<L> From<Ace<L>> for Value<L> {
//     fn from(value: Ace<L>) -> Self {
//         Self::Next(Next::Ace(value))
//     }
// }

impl<L> From<Ploy<Value<L>>> for Value<L> {
    fn from(value: Ploy<Value<L>>) -> Self {
        Self {
            level: 0,
            next: Level::Ploy(value)
        }
    }
}

// impl<L> From<&L> for Value<L>
// where
//     L: Clone,
// {
//     fn from(value: &L) -> Self {
//         Self::Bare(value.clone())
//     }
// }

impl<L> From<&Ace<L>> for Value<L> {
    fn from(value: &Ace<L>) -> Self {
        Self {
            level: 0,
            next: Level::Ace(value.clone())
        }
    }
}

// // impl<L> From<&Ploy<Ace<L>>> for Value<L> {
// //     fn from(value: &Ploy<Ace<L>>) -> Self {
// //         Self::Ploy(value.clone())
// //     }
// // }

impl From<&str> for Value<String> {
    fn from(value: &str) -> Self {
        Self::bare(value.to_owned())
    }
}

impl<L> From<&Value<L>> for Value<L>
where
    L: Clone,
{
    fn from(value: &Value<L>) -> Self {
        value.clone()
    }
}

// // impl<L> From<Ploy<Ploy<Ace<L>>>> for Value<L>
// // where
// //     L: 'static + SendSync,
// // {
// //     fn from(value: Ploy<Ploy<Ace<L>>>) -> Self {
// //         Self::Ploy(Pipe::new(value).ploy())
// //     }
// // }

// // impl<L> From<&Ploy<Ploy<Ace<L>>>> for Value<L>
// // where
// //     L: 'static + SendSync,
// // {
// //     fn from(value: &Ploy<Ploy<Ace<L>>>) -> Self {
// //         Self::Ploy(Pipe::new(value.clone()).ploy())
// //     }
// // }




// // impl<L> From<&Vec<Value<L>>> for Value<L>
// // where
// //     L: Clone
// // {
// //     fn from(value: &Vec<Value<L>>) -> Self {
// //         value.clone()
// //     }
// // }

// // impl From<&str> for &Value<String> {
// //     fn from(value: &str) -> Self {
// //         Self::Bare(value.to_owned())
// //     }
// // }
