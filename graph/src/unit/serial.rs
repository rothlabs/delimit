use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::meta::Id;

#[derive(Serialize, Deserialize)]
pub struct Serial {
    crap: Vec<String>,
    root: String,
    nodes: HashMap<Id, u64>,
    parts: HashMap<u64, String>,
}

impl Serial {
    pub fn add<N: Serialize>(&mut self, node: N) -> &mut Self {
        if let Ok(node) = serde_json::to_string(&node) {
            self.crap.push(node);
        } else {
            eprintln!("failed to serialize node");
        }
        self
    }
}

pub trait ToSerial {
    fn serial(&mut self, serial: &'static mut Serial) -> &mut Serial;
}
