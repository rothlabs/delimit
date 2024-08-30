use super::*;

#[derive(Default, Hash, Serialize, Deserialize, Debug)]
pub struct Attribute {
    html_attribute: u8,
    name: Apex,
    content: Apex,
}

impl Attribute {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, name: impl Into<Apex>) -> Self {
        self.name = name.into();
        self
    }
    pub fn content(mut self, content: impl Into<Apex>) -> Self {
        self.content = content.into();
        self
    }
    fn trade(&mut self, trade: &dyn Trade) -> Result<Memo> {
        self.name = self.name.deal(trade);
        self.content = self.content.deal(trade);
        adapt_ok()
    }
    fn main(&self) -> Result<Gain> {
        List::new()
            .push(self.name.at(PLAIN)?)
            .push(r#"=""#)
            .push(self.content.at(PLAIN)?)
            .push(r#"""#)
            .apex()
            .gain()
    }
    fn all(&self) -> Result<Gain> {
        Ok(Gain::Apexes(vec![self.name.clone(), self.content.clone()]))
    }
}

impl Adapt for Attribute {
    fn adapt(&mut self, post: Post) -> Result<Memo> {
        match post {
            Post::Trade(deal) => self.trade(deal),
            _ => post.no_handler(self),
        }
    }
}

impl Solve for Attribute {
    fn solve(&self, task: Task) -> Result<Gain> {
        match task {
            Task::Main => self.main(),
            Task::All => self.all(),
            Task::Serial => self.serial(),
            Task::Digest(state) => self.digest(state),
            _ => task.no_handler(self),
        }
    }
}

pub const ID: &str = "id";
pub const LANG: &str = "lang";
pub const CHARSET: &str = "charset";
pub const NAME: &str = "name";
pub const CONTENT: &str = "content";
pub const TYPE: &str = "type";
pub const SRC: &str = "src";

pub const ATTRIBUTES: [&str; 7] = [ID, LANG, CHARSET, NAME, CONTENT, TYPE, SRC];
