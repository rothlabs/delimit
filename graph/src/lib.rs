use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

pub trait ToGraph {
    fn graph(&self) -> Graph;
}

#[derive(Default, Serialize, Deserialize)]
pub struct Graph {
    nodes: HashMap<Id, Node>,
}

type Cast = Vec<String>;

#[derive(Default, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Id {
    node: String,
    snap: String,
}

impl Eq for Id {}

#[derive(Default, Serialize, Deserialize)]
pub struct Node {
    id: Id,
    root: Id,
    cast: Cast,
    body: Body,
}

impl Node {
    pub fn id(&mut self, id: &str) -> &mut Self {
        self.id.node = id.to_string();
        self
    }
    pub fn snap(&mut self, id: &str) -> &mut Self {
        self.id.snap = id.to_string();
        self
    }
    pub fn root(&mut self, id: &Id) -> &mut Self {
        self.root = id.clone();
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

pub fn node() -> Node {
    Node::default()
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
