use std::cell::RefCell;
use std::collections::HashMap;

use crate::html::*;

pub struct Doc {
    root: Option<Box<RefCell<Doc>>>,
    tag_name: &'static str,
    tag: Hold<Html<Tag>, Item>,
    element: Hold<Html<Element>, Item>,
    tag_names: HashMap<&'static str, Item>,
    att_names: HashMap<&'static str, Item>,
}

impl Doc {
    pub fn new() -> Self {
        Doc::default()
    }
    pub fn string(&self) -> String {
        self.element.link.grant().grant().load()
    }
    pub fn add_str(&mut self, value: &str) -> &mut Self {
        self.element.link.writer(|pack| {
            pack.unit.items.add_view(plain::string(value));
        });
        self
    }
    pub fn root(self) -> Self {
        let root = self
            .root
            .as_ref()
            .expect("element should have a root")
            .replace(Doc::new());
        root.element.link.writer(|pack| {
            pack.unit.items.back(pack.root).add_view(&self.element.view);
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
        if let Some(item) = self.att_names.get(name) {
            let hold = Attribute::new(item, &plain::string(value));
            self.tag.link.writer(|pack| {
                pack.unit.attributes.back(pack.root).add_view(&hold.view);
            });
        }
        self
    }
    pub fn stem(self, tag_name: &'static str) -> Self {
        let tag_leaf = self.tag_names.get(tag_name).unwrap();
        let tag = Tag::new(tag_leaf);
        let close = match tag_name {
            "meta" => None,
            _ => Some(tag_leaf),
        };
        Doc {
            tag_name,
            tag_names: self.tag_names.clone(),
            att_names: self.att_names.clone(),
            element: Element::new(&tag.view, close),
            tag,
            root: Some(Box::new(RefCell::new(self))),
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
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.add_attribute(ID, val)
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
    pub fn r#type(&mut self, val: &str) -> &mut Self {
        self.add_attribute(TYPE, val)
    }
    pub fn src(&mut self, val: &str) -> &mut Self {
        self.add_attribute(SRC, val)
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
            tag_name: DOCTYPE,
            root: None,
            element: Element::new(&tag.view, None),
            tag,
            tag_names: tags,
            att_names: atts,
        }
    }
}
