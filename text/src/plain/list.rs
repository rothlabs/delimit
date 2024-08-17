use super::*;
use std::hash::Hash;

#[derive(Default, Hash, Serialize, Deserialize, Debug)]
pub struct List {
    plain_list: u8,
    items: Vec<Apex>,
    separator: Apex,
}

impl List {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_separator(&mut self, separator: impl Into<Apex>) -> &mut Self {
        self.separator = separator.into();
        self
    }
    pub fn separator(mut self, separator: impl Into<Apex>) -> Self {
        self.separator = separator.into();
        self
    }
    pub fn extend(mut self, items: Vec<impl Into<Apex>>) -> Self {
        self.items.extend(items.into_iter().map(|item| item.into()));
        self
    }
    pub fn push(mut self, item: impl Into<Apex>) -> Self {
        self.items.push(item.into());
        self
    }
    pub fn remove(&mut self, index: usize) -> &mut Self {
        self.items.remove(index);
        self
    }
    fn trade(&mut self, deal: &dyn Trade) -> adapt::Result {
        self.items = self.items.deal(deal);
        self.separator = self.separator.deal(deal);
        adapt_ok()
    }
    fn main(&self) -> solve::Result {
        if self.items.is_empty() {
            return Ok(Gain::None);
        }
        let last = self.items.len() - 1;
        let mut string = String::new();
        self.separator.read_string(|sep| {
            for i in 0..last {
                self.items[i].read_string(|s| string += s);
                string += sep;
            }
        });
        self.items[last].read_string(|s| string += s);
        Ok(string.leaf().apex().gain())
    }
    fn stems(&self) -> solve::Result {
        let mut apexes = self.items.clone();
        apexes.push(self.separator.clone());
        Ok(apexes.gain())
    }
}

impl Adapt for List {
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(deal) => self.trade(deal),
            _ => no_adapter(post),
        }
    }
}

impl Solve for List {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            Task::Stems => self.stems(),
            Task::Serial => self.serial(),
            Task::Digest => self.digest(),
            _ => no_solver(),
        }
    }
}
