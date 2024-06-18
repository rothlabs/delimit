use serde::Serialize;

use crate::Edge;

#[derive(Clone, Serialize)]
pub struct LeafStr(Edge<String, (), ()>);

impl LeafStr {
    pub fn new(unit: &str) -> Self {
        LeafStr(Edge::new(unit.to_owned()))
    }
    pub fn read<F: FnOnce(&String)>(&self, read: F) { 
        self.0.read(read);
    }
    pub fn write<F: FnOnce(&mut String)>(&self, write: F) { 
        self.0.write(write);
    }
    pub fn string(&self) -> String {
        let mut string = String::new();
        self.0.read(|unit| string = unit.clone());
        string
    }
}