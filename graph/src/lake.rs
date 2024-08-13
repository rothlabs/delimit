use super::*;
use std::{collections::HashMap, result};

pub type Result = result::Result<Lake, Error>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Lake {
    root: String,
    serials: HashMap<Key, String>,
}

impl Lake {
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the root serial
    pub fn root(&mut self, root: String) -> &mut Self {
        self.root = root;
        self
    }
    
}

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
