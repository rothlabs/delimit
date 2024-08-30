use super::*;

#[derive(Default, Hash, Debug, Serialize, Deserialize)]
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
    fn set_at(&mut self, index: usize, apex: Apex) -> adapt::Result {
        self.items[index] = apex;
        adapt_ok()
    }
    fn main(&self) -> solve::Result {
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
        base.leaf().apex().gain()
    }
    fn all(&self) -> solve::Result {
        let mut apexes = vec![self.separator.clone()];
        apexes.extend(self.items.clone());
        apexes.gain()
    }
}

impl Adapt for List {
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(deal) => self.trade(deal),
            Post::SetAt(index, apex) => self.set_at(index, apex),
            _ => post.no_handler(self),
        }
    }
}

impl Solve for List {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            Task::All => self.all(),
            Task::Serial => self.serial(),
            Task::Digest(state) => self.digest(state),
            Task::React => solve_ok(),
            _ => task.no_handler(self),
        }
    }
}