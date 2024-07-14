use crate::html::*;

pub struct Attribute {
    name: Item,
    value: Item,
}

impl Attribute {
    pub fn new(name: &Item, value: &Item) -> Hold<Html<Self>, Item> {
        let link = Html::new(Self {
            name: name.clone(),
            value: value.clone(),
        });
        let view = Item::Role(Role {
            actual: Actual::Attribute(link.clone()),
            method: link.ploy(),
        });
        Hold { link, view }
    }
}

impl Grant for Attribute {
    type Load = Load;
    fn grant(&self) -> Load {
        let Hold { link, view } = "".list();
        link.writer(|pack| {
            pack.unit
                .items
                .back(pack.back)
                .add_item(&self.name)
                .add_str(r#"=""#)
                .add_item(&self.value)
                .add_str(r#"""#);
        });
        view
    }
}

pub const ID: &str = "id";
pub const LANG: &str = "lang";
pub const CHARSET: &str = "charset";
pub const NAME: &str = "name";
pub const CONTENT: &str = "content";
pub const TYPE: &str = "type";
pub const SRC: &str = "src";

pub const ATTRIBUTES: [&str; 7] = [ID, LANG, CHARSET, NAME, CONTENT, TYPE, SRC];
