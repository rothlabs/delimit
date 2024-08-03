pub use query::{Query, ToQuery};
pub use task::Task;
pub use tray::{IntoTray, Tray};

use super::*;
use std::result;

mod query;
mod task;
mod tray;

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

// pub trait DoSolve {
//     /// For graph internals to handle solve calls
//     fn do_solve(&mut self, task: Task, back: &Back) -> Result;
// }
