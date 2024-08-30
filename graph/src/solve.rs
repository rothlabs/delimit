pub use gain::*;
pub use task::*;

use thiserror::Error;
use super::*;
use std::result;

mod gain;
mod task;

pub type Result = result::Result<Gain, crate::Error>;

pub trait Solve {
    /// Solve a task.
    /// The apex will run computations or return existing results.
    fn solve(&self, task: Task) -> Result;
}

pub trait SolveMut {
    /// For graph internals to handle solve calls
    fn solve(&mut self, task: Task) -> Result;
}

pub trait Act {
    /// Perform an external action. 
    fn act(&self) -> crate::Result<()>;
}

impl<T: Act> Solve for T {
    fn solve(&self, _: Task) -> Result {
        self.act()?;
        solve_ok()
    }
}

pub fn solve_ok() -> solve::Result {
    Ok(Gain::None)
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("no handler (Task: {task}, Unit: {unit})")]
    NoHandler { task: String, unit: String },
    #[error(transparent)]
    Gain(#[from] gain::Error),
    #[error(transparent)]
    Aim(#[from] aim::Error),
    #[error(transparent)]
    Apex(#[from] apex::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

pub trait NoSolver {
    /// Return solve::Error::NoHandler
    fn no_solver(&self, task: Task) -> solve::Result;
}

impl<T: Solve + Debug> NoSolver for T {
    /// Return solve::Error::NoHandler
    fn no_solver(&self, task: Task) -> solve::Result {
        task.no_solver(self)
    }
}