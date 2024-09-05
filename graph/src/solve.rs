pub use gain::*;
pub use task::*;

use super::*;
use thiserror::Error;

mod gain;
mod task;

// pub type Result = result::Result<Gain, crate::Error>;

pub trait Solve {
    /// Solve a task.
    /// The hub will run computations or return existing results.
    fn solve(&self, task: Task) -> Result<Gain>;
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

pub fn solve_ok() -> Result<Gain> {
    Ok(Gain::None)
}

pub trait Act {
    /// Perform an external action.
    fn act(&self) -> Result<()>;
}

impl<T: Act> Solve for T {
    fn solve(&self, _: Task) -> Result<Gain> {
        self.act()?;
        solve_ok()
    }
}

pub trait SolveMut {
    /// For graph internals to handle solve calls
    fn solve(&mut self, task: Task) -> Result<Gain>;
}
