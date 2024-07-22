use crate::html::*;

pub struct Attribute {
    name: Stem,
    value: Stem,
}

impl Attribute {
    pub fn hold(name: impl Into<Stem>, value: impl Into<Stem>) -> Hold<Link<Self>, Role> {
        let link = Link::make(|back| Self {
            name: name.into().backed(back),
            value: value.into().backed(back),
        });
        let role = Role {
            part: Part::Attribute(link.clone()),
            form: link.ploy(),
        };
        Hold { link, role }
    }
}

impl Grant for Attribute {
    type Load = Load;
    fn grant(&self) -> Load {
        List::role(|back| {
            let items = ViewsBuilder::new(back)
                .push(&self.name.grant())
                .str(r#"=""#)
                .push(&self.value.grant())
                .str(r#"""#)
                .build();
            List {
                items,
                separator: None,
            }
        })
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

// impl Grant for Attribute {
//     type Load = Load;
//     fn grant(&self) -> Load {
//         let Hold { link, role } = "".list();
//         link.write(|Pack { unit, back }| {
//             unit.items
//                 .back(back)
//                 .push(&self.name.grant())
//                 // .use_ploy(&self.name)
//                 .str(r#"=""#)
//                 .push(&self.value.grant())
//                 // .use_ploy(&self.value)
//                 .str(r#"""#);
//         });
//         role
//     }
// }

// impl Attribute {
//     pub fn hold(name: &Stem, value: &Stem) -> Hold<Link<Self>, Role> {
//         let link = Link::make(|back| Self {
//             name: name.backed(back),
//             value: value.backed(back),
//         });
//         let role = Role {
//             part: Part::Attribute(link.clone()),
//             form: link.ploy(),
//         };
//         Hold { link, role }
//     }
// }
