pub use gain::*;
use thiserror::Error;

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

pub fn empty_apexes() -> solve::Result {
    Ok(Gain::Apexes(vec![]))
}

pub fn solve_ok() -> solve::Result {
    Ok(Gain::None)
}

#[derive(Debug)]
pub enum Task<'a> {
    Main,
    All,
    React,
    Serial,
    Hash,
    Digest(&'a mut UnitHasher),
    Imports,
    Get(&'a Key),
    Map,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("No solver. Task: {task}, Unit: {unit}")]
    NoSolver{task: String, unit: String},
    #[error("wrong gain (expected {expected:?}, found {found:?})")]
    WrongGain{expected: String, found: String},
    #[error("index out of bounds: {0}")]
    IndexOutOfBounds(usize),
    #[error("apex error")]
    Apex(#[from] apex::Error),
    #[error("graph error")]
    Graph(#[from] crate::Error),
    #[error("serde error")]
    Serde(#[from] serde_json::Error),
}

pub fn no_solver(unit: &dyn Debug, task: Task) -> solve::Result {
    Err(Error::NoSolver{unit: format!("{:?}",unit), task: format!("{:?}",task)})
    //Err(format!("No solver. Task: {:?} Unit: {:?}", task, unit))?
}

// pub fn wrong_gain(expected: &str, found: &str) -> solve::Result {
//     Err(Error::WrongGain { expected: expected.into(), found: found.into() })
//     //Err(format!("No solver. Task: {:?} Unit: {:?}", task, unit))?
// }

// pub fn no_adapter(post: Post) -> adapt::Result {
//     Err(AdaptError::NoAdapter)
//     //Err(format!("No adapter: {:?}", post))?
// }