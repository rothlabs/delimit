use std::{borrow::Cow, cell::RefCell};

use super::{attribute::*, html, tag::*, Html};
use crate::text::{text, unit::{list, List}, Text};
use graph::{Edge, Snap};
use serde::Serialize;

pub fn doc(snap: &Snap) -> Element {
    Element {
        tag: &DOCTYPE,
        root: None,
        items: vec![],
        attributes: vec![],
        snap: snap.clone(),
    }
}

// impl Unit for Text {
//     fn text(&self, _: &Snap) -> Text {
//         self.clone()
//     }
// }

#[derive(Clone, Serialize)]
pub struct Element {
    tag: &'static Tag,
    root: Option<Box<RefCell<Element>>>, // Todo: change to Option<Html>?
    items: Vec<Item>,
    attributes: Vec<Attribute>,
    snap: Snap,
}

impl Element {
    pub fn text(&self) -> Text {
        let mut open_tag = list(&self.snap);
        open_tag.add_str(&self.tag.open);
        for att in self.attributes.iter() {
            att.to_list(&mut open_tag);
        }
        open_tag.add_str(">").separator(" ");
        let mut items = list();
        items.add_list(open_tag);
        for item in self.items.iter() {
            item.to_list(&mut items); //el.add_text(&stem.text());
        }
        items.add_str(&self.tag.close).separator("\n");
        items.text()
    }
    // pub fn string(&self) -> String {
    //     self.text().string()
    // }
    // pub fn leaf(&mut self, string: &str) -> &mut Self {
    //     self.items.push(html(Edge::new(string.to_owned())));
    //     self
    // }
    pub fn add_string(&mut self, unit: &str) -> &mut Self {
        self.items.push(Item::String(unit.to_owned()));
        self
    }
    pub fn root(self, snap: &Snap) -> Self {
        let mut root = self
            .root
            .as_ref()
            .expect("element should have a root")
            .replace(Element::default());
        root.items.push(Item::Html(html(snap, self)));
        root
    }
    fn up(self, tag: &Tag) -> Self {
        let mut root = self.root();
        for _ in 0..100 {
            if root.tag.open == tag.open {
                return root;
            }
            root = root.root();
        }
        panic!("element should have a root with given tag");
    }
    pub fn add_attribute(&mut self, name: &'static str, value: &str) -> &mut Self {
        self.attributes
            .push(Attribute::String(format!(r#"{name}="{value}""#)));
        self
    }
    pub fn stem(self, tag: &'static Tag) -> Self {
        Element {
            tag,
            root: Some(Box::new(RefCell::new(self))),
            items: vec![],
            attributes: vec![],
        }
    }
    pub fn html(self) -> Self {
        self.stem(&HTML)
    }
    pub fn head(self) -> Self {
        self.stem(&HEAD)
    }
    pub fn title(self) -> Self {
        self.stem(&TITLE)
    }
    pub fn meta(self) -> Self {
        self.stem(&META)
    }
    pub fn body(self) -> Self {
        self.stem(&BODY)
    }
    pub fn div(self) -> Self {
        self.stem(&DIV)
    }
    pub fn h1(self) -> Self {
        self.stem(&H1)
    }
    pub fn up_to_html(self) -> Self {
        self.up(&HTML)
    }
    pub fn up_to_doc(self) -> Self {
        self.up(&DOCTYPE)
    }
    pub fn lang(&mut self, val: &str) -> &mut Self {
        self.add_attribute(&LANG, val)
    }
    pub fn charset(&mut self, val: &str) -> &mut Self {
        self.add_attribute(&CHARSET, val)
    }
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.add_attribute(&NAME, val)
    }
    pub fn content(&mut self, val: &str) -> &mut Self {
        self.add_attribute(&CONTENT, val)
    }
}

// impl Default for Element {
//     fn default() -> Self {
//         Element {
//             tag: &DOCTYPE,
//             root: None,
//             items: vec![],
//             attributes: vec![],
//         }
//     }
// }

#[derive(Clone, Serialize)]
enum Item {
    String(String),
    Text(Text),
    Html(Html),
}

impl Item {
    fn to_list(&self, list: &mut List) {
        match self {
            Item::String(s) => list.add_str(s), 
            Item::Text(t) => list.add_text(t),
            Item::Html(h) => list.add_text(&h.text()),
        };
    }
}

#[derive(Clone, Serialize)]
enum Attribute {
    String(String),
    Text(Text),
}

impl Attribute {
    fn to_list(&self, list: &mut List) {
        match self {
            Attribute::String(s) => list.add_str(s), 
            Attribute::Text(t) => list.add_text(t),
        };
    }
}


// impl Unit for Element {
//     fn text(&self, snap: &Snap) -> Text {
//         let mut ot = list();
//         ot.add_string(snap, &self.tag.open);
//         for att in self.attributes.iter() {
//             ot.add_text(att);
//         }
//         ot.add_string(snap, ">").separator(" ");
//         let mut el = list();
//         el.add_list(ot);
//         for stem in self.items.iter() {
//             el.add_text(&stem.text());
//         }
//         el.add_string(&self.tag.close).separator("\n");
//         text(el)
//     }
// }



// impl Serialize for Element {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
        
//     }
// }

// fn attribute(name: &'static str, value: &str) -> Text {
//     text(leaf_str(&format!(r#"{name}="{value}""#)))
// }
