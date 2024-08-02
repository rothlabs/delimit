pub use edit::ToEdit;
pub use post::Post;
pub use report::Report;

use super::*;
use std::result;

pub mod post;

mod edit;
mod report;

pub type Result = result::Result<Report, Error>;

pub trait Alter {
    /// Alter a node.
    /// Useful for inserting, removing, and more.
    fn alter(&mut self, post: Post) -> Result;
}

pub trait DoAlter {
    /// For graph internals to handle alter calls
    fn alter(&self, post: Post) -> Result;
}
