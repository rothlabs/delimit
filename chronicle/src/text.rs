use std::rc::Rc;

use graph::*;

pub fn list() -> List {
    List::default()
}

pub trait Text {
    fn string(&self) -> Rc<String>;
}

pub struct Leaf {
    string: Rc<String>,
    id: Option<Id>,
}

impl Text for Leaf {
    fn string(&self) -> Rc<String> {
        Rc::clone(&self.string)
    }
}

pub fn leaf(string: &str) -> Rc<dyn Text> {
    Rc::new(Leaf{string: Rc::new(string.to_owned()), id:None})
}

#[derive(Default)]
pub struct List {
    pub stems: Vec<Rc<dyn Text>>,
    pub separator: String, 
    pub id: Option<Id>,
}

impl List {
    pub fn text(self) -> Rc<dyn Text> {
        Rc::new(self)
    }
    pub fn node(&mut self, node: &Rc<dyn Text>) -> &mut Self {
        self.stems.push(Rc::clone(node));
        self
    }
    pub fn leaf(&mut self, string: &str) -> &mut Self {
        self.stems.push(leaf(string));
        self
    }
    pub fn list(&mut self, list: List) -> &mut Self {
        self.stems.push(Rc::new(list));
        self
    }
    pub fn separator(&mut self, sep: &str) -> &mut Self {
        self.separator = sep.to_owned();
        self
    }
}

impl Text for List {
    fn string(&self) -> Rc<String> {
        let strings: Vec<Rc<String>> = self.stems.iter().map(|n| n.string()).collect();
        let list: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
        Rc::new(list.join(&self.separator))
    }
}

impl ToGraph for List {
    fn graph(&self) -> Graph {
        let mut graph = Graph::default();
        
        graph
    }
}

// if let Some(sep) = &self.separator {
//     list.join(&sep.string())
// } else {
//     list.join("")
// }
