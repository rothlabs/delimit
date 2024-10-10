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

pub trait Adapt {
    fn adapt(&mut self, _: &mut dyn Deal) -> Result<()> {
        Err(anyhow!("Adapt::adapt not implemented"))?
    }
    fn back(&mut self, _: &Back) -> Result<()> {
        Err(anyhow!("Adapt::back not implemented"))?
    }
}

pub trait AdaptEdge {
    /// For graph internals to handle alter calls
    fn adapt_get(&self, deal: &mut dyn Deal) -> Result<()>;
    /// For graph internals to handle alter calls
    fn adapt_set<'a>(&'a self, deal: &'a mut dyn Deal) -> GraphFuture<Result<()>>;
    fn transient_set(&self, deal: &mut dyn Deal) -> Result<Ring>;
}

pub trait AdaptMut {
    fn adapt_get(&mut self, deal: &mut dyn Deal) -> Result<()>;
    fn adapt_set(&mut self, deal: &mut dyn Deal) -> Result<Ring>;
}
