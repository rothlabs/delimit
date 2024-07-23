use super::*;

pub struct AceUnit<L> {
    pub load: L,
}

impl<L: Clone> Grant for AceUnit<L> {
    type Load = Ace<L>;
    fn grant(&self) -> Self::Load {
        Ace::new(self.load.clone())
    }
}
