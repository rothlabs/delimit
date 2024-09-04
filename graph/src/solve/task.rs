use super::*;

#[derive(Debug)]
pub enum Task<'a> {
    None,
    Main,
    Rank,
    Serial,
    Digest(&'a mut UnitHasher),
    Hash,
    React,
    // TODO: make imports separate trait
    Imports,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("no handler (Task: {task}, Unit: {unit})")]
    NoHandler { task: String, unit: String },
}

impl Task<'_> {
    /// Emit `NoHandler` error.
    pub fn no_handler(&self, unit: &dyn Debug) -> Result<Gain> {
        Err(solve::Error::from(Error::NoHandler {
            task: format!("{:?}", self),
            unit: format!("{:?}", unit),
        }))?
    }
}
