pub use tray::{Tray, IntoTray};
pub use query::{Query, ToQuery};
pub use task::Task;

use std::result;
use super::*;

mod tray;
mod query;
mod task;

pub type Result = result::Result<Tray, Error>;

pub trait Solve {
    /// Solve a task.
    /// The node will run computations or return existing results.
    fn solve(&self, task: Task) -> Result;
}

pub trait DoSolve {
    /// For graph internals to handle solve calls
    fn do_solve(&mut self, task: Task, back: &Back) -> Result;
}
