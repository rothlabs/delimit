use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
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
    fn stems(&self) -> solve::Result {
        let mut stems = self.items.clone();
        stems.push(self.tag.clone());
        if let Some(node) = &self.close {
            stems.push(node.clone());
        }
        Ok(Tray::Nodes(stems))
    }
    fn main(&self) -> solve::Result {
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

impl Solve for Element {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            Task::Stems => self.stems(),
            _ => Ok(Tray::None)
        }
    }
}

impl Alter for Element {
    fn alter(&mut self, _: Post) -> alter::Result {
        Ok(Report::None)
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
