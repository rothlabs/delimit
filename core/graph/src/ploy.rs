use super::*;

// TODO: break these up into separate Gate and Ploy modules

/// `Link` to domain-specific node.
/// The unit type is erased. To keep unit type intact, use `Node` instead.
pub type Ploy<T> = Link<dyn Engage<Base = T>>;

pub type PloyEdge<T> = Pointer<dyn Engage<Base = T>>;

/// General engagement of Ploy with erased unit type.
pub trait Engage: SolvePloy + AdaptEdge + Update + SetRoot + Reckon + Debug {}
impl<E> Engage for E where E: SolvePloy + AdaptEdge + Update + SetRoot + Reckon + Debug {}

pub trait SolvePloy {
    type Base; //: Transmit;
    fn solve(&self) -> GraphFuture<Result<Hub<Self::Base>>>;
    fn backed(&self, back: &Back) -> PloyEdge<Self::Base>;
}

pub type Gate<T> = Link<dyn Employ<Base = T>>;

pub type GateEdge<T> = Pointer<dyn Employ<Base = T>>;

pub trait Employ: SolveGate + AdaptEdge + Update + SetRoot + Debug {}
impl<E> Employ for E where E: SolveGate + AdaptEdge + Update + SetRoot + Debug {}

pub trait SolveGate {
    type Base;
    fn solve(&self) -> GraphFuture<Result<Hub<Self::Base>>>;
    fn backed(&self, back: &Back) -> GateEdge<Self::Base>;
}

impl<T: Transmit> Serialize for Gate<T> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_unit()
    }
}
