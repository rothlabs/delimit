use super::*;

/// `Link` to domain-specific node.
/// The unit type is erased. To keep unit type intact, use `Node` instead.
pub type Ploy<T> = Link<dyn Engage<Base = T>>;

pub type PloyEdge<T> = Pointer<dyn Engage<Base = T>>;

/// General engagement of Ploy with erased unit type.
pub trait Engage: Based + Adapt + Update + SetRoot + Reckon + Debug {}
impl<E> Engage for E where E: Based + Adapt + Update + SetRoot + Reckon + Debug {}

pub trait Based {
    type Base: Payload;
    fn solve(&self) -> GraphFuture<Result<Hub<Self::Base>>>;
    fn backed(&self, back: &Back) -> PloyEdge<Self::Base>;
}

pub type Wing<T> = Link<dyn Employ<Base = T>>;

pub type WingEdge<T> = Pointer<dyn Employ<Base = T>>;

pub trait Employ: Employed + Adapt + Update + SetRoot + Debug {}
impl<E> Employ for E where E: Employed + Adapt + Update + SetRoot + Debug {}

pub trait Employed {
    type Base: Payload;
    fn solve(&self) -> GraphFuture<Result<Hub<Self::Base>>>;
    fn backed(&self, back: &Back) -> WingEdge<Self::Base>;
}

impl<T: Payload> Serialize for Wing<T> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_unit()
    }
}
