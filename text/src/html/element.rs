use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Element {
    tag: Node,
    pub items: Vec<Node>,
    close: Option<Node>,
    story: Node,
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
    pub fn story(&mut self, story: impl Into<Node>) -> &mut Self {
        self.story = story.into();
        self
    }
    fn main(&self) -> solve::Result {
        let mut element = List::new();
        element.separator("\n").push(self.tag.at(PLAIN)?);
        element.extend(self.items.at(PLAIN)?);
        if let Some(close) = &self.close {
            let close = List::new()
                .push("</")
                .push(close.at(PLAIN)?)
                .push(">")
                .node();
            element.push(close);
        }
        let element = element.node();
        Ok(element.tray())
    }
    fn stems(&self) -> solve::Result {
        let mut nodes = self.items.clone();
        nodes.push(self.tag.clone());
        if let Some(node) = &self.close {
            nodes.push(node.clone());
        }
        Ok(nodes.tray())
    }
}

impl Adapt for Element {
    fn adapt(&mut self, _: Post) -> adapt::Result {
        did_not_adapt()
    }
}

impl Solve for Element {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            Task::Stems => self.stems(),
            _ => did_not_solve(),
        }
    }
}

impl Make for Element {
    fn make(&self, back: &Back) -> Self {
        Self {
            tag: self.tag.backed(back),
            items: self.items.backed(back),
            close: self.close.as_ref().map(|close| close.backed(back)),
            story: self.story.backed(back),
        }
    }
}
