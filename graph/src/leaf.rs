use std::any::Any;

use serde::Serialize;

use crate::{Edge, node};

type RootStr = Node<BoxAny, (), (), ()>;
type StemStr = node::Main<String, (), (), ()>;

#[derive(Clone, Serialize)]
pub struct LeafStr(pub Edge<RootStr, StemStr>);

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
