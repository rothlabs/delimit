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
    fn main(&self) -> Result<Gain> {
        let items = List::new()
            .separator(" ")
            .push(self.name.down(PLAIN)?)
            .extend(self.attributes.down(PLAIN)?)
            .apex();
        List::new().push("<").push(&items).push(">").apex().gain()
    }
}

impl Adapt for Tag {
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<Memo> {
        self.name.deal("name", deal)?;
        self.attributes.deal("attributes", deal)?;
        adapt_ok()
    }
}

impl Solve for Tag {
    fn solve(&self, task: Task) -> Result<Gain> {
        match task {
            Task::Main => self.main(),
            Task::Serial => self.serial(),
            Task::Digest(state) => self.digest(state),
            Task::React => solve_ok(),
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
