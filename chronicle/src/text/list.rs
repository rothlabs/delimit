use crate::text::*;

#[derive(Default)]
pub struct List {
    items: Vec<Item>,
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
    pub fn add_text<U: Solve<Task = Task, Load = Load> + 'static>(&mut self, text: Text<U>) {
        self.items.push(Item::Text(text.0.to_solver()));
    }
    pub fn add_solver(&mut self, text: TextSolver) {
        self.items.push(Item::Text(text));
    }
    pub fn remove(&mut self, index: usize) {
        self.items.remove(index);
    }
    pub fn text(self) -> Text<Self> {
        Text::new(self)
    }
}

impl GraphString for List {
    fn string(&self) -> String {
        let mut string = String::new();
        if self.items.is_empty() {
            return string;
        }
        for i in 0..self.items.len() - 1 {
            self.items[i].read(|s| string += s);
            string += &self.separator;
        }
        if let Some(item) = self.items.last() {
            item.read(|s| string += s);
        }
        string
    }
}

impl Solve for List {
    type Task = Task;
    type Load = Load;
    fn solve(&self, task: Self::Task) -> Self::Load {
        match task {
            Task::String => Load::String(self.string()),
            Task::Leaf => Load::Leaf(self.leaf()),
        }
    }
}

impl React for List {
    fn clear(&mut self) -> Reactors {
        Reactors::default()
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

//impl Unit for List {}

// pub fn list() -> List {
//     List {
//         items: vec![],
//         separator: "".into(),
//     }
// }

// #[derive(Clone, Serialize)]

// impl List {
//     pub fn text(self) -> Text {
//         text(Box::new(self))
//     }
//     pub fn separator(&mut self, sep: &str) -> &mut Self {
//         self.separator = sep.to_owned();
//         self
//     }
//     // pub fn add_text(&mut self, text: &Text) -> &mut Self {
//     //     self.items.push(Stem::Text(text.clone()));
//     //     self
//     // }
//     // pub fn add_str(&mut self, unit: &str) -> &mut Self {
//     //     self.items.push(Stem::String(unit.to_owned()));
//     //     self
//     // }
//     // pub fn add_list(&mut self, list: List) -> &mut Self {
//     //     self.items.push(Stem::Text(text(Box::new(list))));
//     //     self
//     // }
//     // pub fn add_leaf(&mut self, leaf: &Leaf<String>) -> &mut Self {
//     //     self.items.push(Stem::Leaf(leaf.clone()));
//     //     self
//     // }
// }

// impl Unit for List {
//     fn leaf(&self) -> Leaf<String> {
//         let mut string = String::new();
//         if self.items.len() < 1 {
//             return Leaf::from_unit(string);
//         }
//         for i in 0..self.items.len() - 1 {
//             self.items[i].read(|s| string += s);
//             string += &self.separator;
//         }
//         if let Some(item) = self.items.last() {
//             item.read(|s| string += s);
//         }
//         Leaf::from_unit(string)
//     }
//     fn serial(&self) -> String {
//         String::new()
//         // serde_json::to_string(self).unwrap()
//     }
//     fn string(&self) -> String {
//         self.leaf().unit()
//     }
//     fn add_item(&mut self, item: Stem) {
//         self.items.push(item);
//     }
// }
