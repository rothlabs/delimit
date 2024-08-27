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

pub fn adapt_ok() -> adapt::Result {
    Ok(Memo::None)
}

pub enum Memo {
    None,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("no adapter")]
    NoAdapter,
    #[error("apex error")]
    Apex(#[from] apex::Error),
    #[error("graph error")]
    Graph(#[from] crate::Error)
}

pub fn no_adapter(post: Post) -> adapt::Result {
    Err(Error::NoAdapter)
    //Err(format!("No adapter: {:?}", post))?
}
