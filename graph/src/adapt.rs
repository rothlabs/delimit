pub use alter::ToAlter;
pub use memo::Memo;
pub use post::Post;

use super::*;
use std::result;

pub mod post;

mod alter;
mod memo;

pub type Result = result::Result<Memo, Error>;

pub trait Adapt {
    /// Alter a node.
    /// Useful for inserting, removing, and more.
    fn adapt(&mut self, post: Post) -> Result;
}

pub trait AdaptInner {
    /// For graph internals to handle alter calls
    fn adapt(&self, post: Post) -> Result;
}

pub fn no_adapter(post: Post) -> adapt::Result {
    Err(format!("did not adept: {:?}", post).into())
}

pub fn adapt_ok() -> adapt::Result {
    Ok(Memo::None)
}
