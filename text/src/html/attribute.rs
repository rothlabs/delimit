use crate::html::*;

#[derive(Default, Clone)]
pub struct Attribute {
    name: Value<String>,
    value: Value<String>,
}

impl Attribute {
    pub fn link(&self) -> Deuce<Self> {
        Deuce::make(|back| 
            Self { 
                name: self.name.backed(back),
                value: self.value.backed(back),
            }
        )
    }
    pub fn name(&mut self, name: impl Into<Value<String>>) -> &mut Self {
        self.name = name.into();
        self 
    }
    pub fn value(&mut self, value: impl Into<Value<String>>) -> &mut Self {
        self.value = value.into();
        self 
    }
}

impl Grant for Attribute {
    type Load = Ploy<Ace<String>>;
    fn grant(&self) -> Self::Load {
        List::new().item(&self.name).item(r#"=""#).item(&self.value).item(r#"""#).link().ploy()
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





// impl Attribute {
//     pub fn hold(name: impl Into<Stem>, value: impl Into<Stem>) -> Hold<Link<Self>, Role> {
//         let link = Link::make(|back| Self {
//             name: name.into().backed(back),
//             value: value.into().backed(back),
//         });
//         let role = Role {
//             part: OldPart::Attribute(link.clone()),
//             form: link.ploy(),
//         };
//         Hold { link, role }
//     }
// }

// impl Grant for Attribute {
//     type Load = Load;
//     fn grant(&self) -> Load {
//         List::role(|back| {
//             let items = ViewsBuilder::new(back)
//                 .view(&self.name.grant())
//                 .str(r#"=""#)
//                 .view(&self.value.grant())
//                 .str(r#"""#)
//                 .build();
//             List {
//                 items,
//                 separator: None,
//             }
//         })
//     }
// }














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
