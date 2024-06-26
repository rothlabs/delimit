use graph::*;
use crate::text::*;

#[derive(Default)]
pub struct List {
    pub items: Vec<Stem>,
    pub separator: String,
}

impl List {
    pub fn add_str(&mut self, item: &str) -> &mut Self {
        self.items.push(Stem::String(item.to_owned()));
        self
    }
    pub fn separator(&mut self, sep: &str) -> &mut Self {
        self.separator = sep.to_owned();
        self
    }
    pub fn text(self) -> Text {
        Text::from_unit(Box::new(self))
    }
}

impl ToString for List {
    fn string(&self) -> String {
        let mut string = String::new();
        if self.items.len() < 1 {
            return string
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

impl Unit for List {}



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
