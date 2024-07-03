use crate::plain::*;

#[derive(Default)]
pub struct List {
    pub items: Vec<Item>,
    separator: String,
}

impl List {
    pub fn new() -> Self {
        List::default()
    }
    pub fn from_separator(sep: &str) -> Self {
        Self {
            items: vec![],
            separator: sep.to_owned(),
        }
    }
    pub fn separator(&mut self, sep: &str) -> &mut Self {
        sep.clone_into(&mut self.separator);
        self
    }
    pub fn add_str(&mut self, item: &str) {
        self.items.push(Item::String(item.to_owned()));
    }
    pub fn add_leaf(&mut self, item: Leaf<String>) {
        self.items.push(Item::Leaf(item));
    }
    pub fn add_view(&mut self, view: &View, reactor: &Reactor) {
        self.items.push(Item::View(view.with_reactor(reactor)));
    }
    pub fn remove(&mut self, index: usize) {
        self.items.remove(index);
    }
    pub fn text(self) -> Text<Self> {
        Text::new(self)
    }
}

impl Solve for List {
    type Load = Leaf<String>;
    fn solve(&self) -> Self::Load {
        let mut string = String::new();
        if self.items.is_empty() {
            return string.into_leaf();
        }
        for i in 0..self.items.len() - 1 {
            self.items[i].read(|s| string += s);
            string += &self.separator;
        }
        if let Some(item) = self.items.last() {
            item.read(|s| string += s);
        }
        string.into_leaf()
    }
}

// TODO: remove need for reactor for Unit that does not need to react
impl React for List {
    fn clear(&mut self) -> Reactors {
        Reactors::new()
    }
    fn react(&mut self) {}
}

pub trait TextList {
    fn text_list(self) -> Text<List>;
}

impl TextList for &str {
    fn text_list(self) -> Text<List> {
        List::from_separator(self).text()
    }
}

