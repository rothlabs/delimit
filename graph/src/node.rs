use std::error::Error;
use super::*;

pub type Result<L> = std::result::Result<Node<L>, Box<dyn Error>>;

#[derive(Clone, Default, PartialEq)]
pub struct Node<L> {
    rank: usize,
    form: Form<L>,
}

impl<L> ToLoad for Node<L>
where
    L: 'static + Clone + Default,
{
    type Load = L;
    // TODO: load should take a link with repo traits
    fn load(&self) -> Self::Load {
        self.form.load()
    }
}

impl<L> Read for Node<L>
where
    L: 'static + SendSync + Default,
{
    type Item = L;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        self.form.read(read)
    }
}

impl<L: 'static + Clone + Default> Grant for Node<L> {
    type Load = Self;
    fn grant(&self) -> Self::Load {
        Self {
            rank: self.rank - 1,
            form: self.form.grant(),
        }
    }
}

impl<L> Backed for Node<L>
where
    L: Clone,
{
    fn backed(&self, back: &Back) -> Self {
        Self {
            rank: self.rank,
            form: self.form.backed(back),
        }
    }
}

pub trait RankDown {
    fn down(&self, level: usize) -> Self;
}

impl<L> RankDown for Node<L>
where
    L: 'static + Clone + Default,
{
    fn down(&self, level: usize) -> Self {
        let mut value = self.clone();
        while value.rank > level {
            value = value.grant();
        }
        value
    }
}

impl<L> RankDown for Vec<Node<L>>
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
pub enum Form<L> {
    Meta(Meta),
    Bare(L),
    Ace(Ace<L>),
    Ploy(Ploy<Node<L>>),
}

impl<L> Default for Form<L>
where
    L: Default,
{
    fn default() -> Self {
        Self::Bare(L::default())
    }
}

impl<L> ToLoad for Form<L>
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

impl<L> Read for Form<L>
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

impl<L: 'static + Clone + Default> Grant for Form<L> {
    type Load = Form<L>;
    fn grant(&self) -> Self::Load {
        match self {
            Self::Meta(_) => panic!("wrong level variant: meta"),
            Self::Bare(_) => panic!("wrong level variant: bare"),
            Self::Ace(_) => panic!("wrong level variant: ace"),
            Self::Ploy(ploy) => ploy.grant().form,
        }
    }
}

impl<L> Backed for Form<L>
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

impl<L> From<L> for Node<L> {
    fn from(value: L) -> Self {
        Self {
            rank: 0,
            form: Form::Bare(value),
        }
    }
}

impl<L> From<Ace<L>> for Node<L> {
    fn from(value: Ace<L>) -> Self {
        Self {
            rank: 0,
            form: Form::Ace(value),
        }
    }
}

impl<L: 'static + Default> From<Ploy<Node<L>>> for Node<L> {
    fn from(value: Ploy<Node<L>>) -> Self {
        Self {
            rank: value.grant().rank + 1,
            form: Form::Ploy(value),
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

impl<L> From<&Ace<L>> for Node<L> {
    fn from(value: &Ace<L>) -> Self {
        Self {
            rank: 0,
            form: Form::Ace(value.clone()),
        }
    }
}

// // impl<L> From<&Ploy<Ace<L>>> for Value<L> {
// //     fn from(value: &Ploy<Ace<L>>) -> Self {
// //         Self::Ploy(value.clone())
// //     }
// // }

impl From<&str> for Node<String> {
    fn from(value: &str) -> Self {
        Self {
            rank: 0,
            form: Form::Bare(value.to_owned()),
        }
    }
}

impl<L> From<&Node<L>> for Node<L>
where
    L: Clone,
{
    fn from(value: &Node<L>) -> Self {
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
