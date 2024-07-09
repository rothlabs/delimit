use crate::html::*;

pub const LANG: &str = "lang";
pub const CHARSET: &str = "charset";
pub const NAME: &str = "name";
pub const CONTENT: &str = "content";

pub struct Attribute {
    name: Item,
    value: Item,
}

impl Attribute {
    pub fn new() -> (Item, Html<Attribute>) {
        let exact = Html::new(Attribute {
            name: plain::string("untitled"),
            value: plain::string("empty"),
        });
        let role = Item::Role(Role {
            exact: Exact::Attribute(exact.clone()),
            solver: exact.solver(),
        });
        (role, exact)
    }
}

impl Solve for Attribute {
    type Load = Load;
    fn solve(&self) -> Self::Load {
        let (text, list) = "".list();
        list.writer(|pack| {
            pack.unit
                .items
                .root(pack.root)
                .add_view(&self.name)
                .add_str(r#"=""#)
                .add_view(&self.value)
                .add_str(r#"""#);
        });
        text
    }
}

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
