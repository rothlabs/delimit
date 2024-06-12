use std::cell::RefCell;

use graph::leaf::{Leaf, leaf_str};
use crate::text::{Text, text, unit::list};
use super::{Html, html, App, tag::*, attribute::*};

pub fn doc() -> Element {
    Element::default()
}

impl App for Leaf<String> {
    fn text(&self) -> Text {
        text(self.clone())
    }
}

pub struct Element {
    tag: &'static Tag,
    root: Option<Box<RefCell<Element>>>, // Todo: change to Option<Html>?
    stems: Vec<Html>,
    attributes: Vec<Text>,
}

impl Element {
    pub fn string(&self) -> String { 
        self.text().string()
    }
    pub fn text(self) -> Text {
        html(self).text() // (self as &dyn App).text() 
    }
    pub fn leaf(&mut self, string: &str) -> &mut Self {
        self.stems.push(html(leaf_str(string)));
        self
    }
    pub fn root(self) -> Self {
        let mut root = self
            .root
            .as_ref()
            .expect("element should have a root")
            .replace(Element::default());
        root.stems.push(html(self));
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
    pub fn attribute(&mut self, name: &'static str, value: &str) -> &mut Self {
        self.attributes.push(text(leaf_str(&format!(r#"{name}="{value}""#))));
        self
    }
    pub fn stem(self, tag: &'static Tag) -> Self {
        Element {
            tag,
            root: Some(Box::new(RefCell::new(self))),
            stems: vec![],
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

impl App for Element {
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
        text(el)
    }
}

// fn attribute(name: &'static str, value: &str) -> Text {
//     text(leaf_str(&format!(r#"{name}="{value}""#)))
// }