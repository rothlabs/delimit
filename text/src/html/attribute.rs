use crate::html::*;

#[derive(Default, Clone)]
pub struct Attribute {
    name: Node<String>,
    content: Node<String>,
}

impl Attribute {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn link(&self) -> Deuce<Self> {
        Deuce::make(|back| Self {
            name: self.name.backed(back),
            content: self.content.backed(back),
        })
    }
    pub fn value(&self) -> Node<String> {
        self.link().ploy().into()
    }
    pub fn name(&mut self, name: impl Into<Node<String>>) -> &mut Self {
        self.name = name.into();
        self
    }
    pub fn content(&mut self, content: impl Into<Node<String>>) -> &mut Self {
        self.content = content.into();
        self
    }
}

impl Grant for Attribute {
    type Load = Node<String>;
    fn grant(&self) -> Self::Load {
        List::new()
            .item(self.name.down(1))
            .item(r#"=""#)
            .item(self.content.down(1))
            .item(r#"""#)
            .value()
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
