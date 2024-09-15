use async_trait::async_trait;
pub use gain::*;
pub use task::*;

use super::*;
use thiserror::Error;

mod gain;
mod task;

// pub type Result = result::Result<Gain, crate::Error>;

// #[async_trait(?Send)]
pub trait Solve {
    type Base: 'static + Payload;
    /// Solve a task.
    /// The hub will run computations or return existing results.
    async fn solve(&self, task: Task<'_>) -> Result<Gain<Self::Base>>;
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

pub fn solve_ok<T>() -> Result<Gain<T>>
where
    T: Payload,
{
    Ok(Gain::None)
}

pub trait Act {
    /// Perform an external action.
    fn act(&self) -> Result<()>;
}

// #[async_trait(?Send)]
impl<A: Act> Solve for A {
    type Base = ();
    async fn solve(&self, _: Task<'_>) -> Result<Gain<()>> {
        self.act()?;
        solve_ok()
    }
}

pub trait SolveMut {
    type Base: 'static + Payload;
    /// For graph internals to handle solve calls
    async fn solve(&mut self, task: Task) -> Result<Gain<Self::Base>>;
}
