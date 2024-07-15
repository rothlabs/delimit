use crate::*;

/// A bare Load or Ace<Load>
#[derive(Clone, Serialize)]
pub enum AceView<L> {
    Bare(L),
    Ace(Ace<L>),
}

impl<L> Default for AceView<L>
where
    L: Default,
{
    fn default() -> Self {
        Self::Bare(L::default())
    }
}

impl From<&str> for AceView<String> {
    fn from(value: &str) -> Self {
        Self::Bare(value.into())
    }
}

impl<L> FromAce for AceView<L> {
    type Load = L;
    fn from_ace(ace: Ace<L>) -> Self {
        Self::Ace(ace)
    }
}

impl<L> Reader for AceView<L>
where
    L: 'static + Send + Sync,
{
    type Item = L;
    fn reader<F: FnOnce(&L)>(&self, read: F) {
        match self {
            AceView::Bare(bare) => read(bare),
            AceView::Ace(ace) => ace.reader(read),
        };
    }
}

impl<L> Grant for AceView<L>
where
    L: Clone,
{
    type Load = Ace<L>;
    fn grant(&self) -> Ace<L> {
        match self {
            AceView::Bare(bare) => bare.ace(),
            AceView::Ace(ace) => ace.clone(),
        }
    }
}

impl<L> Backed for AceView<L>
where
    L: Clone,
{
    type Back = Back;
    fn backed(&self, root: &Self::Back) -> Self {
        match self {
            AceView::Bare(bare) => AceView::Bare(bare.clone()),
            AceView::Ace(ace) => AceView::Ace(ace.backed(root)),
        }
    }
}

// impl<L> From<Ace<L>> for AceView<L> {
//     fn from(ace: Ace<L>) -> Self {
//         Self::Ace(ace)
//     }
// }
