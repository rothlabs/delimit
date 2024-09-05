use super::*;

#[derive(Default, Hash, Debug, Serialize, Deserialize)]
pub struct List {
    plain_list: u8,
    items: Vec<Hub>,
    separator: Hub,
}

impl List {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_separator(&mut self, separator: impl Into<Hub>) -> &mut Self {
        self.separator = separator.into();
        self
    }
    pub fn separator(mut self, separator: impl Into<Hub>) -> Self {
        self.separator = separator.into();
        self
    }
    pub fn extend(mut self, items: Vec<impl Into<Hub>>) -> Self {
        self.items.extend(items.into_iter().map(|item| item.into()));
        self
    }
    pub fn push(mut self, item: impl Into<Hub>) -> Self {
        self.items.push(item.into());
        self
    }
    pub fn remove(&mut self, index: usize) -> &mut Self {
        self.items.remove(index);
        self
    }
    fn main(&self) -> Result<Gain> {
        if self.items.is_empty() {
            return solve_ok();
        }
        let last = self.items.len() - 1;
        let mut base = String::new();
        let separator = self.separator.string().unwrap_or_default();
        for i in 0..last {
            self.items[i].view().string(|x| base += x)?;
            base += &separator;
        }
        self.items[last].view().string(|x| base += x)?;
        base.leaf().hub().gain()
    }
}

impl Adapt for List {
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
        self.items.deal("items", deal)?;
        self.separator.deal("separator", deal)?;
        Ok(())
    }
}

impl Solve for List {
    fn solve(&self, task: Task) -> Result<Gain> {
        match task {
            Task::Rank => 1.gain(),
            Task::Main => self.main(),
            Task::Serial => self.serial(),
            Task::Digest(state) => self.digest(state),
            Task::React => solve_ok(),
            _ => task.no_handler(self),
        }
    }
}

// fn set_at(&mut self, index: usize, hub: Hub) -> Result<Memo> {
//     self.items[index] = hub;
//     adapt_ok()
// }

// fn all(&self) -> Result<Gain> {
//     let mut hubes = vec![self.separator.clone()];
//     hubes.extend(self.items.clone());
//     hubes.gain()
// }
// fn map(&self) -> Result<Gain> {
//     let mut map = Map::new();
//     map.insert("items", &self.items);
//     map.insert("separator", &self.separator);
//     map.gain()
// }

// impl Adapt for List {
//     fn adapt(&mut self, post: Post) -> Result<Memo> {
//         match post {
//             Post::Trade(deal) => self.trade(deal),
//             Post::SetAt(index, hub) => self.set_at(index, hub),
//             _ => post.no_handler(self),
//         }
//     }
// }

// fn trade(&mut self, deal: &dyn Trade) -> Result<Memo> {
//     self.items = self.items.deal(deal);
//     self.separator = self.separator.deal(deal);
//     adapt_ok()
// }
