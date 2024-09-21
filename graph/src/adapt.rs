use super::*;
use thiserror::Error;

pub trait Adapt {
    /// Alter an hub.
    /// Useful for inserting, removing, and more.
    fn adapt(&mut self, _: &mut dyn Deal) -> Result<()> {
        Ok(())
    }
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

pub trait AdaptOut {
    /// Alter a hub.
    /// Useful for inserting, removing, and more.
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<(Ring, u64)>;
}

pub trait AdaptGet {
    /// For graph internals to handle alter calls
    fn adapt_get(&self, deal: &mut dyn Deal) -> Result<()>;
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
pub trait AdaptSet {
    /// For graph internals to handle alter calls
    async fn adapt_set(&self, deal: &mut dyn Deal) -> Result<()>;
}
