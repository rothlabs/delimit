use super::*;

#[derive(Debug)]
pub enum Task<'a> {
    // for units
    Rank,
    Serial,
    Digest(&'a mut UnitHasher),
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
    pub fn no_handler(&self, unit: &dyn Debug) -> Result<Gain> {
        Err(solve::Error::from(Error::NoHandler {
            task: format!("{:?}", self),
            unit: format!("{:?}", unit),
        }))?
    }
}
