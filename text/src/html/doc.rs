use std::cell::RefCell;
use std::collections::HashMap;

use super::*;

pub type AttributeSet = HashMap<&'static str, Ace<String>>;

pub struct Doc {
    root: Option<Box<RefCell<Option<Doc>>>>,
    tag_name: &'static str,
    tag: Hold<Link<Tag>, Role>,
    element: Hold<Link<Element>, Role>,
    tag_names: HashMap<&'static str, Stem>,
    attributes: AttributeSet,
}

pub fn attribute_set() -> AttributeSet {
    let mut atts = HashMap::new();
    for att in ATTRIBUTES {
        atts.insert(att, att.to_owned().ace());
    }
    atts
}

impl Doc {
    pub fn new(atts: &AttributeSet) -> Self {
        let mut tags = HashMap::new();
        for tag in TAGS {
            tags.insert(tag, plain::str(tag));
        }
        let doctype = tags.get(DOCTYPE).unwrap();
        let tag = Tag::hold(doctype);
        Self {
            tag_name: DOCTYPE,
            root: None,
            tag: tag.clone(),
            element: Element::hold(&Stem::Role(tag.role), None),
            tag_names: tags,
            attributes: atts.clone(),
        }
    }
    pub fn pipe(&self) -> Pipe {
        Pipe::make(|back| self.element.role.backed(back))
    }
    pub fn link(&self) -> Link<Element> {
        self.element.link.clone()
    }
    pub fn string(&self) -> String {
        let plain = self.element.link.grant();
        let ace = plain.grant();
        ace.load()
    }
    pub fn add_str(&mut self, value: &str) -> &mut Self {
        self.element.link.write(|pack| {
            pack.unit.items.push(plain::str(value));
        });
        self
    }
    pub fn root(self) -> Self {
        let root = self
            .root
            .as_ref()
            .expect("element should have a root")
            .replace(None)
            .unwrap();
        root.element.link.write(|pack| {
            pack.unit.items.back(pack.back).add_role(&self.element.role);
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
    pub fn attribute(&mut self, name: &'static str, value: &str) -> &mut Self {
        if let Some(name_ace) = self.attributes.get(name) {
            let hold = Attribute::hold(&plain::ace(name_ace), &plain::str(value));
            self.tag.link.write(|Pack { unit, back }| {
                unit.attributes.back(back).add_role(&hold.role);
            });
        }
        self
    }
    pub fn stem(self, tag_name: &'static str) -> Self {
        let tag_leaf = self.tag_names.get(tag_name).unwrap();
        let tag = Tag::hold(tag_leaf);
        let close = match tag_name {
            "meta" => None,
            _ => Some(tag_leaf),
        };
        Doc {
            tag_name,
            tag_names: self.tag_names.clone(),
            attributes: self.attributes.clone(),
            tag: tag.clone(),
            element: Element::hold(&Stem::Role(tag.role), close),
            root: Some(Box::new(RefCell::new(Some(self)))),
        }
    }
    pub fn html(self) -> Self {
        self.stem(HTML)
    }
    pub fn head(self) -> Self {
        self.stem(HEAD)
    }
    pub fn title(self) -> Self {
        self.stem(TITLE)
    }
    pub fn meta(self) -> Self {
        self.stem(META)
    }
    pub fn script(self) -> Self {
        self.stem(SCRIPT)
    }
    pub fn body(self) -> Self {
        self.stem(BODY)
    }
    pub fn div(self) -> Self {
        self.stem(DIV)
    }
    pub fn canvas(self) -> Self {
        self.stem(CANVAS)
    }
    pub fn h1(self) -> Self {
        self.stem(H1)
    }
    pub fn up_to_html(self) -> Self {
        self.up(HTML)
    }
    pub fn up_to_doc(self) -> Self {
        self.up(DOCTYPE)
    }
}

// pub fn id(&mut self, val: &str) -> &mut Self {
//     self.add_attribute(ID, val)
// }
// pub fn lang(&mut self, val: &str) -> &mut Self {
//     self.add_attribute(LANG, val)
// }
// pub fn charset(&mut self, val: &str) -> &mut Self {
//     self.add_attribute(CHARSET, val)
// }
// pub fn name(&mut self, val: &str) -> &mut Self {
//     self.add_attribute(NAME, val)
// }
// pub fn content(&mut self, val: &str) -> &mut Self {
//     self.add_attribute(CONTENT, val)
// }
// pub fn r#type(&mut self, val: &str) -> &mut Self {
//     self.add_attribute(TYPE, val)
// }
// pub fn src(&mut self, val: &str) -> &mut Self {
//     self.add_attribute(SRC, val)
// }

// let mut atts = HashMap::new();
// for att in ATTRIBUTES {
//     atts.insert(att, plain::ace(att));
// }

// impl Default for Doc {
//     fn default() -> Self {
//         let mut tags = HashMap::new();
//         for tag in TAGS {
//             tags.insert(tag, plain::str(tag));
//         }
//         // let mut atts = HashMap::new();
//         // for att in ATTRIBUTES {
//         //     atts.insert(att, plain::ace(att));
//         // }
//         let doctype = tags.get(DOCTYPE).unwrap();
//         let tag = Tag::new(doctype);
//         Self {
//             tag_name: DOCTYPE,
//             root: None,
//             tag: tag.clone(),
//             element: Element::new(&Stem::Role(tag.role), None),
//             tag_names: tags,
//             att_names: atts,
//         }
//     }
// }
