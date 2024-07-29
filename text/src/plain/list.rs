use super::*;

#[derive(Default, Clone)]
pub struct List {
    pub items: Vec<Value<String>>,
    pub separator: Value<String>,
}

impl List {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn link(&self) -> Deuce<Self> {
        Deuce::make(|back| Self {
            items: self.items.backed(back),
            separator: self.separator.backed(back),
        })
    }
    pub fn value(&self) -> Value<String> {
        self.link().ploy().into()
    }
    pub fn separator(&mut self, separator: impl Into<Value<String>>) -> &mut Self {
        self.separator = separator.into();
        self
    }
    pub fn extend(&mut self, items: Vec<impl Into<Value<String>>>) -> &mut Self {
        self.items.extend(items.into_iter().map(|item| item.into()));
        self
    }
    pub fn item(&mut self, item: impl Into<Value<String>>) -> &mut Self {
        self.items.push(item.into());
        self
    }
    pub fn remove(&mut self, index: usize) -> &mut Self {
        self.items.remove(index);
        self
    }
}

impl Grant for List {
    type Load = Value<String>;
    fn grant(&self) -> Self::Load {
        if self.items.is_empty() {
            return Value::default(); //string.into_ace();
        }
        let mut string = String::new();
        self.separator.read(|sep| {
            for i in 0..self.items.len() - 1 {
                self.items[i].read(|s| string += s);
                string += sep;
            }
        });
        if let Some(item) = self.items.last() {
            item.read(|s| string += s);
        }
        string.ace().into()
    }
}

// pub fn role<F: FnOnce(&Back) -> Self>(make: F) -> Role {
//     let link = Link::make(make);
//     Role {
//         part: OldPart::List(link.clone()),
//         form: link.ploy(),
//     }
// }

// pub trait ToList {
//     fn list(self) -> Hold<Link<List>, Role>;
// }

// impl ToList for &str {
//     fn list(self) -> Hold<Link<List>, Role> {
//         let link = Link::new(List {
//             separator: Some(self.into()),
//             items: vec![],
//         });
//         let role = Role {
//             part: OldPart::List(link.clone()),
//             form: link.ploy(),
//         };
//         Hold { link, role }
//     }
// }

// pub fn new() -> Self {
//     List::default()
// }
// pub fn from_separator(sep: &str) -> Self {
//     Self {
//         items: vec![],
//         separator: sep.to_owned(),
//     }
// }
