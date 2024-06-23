use serde::Serialize;

use graph::{
    link::{CloneUnit, Leaf},
    FromUnit,
};

use super::{text, Stem, Text, Unit};

pub fn list() -> List {
    List {
        items: vec![],
        separator: "".into(),
    }
}

#[derive(Clone, Serialize)]
pub struct List {
    pub items: Vec<Stem>,
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
    // pub fn add_text(&mut self, text: &Text) -> &mut Self {
    //     self.items.push(Stem::Text(text.clone()));
    //     self
    // }
    // pub fn add_str(&mut self, unit: &str) -> &mut Self {
    //     self.items.push(Stem::String(unit.to_owned()));
    //     self
    // }
    // pub fn add_list(&mut self, list: List) -> &mut Self {
    //     self.items.push(Stem::Text(text(Box::new(list))));
    //     self
    // }
    // pub fn add_leaf(&mut self, leaf: &Leaf<String>) -> &mut Self {
    //     self.items.push(Stem::Leaf(leaf.clone()));
    //     self
    // }
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
    fn add_item(&mut self, item: Stem) {
        self.items.push(item);
        // match link {
        //     //Stem::String(s) => f(s),
        //     Stem::Leaf(l) => {
        //         self.items.push(Stem::Leaf(leaf.clone()));
        //     },
        //     //Stem::Text(t) => t.leaf().read(f),
        //     _ => (),
        // };
    }
}
