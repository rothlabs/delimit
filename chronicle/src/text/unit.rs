use serde::Serialize;

use super::{text, Text, Unit};
use graph::LeafStr;

#[derive(Clone, Serialize)]
pub struct List {
    pub items: Vec<Item>,
    pub separator: String,
}

impl List {
    pub fn text(self) -> Text {
        //Text(self.snap.edge(Box::new(self)))
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
        LeafStr::new(&string) // snap.edge(string) // 
    }
    fn serial(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    fn string(&self) -> String {
        self.leaf().string()
    }
}

pub fn list() -> List {
    List {
        items: vec![],
        separator: "".into(),
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



// impl Unit for EdgeStr {
//     fn leaf(&self) -> EdgeStr {
//         self.clone()
//         //self.snap.edge(self.unit.clone())
//     }
//     fn serial(&self) -> String {
//         serde_json::to_string(self).unwrap()
//     }
// }



        // //let leafs: Vec<LeafStr> = self.items.iter().map(|i| i.leaf()).collect();
        // let mut string = String::new();
        // for i in 0..self.items.len() - 1 {
        //     match &self.items[i] {
        //         Item::Text(t) => t.leaf().read(|unit| string += unit),
        //         Item::String(s) => string += &s, 
        //     }
        //     // /leafs[i].read(|unit| string += unit); // string += &leafs[i].solve(());
        //     string += &self.separator;
        // }
        // if let Some(item) = self.items.last() {
        //     match item {
        //         Item::Text(t) => t.leaf().read(|unit| string += unit),
        //         Item::String(s) => string += &s, 
        //     }
        //     //leaf.read(|unit| string += unit);
        // }
        // snap.edge(string) // Edge::new(&snap, string)


// impl Unit for LeafStr {
//     fn leaf(&self, _: Snap) -> LeafStr {
//         self.clone()
//     }
//     fn serial(&self) -> String {
//         serde_json::to_string(self).unwrap()
//     }
// }


// impl Base<Edge<String, ()>> for List {
//     fn compute(&self) -> Edge<String, ()> {
//         self.leaf()
//     }
// }

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
