use std::any::Any;

use serde::Serialize;

use crate::{root::AnyRoot, Root, Stem};

#[derive(Clone, Serialize)]
pub struct LeafStr{
    pub stem: Stem<String, (), ()>,
    pub root: Option<AnyRoot>,
}

impl LeafStr {
    pub fn new(unit: &str) -> Self {
        Self {
            stem: Stem::new(unit.to_owned()),
            root: None, 
        }
    }
    pub fn read<F: FnOnce(&String)>(&self, read: F) {
        self.stem.read(read);
    }
    pub fn write<F: FnOnce(&mut String)>(&self, write: F) {
        self.stem.write(write);
        if let Some(root) = &self.root {
            //oot.0.
        }
    }
    pub fn unit(&self) -> String {
        self.stem.unit()
    }
}

// impl Stem for LeafStr {

// }
