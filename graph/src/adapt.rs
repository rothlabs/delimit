use super::*;
use thiserror::Error;

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

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
pub trait Adapt {
    /// For graph internals to handle alter calls
    fn adapt_get(&self, deal: &mut dyn Deal) -> Result<()>;
    /// For graph internals to handle alter calls
    async fn adapt_set(&self, deal: &mut dyn Deal) -> Result<()>;
    fn transient_set(&self, deal: &mut dyn Deal) -> Result<Ring>;
}

pub trait AdaptMut {
    fn adapt_get(&mut self, deal: &mut dyn Deal) -> Result<()>;
    fn adapt_set(&mut self, deal: &mut dyn Deal) -> Result<Ring>;
}

// pub trait AdaptGetMut {
//     fn adapt_get(&mut self, deal: &mut dyn Deal) -> Result<()>;
// }

// pub trait AdaptGet {
//     /// For graph internals to handle alter calls
//     fn adapt_get(&self, deal: &mut dyn Deal) -> Result<()>;
//     fn adapt_set(&self, deal: &mut dyn Deal) -> Result<()>;
// }

