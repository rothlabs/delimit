use super::*;

/// Asset unit always grants the same Ace link.
/// Useful for a bare load to be converted into an Asset link.
pub struct Asset<L> {
    pub ace: Ace<L>,
}

impl<L> Asset<L> {
    pub fn link(load: L) -> Deuce<Self> {
        Deuce::new(Self {
            ace: Ace::new(load),
        })
    }
}

impl<L> Grant for Asset<L> {
    type Load = Ace<L>;
    fn grant(&self) -> Self::Load {
        self.ace.clone()
    }
}
