use super::*;

#[derive(Default, Hash, Serialize, Deserialize, Debug)]
pub struct Tag {
    html_tag: u8,
    // imports: Vec<Import>,
    name: Apex,
    attributes: Vec<Apex>,
}

impl Tag {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, name: impl Into<Apex>) -> Self {
        self.name = name.into();
        self
    }
    pub fn attribute(mut self, attribute: impl Into<Apex>) -> Self {
        self.attributes.push(attribute.into());
        self
    }
    fn trade(&mut self, trade: &dyn Trade) -> adapt::Result {
        self.name = self.name.deal(trade);
        self.attributes = self.attributes.deal(trade);
        adapt_ok()
    }
    fn main(&self) -> solve::Result {
        let items = List::new()
            .separator(" ")
            .push(self.name.at(PLAIN)?)
            .extend(self.attributes.at(PLAIN)?)
            .apex();
        List::new().push("<").push(&items).push(">").apex().gain()
    }
    fn stems(&self) -> solve::Result {
        let mut apexes = vec![self.name.clone()];
        apexes.extend(self.attributes.clone());
        apexes.gain()
    }
}

impl Adapt for Tag {
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(deal) => self.trade(deal),
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
            Task::Digest => self.digest(),
            // Task::Imports => self.imports.gain(),
            Task::Get(_) => self.name.clone().gain(),
            _ => no_gain(), // no_solver(self, task),
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
