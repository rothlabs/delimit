use super::*;

/// `Link` to domain-specific node.
/// The unit type is erased. To keep unit type intact, use `Node` instead.
pub type Ploy<T> = Link<dyn Engage<Base = T>>;

pub type PloyEdge<T> = Pointer<dyn Engage<Base = T>>;

/// General engagement of Ploy with erased unit type.
pub trait Engage: Based + Adapt + Update + SetRoot + Debug {}
impl<E> Engage for E where E: Based + Adapt + Update + SetRoot + Debug {}

pub trait Based {
    type Base: Payload;
    fn solve(&self) -> GraphFuture<Result<Hub<Self::Base>>>;
    fn backed(&self, back: &Back) -> PloyEdge<Self::Base>;
    fn reckon(&self, task: Task) -> Result<Gain>;
}
