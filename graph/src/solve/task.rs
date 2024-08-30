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
}

impl Task<'_> {
    /// Return solve::Error::NoHandler
    pub fn no_solver(&self, unit: &dyn Debug) -> solve::Result {
        Err(Error::NoHandler {
            task: format!("{:?}", self),
            unit: format!("{:?}", unit),
        })?
    }
}