use crate::html::*;

#[derive(Default)]
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
}

impl Make for Attribute {
    fn make(&self, back: &Back) -> Self {
        Self {
            name: self.name.backed(back),
            content: self.content.backed(back),
        }
    }
}

impl Solve for Attribute {
    fn solve(&self, _: Task) -> solve::Result {
        let node = List::new()
            .push(self.name.at(PLAIN)?)
            .push(r#"=""#)
            .push(self.content.at(PLAIN)?)
            .push(r#"""#)
            .node();
        Ok(Tray::Node(node))
    }
}

impl Alter for Attribute {
    fn alter(&mut self, _: Post) -> alter::Result {
        Ok(Report::default())
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