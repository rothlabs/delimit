use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Hash)]
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
    fn trade(&mut self, trade: &dyn Trade) -> adapt::Result {
        self.tag = self.tag.trade(trade);
        self.items = self.items.trade(trade);
        if let Some(close) = &self.close {
            self.close = Some(close.trade(trade));
        }
        Ok(Gain::None)
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
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(trade) => self.trade(trade.as_ref()),
            _ => did_not_adapt(post),
        }
    }
}

impl Solve for Element {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            Task::Stems => self.stems(),
            Task::Serial => self.serial(),
            Task::Hash => self.digest(),
            _ => did_not_solve(),
        }
    }
}
