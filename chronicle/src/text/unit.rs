use serde::Serialize;

use graph::{
    link::{CloneUnit, Leaf, Read},
    New,
};

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
        text(Box::new(self))
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
        self.items.push(Item::Text(text(Box::new(list))));
        self
    }
    pub fn add_leaf(&mut self, leaf: &Leaf<String>) -> &mut Self {
        self.items.push(Item::Leaf(leaf.clone()));
        self
    }
}

impl Unit for List {
    fn leaf(&self) -> Leaf<String> {
        let mut string = String::new();
        for i in 0..self.items.len() - 1 {
            self.items[i].read(|s| string += s);
            string += &self.separator;
        }
        if let Some(item) = self.items.last() {
            item.read(|s| string += s);
        }
        Leaf::new(string)
    }
    fn serial(&self) -> String {
        String::new()
        // serde_json::to_string(self).unwrap()
    }
    fn string(&self) -> String {
        self.leaf().unit()
    }
}

#[derive(Clone, Serialize)]
pub enum Item {
    String(String),
    Leaf(Leaf<String>),
    Text(Text),
}

impl Item {
    fn read<F: FnOnce(&String)>(&self, f: F) {
        match self {
            Item::String(s) => f(s),
            Item::Leaf(l) => l.read(f),
            Item::Text(t) => t.leaf().read(f), //f(t.leaf().read()),
        };
    }
}
