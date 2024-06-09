use std::{borrow::Cow, hash::Hash};

use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

pub mod leaf;

pub const LEAF: &str = "leaf";

type Cast = Cow<'static, str>;

#[derive(Default, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Id {
    cast: Cast,
    node: String,
    snap: String,
}

impl Eq for Id {}

impl Id {
    pub fn new(cast: &'static str) -> Self {
        Id {
            cast: Cow::Borrowed(cast),
            node: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
            snap: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
        }
    }
}

// pub trait Node {
//     fn save(&mut self, graph: &mut Graph);
// }

// #[derive(Serialize, Deserialize)]
// pub struct Graph {
//     units: HashMap<Id, Unit>,
//     stack: Vec<Id>,
// }

// impl Graph {
//     pub fn save(&mut self, id: &Option<Id>, func: &dyn Fn(&mut Unit)) -> Option<Id> {
//         if let Some(id) = id {
//             if let Some(unit) = self.units.get_mut(id) {
//                 unit.clear();
//                 func(unit);
//                 self.stack.push(id.clone());
//                 return Some(id.clone());
//             }
//             panic!("there should be a node at the given id")
//         }
//         self.stack.push(Id::new());
//         let id = self.stack.last().unwrap();
//         self.units.insert(id.clone(), Unit::default());
//         func(self.units.get_mut(&id).unwrap());
//         Some(id.clone())
//     }
// }

// #[derive(Default, Serialize, Deserialize)]
// pub struct Unit {
//     cast: Cast,
//     root: Option<Id>,
//     terms: Terms,
// }

// impl Unit {
//     pub fn cast(&mut self, cast: &'static str) -> &mut Self {
//         self.cast = Cow::Borrowed(cast);
//         self
//     }
//     pub fn root(&mut self, id: &Id) -> &mut Self {
//         self.root = Some(id.clone());
//         self
//     }
//     pub fn clear(&mut self) -> &mut Self {
//         self.terms.clear();
//         self
//     }
//     pub fn id(&mut self, term: &str, id: &Id) -> &mut Self {
//         self.terms.push(term, Stem::Id(id.clone()));
//         self
//     }
//     pub fn nodes(&mut self, graph: &mut Graph, term: &str, node: &mut dyn Node) -> &mut Self {
//         node.save(graph);
//         let id = graph.stack.pop().unwrap();
//         self.terms.push(term, Stem::Id(id));
//         // for node in nodes.iter() {
//         //     self.terms.push(term, Stem::Id(node.graph(graph)));
//         // }
//         self
//     }
//     pub fn string(&mut self, term: &str, stem: &str) -> &mut Self {
//         self.terms.push(term, Stem::String(stem.into()));
//         self
//     }
// }

// type Terms = HashMap<String, Vec<Stem>>;

// trait PushTerm {
//     fn push(&mut self, term: &str, stem: Stem);
// }

// impl PushTerm for Terms {
//     fn push(&mut self, term: &str, stem: Stem) {
//         if let Some(term) = self.get_mut(term) {
//             term.push(stem);
//         } else {
//             self.insert(term.into(), vec![stem]);
//         }
//     }
// }

// #[derive(Serialize, Deserialize)]
// enum Stem {
//     Id(Id),
//     Bool(bool),
//     I32(i32),
//     F64(f64),
//     String(String),
// }

// impl Default for Stem {
//     fn default() -> Self {
//         Stem::Bool(false)
//     }
// }
