// use derive_builder::Builder;

use super::*;

#[derive(Clone)]
pub struct List {
    pub items: Vec<Stem>,
    pub separator: Option<String>,
}

impl List {
    pub fn role<F: FnOnce(&Back) -> Self>(make: F) -> Role {
        let link = Link::make(make);
        Role {
            part: Part::List(link.clone()),
            form: link.ploy(),
        }
    }
    pub fn separator(&mut self, sep: &str) {
        self.separator = Some(sep.to_owned());
        //sep.clone_into(&mut self.separator);
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
        if let Some(sep) = &self.separator {
            for i in 0..self.items.len() - 1 {
                self.items[i].read(|s| string += s);
                string += sep;
            }
        } else {
            for i in 0..self.items.len() - 1 {
                self.items[i].read(|s| string += s);
            }
        }
        if let Some(item) = self.items.last() {
            item.read(|s| string += s);
        }
        string.into_ace()
    }
}

pub trait ToList {
    fn list(self) -> Hold<Link<List>, Role>;
}

impl ToList for &str {
    fn list(self) -> Hold<Link<List>, Role> {
        let link = Link::new(List {
            separator: Some(self.into()),
            items: vec![],
        });
        let role = Role {
            part: Part::List(link.clone()),
            form: link.ploy(),
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
