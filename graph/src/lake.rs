use super::*;
use std::{collections::HashMap, result};

pub type Result = result::Result<Lake, Error>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Lake {
    root: String,
    serials: HashMap<u64, String>,
}

impl Lake {
    pub fn new() -> Self {
        Self::default()
    }
    /// Serialize the given node as the root of the lake. 
    pub fn root(&mut self, node: &Node) -> adapt::Result {
        self.root = node.serial()?;
        adapt_ok()
    }
    /// Insert a node into the lake as hash-serial pair. 
    pub fn insert(&mut self, node: &Node) -> adapt::Result {
        self.serials.insert(node.digest()?, node.serial()?);
        adapt_ok()
    }
}


// /// Set the root serial. 
// pub fn root(&mut self, root: String) -> &mut Self {
//     self.root = root;
//     self
// }

// /// Insert hash-serial pair 
// pub fn insert(&mut self, hash: u64, serial: String) -> &mut Self {
//     self.serials.insert(hash, serial);
//     self
// }

// pub fn new(ploy: impl Into<Node>) -> Result {
//     let mut serials = HashMap::new();

//     Ok(Self {
//         root: "".into(),
//         serials,
//     })
// }

// impl Adapt for Lake {
//     fn adapt(&mut self, post: Post) -> adapt::Result {
//         match post {
//             // Post::Trade(_) => Ok(Gain::None),
//             // Post::Extend(nodes) => self.extend(nodes),
//             // Post::Import => self.import(),
//             _ => did_not_adapt(post),
//         }
//     }
// }

// impl Solve for Lake {
//     fn solve(&self, task: Task) -> solve::Result {
//         match task {
//             // Task::Stems => self.stems(),
//             // Task::Export => self.export(),
//             // Task::Find(regex) => self.find(&regex),
//             _ => did_not_solve(),
//         }
//     }
// }
