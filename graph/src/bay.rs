use super::*;
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Bay {
    bay: u8,
    apexes: HashMap<Key, Apex>,
}

impl Hash for Bay {
    fn hash<H: Hasher>(&self, h: &mut H) {
        let mut pairs: Vec<_> = self.apexes.iter().collect();
        pairs.sort_by_key(|i| i.0);
        Hash::hash(&pairs, h);
    }
}

impl Bay {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, key: impl Into<Key>, apex: impl Into<Apex>) -> adapt::Result {
        self.apexes.insert(key.into(), apex.into());
        adapt_ok()
    }
    fn extend(&mut self, apexes: HashMap<Key, Apex>) -> adapt::Result {
        self.apexes.extend(apexes);
        adapt_ok()
    }
    fn stems(&self) -> solve::Result {
        let stems: Vec<Apex> = self.apexes.values().cloned().collect();
        stems.gain()
    }
    pub fn get(&self, key: &Key) -> solve::Result {
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
            Post::Insert(key, apex) => self.insert(key, apex),
            Post::Extend(apexes) => self.extend(apexes),
            _ => no_adapter(post),
        }
    }
}

impl Solve for Bay {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Stems => self.stems(),
            Task::Digest => self.digest(),
            Task::Serial => self.serial(),
            Task::Get(key) => self.get(key),
            _ => no_solver(self, task),
        }
    }
}
