// pub use gain::*;
// pub use task::*;

use super::*;
use std::future::Future;
use thiserror::Error;

// mod gain;
// mod task;

#[derive(Error, Debug)]
pub enum Error {
    // #[error(transparent)]
    // Task(#[from] task::Error),
    // #[error(transparent)]
    // Gain(#[from] gain::Error),
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
    type Base: 'static + SendSync; // + Payload;
    /// Solve a task.
    /// The hub will run computations or return existing results.
    fn solve(&self) -> impl Future<Output = node::Result<Self::Base>> + IsSend {
        async { solve_ok() }
    }
    fn rank(&self) -> u16 {
        0
    }
}

pub trait Act {
    fn act(&self) -> impl Future<Output = node::Result<()>> + IsSend;
}

// impl<T: SendSync> Act for Grc<T> {
//     async fn act(&self) -> Result<()> {
//         Ok(())
//     }
// }

impl<T: Act + SendSync> Solve for T {
    type Base = ();
    async fn solve(&self) -> node::Result<()> {
        self.act().await?;
        solve_ok()
    }
}

// pub fn reckon_ok() -> Result<Gain> {
//     Ok(Gain::None)
// }

pub fn solve_ok<T>() -> node::Result<T>
where
    T: 'static + SendSync, //+ Payload,
{
    Ok(Hub::none())
}

pub trait SolveAdapt {
    type Base: 'static + SendSync; //Payload;
    /// For graph internals to handle solve calls
    fn solve(&mut self) -> GraphFuture<Result<Hub<Self::Base>>> {
        Box::pin(async move { Ok(solve_ok()?) })
    }
    fn adapt(&mut self, _: &mut dyn Deal) -> Result<()> {
        Err(anyhow!("SolveAdapt::adapt not implemented"))?
    }
    fn back(&mut self, _: &Back) -> Result<()> {
        Err(anyhow!("SolveAdapt::back not implemented"))?
    }
}
