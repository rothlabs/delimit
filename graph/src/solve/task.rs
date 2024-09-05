use super::*;

#[derive(Debug)]
pub enum Task<'a> {
    // for units
    Main,
    Rank,
    Serial,
    Digest(&'a mut UnitHasher),
    React,
    // for graph internals
    None,
    Hash,
    Imports,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("no handler (Task: {task}, Unit: {unit})")]
    NoHandler { task: String, unit: String },
}

impl Task<'_> {
    /// Emit `NoHandler` error.
    pub fn no_handler<T>(&self, unit: &dyn Debug) -> Result<Gain<T>> {
        Err(solve::Error::from(Error::NoHandler {
            task: format!("{:?}", self),
            unit: format!("{:?}", unit),
        }))?
    }
}
