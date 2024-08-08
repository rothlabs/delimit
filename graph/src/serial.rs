use super::*;
use std::{collections::HashMap, result};

pub type Result = result::Result<(), Error>;

#[derive(Default, Serialize, Deserialize)]
pub struct Serial {
    pub parts: HashMap<Path, String>,
}

impl Serial {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn contains(&self, meta: &Meta) -> bool {
        self.parts.contains_key(&meta.path)
    }
    pub fn insert(&mut self, meta: &Meta, node: String) {
        self.parts.insert(meta.path.clone(), node);
    }
    pub fn string(&self) -> result::Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

pub trait SerializeGraph {
    fn serial(&self, serial: &mut Serial) -> Result;
}

impl<T> SerializeGraph for Vec<T>
where
    T: SerializeGraph,
{
    fn serial(&self, serial: &mut Serial) -> Result {
        for item in self {
            item.serial(serial)?;
        }
        Ok(())
    }
}

pub trait SerializeGraphInner {
    fn serial(&mut self, serial: &mut Serial, back: &Back) -> Result;
}
