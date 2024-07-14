use serde::Serialize;

use crate::plain::*;

#[derive(Default, Serialize)]
pub struct List {
    pub items: Vec<Item>,
    separator: String,
}

impl List {
    pub fn separator(&mut self, sep: &str) -> &mut Self {
        sep.clone_into(&mut self.separator);
        self
    }
    pub fn remove(&mut self, index: usize) {
        self.items.remove(index);
    }
}

impl Grant for List {
    type Load = Load;
    fn grant(&self) -> Load {
        let mut string = String::new();
        if self.items.is_empty() {
            return string.into_ace();
        }
        for i in 0..self.items.len() - 1 {
            self.items[i].reader(|s| string += s);
            string += &self.separator;
        }
        if let Some(item) = self.items.last() {
            item.reader(|s| string += s);
        }
        string.into_ace()
    }
}

pub trait ToList {
    fn list(self) -> Hold<Text<List>, Role>;
}

impl ToList for &str {
    fn list(self) -> Hold<Text<List>, Role> {
        let link = Text::new(List {
            separator: self.into(),
            items: vec![],
        });
        let role = Role {
            actual: Actual::List(link.clone()),
            method: link.ploy(),
        };
        Hold { link, role }
    }
}

// pub fn new() -> Self {
//     List::default()
// }
// pub fn from_separator(sep: &str) -> Self {
//     Self {
//         items: vec![],
//         separator: sep.to_owned(),
//     }
// }
