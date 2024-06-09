use serde::Serialize;

use graph::{node::Id, leaf::{Leaf, leaf_str}};
use super::{Node, Text, text};

impl Node for Leaf<String> {
    fn leaf(&self) -> Leaf<String> {
        self.clone()
    }
    fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    fn id(&self) -> Id {
        self.get().id.clone() 
    }
}

#[derive(Serialize)]
pub struct List {
    pub items: Vec<Text>,
    pub separator: String,
    pub id: Id, // TODO: Vec<Rc<graph::node::Id>>
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
        self.items.push(text.clone());
        self
    }
    pub fn add_leaf(&mut self, leaf: &Leaf<String>) -> &mut Self {
        self.items.push(text(leaf.clone()));
        self
    }
    pub fn add_string(&mut self, string: &str) -> &mut Self {
        self.items.push(text(leaf_str(string)));
        self
    }
    pub fn add_list(&mut self, list: List) -> &mut Self {
        self.items.push(text(list));
        self
    }
}

impl Node for List {
    fn leaf(&self) -> Leaf<String> {
        let cells: Vec<Leaf<String>> = self.items.iter().map(|i| i.get().leaf()).collect();
        let mut string = String::new();
        for i in 0..cells.len()-1 {
            string += &cells[i].get().value;
            string += &self.separator;
        }
        if let Some(cell) = cells.last() {
            string += &cell.get().value;
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





// pub fn leaf(value: &str) -> Rc<Leaf> {
//     Rc::new(Leaf {
//         string: leaf_str(value),
//         id: Id::new("text/leaf"),
//     })
// }

// pub fn leaf_node(value: &str) -> Text {
//     Text(leaf(value))
// }


// .trim_end_matches(&self.separator)
        //let list: Vec<&str> = strings.iter().map(|s| s.0.as_ref().borrow()).collect();
        // let mut list = vec![];
        // for cell in cells {
        //     let s = cell.0.as_ref().borrow();
        //     list.push(s.as_str());
        // };
        //StringCell( Rc::new(list.join(&self.separator)))
        //string_unit(&list.join(&self.separator))