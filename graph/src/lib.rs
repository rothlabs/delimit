use std::collections::HashMap;

use serde::{Deserialize, Serialize};

type Id = String;
type Terms = HashMap<String, Vec<Id>>;

#[derive(Default, Serialize, Deserialize)]
struct Node {
    id: Id,
    pack: Id,
    root: Id,
    body: Body,
}

impl Node {
    pub fn id(&mut self, id: &str) -> &mut Self {
        self.id = id.to_string();
        self 
    }
    pub fn pack(&mut self, id: &str) -> &mut Self {
        self.pack = id.to_string();
        self 
    }
    pub fn root(&mut self, id: &str) -> &mut Self {
        self.root = id.to_string();
        self 
    }
    pub fn stem(&mut self, term: &str, id: &Id) -> &mut Self {
        if let Body::Terms(terms) = &mut self.body {
            if terms.contains_key(term) {
                let terms = terms.get_mut(term).unwrap(); 
                terms.push(id.to_string());
            } else {
                terms.insert(term.to_string(), vec![id.to_string()]);
            }
        } else {
            let mut terms = Terms::new();
            terms.insert(term.to_string(), vec![id.to_string()]);
            self.body = Body::Terms(terms);
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


// #[derive(Default, Serialize, Deserialize)]
// struct Id {
//     node: String,
//     pack: String, // [u8; 32]
// }