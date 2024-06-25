mod leaf;
mod pointer;
mod work;
mod solver;

pub use leaf::Leaf;
pub use pointer::Pointer;
pub use work::Work;
pub use solver::Solver;


// pub trait Respond {
//     type Memo;
//     fn respond(&mut self, memo: Self::Memo);
// }