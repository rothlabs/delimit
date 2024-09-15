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
    async fn solve(&self) -> Result<Hub<Self::Base>>;
}

pub trait Reckon {
    fn reckon(&self, task: Task) -> Result<Gain>;
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

pub fn reckon_ok() -> Result<Gain>
{
    Ok(Gain::None)
}

pub fn solve_ok<T>() -> Result<Hub<T>>
where 
    T: 'static + Payload
{
    Ok(Hub::none())
}

pub trait Act {
    /// Perform an external action.
    async fn act(&self) -> Result<()>;
}

// #[async_trait(?Send)]
impl<A: Act> Solve for A {
    type Base = ();
    async fn solve(&self) -> Result<Hub<()>> {
        self.act().await?;
        solve_ok()
    }
}

pub trait SolveMut {
    type Base: 'static + Payload;
    /// For graph internals to handle solve calls
    async fn solve(&mut self) -> Result<Hub<Self::Base>>;
}

pub trait ReckonMut {
    fn reckon(&mut self, task: Task) -> Result<Gain>;
}
