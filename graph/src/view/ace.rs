use crate::*;

/// A bare load or `link::Ace<Load>`
#[derive(Clone, Serialize)]
pub enum Ace<L> {
    Bare(L),
    Link(link::Ace<L>),
}

impl<L> Default for Ace<L>
where
    L: Default,
{
    fn default() -> Self {
        Self::Bare(L::default())
    }
}

impl From<&str> for Ace<String> {
    fn from(value: &str) -> Self {
        Self::Bare(value.into())
    }
}

impl<L> FromAce for Ace<L> {
    type Load = L;
    fn from_ace(ace: link::Ace<L>) -> Self {
        Self::Link(ace)
    }
}

impl<L> Reader for Ace<L>
where
    L: 'static + Send + Sync,
{
    type Item = L;
    fn read<F: FnOnce(&L)>(&self, read: F) {
        match self {
            Ace::Bare(bare) => read(bare),
            Ace::Link(ace) => ace.read(read),
        };
    }
}

impl<L> Grant for Ace<L>
where
    L: Clone,
{
    type Load = link::Ace<L>;
    fn grant(&self) -> link::Ace<L> {
        match self {
            Ace::Bare(bare) => bare.ace(),
            Ace::Link(ace) => ace.clone(),
        }
    }
}

impl<L> Backed for Ace<L>
where
    L: Clone,
{
    fn backed(&self, back: &Back) -> Self {
        match self {
            Ace::Bare(bare) => Ace::Bare(bare.clone()),
            Ace::Link(ace) => Ace::Link(ace.backed(back)),
        }
    }
}

// impl<L> From<Ace<L>> for AceView<L> {
//     fn from(ace: Ace<L>) -> Self {
//         Self::Ace(ace)
//     }
// }
