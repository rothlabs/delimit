use super::*;

#[derive(Default, Clone)]
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
    fn solve(&self) -> solve::Result {
        if self.items.is_empty() {
            return Node::empty();
        }
        let mut string = String::new();
        self.separator.read_string(|sep| {
            for i in 0..self.items.len() - 1 {
                self.items[i].read_string(|s| string += s);
                string += sep;
            }
        });
        self.items[self.items.len() - 1].read_string(|s| string += s);
        Ok(string.ace().node().tray())
    }
}

// if let Some(item) = self.items.last() {
//     item.read(|s| string += s);
// }
