use super::*;

pub type Edge<T> = Grc<dyn Engage<Base = T>>;

/// General engagement of Ploy with erased unit type.
pub trait Engage: Solve + Adapt + Update + Reckon + Debug {}
impl<E> Engage for E where E: Solve + Adapt + Update + Reckon + Debug {}

pub trait Solve {
    type Base;
    fn solve(&self, root: Root) -> GraphFuture<Result<Hub<Self::Base>>>;
    fn backed(&self, back: &Back) -> Edge<Self::Base>;
}
