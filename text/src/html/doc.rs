use std::cell::RefCell;
use std::collections::HashMap;

use super::*;

pub type AttributeSet = HashMap<&'static str, Leaf>;

pub struct Doc {
    root: Option<Box<RefCell<Option<Doc>>>>,
    tag_name: &'static str,
    tag: Agent<Tag>,
    element: Agent<Element>,
    tag_names: HashMap<&'static str, Node>,
    attributes: AttributeSet,
    repo: Node,
}

pub fn attribute_set() -> AttributeSet {
    let mut atts = HashMap::new();
    for att in ATTRIBUTES {
        atts.insert(att, att.leaf());
    }
    atts
}

impl Doc {
    pub fn new(repo: &Node, atts: &AttributeSet) -> Self {
        let repo = repo.clone();
        let mut tags = HashMap::new();
        for tag in TAGS {
            tags.insert(tag, tag.leaf().node());
        }
        let doctype = tags.get(DOCTYPE).unwrap();
        let tag = Tag::new().repo(&repo).name(doctype).agent();
        Self {
            tag_name: DOCTYPE,
            root: None,
            element: Element::new().repo(&repo).tag(tag.ploy()).agent(),
            tag,
            tag_names: tags,
            attributes: atts.clone(),
            repo,
        }
    }
    pub fn node(&self) -> Node {
        self.element.node()
        //graph::Agent::make(|back| self.element.backed(back)).ploy()
    }
    pub fn link(&self) -> Agent<Element> {
        self.element.clone()
    }
    pub fn string(&self) -> Result<String, Error> {
        if let Load::String(string) = self.element.node().load()? {
            return Ok(string);
        }
        Err("not a string".into())
    }
    pub fn add_str(&mut self, str: &str) -> &mut Self {
        self.element
            .write(|pack| {
                pack.unit.item(str);
            })
            .ok();
        self
    }
    pub fn root(self) -> Self {
        let root = self
            .root
            .as_ref()
            .expect("element should have a root")
            .replace(None)
            .unwrap();
        let element = self.element.node();
        root.element
            .write(|Pack { unit, back }| {
                unit.item(element.backed(back));
            })
            .ok();
        self.repo.edit().insert(element).run().ok();
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
    pub fn attribute(&mut self, name: &str, value: &str) -> &mut Self {
        if let Some(name) = self.attributes.get(name) {
            let attribute = Attribute::new()
                .repo(&self.repo)
                .name(name)
                .content(value)
                .node();
            self.tag
                .write(|Pack { unit, back }| {
                    unit.attribute(attribute.backed(back));
                })
                .ok();
            self.repo.edit().insert(attribute).run().ok();
        }
        self
    }
    pub fn stem(self, tag_name: &'static str) -> Self {
        let tag_leaf = self.tag_names.get(tag_name).unwrap();
        let tag = Tag::new().repo(&self.repo).name(tag_leaf).agent();
        let mut element = Element::new();
        let element = match tag_name {
            "meta" => &mut element,
            _ => element.close(tag_leaf),
        }
        .repo(&self.repo)
        .tag(tag.ploy())
        .agent();
        Doc {
            tag_name,
            tag_names: self.tag_names.clone(),
            attributes: self.attributes.clone(),
            element,
            tag,
            repo: self.repo.clone(),
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
