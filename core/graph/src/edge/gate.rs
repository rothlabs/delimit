use super::*;

pub type Edge<T> = Grc<dyn Engage<Base = T>>;

pub trait Engage: Solve + Adapt + Update + Debug {}
impl<E> Engage for E where E: Solve + Adapt + Update + Debug {}

pub trait Solve {
    type Base;
    fn solve(&self, root: Root) -> GraphFuture<Result<Hub<Self::Base>>>;
    fn backed(&self, back: &Back) -> (Edge<Self::Base>, Root);
}

// impl<T: Transmit> Serialize for Gate<T> {
//     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         serializer.serialize_unit()
//     }
// }
