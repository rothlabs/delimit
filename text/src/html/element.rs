use std::cell::RefCell;

use crate::html::*;
use crate::plain::List;

pub struct Element {
    tag: &'static Tag,
    root: Option<Box<RefCell<Element>>>, // Todo: change to Option<Html>?
    items: Vec<Item>,
    attributes: Vec<Attribute>,
}

impl Element {
    pub fn new() -> Self {
        Element::default()
    }
    fn write_open(&self, pack: &mut WriterPack<List>) {
        pack.unit.items.add_str(&self.tag.open);
        for att in self.attributes.iter() {
            att.collect(pack);
        }
        pack.unit.items.add_str(">");
    }
    fn write_items_and_close(&self, pack: &mut WriterPack<List>) {
        for item in self.items.iter() {
            item.collect(pack);
        }
        pack.unit.items.add_str(&self.tag.close);
    }
    pub fn add_str(&mut self, value: &str) -> &mut Self {
        self.items.push(Item::String(value.to_owned()));
        self
    }
    pub fn root(self) -> Self {
        let mut root = self
            .root
            .as_ref()
            .expect("element should have a root")
            .replace(Element::new());
        root.items.push(Item::Html(Html::new(self)));
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

impl Default for Element {
    fn default() -> Self {
        Self {
            tag: &DOCTYPE,
            root: None,
            items: vec![],
            attributes: vec![],
        }
    }
}

impl Solve for Element {
    type Load = plain::Role;
    fn solve(&self) -> Self::Load {
        let (open_tag, open_tag_list) = " ".list();
        open_tag_list.writer_pack(|pack| self.write_open(pack));
        let (text, text_list) = "\n".list();
        text_list.writer_pack(|pack| {
            pack.unit
                .items
                .add_role(&open_tag, pack.reactor);
            self.write_items_and_close(pack);
        });
        text
    }
}
