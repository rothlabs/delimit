use serde::Serialize;

use super::*;

#[derive(Default, Clone, Serialize)]
pub struct Element {
    pub tag: Node,
    pub items: Vec<Node>,
    pub close: Option<Node>,
    repo: Node,
}

impl Element {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn tag(&mut self, tag: impl Into<Node>) -> &mut Self {
        self.tag = tag.into();
        self
    }
    pub fn item(&mut self, item: impl Into<Node>) -> &mut Self {
        self.items.push(item.into());
        self
    }
    pub fn close(&mut self, close: impl Into<Node>) -> &mut Self {
        self.close = Some(close.into());
        self
    }
    pub fn repo(&mut self, repo: impl Into<Node>) -> &mut Self {
        self.repo = repo.into();
        self
    }
}

impl Make for Element {
    fn make(&self, back: &Back) -> Self {
        Self {
            tag: self.tag.backed(back),
            items: self.items.backed(back),
            close: self.close.as_ref().map(|close| close.backed(back)),
            repo: self.repo.clone(),
        }
    }
}

impl Solve for Element {
    fn solve(&self, _: Task) -> solve::Result {
        let mut edit = self.repo.edit();
        let mut element = List::new();
        element.separator("\n").push(self.tag.at(PLAIN)?);
        element.extend(self.items.at(PLAIN)?);
        if let Some(close) = &self.close {
            let close = List::new()
                .push("</")
                .push(close.at(PLAIN)?)
                .push(">")
                .node();
            edit.insert(&close);
            element.push(close);
        }
        let element = element.node();
        edit.insert(&element).run()?;
        Ok(element.tray())
    }
}

impl Alter for Element {
    fn alter(&mut self, _: Post) -> alter::Result {
        Ok(Report::None)
    }
}
