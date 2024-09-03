pub use post::Post;

use super::*;
use thiserror::Error;

pub mod post;

pub trait Adapt {
    /// Alter an apex.
    /// Useful for inserting, removing, and more.
    fn adapt<'a>(&'a mut self, deal: &'a mut dyn Deal<'a>) -> Result<Memo>;
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Post(#[from] post::Error),
    #[error(transparent)]
    Aim(#[from] aim::Error),
    #[error(transparent)]
    Solve(#[from] solve::Error),
    #[error(transparent)]
    Apex(#[from] apex::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

pub enum Memo {
    None,
}

pub fn adapt_ok() -> Result<Memo> {
    Ok(Memo::None)
}

pub trait AdaptOut {
    /// Alter a apex.
    /// Useful for inserting, removing, and more.
    fn adapt<'a>(&'a mut self, deal: &'a mut dyn Deal<'a>)-> Result<write::Out<Memo>>;
}

pub trait AdaptMid {
    /// For graph internals to handle alter calls
    fn adapt<'a>(&'a self, deal: &'a mut dyn Deal<'a>) -> Result<Memo>;
}
