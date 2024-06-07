use std::{collections::HashMap, hash::Hash};

use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

pub trait ToGraph {
    fn graph(&self, graph: &mut Graph);
}

#[derive(Default, Serialize, Deserialize)]
pub struct Graph {
    nodes: HashMap<Id, Node>,
}

impl Graph {
    pub fn node(&mut self, id: &Option<Id>) -> &mut Node {
        if let Some(id) = id {
            if let Some(node) = self.nodes.get_mut(id) {
                return node;
            }
            panic!("there should be a node at the given id")
        } 
        let id = Id::new();
        self.nodes.insert(id.clone(), Node::default());
        self.nodes.get_mut(&id).unwrap()
    }
}

type Cast = Vec<String>;

#[derive(Default, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Id {
    node: String,
    snap: String,
}

impl Eq for Id {}

impl Id {
    fn new() -> Self {
        Id {
            node: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
            snap: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Node {
    cast: Cast,
    body: Body,
    root: Option<Id>,
}

impl Node {
    pub fn root(&mut self, id: &Id) -> &mut Self {
        self.root = Some(id.clone());
        self
    }
    pub fn stem(&mut self, term: &str, id: &Id) -> &mut Self {
        if let Body::Terms(terms) = &mut self.body {
            terms.push(term, id);
        } else {
            self.body = Body::Terms(Terms::make(term, id));
        }
        self
    }
    pub fn string(&mut self, s: &str) -> &mut Self {
        self.body = Body::Leaf(Leaf::String(s.to_owned()));
        self
    }
}

type Terms = HashMap<String, Vec<Id>>;

trait NodeTerms {
    fn make(term: &str, id: &Id) -> Terms;
    fn push(&mut self, term: &str, id: &Id);
}

impl NodeTerms for Terms {
    fn make(term: &str, id: &Id) -> Terms {
        let mut terms = Terms::new();
        terms.insert(term.to_string(), vec![id.clone()]);
        terms
    }
    fn push(&mut self, term: &str, id: &Id) {
        if self.contains_key(term) {
            let terms = self.get_mut(term).unwrap();
            terms.push(id.clone());
        } else {
            self.insert(term.to_string(), vec![id.clone()]);
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Body {
    Leaf(Leaf),
    Terms(Terms),
}

impl Default for Body {
    fn default() -> Self {
        Body::Leaf(Leaf::default())
    }
}

#[derive(Serialize, Deserialize)]
enum Leaf {
    Bool(bool),
    I32(i32),
    F64(f64),
    String(String),
}

impl Default for Leaf {
    fn default() -> Self {
        Leaf::Bool(false)
    }
}


// pub fn node(id: &Option<Id>) -> Node {
//     if let Some(id) = id {
//         Node {
//             id: id.clone(),
//             ..Default::default()
//         }
//     } else {
//         Node {
//             id: Id::new(),
//             ..Default::default()
//         }
//     }
// }

// pub fn id(&mut self, id: &str) -> &mut Self {
//     self.id.node = id.to_string();
//     self
// }
// pub fn snap(&mut self, id: &str) -> &mut Self {
//     self.id.snap = id.to_string();
//     self
// }
