use serde::Serialize;

use super::*;

#[derive(Clone, Default, Serialize)]
pub struct Tag {
    pub name: Node,
    pub attributes: Vec<Node>,
    pub repo: Node,
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
    pub fn repo(&mut self, repo: impl Into<Node>) -> &mut Self {
        self.repo = repo.into();
        self
    }
    fn stems(&self) -> solve::Result {
        let mut stems = self.attributes.clone();
        stems.push(self.name.clone());
        Ok(Tray::Nodes(stems))
    }
    fn main(&self) -> solve::Result {
        let items = List::new()
            .separator(" ")
            .push(self.name.at(PLAIN)?)
            .extend(self.attributes.at(PLAIN)?)
            .node();
        let tag = List::new().push("<").push(&items).push(">").node();
        self.repo.edit().insert(items).insert(&tag).run()?;
        Ok(tag.tray())
    }
}

impl Make for Tag {
    fn make(&self, back: &Back) -> Self {
        Self {
            name: self.name.backed(back),
            attributes: self.attributes.backed(back),
            repo: self.repo.clone(),
        }
    }
}

impl Solve for Tag {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            Task::Stems => self.stems(),
            _ => Ok(Tray::None)
        }
    }
}

impl Alter for Tag {
    fn alter(&mut self, _: Post) -> alter::Result {
        Ok(Report::None)
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
