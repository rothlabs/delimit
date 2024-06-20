use serde::Serialize;

use crate::Stem;

#[derive(Clone, Serialize)]
pub struct LeafStr(pub Stem<String, (), ()>);

impl LeafStr {
    pub fn new(unit: &str) -> Self {
        LeafStr(Stem::new(unit.to_owned()))
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
