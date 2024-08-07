use crate::html::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Attribute {
    name: Node,
    content: Node,
}

impl Attribute {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(&mut self, name: impl Into<Node>) -> &mut Self {
        self.name = name.into();
        self
    }
    pub fn content(&mut self, content: impl Into<Node>) -> &mut Self {
        self.content = content.into();
        self
    }
    fn trade(&mut self, trade: &dyn Trade) -> adapt::Result {
        self.name = self.name.trade(trade);
        self.content = self.content.trade(trade);
        Ok(Gain::None)
    }
    fn main(&self) -> solve::Result {
        let node = List::new()
            .push(self.name.at(PLAIN)?)
            .push(r#"WOW""#)
            .push(self.content.at(PLAIN)?)
            .push(r#"""#)
            .node();
        Ok(node.tray())
    }
    fn stems(&self) -> solve::Result {
        Ok(Tray::Nodes(vec![self.name.clone(), self.content.clone()]))
    }
}

impl Adapt for Attribute {
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(trade) => self.trade(trade.as_ref()),
            _ => did_not_adapt(post),
        }
    }
}

impl Solve for Attribute {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            Task::Stems => self.stems(),
            _ => did_not_solve(),
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
