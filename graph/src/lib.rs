use std::{borrow::{Borrow, Cow}, collections::HashMap, hash::Hash, rc::Rc};

use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

pub const LEAF: &str = "leaf";

pub trait Node {
    fn save(&self, graph: &mut Graph) -> Id;
}

#[derive(Serialize, Deserialize)]
pub struct Graph {
    units: HashMap<Id, Unit>,
}

impl Graph {
    pub fn unit(&mut self, id: &Option<Id>) -> (&mut Unit, Id) {
        if let Some(id) = id {
            if let Some(unit) = self.units.get_mut(id) {
                unit.clear();
                return (unit, id.clone());
            }
            panic!("there should be a node at the given id")
        } 
        let id = Id::new();
        self.units.insert(id.clone(), Unit::default());
        (self.units.get_mut(&id).unwrap(), id)
    }
}

#[derive(Clone, Hash, PartialEq, Serialize, Deserialize)]
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

type Cast = Cow<'static, str>;

#[derive(Default, Serialize, Deserialize)]
pub struct Unit {
    cast: Cast,
    root: Option<Id>,
    terms: Terms,
}

impl Unit {
    pub fn cast(&mut self, cast: &'static str) -> &mut Self {
        self.cast = Cow::Borrowed(cast);
        self
    }
    pub fn root(&mut self, id: &Id) -> &mut Self {
        self.root = Some(id.clone());
        self
    }
    pub fn clear(&mut self) -> &mut Self {
        self.terms.clear();
        self
    }
    pub fn id(&mut self, term: &str, id: &Id) -> &mut Self {
        self.terms.push(term, Stem::Id(id.clone()));
        self
    }
    pub fn nodes(&mut self, graph: &mut Graph, term: &str, node: &dyn Node) -> &mut Self {
        self.terms.push(term, Stem::Id(node.save(graph)));
        // for node in nodes.iter() {
        //     self.terms.push(term, Stem::Id(node.graph(graph)));
        // } 
        self
    }
    pub fn string(&mut self, term: &str, stem: &str) -> &mut Self {
        self.terms.push(term, Stem::String(stem.into()));
        self
    }
}

type Terms = HashMap<String, Vec<Stem>>;

trait PushTerm {
    fn push(&mut self, term: &str, stem: Stem);
}

impl PushTerm for Terms {
    fn push(&mut self, term: &str, stem: Stem) {
        if let Some(term) = self.get_mut(term) {
            term.push(stem);
        } else {
            self.insert(term.into(), vec![stem]);
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Stem {
    Id(Id),
    Bool(bool),
    I32(i32),
    F64(f64),
    String(String),
}

impl Default for Stem {
    fn default() -> Self {
        Stem::Bool(false)
    }
}