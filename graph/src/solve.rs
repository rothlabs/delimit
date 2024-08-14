pub use gain::*;

use super::*;
use std::result;

mod gain;

pub type Result = result::Result<Gain, Error>;

pub trait Solve {
    /// Solve a task.
    /// The node will run computations or return existing results.
    fn solve(&self, task: Task) -> Result;
}

pub trait DoSolve {
    /// For graph internals to handle solve calls
    fn do_solve(&mut self, task: Task) -> Result;
}

pub fn no_solver() -> solve::Result {
    Err("No solver.")?
}

pub fn empty_nodes() -> solve::Result {
    Ok(Gain::Nodes(vec![]))
}

pub enum Task {
    Main,
    Stems,
    React,
    Export,
    Find(String),
    Serial,
    Hash,
}
