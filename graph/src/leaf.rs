use std::any::Any;

use serde::Serialize;

use crate::{edge::BoxAny, Edge};

#[derive(Clone, Serialize)]
pub struct LeafStr(pub Edge<String, (), (), (), BoxAny, BoxAny, BoxAny, BoxAny>);

impl LeafStr {
    pub fn new(unit: &str) -> Self {
        Self (Edge::new(unit.to_owned()))
    }
    pub fn read<F: FnOnce(&String)>(&self, read: F) {
        self.0.read(read);
    }
    pub fn write<F: FnOnce(&mut String)>(&self, write: F) {
        self.0.write(write);
    }
    pub fn unit(&self) -> String {
        self.0.unit()
    }
}

// impl Stem for LeafStr {

// }
