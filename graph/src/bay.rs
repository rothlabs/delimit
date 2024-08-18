use super::*;
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Bay {
    apexes: HashMap<Key, Apex>,
}

impl Bay {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, key: impl Into<Key>, apex: impl Into<Apex>) {
        self.apexes.insert(key.into(), apex.into());
    }
    fn extend(&mut self, apexes: HashMap<Key, Apex>) -> adapt::Result {
        self.apexes.extend(apexes);
        adapt_ok()
    }
    fn stems(&self) -> solve::Result {
        let stems: Vec<Apex> = self.apexes.values().cloned().collect();
        stems.gain()
    }
    fn get(&self, key: &Key) -> solve::Result {
        if let Some(apex) = self.apexes.get(key) {
            apex.pathed(key).gain()
        } else {
            no_gain()
        }
    }
}

impl Adapt for Bay {
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(_) => adapt_ok(),
            Post::Extend(apexes) => self.extend(apexes),
            _ => no_adapter(post),
        }
    }
}

impl Solve for Bay {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Stems => self.stems(),
            Task::Get(key) => self.get(key),
            _ => no_solver(),
        }
    }
}
