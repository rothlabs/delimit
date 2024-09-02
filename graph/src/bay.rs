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
    pub fn insert<'a>(&mut self, aim: impl Into<Aim<'a>>, fit: impl Into<Fit>) -> Result<Memo> {
        self.map.insert(aim, fit)?;
        adapt_ok()
    }
    fn extend(&mut self, apexes: Map) -> Result<Memo> {
        self.map.extend(apexes);
        adapt_ok()
    }
    // fn trade(&mut self, deal: &dyn Trade) -> Result<Memo> {
    //     self.map = self.map.trade(deal);
    //     adapt_ok()
    // }
    // pub fn get(&self, key: &Key) -> Result<Gain> {
    //     if let Some(apex) = self.map.get(key) {
    //         apex.pathed(key).gain()
    //     } else {
    //         solve_ok()
    //     }
    // }
}

impl Adapt for Bay {
    fn adapt<D: Trade>(&mut self, deal: &mut D) -> Result<Memo> {
        self.map.deal("map", deal)?;
        adapt_ok()
    }
}

impl Solve for Bay {
    fn solve(&self, task: Task) -> Result<Gain> {
        match task {
            // Task::All => self.map.vec().gain(),
            Task::Map => (&self.map).gain(),
            Task::Digest(state) => self.digest(state),
            Task::Serial => self.serial(),
            // Task::Get(key) => self.get(key),
            Task::React => solve_ok(),
            _ => task.no_handler(self),
        }
    }
}

// impl Adapt for Bay {
//     fn adapt(&mut self, post: Post) -> Result<Memo> {
//         match post {
//             Post::Trade(deal) => self.trade(deal),
//             Post::Insert(aim, apex) => self.map.insert(aim, apex),
//             Post::Extend(map) => self.extend(map),
//             _ => post.no_handler(self),
//         }
//     }
// }
