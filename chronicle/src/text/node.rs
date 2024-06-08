use std::rc::Rc;

use serde::Serialize;

use graph::{cell::string_unit, Id};
use super::{StringUnit, Text};

pub trait Node {
    fn string(&self) -> StringUnit;
    fn serialize(&self) -> String;
    fn id(&self) -> &Id;
}

#[derive(Serialize)]
pub struct Leaf {
    pub string: StringUnit,
    pub id: Id,
}

impl Node for Leaf {
    fn string(&self) -> StringUnit {
        self.string.clone()
    }
    fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    fn id(&self) -> &Id {
        &self.id
    }
}

pub fn leaf(value: &str) -> Rc<Leaf> {
    Rc::new(Leaf {
        string: string_unit(value),
        id: Id::new("text/leaf"),
    })
}

pub fn leaf_node(value: &str) -> Text {
    Text(leaf(value))
}

#[derive(Serialize)]
pub struct List {
    pub items: Vec<Text>,
    pub separator: String,
    pub id: Id,
}

impl List {
    pub fn text(self) -> Text {
        Text(Rc::new(self))
    }
    pub fn add_text(&mut self, node: &Text) -> &mut Self {
        self.items.push(node.clone());
        self
    }
    pub fn add_leaf(&mut self, value: Rc<Leaf>) -> &mut Self {
        self.items.push(Text(value));
        self
    }
    pub fn add_string(&mut self, string: &str) -> &mut Self {
        self.items.push(leaf_node(string));
        self
    }
    pub fn add_list(&mut self, list: List) -> &mut Self {
        self.items.push(Text(Rc::new(list)));
        self
    }
    pub fn separator(&mut self, sep: &str) -> &mut Self {
        self.separator = sep.to_owned();
        self
    }
}

impl Node for List {
    fn string(&self) -> StringUnit {
        let strings: Vec<StringUnit> = self.items.iter().map(|i| i.0.string()).collect();
        let list: Vec<&str> = strings.iter().map(|s| s.0.as_str()).collect();
        StringUnit(Rc::new(list.join(&self.separator)))
    }
    fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    fn id(&self) -> &Id {
        &self.id
    }
}