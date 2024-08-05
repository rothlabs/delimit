use super::*;
use std::{collections::HashMap, result};

pub type Result = result::Result<(), Error>;

#[derive(Default, Serialize, Deserialize)]
pub struct Serial {
    pub nodes: HashMap<Id, String>,
}

impl Serial {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn contains(&self, meta: &Meta) -> bool {
        self.nodes.contains_key(&meta.id)
    }
    pub fn insert(&mut self, meta: &Meta, node: String) {
        self.nodes.insert(meta.id.clone(), node);
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
    T: SerializeGraph
{
    fn serial(&self, serial: &mut Serial) -> Result {
        for item in self {
            item.serial(serial)?;
        }
        Ok(())
    }
}

pub trait DoSerializeGraph {
    fn serial(&mut self, serial: &mut Serial, back: &Back) -> Result;
}

// pub trait SerializeGraph {
//     fn serialize<S: Serializer>(&self, serializer: S, serial: &mut Serial) -> Result<S::Ok, S::Error>;
// }

// impl Serial {
//     pub fn contains(&self, meta: &Meta) -> bool {
//         self.nodes.contains_key(meta)
//     }
//     pub fn insert(&mut self, meta: &Meta, node: String) {
//         self.nodes.insert(meta.clone(), node);
//     }
// }



