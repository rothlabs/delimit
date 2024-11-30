pub use leaf::Leaf;
pub use node::Node;

use super::*;

mod leaf;
mod node;

pub trait SolveAdapt {
    type Base: 'static + SendSync;
    /// For graph internals to handle solve calls
    fn solve(&mut self) -> GraphFuture<Result<Hub<Self::Base>>>;
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()>;
    fn back(&mut self, back: &Back);
}
