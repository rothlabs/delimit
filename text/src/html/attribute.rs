use crate::html::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Attribute {
    name: Node,
    content: Node,
    // repo: Node,
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
    // pub fn repo(&mut self, repo: impl Into<Node>) -> &mut Self {
    //     self.repo = repo.into();
    //     self
    // }
    fn stems(&self) -> solve::Result {
        Ok(Tray::Nodes(vec![self.name.clone(), self.content.clone()]))
    }
    fn main(&self) -> solve::Result {
        let node = List::new()
            .push(self.name.at(PLAIN)?)
            .push(r#"=""#)
            .push(self.content.at(PLAIN)?)
            .push(r#"""#)
            .node();
        // self.repo.edit().insert(&node).run()?;
        Ok(node.tray())
    }
}

impl Solve for Attribute {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            Task::Stems => self.stems(),
            _ => Ok(Tray::None),
        }
    }
}

impl Alter for Attribute {
    fn alter(&mut self, _: Post) -> alter::Result {
        Ok(Report::None)
    }
}

impl Make for Attribute {
    fn make(&self, back: &Back) -> Self {
        Self {
            name: self.name.backed(back),
            content: self.content.backed(back),
            // repo: self.repo.clone(),
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
