pub use post::Post;

use thiserror::Error;
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

pub enum Memo {
    None,
}

pub fn adapt_ok() -> adapt::Result {
    Ok(Memo::None)
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("no handler (Post: {post}, Unit: {unit})")]
    NoHandler{post: String, unit: String},
    #[error(transparent)]
    Solve(#[from] solve::Error),
    #[error(transparent)]
    Apex(#[from] apex::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error)
}

pub fn no_adapter(unit: &dyn Debug, post: Post) -> adapt::Result {
    Err(Error::NoHandler{post: format!("{:?}", post), unit: format!("{:?}", unit)})
}
