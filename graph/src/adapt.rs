pub use post::Post;

use super::*;
use std::result;

pub mod post;

pub type Result = result::Result<Memo, Error>;

pub trait Adapt {
    /// Alter an apex.
    /// Useful for inserting, removing, and more.
    fn adapt(&mut self, post: Post) -> Result;
}

pub trait AdaptOut {
    /// Alter a apex.
    /// Useful for inserting, removing, and more.
    fn adapt(&mut self, post: Post) -> write::Out<Result>;
}

pub trait AdaptMid {
    /// For graph internals to handle alter calls
    fn adapt(&self, post: Post) -> Result;
}

pub fn no_adapter(post: Post) -> adapt::Result {
    Err(format!("No adapter: {:?}", post))?
}

pub fn adapt_ok() -> adapt::Result {
    Ok(Memo::None)
}

pub enum Memo {
    None,
}
