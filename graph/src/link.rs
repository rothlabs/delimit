mod leaf;
mod link;
mod solver;

pub use leaf::Leaf;
pub use link::Link;
pub use solver::Solver;

// use crate::*;

// pub trait LinkMeta {
//     fn meta(&self) -> Meta;
// }

// impl<T: LinkMeta> PartialEq for Link<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.meta() == other.meta()
//     }
// }
