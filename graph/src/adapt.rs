// pub use post::Post;

use super::*;
use thiserror::Error;

// pub mod post;

pub trait Adapt {
    /// Alter an hub.
    /// Useful for inserting, removing, and more.
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()>;
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Aim(#[from] aim::Error),
    #[error(transparent)]
    Solve(#[from] solve::Error),
    #[error(transparent)]
    Hub(#[from] hub::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

// pub enum Memo {
//     None,
// }

// pub fn adapt_ok() -> Result<Memo> {
//     Ok(Memo::None)
// }

pub trait AdaptOut {
    /// Alter a hub.
    /// Useful for inserting, removing, and more.
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<(Vec<Root>, u64)>;
}

pub trait AdaptMid {
    /// For graph internals to handle alter calls
    fn adapt(&self, deal: &mut dyn Deal) -> Result<()>;
}
