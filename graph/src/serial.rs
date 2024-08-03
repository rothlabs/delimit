// use serde::{Serialize, Serializer};

use super::*;
use std::{collections::HashMap, result};

pub type Result = result::Result<(), Error>;

// #[derive(Serialize)]
pub struct Serial {
    pub nodes: HashMap<Meta, String>,
}

impl Serial {
    pub fn contains(&self, meta: &Meta) -> bool {
        self.nodes.contains_key(meta)
    }
    pub fn insert(&mut self, meta: &Meta, node: String) {
        self.nodes.insert(meta.clone(), node);
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



