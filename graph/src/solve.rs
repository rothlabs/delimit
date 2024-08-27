pub use gain::*;

use super::*;
use std::result;

mod gain;

pub type Result = result::Result<Gain, Error>;

pub trait Solve {
    /// Solve a task.
    /// The apex will run computations or return existing results.
    fn solve(&self, task: Task) -> Result;
}

impl Solve for Box<dyn Engage> {
    fn solve(&self, task: Task) -> solve::Result {
        self.as_ref().solve(task)
    }
}

pub trait DoSolve {
    /// For graph internals to handle solve calls
    fn do_solve(&mut self, task: Task) -> Result;
}

pub fn no_solver(unit: &dyn Debug, task: Task) -> solve::Result {
    Err(format!(
        "No solver.\n Task: {:?}\n Unit: {:?}\n",
        task, unit
    ))?
}

pub fn empty_apexes() -> solve::Result {
    Ok(Gain::Apexes(vec![]))
}

pub fn no_gain() -> solve::Result {
    Ok(Gain::None)
}

#[derive(Debug)]
pub enum Task<'a> {
    Main,
    Stems,
    React,
    Serial,
    Digest,
    Imports,
    Get(&'a Key),
    Map,
}
