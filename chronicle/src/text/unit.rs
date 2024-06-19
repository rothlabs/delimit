use serde::Serialize;

use graph::{LeafStr, Stem};

use super::{text, Text, Unit};

pub fn list() -> List {
    List {
        items: vec![],
        separator: "".into(),
    }
}

#[derive(Clone, Serialize)]
pub struct List {
    pub items: Vec<Item>,
    pub separator: String,
}

impl List {
    pub fn text(self) -> Text {
        text(self)
    }
    pub fn separator(&mut self, sep: &str) -> &mut Self {
        self.separator = sep.to_owned();
        self
    }
    pub fn add_text(&mut self, text: &Text) -> &mut Self {
        self.items.push(Item::Text(text.clone()));
        self
    }
    pub fn add_str(&mut self, unit: &str) -> &mut Self {
        self.items.push(Item::String(unit.to_owned()));
        self
    }
    pub fn add_list(&mut self, list: List) -> &mut Self {
        self.items.push(Item::Text(text(list)));
        self
    }
    pub fn add_leaf(&mut self, leaf: &LeafStr) -> &mut Self {
        self.items.push(Item::LeafStr(leaf.clone()));
        self
    }
}

impl Unit for List {
    fn leaf(&self) -> LeafStr {
        let mut string = String::new();
        for i in 0..self.items.len() - 1 {
            self.items[i].read(|s| string += s);
            string += &self.separator;
        }
        if let Some(item) = self.items.last() {
            item.read(|s| string += s);
        }
        LeafStr::new(&string)
    }
    fn serial(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    fn string(&self) -> String {
        self.leaf().unit()
    }
    fn all_stems(&self) -> Vec<Box<dyn Stem>> {
        let mut stems = vec![];
        for item in self.items.iter() {
            match item {
                Item::LeafStr(s) => stems.push(Box::new(s.0.clone()) as Box<dyn Stem>),
                Item::Text(s) => stems.push(Box::new(s.0.clone()) as Box<dyn Stem>),
                _ => ()
            }
        }
        stems
    }
}

#[derive(Clone, Serialize)]
pub enum Item {
    String(String),
    LeafStr(LeafStr),
    Text(Text),
}

impl Item {
    fn read<F: FnOnce(&String)>(&self, f: F) {
        match self {
            Item::String(s) => f(s),
            Item::LeafStr(l) => l.read(f),
            Item::Text(t) => t.leaf().read(f),
        };
    }
}
