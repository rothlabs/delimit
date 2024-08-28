pub use gain::*;
use thiserror::Error;

use super::*;
use std::result;

mod gain;

pub type Result = result::Result<Gain, crate::Error>;

pub trait Solve {
    /// Solve a task.
    /// The apex will run computations or return existing results.
    fn solve(&self, task: Task) -> Result;
}

pub trait DoSolve {
    /// For graph internals to handle solve calls
    fn do_solve(&mut self, task: Task) -> Result;
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
    #[error("no handler (Task: {task}, Unit: {unit})")]
    NoHandler { task: String, unit: String },
    #[error("wrong gain (expected: {expected:?}, found: {found:?})")]
    WrongGain { expected: String, found: String },
    #[error("index out of bounds: {0}")]
    IndexOutOfBounds(usize),
    #[error(transparent)]
    Apex(#[from] apex::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

pub fn no_solver(unit: &dyn Debug, task: Task) -> solve::Result {
    Err(Error::NoHandler {
        task: format!("{:?}", task),
        unit: format!("{:?}", unit),
    })?
}

// pub fn wrong_gain(expected: &str, found: &str) -> solve::Result {
//     Err(Error::WrongGain { expected: expected.into(), found: found.into() })
// }

// pub fn no_adapter(post: Post) -> adapt::Result {
//     Err(AdaptError::NoAdapter)
//     //Err(format!("No adapter: {:?}", post))?
// }
