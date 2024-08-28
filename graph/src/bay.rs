use super::*;

#[derive(Default, Hash, Serialize, Deserialize, Debug)]
pub struct Bay {
    bay: u8,
    map: Map,
}

impl Bay {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, key: impl Into<Key>, apex: impl Into<Apex>) -> adapt::Result {
        self.map.insert(key.into(), apex.into());
        adapt_ok()
    }
    fn extend(&mut self, apexes: Map) -> adapt::Result {
        self.map.extend(apexes);
        adapt_ok()
    }
    fn trade(&mut self, deal: &dyn Trade) -> adapt::Result {
        self.map = self.map.trade(deal);
        adapt_ok()
    }
    pub fn get(&self, key: &Key) -> solve::Result {
        if let Some(apex) = self.map.get(key) {
            apex.pathed(key).gain()
        } else {
            solve_ok()
        }
    }
}

impl Adapt for Bay {
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(deal) => self.trade(deal),
            Post::Insert(key, apex) => self.insert(key, apex),
            Post::Extend(map) => self.extend(map),
            _ => no_adapter(self, post),
        }
    }
}

impl Solve for Bay {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::All => self.map.vec().gain(),
            Task::Digest(state) => self.digest(state),
            Task::Serial => self.serial(),
            Task::Get(key) => self.get(key),
            Task::React => solve_ok(),
            _ => no_solver(self, task),
        }
    }
}
