use serde::Serialize;

use graph::{base::Dum, Edge, Base};
use super::{Unit, Text, text};

impl Unit for Edge<String> {
    fn leaf(&self) -> Edge<String> {
        self.clone()
    }
    fn json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub fn list() -> List {
    List {
        items: vec![],
        separator: "".into(),
    }
}

#[derive(Clone, Serialize)]
pub struct List {
    pub items: Vec<Text>,
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
        self.items.push(text.clone());
        self
    }
    pub fn add_leaf(&mut self, leaf: &Edge<String>) -> &mut Self {
        self.items.push(text(leaf.clone()));
        self
    }
    pub fn add_string(&mut self, unit: &str) -> &mut Self {
        self.items.push(text(Edge::str(unit))); 
        self
    }
    pub fn add_list(&mut self, list: List) -> &mut Self {
        self.items.push(text(list));
        self
    }
}

impl Unit for List {
    fn leaf(&self) -> Edge<String> {
        let leafs: Vec<Edge<String>> = self.items.iter().map(|i| i.leaf()).collect();
        let mut string = String::new();
        for i in 0..leafs.len()-1 {
            string += &leafs[i].read().read();
            string += &self.separator;
        }
        if let Some(leaf) = leafs.last() {
            string += &leaf.read().read();
        }
        Edge::new(string)
    }
    fn json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl Base<Edge<String>> for List {
    fn compute(&self) -> Edge<String> {
        self.leaf()
    }   
}

// impl Base for List {
//     fn edges(&self) -> Vec<Box<dyn Base>> {
//         let mut edges = vec![];
//         for item in self.items.iter() {
//             let edge = item.0.clone(); // as Edge<Box<dyn Base>>;
//             let wow = edges.push(edge);
//         }
//         edges
//     }
// }





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