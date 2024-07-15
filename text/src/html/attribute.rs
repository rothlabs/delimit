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
            part: Part::Attribute(link.clone()),
            form: link.ploy(),
        });
        Hold { link, role }
    }
}

impl Grant for Attribute {
    type Load = Load;
    fn grant(&self) -> Load {
        let Hold { link, role } = "".list();
        link.write(|Pack { unit, back }| {
            unit.items
                .back(back)
                .push(&self.name.grant())
                // .use_ploy(&self.name)
                .str(r#"=""#)
                .push(&self.value.grant())
                // .use_ploy(&self.value)
                .str(r#"""#);
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
