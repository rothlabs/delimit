//use crate::plain::List;

use crate::html::*;

pub const LANG: &str = "lang";
pub const CHARSET: &str = "charset";
pub const NAME: &str = "name";
pub const CONTENT: &str = "content";

pub struct Attribute {
    name: Item,
    value: Item,
}

impl Solve for Attribute {
    type Load = Load;
    fn solve(&self) -> Self::Load {
        let (text, list) = "".list();
        list.writer(|pack|{
            pack.unit.items.reactor(pack.reactor)
                .add(&self.name)
                .add_str(r#"=""#)
                .add(&self.value);
        });
        text
    }
}

pub fn attribute() -> (plain::View<Exact>, Html<Attribute>) {
    let rust = Html::new(Attribute {
        name: plain::string("Untitled"),
        fields: vec![],
    });
    let role = plain::View::Role(Role {
        exact: Exact::Generics(rust.clone()),
        solver: rust.solver(),
    });
    (role, rust)
}
