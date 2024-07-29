use super::*;

#[derive(Clone, Default, PartialEq)]
pub struct Value<L> {
    level: usize,
    next: Level<L>,
}

impl<L> Value<L>
where
    L: 'static + Default,
{
    pub fn ace(load: L) -> Self {
        Self {
            level: 0,
            next: Level::Ace(Ace::new(load)),
        }
    }
    pub fn bare(load: L) -> Self {
        Self {
            level: 0,
            next: Level::Bare(load),
        }
    }
    pub fn ploy(value: Ploy<Value<L>>) -> Self {
        Self {
            level: value.grant().level + 1,
            next: Level::Ploy(value),
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

impl<L: 'static + Clone + Default> Grant for Value<L> {
    type Load = Self;
    fn grant(&self) -> Self::Load {
        Self {
            level: self.level - 1,
            next: self.next.grant(),
        }
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

pub trait DownToLevel {
    fn down(&self, level: usize) -> Self;
}

impl<L> DownToLevel for Value<L>
where
    L: 'static + Clone + Default,
{
    fn down(&self, level: usize) -> Self {
        let mut value = self.clone();
        while value.level > level {
            value = value.grant();
        }
        value
    }
}

impl<L> DownToLevel for Vec<Value<L>>
where
    L: 'static + Clone + Default,
{
    fn down(&self, level: usize) -> Self {
        self.iter().map(|x| x.down(level)).collect()
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

impl<L: 'static + Clone + Default> Grant for Level<L> {
    type Load = Level<L>;
    fn grant(&self) -> Self::Load {
        match self {
            Self::Meta(_) => panic!("wrong level variant: meta"),
            Self::Bare(_) => panic!("wrong level variant: bare"),
            Self::Ace(_) => panic!("wrong level variant: ace"),
            Self::Ploy(ploy) => ploy.grant().next,
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

impl<L> From<L> for Value<L> {
    fn from(value: L) -> Self {
        Self {
            level: 0,
            next: Level::Bare(value),
        }
    }
}

impl<L> From<Ace<L>> for Value<L> {
    fn from(value: Ace<L>) -> Self {
        Self {
            level: 0,
            next: Level::Ace(value),
        }
    }
}

impl<L: 'static + Default> From<Ploy<Value<L>>> for Value<L> {
    fn from(value: Ploy<Value<L>>) -> Self {
        Self::ploy(value)
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
            next: Level::Ace(value.clone()),
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
