pub use gain::*;
pub use task::*;

use super::*;
use std::future::Future;
use thiserror::Error;

mod gain;
mod task;

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

pub trait Solve {
    type Base: 'static + Payload;
    /// Solve a task.
    /// The hub will run computations or return existing results.
    fn solve(&self) -> impl Future<Output = Result<Hub<Self::Base>>> + IsSend {
        async { solve_ok() }
    }
    fn rank(&self) -> u16 {
        0
    }
}

pub trait Act {
    fn act(&self) -> impl Future<Output = Result<()>> + IsSend;
}

impl<T: Act + SendSync> Solve for T {
    type Base = ();
    async fn solve(&self) -> Result<Hub<()>> {
        self.act().await?;
        solve_ok()
    }
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

pub trait SolveAdapt {
    type Base: 'static + Payload;
    /// For graph internals to handle solve calls
    fn solve(&mut self) -> GraphFuture<Result<Hub<Self::Base>>> {
        Box::pin(async move { solve_ok() })
    }
    fn adapt(&mut self, _: &mut dyn Deal) -> Result<()> {
        Err(anyhow!("SolveAdapt::adapt not implemented"))?
    }
    fn back(&mut self, _: &Back) -> Result<()> {
        Err(anyhow!("SolveAdapt::back not implemented"))?
    }
}
