pub use alter::ToAlter;
pub use post::Post;
pub use gain::Gain;

use super::*;
use std::result;

pub mod post;

mod alter;
mod gain;

pub type Result = result::Result<Gain, Error>;

pub trait Adapt {
    /// Alter a node.
    /// Useful for inserting, removing, and more.
    fn adapt(&mut self, post: Post) -> Result;
}

pub trait AdaptInner {
    /// For graph internals to handle alter calls
    fn adapt(&self, post: Post) -> Result;
}
