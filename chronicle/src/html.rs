mod attribute;
mod tag;

use std::{cell::RefCell, rc::Rc};

use crate::text::{self, *};
use attribute::*;
use graph::Id;
use tag::*;

pub fn doc() -> Element {
    Element::default()
}

trait Html {
    fn text(&self) -> Text;
}

struct Leaf {
    text: Text,
    id: Option<Id>,
}

impl Html for Leaf {
    fn text(&self) -> Text {
        self.text.clone()
    }
}

fn leaf(string: &str) -> Rc<dyn Html> {
    let text = text::node::leaf_node(string);
    Rc::new(Leaf { text, id: None })
}

pub struct Element {
    tag: &'static Tag,
    root: Option<Box<RefCell<Element>>>,
    stems: Vec<Rc<dyn Html>>,
    attributes: Vec<Text>,
}

impl Element {
    pub fn string(&self) -> String {
        self.text().string()
    }
    pub fn text(&self) -> Text {
        (self as &dyn Html).text()
    }
    pub fn leaf(&mut self, string: &str) -> &mut Self {
        self.stems.push(leaf(string));
        self
    }
    pub fn root(self) -> Self {
        let mut root = self
            .root
            .as_ref()
            .expect("element should have a root")
            .replace(Element::default());
        root.stems.push(Rc::new(self));
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
    pub fn attribute(&mut self, name: &'static str, val: &str) -> &mut Self {
        self.attributes.push(attribute(name, val));
        self
    }
    pub fn html(self) -> Self {
        element(self, &HTML)
    }
    pub fn head(self) -> Self {
        element(self, &HEAD)
    }
    pub fn title(self) -> Self {
        element(self, &TITLE)
    }
    pub fn meta(self) -> Self {
        element(self, &META)
    }
    pub fn body(self) -> Self {
        element(self, &BODY)
    }
    pub fn div(self) -> Self {
        element(self, &DIV)
    }
    pub fn h1(self) -> Self {
        element(self, &H1)
    }
    pub fn up_to_html(self) -> Self {
        self.up(&HTML)
    }
    pub fn up_to_doc(self) -> Self {
        self.up(&DOCTYPE)
    }
    pub fn lang(&mut self, val: &str) -> &mut Self {
        self.attribute(&LANG, val)
    }
    pub fn charset(&mut self, val: &str) -> &mut Self {
        self.attribute(&CHARSET, val)
    }
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.attribute(&NAME, val)
    }
    pub fn content(&mut self, val: &str) -> &mut Self {
        self.attribute(&CONTENT, val)
    }
}

impl Default for Element {
    fn default() -> Self {
        Element {
            tag: &DOCTYPE,
            root: None,
            stems: vec![],
            attributes: vec![],
        }
    }
}

impl Html for Element {
    fn text(&self) -> Text {
        let mut ot = list();
        ot.add_string(&self.tag.open);
        for att in self.attributes.iter() {
            ot.add_text(att);
        }
        ot.add_string(">").separator(" ");
        let mut el = list();
        el.add_list(ot);
        for stem in self.stems.iter() {
            el.add_text(&stem.text());
        }
        el.add_string(&self.tag.close).separator("\n");
        Text(Rc::new(el))
    }
}

fn element(root: Element, tag: &'static Tag) -> Element {
    Element {
        tag,
        root: Some(Box::new(RefCell::new(root))),
        stems: vec![],
        attributes: vec![],
    }
}

fn attribute(name: &'static str, value: &str) -> Text {
    text::node::leaf_node(&format!(r#"{name}="{value}""#))
}
