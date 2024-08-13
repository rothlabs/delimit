use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash)]
pub struct Tag {
    pub name: Node,
    pub attributes: Vec<Node>,
}

impl Tag {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(&mut self, name: impl Into<Node>) -> &mut Self {
        self.name = name.into();
        self
    }
    pub fn attribute(&mut self, attribute: impl Into<Node>) -> &mut Self {
        self.attributes.push(attribute.into());
        self
    }
    fn trade(&mut self, trade: &dyn Trade) -> adapt::Result {
        self.name = self.name.trade(trade);
        self.attributes = self.attributes.trade(trade);
        adapt_ok()
    }
    fn main(&self) -> solve::Result {
        let items = List::new()
            .separator(" ")
            .push(self.name.at(PLAIN)?)
            .extend(self.attributes.at(PLAIN)?)
            .node();
        let tag = List::new().push("<").push(&items).push(">").node();
        Ok(tag.tray())
    }
    fn stems(&self) -> solve::Result {
        let mut nodes = self.attributes.clone();
        nodes.push(self.name.clone());
        Ok(nodes.tray())
    }
}

impl Adapt for Tag {
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(trade) => self.trade(trade.as_ref()),
            _ => no_adapter(post),
        }
    }
}

impl Solve for Tag {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            Task::Stems => self.stems(),
            Task::Serial => self.serial(),
            Task::Hash => self.digest(),
            _ => no_solver(),
        }
    }
}

pub const DOCTYPE: &str = "!DOCTYPE html";
pub const HTML: &str = "html";
pub const HEAD: &str = "head";
pub const TITLE: &str = "title";
pub const META: &str = "meta";
pub const SCRIPT: &str = "script";
pub const BODY: &str = "body";
pub const DIV: &str = "div";
pub const CANVAS: &str = "canvas";
pub const H1: &str = "h1";

pub const TAGS: [&str; 10] = [
    DOCTYPE, HTML, HEAD, TITLE, META, SCRIPT, BODY, DIV, CANVAS, H1,
];
