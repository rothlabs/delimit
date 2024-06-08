use std::{cell::RefCell, rc::Rc};

use serde::Serialize;

use graph::{leaf::{leaf_str, Leaf}, Id};
use super::Text;

pub trait Node {
    fn string(&self) -> Leaf<String>;
    fn serialize(&self) -> String;
    fn id(&self) -> Id;
}

// #[derive(Serialize)]
// pub struct Leaf {
//     pub string: StringCell<String>,
//     pub id: Id,
// }

impl Node for Leaf<String> {
    fn string(&self) -> Leaf<String> {
        self.clone()
    }
    fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    fn id(&self) -> Id {
        self.0.as_ref().borrow().id.clone()
    }
}

// pub fn leaf(value: &str) -> Rc<Leaf> {
//     Rc::new(Leaf {
//         string: leaf_str(value),
//         id: Id::new("text/leaf"),
//     })
// }

// pub fn leaf_node(value: &str) -> Text {
//     Text(leaf(value))
// }

#[derive(Serialize)]
pub struct List {
    pub items: Vec<Text>,
    pub separator: String,
    pub id: Id,
}

impl List {
    pub fn text(self) -> Text {
        Text(Rc::new(RefCell::new(self)))
    }
    pub fn add_text(&mut self, text: &Text) -> &mut Self {
        self.items.push(text.clone());
        self
    }
    pub fn add_leaf(&mut self, leaf: Leaf<String>) -> &mut Self {
        //self.items.push(Text(leaf.clone()));
        self
    }
    pub fn add_string(&mut self, string: &str) -> &mut Self {
        //self.items.push(leaf_node(string));
        self
    }
    pub fn add_list(&mut self, list: List) -> &mut Self {
        self.items.push(Text(Rc::new(RefCell::new(list))));
        self
    }
    pub fn separator(&mut self, sep: &str) -> &mut Self {
        self.separator = sep.to_owned();
        self
    }
}

impl Node for List {
    fn string(&self) -> Leaf<String> {
        let cells: Vec<Leaf<String>> = self.items.iter().map(|i| i.0.borrow().string()).collect();
        let mut string = String::new();
        for i in 0..cells.len()-1 {
            string += &cells[i].0.borrow().at;
            string += &self.separator;
        }
        if let Some(cell) = cells.last() {
            string += &cell.0.borrow().at;
        }
        leaf_str(&string) 
    }
    fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    fn id(&self) -> Id {
        self.id.clone()
    }
}

pub fn list() -> List {
    List {
        items: vec![],
        separator: "".into(),
        id: Id::new("text/list"),
    }
}



// .trim_end_matches(&self.separator)
        //let list: Vec<&str> = strings.iter().map(|s| s.0.as_ref().borrow()).collect();
        // let mut list = vec![];
        // for cell in cells {
        //     let s = cell.0.as_ref().borrow();
        //     list.push(s.as_str());
        // };
        //StringCell( Rc::new(list.join(&self.separator)))
        //string_unit(&list.join(&self.separator))