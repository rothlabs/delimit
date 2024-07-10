use std::cell::RefCell;
use std::collections::HashMap;

use crate::html::*;

pub struct Doc {
    root: Option<Box<RefCell<Doc>>>,
    tag_name: &'static str,
    tag: Hold<Html<Tag>, Item>,
    element: Hold<Html<Element>, Item>,
    tags: HashMap<&'static str, Item>, 
    atts: HashMap<&'static str, Item>, 
}

impl Doc {
    pub fn new() -> Self {
        Doc::default()
    }
    pub fn root(self) -> Self {
        let root = self
            .root
            .as_ref()
            .expect("element should have a root")
            .replace(Doc::new());
        root.element.link.writer(|pack| {
            pack.unit.items.root(pack.root).add_view(&self.element.view);
            self.tag.link.reader(|unit| {
                pack.unit.close = Some(unit.name.clone());
            });
        });
        root
    }
    fn up(self, tag: &str) -> Self {
        let mut root = self.root();
        for _ in 0..100 {
            if root.tag_name == tag {
                return root;
            }
            root = root.root();
        }
        panic!("element should have a root with given tag");
    }
    pub fn add_attribute(&mut self, name: &'static str, value: &str) -> &mut Self {
        if let Some(item) = self.atts.get(name) {
            let hold = Attribute::new(item, &plain::string(value));
            self.tag.link.writer(|pack| {
                pack.unit.attributes.root(pack.root).add_view(&hold.view);
            });
        }
        self
    }
    pub fn stem(self, tag_name: &'static str) -> Self {
        let tag = Tag::new(self.tags.get(tag_name).unwrap());
        Doc {
            tag_name,
            tags: self.tags.clone(),
            atts: self.atts.clone(),
            root: Some(Box::new(RefCell::new(self))),
            element: Element::new(&tag.view),
            tag,
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
        self.add_attribute(LANG, val)
    }
    pub fn charset(&mut self, val: &str) -> &mut Self {
        self.add_attribute(CHARSET, val)
    }
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.add_attribute(NAME, val)
    }
    pub fn content(&mut self, val: &str) -> &mut Self {
        self.add_attribute(CONTENT, val)
    }
}

impl Default for Doc {
    fn default() -> Self {
        let mut tags = HashMap::new();
        for tag in TAGS {
            tags.insert(tag, plain::leaf(tag));
        }
        let mut atts = HashMap::new();
        for att in ATTRIBUTES {
            atts.insert(att, plain::leaf(att));
        }
        let doctype = tags.get(DOCTYPE).unwrap();
        let tag = Tag::new(doctype);
        Self {
            tag_name: &DOCTYPE,
            root: None,
            element: Element::new(&tag.view),
            tag,
            tags,
            atts,
        }
    }
}

// use std::cell::RefCell;

// use crate::html::*;
// use crate::plain::List;

// pub struct Element {
//     tag: &'static Tag,
//     root: Option<Box<RefCell<Element>>>, // Todo: change to Option<Html>?
//     items: Vec<Item>,
//     attributes: Vec<Attribute>,
// }

// impl Element {
//     pub fn new() -> Self {
//         Element::default()
//     }
//     fn write_open(&self, pack: &mut Pack<List>) {
//         pack.unit.items.add_str(&self.tag.open);
//         for att in self.attributes.iter() {
//             att.collect(pack);
//         }
//         pack.unit.items.add_str(">");
//     }
//     fn write_items_and_close(&self, pack: &mut Pack<List>) {
//         for item in self.items.iter() {
//             item.collect(pack);
//         }
//         pack.unit.items.add_str(&self.tag.close);
//     }
//     pub fn add_str(&mut self, value: &str) -> &mut Self {
//         self.items.push(Item::String(value.to_owned()));
//         self
//     }
//     pub fn root(self) -> Self {
//         let mut root = self
//             .root
//             .as_ref()
//             .expect("element should have a root")
//             .replace(Element::new());
//         root.items.push(Item::Html(Html::new(self)));
//         root
//     }
//     fn up(self, tag: &Tag) -> Self {
//         let mut root = self.root();
//         for _ in 0..100 {
//             if root.tag.open == tag.open {
//                 return root;
//             }
//             root = root.root();
//         }
//         panic!("element should have a root with given tag");
//     }
//     pub fn add_attribute(&mut self, name: &'static str, value: &str) -> &mut Self {
//         self.attributes
//             .push(Attribute::String(format!(r#"{name}="{value}""#)));
//         self
//     }
//     pub fn stem(self, tag: &'static Tag) -> Self {
//         Element {
//             tag,
//             root: Some(Box::new(RefCell::new(self))),
//             items: vec![],
//             attributes: vec![],
//         }
//     }
//     pub fn html(self) -> Self {
//         self.stem(&HTML)
//     }
//     pub fn head(self) -> Self {
//         self.stem(&HEAD)
//     }
//     pub fn title(self) -> Self {
//         self.stem(&TITLE)
//     }
//     pub fn meta(self) -> Self {
//         self.stem(&META)
//     }
//     pub fn body(self) -> Self {
//         self.stem(&BODY)
//     }
//     pub fn div(self) -> Self {
//         self.stem(&DIV)
//     }
//     pub fn h1(self) -> Self {
//         self.stem(&H1)
//     }
//     pub fn up_to_html(self) -> Self {
//         self.up(&HTML)
//     }
//     pub fn up_to_doc(self) -> Self {
//         self.up(&DOCTYPE)
//     }
//     pub fn lang(&mut self, val: &str) -> &mut Self {
//         self.add_attribute(LANG, val)
//     }
//     pub fn charset(&mut self, val: &str) -> &mut Self {
//         self.add_attribute(CHARSET, val)
//     }
//     pub fn name(&mut self, val: &str) -> &mut Self {
//         self.add_attribute(NAME, val)
//     }
//     pub fn content(&mut self, val: &str) -> &mut Self {
//         self.add_attribute(CONTENT, val)
//     }
// }

// impl Default for Element {
//     fn default() -> Self {
//         Self {
//             tag: &DOCTYPE,
//             root: None,
//             items: vec![],
//             attributes: vec![],
//         }
//     }
// }

// impl Solve for Element {
//     type Load = plain::Role;
//     fn solve(&self) -> Self::Load {
//         let (open_tag, open_tag_list) = " ".list();
//         open_tag_list.writer(|pack| self.write_open(pack));
//         let (text, text_list) = "\n".list();
//         text_list.writer(|pack| {
//             pack.unit.items.add_role(&open_tag, pack.reactor);
//             self.write_items_and_close(pack);
//         });
//         text
//     }
// }