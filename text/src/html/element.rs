use super::*;

#[derive(Default, Hash, Serialize, Deserialize, Debug)]
pub struct Element {
    html_element: u8,
    tag: Apex,
    pub items: Vec<Apex>,
    close: Option<Apex>,
    story: Apex,
}

impl Element {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn tag(mut self, tag: impl Into<Apex>) -> Self {
        self.tag = tag.into();
        self
    }
    pub fn item(mut self, item: impl Into<Apex>) -> Self {
        self.items.push(item.into());
        self
    }
    pub fn close(mut self, close: impl Into<Apex>) -> Self {
        self.close = Some(close.into());
        self
    }
    pub fn story(mut self, story: impl Into<Apex>) -> Self {
        self.story = story.into();
        self
    }
    fn trade(&mut self, deal: &dyn Trade) -> adapt::Result {
        self.tag = self.tag.deal(deal);
        self.items = self.items.deal(deal);
        if let Some(close) = &self.close {
            self.close = Some(close.deal(deal));
        }
        adapt_ok()
    }
    fn main(&self) -> solve::Result {
        let mut element = List::new()
            .separator("\n")
            .push(self.tag.at(PLAIN)?)
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
    fn stems(&self) -> solve::Result {
        let mut apexes = self.items.clone();
        apexes.push(self.tag.clone());
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
            _ => no_adapter(post),
        }
    }
}

impl Solve for Element {
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
