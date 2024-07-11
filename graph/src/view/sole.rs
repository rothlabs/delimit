use crate::*;

#[derive(Clone, Serialize)]
pub enum SoleView<L> {
    Leaf(Sole<L>),
    Solver(Solver<Sole<L>>),
}

impl<L> SoleView<L> {
    pub fn new(load: L) -> Self {
        Self::Leaf(Sole::new(load))
    }
}

impl<L> Solve for SoleView<L>
where
    L: Clone,
{
    type Load = Sole<L>;
    fn solve(&self) -> Sole<L> {
        match self {
            SoleView::Leaf(leaf) => leaf.clone(),
            SoleView::Solver(solver) => solver.solve(),
        }
    }
}
