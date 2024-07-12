use crate::*;

#[derive(Clone, Serialize)]
pub enum SoleView<L> {
    Sole(Sole<L>),
    Ploy(Ploy<Sole<L>>),
}

impl<L> SoleView<L> {
    pub fn new(load: L) -> Self {
        Self::Sole(Sole::new(load))
    }
}

impl<L> Grant for SoleView<L>
where
    L: Clone,
{
    type Load = Sole<L>;
    fn grant(&self) -> Sole<L> {
        match self {
            SoleView::Sole(sole) => sole.clone(),
            SoleView::Ploy(ploy) => ploy.grant(),
        }
    }
}
