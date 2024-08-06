use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

impl Make for Tag {
    fn make(&self, back: &Back) -> Self {
        Self {
            name: self.name.backed(back),
            attributes: self.attributes.backed(back),
        }
    }
}

impl Adapt for Tag {
    fn adapt(&mut self, _: Post) -> adapt::Result {
        did_not_adapt()
    }
}

impl Solve for Tag {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            Task::Stems => self.stems(),
            _ => did_not_solve(),
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
