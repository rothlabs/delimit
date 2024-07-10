use crate::plain::*;

#[derive(Default)]
pub struct List {
    pub items: Vec<Item>,
    separator: String,
}

impl List {
    // pub fn new() -> Self {
    //     List::default()
    // }
    // pub fn from_separator(sep: &str) -> Self {
    //     Self {
    //         items: vec![],
    //         separator: sep.to_owned(),
    //     }
    // }
    pub fn separator(&mut self, sep: &str) -> &mut Self {
        sep.clone_into(&mut self.separator);
        self
    }
    pub fn remove(&mut self, index: usize) {
        self.items.remove(index);
    }
}

impl Solve for List {
    type Load = Load;
    fn solve(&self) -> Load {
        let mut string = String::new();
        if self.items.is_empty() {
            return string.into_leaf();
        }
        for i in 0..self.items.len() - 1 {
            self.items[i].reader(|s| string += s);
            string += &self.separator;
        }
        if let Some(item) = self.items.last() {
            item.reader(|s| string += s);
        }
        string.into_leaf()
    }
}

pub trait TextList {
    fn list(self) -> Hold<Text<List>, Role>;
}

impl TextList for &str {
    fn list(self) -> Hold<Text<List>, Role> {
        let link = Text::new(List {
            separator: self.into(),
            items: vec![],
        });
        let view = Role {
            exact: Exact::List(link.clone()),
            solver: link.solver(),
        };
        Hold { link, view }
    }
}

// pub trait TextList {
//     fn list(self) -> Text<List>;
// }

// impl TextList for &str {
//     fn list(self) -> Text<List> {
//         List::from_separator(self).text()
//     }
// }

// pub fn text(self) -> Text<Self> {
//     Text::new(self)
// }

// pub fn add_str(&mut self, item: &str) {
//     self.items.push(View::Bare(item.to_owned()));
// }
// pub fn add_leaf(&mut self, item: Leaf<String>) {
//     self.items.push(View::Leaf(item));
// }
// pub fn add_role(&mut self, role: &Role, reactor: &Reactor) {
//     self.items.push(View::Role(role.with_reactor(reactor)));
// }

// // TODO: remove need for reactor for Unit that does not need to react
// impl React for List {
//     fn clear(&mut self) -> Reactors {
//         Reactors::new()
//     }
//     fn react(&mut self) {}
// }
