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
            exact: Exact::Attribute(link.clone()),
            solver: link.solver(),
        });
        Hold { link, view }
    }
}

impl Solve for Attribute {
    type Load = Load;
    fn solve(&self) -> Load {
        let Hold { link, view } = "".list();
        link.writer(|pack| {
            pack.unit
                .items
                .root(pack.root)
                .add_item(&self.name)
                .add_str(r#"=""#)
                .add_item(&self.value)
                .add_str(r#"""#);
        });
        view
    }
}

pub const LANG: &str = "lang";
pub const CHARSET: &str = "charset";
pub const NAME: &str = "name";
pub const CONTENT: &str = "content";

pub const ATTRIBUTES: [&str; 4] = [LANG, CHARSET, NAME, CONTENT];

// pub fn attribute() -> (Item, Html<Attribute>) {
//     let exact = Html::new(Attribute {
//         name: plain::string("untitled"),
//         value: plain::string("empty"),
//     });
//     let role = Item::Role(Role {
//         exact: Exact::Attribute(exact.clone()),
//         solver: exact.solver(),
//     });
//     (role, exact)
// }
