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
        let role = Item::Role(Role {
            actual: Actual::Attribute(link.clone()),
            method: link.ploy(),
        });
        Hold { link, role }
    }
}

impl Grant for Attribute {
    type Load = Load;
    fn grant(&self) -> Load {
        let Hold { link, role } = "".list();
        link.writer(|Pack { unit, back }| {
            unit.items
                .back(back)
                .use_ploy(&self.name)
                .add_str(r#"=""#)
                .use_ploy(&self.value)
                .add_str(r#"""#);
        });
        role
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
