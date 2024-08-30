use super::*;

#[derive(Default, Hash, Serialize, Deserialize, Debug)]
pub struct Element {
    html_element: u8,
    open: Apex,
    items: Vec<Apex>,
    close: Option<Apex>,
}

impl Element {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn open(mut self, open: impl Into<Apex>) -> Self {
        self.open = open.into();
        self
    }
    pub fn item(mut self, item: impl Into<Apex>) -> Self {
        self.items.push(item.into());
        self
    }
    pub fn close(mut self) -> Self {
        let name = self.open.get("name").expect("No opening tag name.");
        self.close = Some(name);
        self
    }
    fn trade(&mut self, deal: &dyn Trade) -> adapt::Result {
        self.open = self.open.deal(deal);
        self.items = self.items.deal(deal);
        if let Some(close) = &self.close {
            self.close = Some(close.deal(deal));
        }
        adapt_ok()
    }
    fn set_at(&mut self, index: usize, apex: Apex) -> adapt::Result {
        self.items[index] = apex;
        adapt_ok()
    }
    fn main(&self) -> solve::Result {
        let mut element = List::new()
            .separator("\n")
            .push(self.open.at(PLAIN)?)
            .extend(self.items.at(PLAIN)?);
        if let Some(close) = &self.close {
            let close = List::new()
                .push("</")
                .push(close.at(PLAIN)?)
                .push(">")
                .apex();
            element = element.push(close);
        }
        element.apex().gain()
    }
    fn all(&self) -> solve::Result {
        let mut apexes = vec![self.open.clone()];
        apexes.extend(self.items.clone());
        if let Some(apex) = &self.close {
            apexes.push(apex.clone());
        }
        apexes.gain()
    }
}

impl Adapt for Element {
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(deal) => self.trade(deal),
            Post::SetAt(index, apex) => self.set_at(index, apex),
            _ => self.no_adapter(post),
        }
    }
}

impl Solve for Element {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            Task::All => self.all(),
            Task::Serial => self.serial(),
            Task::Digest(state) => self.digest(state),
            _ => self.no_solver(task),
        }
    }
}
