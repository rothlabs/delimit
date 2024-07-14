use crate::*;

#[derive(Clone, Serialize)]
pub enum BareSole<L> {
    // SoleView
    Bare(L),
    Sole(Sole<L>),
}

impl<L> Default for BareSole<L>
where
    L: Default,
{
    fn default() -> Self {
        Self::Bare(L::default())
    }
}

impl FromString for BareSole<String> {
    fn from_string(string: &str) -> Self {
        Self::Bare(string.into())
    }
}

impl<L> FromSole for BareSole<L> {
    type Load = L;
    fn from_sole(sole: Sole<L>) -> Self {
        Self::Sole(sole)
    }
}

impl<L> IntoView for BareSole<L> {
    type Item = Sole<L>;
    fn into_view(sole: Self::Item) -> Self {
        Self::Sole(sole)
    }
}

impl<L> Reader for BareSole<L>
where
    L: 'static + Send + Sync,
{
    type Unit = L;
    fn reader<F: FnOnce(&L)>(&self, read: F) {
        match self {
            BareSole::Bare(bare) => read(bare),
            BareSole::Sole(sole) => sole.reader(read),
        };
    }
}

impl<L> Grant for BareSole<L>
where
    L: Clone, // + 'static,
{
    type Load = Sole<L>;
    fn grant(&self) -> Sole<L> {
        match self {
            BareSole::Bare(bare) => bare.clone().into_sole(),
            BareSole::Sole(leaf) => leaf.clone(),
        }
    }
}

impl<L> Backed for BareSole<L>
where
    L: Clone,
{
    type Back = Back;
    fn backed(&self, root: &Self::Back) -> Self {
        match self {
            BareSole::Bare(bare) => BareSole::Bare(bare.clone()),
            BareSole::Sole(sole) => BareSole::Sole(sole.backed(root)),
        }
    }
}
