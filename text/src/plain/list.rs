use super::*;

#[derive(Default)]
pub struct List {
    pub items: Vec<Node>,
    pub separator: Node,
}

impl List {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn separator(&mut self, separator: impl Into<Node>) -> &mut Self {
        self.separator = separator.into();
        self
    }
    pub fn extend(&mut self, items: Vec<impl Into<Node>>) -> &mut Self {
        self.items.extend(items.into_iter().map(|item| item.into()));
        self
    }
    pub fn push(&mut self, item: impl Into<Node>) -> &mut Self {
        self.items.push(item.into());
        self
    }
    pub fn remove(&mut self, index: usize) -> &mut Self {
        self.items.remove(index);
        self
    }
}

impl Backed for List {
    fn backed(&self, back: &Back) -> Self {
        Self {
            items: self.items.backed(back),
            separator: self.separator.backed(back),
        }
    }
}

impl Solve for List {
    fn solve(&self, _: Task) -> solve::Result {
        if self.items.is_empty() {
            return Node::empty();
        }
        let last = self.items.len() - 1;
        let mut string = String::new();
        self.separator.read_string(|sep| {
            for i in 0..last {
                self.items[i].read_string(|s| string += s);
                string += sep;
            }
        });
        self.items[last].read_string(|s| string += s);
        Ok(string.leaf().node().tray())
    }
}

impl Alter for List {
    fn alter(&mut self, _: Post) -> alter::Result {
        Ok(Report::default())
    }
}
