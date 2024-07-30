use super::*;

#[derive(Default, Clone)]
pub struct List {
    pub items: Vec<Node<String>>,
    pub separator: Node<String>,
}

impl List {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn separator(&mut self, separator: impl Into<Node<String>>) -> &mut Self {
        self.separator = separator.into();
        self
    }
    pub fn extend(&mut self, items: Vec<impl Into<Node<String>>>) -> &mut Self {
        self.items.extend(items.into_iter().map(|item| item.into()));
        self
    }
    pub fn item(&mut self, item: impl Into<Node<String>>) -> &mut Self {
        self.items.push(item.into());
        self
    }
    pub fn remove(&mut self, index: usize) -> &mut Self {
        self.items.remove(index);
        self
    }
}

impl ToLink for List {
    fn link(&self) -> Deuce<Self> {
        Deuce::make(|back| Self {
            items: self.items.backed(back),
            separator: self.separator.backed(back),
        })
    }
}

impl Grant for List {
    type Load = Node<String>;
    fn grant(&self) -> Self::Load {
        if self.items.is_empty() {
            return Node::new();
        }
        let mut string = String::new();
        self.separator.read(|sep| {
            for i in 0..self.items.len() - 1 {
                self.items[i].read(|s| string += s);
                string += sep;
            }
        });
        self.items[self.items.len() - 1].read(|s| string += s);
        string.into_ace().into()
    }
}

// if let Some(item) = self.items.last() {
//     item.read(|s| string += s);
// }
