use super::*;

#[derive(Debug)]
pub enum Task<'a> {
    None,
    Main,
    All,
    React,
    Serial,
    Hash,
    Digest(&'a mut UnitHasher),
    Imports,
    Get(&'a Key),
    Map,
    Rank,
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
