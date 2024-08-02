pub use post::{Post, Form};
pub use report::Report;

use super::*;
use std::result;

mod post;
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
