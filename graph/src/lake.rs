use regex::Regex;

use super::*;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Lake {
    /// The nodes can be another lake
    nodes: HashMap<Path, Node>,
    path: Node,
    dump: String,
    #[serde(skip)]
    deserializer: Option<Box<dyn DeserializeNode>>,
}

impl Lake {
    pub fn new() -> Self {
        Self::default()
    }

}

impl Adapt for Lake {
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            // Post::Trade(_) => Ok(Gain::None),
            // Post::Extend(nodes) => self.extend(nodes),
            // Post::Import => self.import(),
            _ => did_not_adapt(post),
        }
    }
}

impl Solve for Lake {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            // Task::Stems => self.stems(),
            // Task::Export => self.export(),
            // Task::Find(regex) => self.find(&regex),
            _ => did_not_solve(),
        }
    }
}
