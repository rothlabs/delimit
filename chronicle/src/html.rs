mod tag;
mod attribute;

use std::{
    rc::Rc,
    cell::RefCell, 
};

use tag::*;
use attribute::*;
use crate::text::*;

pub fn doc() -> Element {
    Element::default()
}

trait Html {
    fn text(&self) -> Rc<dyn Text>;
}

impl Html for String {
    fn text(&self) -> Rc<dyn Text> {
        Rc::new(self.to_owned())
    }
}

pub struct Element {
    tag: &'static Tag,
    root: Option<Box<RefCell<Element>>>,
    stems: Vec<Rc<dyn Html>>,
    attributes: Vec<Rc<String>>,
}

impl Element {
    pub fn text(&self) -> Rc<dyn Text> {
        (self as &dyn Html).text()
    }
    pub fn leaf(&mut self, content: &str) -> &mut Self {
        self.stems.push(Rc::new(content.to_owned()));
        self
    }
    pub fn root(self) -> Self { 
        let mut root = self.root.as_ref()
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
        let att = attribute(name, val);
        self.attributes.push(Rc::new(att));
        self
    } 
    pub fn html(self)  -> Self { element(self, &HTML) }
    pub fn head(self)  -> Self { element(self, &HEAD) }
    pub fn title(self) -> Self { element(self, &TITLE) }
    pub fn meta(self) -> Self { element(self, &META) }
    pub fn body(self)  -> Self { element(self, &BODY) }
    pub fn div(self)   -> Self { element(self, &DIV) }
    pub fn h1(self)    -> Self { element(self, &H1) }
    pub fn up_to_html(self) -> Self { self.up(&HTML) }
    pub fn up_to_doc(self)  -> Self { self.up(&DOCTYPE) }
    pub fn lang(&mut self, val: &str)    -> &mut Self { self.attribute(&LANG, val) } 
    pub fn charset(&mut self, val: &str) -> &mut Self { self.attribute(&CHARSET, val) }
    pub fn name(&mut self, val: &str)    -> &mut Self { self.attribute(&NAME, val) }
    pub fn content(&mut self, val: &str) -> &mut Self { self.attribute(&CONTENT, val) }
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
    fn text(&self) -> Rc<dyn Text> {
        let mut ot = list();
        ot.leaf(&self.tag.open);
        for att in self.attributes.iter() {
            ot.node(&att.text());
        }
        ot.leaf(">").separator(" ");
        let mut el = list();
        el.list(ot);
        for stem in self.stems.iter() {
            el.node(&stem.text());
        }
        el.leaf(&self.tag.close).separator("\n");
        Rc::new(el)
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

fn attribute(name: &'static str, value: &str) -> String {
    format!(r#"{name}="{value}""#) 
}