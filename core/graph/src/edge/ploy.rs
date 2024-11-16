use super::*;

pub type Edge<T> = Pointer<dyn Engage<Base = T>>;

/// General engagement of Ploy with erased unit type.
pub trait Engage: Solve + AdaptEdge + Update + SetRoot + Reckon + Debug {}
impl<E> Engage for E where E: Solve + AdaptEdge + Update + SetRoot + Reckon + Debug {}

pub trait Solve {
    type Base;
    fn solve(&self) -> GraphFuture<Result<Hub<Self::Base>>>;
    fn backed(&self, back: &Back) -> Edge<Self::Base>;
}