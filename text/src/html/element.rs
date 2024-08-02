use super::*;

#[derive(Default)]
pub struct Element {
    pub tag: Node,
    pub items: Vec<Node>,
    pub close: Option<Node>,
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
}

impl Make for Element {
    fn make(&self, back: &Back) -> Self {
        Self {
            tag: self.tag.backed(back),
            items: self.items.backed(back),
            close: self.close.as_ref().map(|close| close.backed(back)),
        }
    }
}

impl Solve for Element {
    fn solve(&self, _: Task) -> solve::Result {
        let mut element = List::new();
        element.separator("\n").push(self.tag.at(PLAIN)?);
        element.extend(self.items.at(PLAIN)?);
        if let Some(close) = &self.close {
            let close = List::new().push("</").push(close.at(PLAIN)?).push(">").node();
            element.push(close);
        }
        Ok(element.node().tray())
    }
}

impl Alter for Element {
    fn alter(&mut self, _: Post, _: &Back) -> alter::Result {
        Ok(Report::None)
    }
}