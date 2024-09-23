use anyhow::anyhow;
use async_trait::async_trait;
pub use gain::*;
pub use task::*;

use super::*;
use thiserror::Error;

mod gain;
mod task;

pub trait Solve {
    type Base: 'static + Payload;
    /// Solve a task.
    /// The hub will run computations or return existing results.
    #[cfg(not(feature = "oneThread"))]
    fn solve(&self) -> impl std::future::Future<Output = Result<Hub<Self::Base>>> + Send;
    #[cfg(feature = "oneThread")]
    fn solve(&self) -> impl std::future::Future<Output = Result<Hub<Self::Base>>>;
    fn reckon(&self, _: Task) -> Result<Gain> {
        Err(anyhow!("reckon not defined"))?
    }
    fn adapt(&mut self, _: &mut dyn Deal) -> Result<()> {
        Err(anyhow!("adapt not defined"))?
    }
    fn back(&mut self, _: &Back) -> Result<()> {
        Err(anyhow!("back not defined"))?
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
pub trait SolveMid {
    type Base: Payload;
    async fn solve(&self) -> Result<Hub<Self::Base>>;
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Task(#[from] task::Error),
    #[error(transparent)]
    Gain(#[from] gain::Error),
    #[error(transparent)]
    Aim(#[from] aim::Error),
    #[error(transparent)]
    Hub(#[from] hub::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

pub fn reckon_ok() -> Result<Gain> {
    Ok(Gain::None)
}

pub fn solve_ok<T>() -> Result<Hub<T>>
where
    T: 'static + Payload,
{
    Ok(Hub::none())
}

pub trait Act {
    #[cfg(not(feature = "oneThread"))]
    fn act(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    #[cfg(feature = "oneThread")]
    fn act(&self) -> impl std::future::Future<Output = Result<()>>;
    fn back(&mut self, _: &Back) -> Result<()> {
        Ok(())
    }
    fn reckon(&self, _: Task) -> Result<Gain> {
        Err(anyhow!("reckon not defined"))?
    }
}

impl<A: Act + SendSync> Solve for A {
    type Base = ();
    async fn solve(&self) -> Result<Hub<()>> {
        self.act().await?;
        solve_ok()
    }
    fn back(&mut self, back: &Back) -> Result<()> {
        self.back(back)
    }
    fn reckon(&self, task: Task) -> Result<Gain> {
        self.reckon(task)
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
pub trait SolveMut {
    type Base: 'static + Payload;
    /// For graph internals to handle solve calls
    async fn solve(&mut self) -> Result<Hub<Self::Base>>;
    fn adapt(&mut self, _: &mut dyn Deal) -> Result<()> {
        Ok(())
    }
    fn back(&mut self, _: &Back) -> Result<()> {
        Ok(())
    }
    fn reckon(&mut self, _: Task) -> Result<Gain> {
        Err(anyhow!("reckon not defined"))?
    }
}
