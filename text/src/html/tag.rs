use super::*;

#[derive(Default, Hash, Serialize, Deserialize, Debug)]
pub struct Tag {
    html_tag: u8,
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
    fn trade(&mut self, trade: &dyn Trade) -> Result<Memo> {
        self.name = self.name.deal(trade);
        self.attributes = self.attributes.deal(trade);
        adapt_ok()
    }
    fn main(&self) -> Result<Gain> {
        let items = List::new()
            .separator(" ")
            .push(self.name.at(PLAIN)?)
            .extend(self.attributes.at(PLAIN)?)
            .apex();
        List::new().push("<").push(&items).push(">").apex().gain()
    }
    // fn all(&self) -> Result<Gain> {
    //     let mut apexes = vec![self.name.clone()];
    //     apexes.extend(self.attributes.clone());
    //     apexes.gain()
    // }
    fn map(&self) -> Result<Gain> {
        let mut map = Map::new();
        map.insert("name", &self.name)?;
        map.insert("attributes", &self.attributes)?;
        map.gain()
    }
}

impl Adapt for Tag {
    fn adapt(&mut self, post: Post) -> Result<Memo> {
        match post {
            Post::Trade(deal) => self.trade(deal),
            _ => post.no_handler(self),
        }
    }
}

impl Solve for Tag {
    fn solve(&self, task: Task) -> Result<Gain> {
        match task {
            Task::Main => self.main(),
            // Task::All => self.all(),
            Task::Serial => self.serial(),
            Task::Digest(state) => self.digest(state),
            Task::Map => self.map(),
            // Task::Get(_) => self.name.clone().gain(),
            _ => task.no_handler(self),
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
