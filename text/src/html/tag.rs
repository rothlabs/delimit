use super::*;

#[derive(Default, Clone)]
pub struct Tag {
    pub name: Node<String>,
    pub attributes: Vec<Node<String>>,
}

impl Tag {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(&mut self, name: impl Into<Node<String>>) -> &mut Self {
        self.name = name.into();
        self
    }
    pub fn attribute(&mut self, attribute: impl Into<Node<String>>) -> &mut Self {
        self.attributes.push(attribute.into());
        self
    }
}

impl Backed for Tag {
    fn backed(&self, back: &Back) -> Self {
        Self {
            name: self.name.backed(back),
            attributes: self.attributes.backed(back),
        }
    }
}

impl Grant for Tag {
    type Load = Node<String>;// node::Result<String>;
    fn grant(&self) -> Self::Load {
        let items = List::new()
            .separator(" ")
            .push(self.name.rank(1))
            .extend(self.attributes.rank(1))
            .node();
        let tag = List::new().push("<").push(items).push(">").node();
        // self.repo.field("nodes").insert(items).insert(tag);
        tag 
    }
}

pub const DOCTYPE: &str = "!DOCTYPE html";
pub const HTML: &str = "html";
pub const HEAD: &str = "head";
pub const TITLE: &str = "title";
pub const META: &str = "meta";
pub const SCRIPT: &str = "script";
pub const BODY: &str = "body";
pub const DIV: &str = "div";
pub const CANVAS: &str = "canvas";
pub const H1: &str = "h1";

pub const TAGS: [&str; 10] = [
    DOCTYPE, HTML, HEAD, TITLE, META, SCRIPT, BODY, DIV, CANVAS, H1,
];

// impl Tag {
//     pub fn hold(name: &Stem) -> Hold<Link<Self>, Role> {
//         let link = Link::make(|back| Self {
//             name: name.backed(back),
//             attributes: vec![],
//         });
//         let role = Role {
//             part: OldPart::Tag(link.clone()),
//             form: link.ploy(),
//         };
//         Hold { link, role }
//     }
// }

// impl Grant for Tag {
//     type Load = Load;
//     fn grant(&self) -> Self::Load {
//         let Hold { link, role } = "".list();
//         link.write(|pack| {
//             let mut tag = pack.unit.items.back(pack.back);
//             let Hold { link, role } = " ".list();
//             link.write(|Pack { unit, back }| {
//                 let mut inner = unit.items.back(back);
//                 inner.view(&self.name.grant());
//                 // inner.use_ploy(&self.name);
//                 for att in &self.attributes {
//                     inner.view(&att.grant());
//                     // inner.use_ploy(att);
//                 }
//             })
//             .ok();
//             tag.str("<").role(&role).str(">");
//         })
//         .ok();
//         role
//     }
// }

// #[derive(Default, Clone, Serialize)]
// pub struct TagName {
//     pub open: Cow<'static, str>,
//     pub close: Cow<'static, str>,
// }

// pub const DOCTYPE: TagName = TagName {
//     open: Cow::Borrowed("<!DOCTYPE html"),
//     close: Cow::Borrowed(""),
// };

// pub const HTML: TagName = TagName {
//     open: Cow::Borrowed("<html"),
//     close: Cow::Borrowed("</html"),
// };

// pub const HEAD: TagName = TagName {
//     open: Cow::Borrowed("<head"),
//     close: Cow::Borrowed("</head>"),
// };

// pub const TITLE: TagName = TagName {
//     open: Cow::Borrowed("<title"),
//     close: Cow::Borrowed("</title>"),
// };

// pub const META: TagName = TagName {
//     open: Cow::Borrowed("<meta"),
//     close: Cow::Borrowed(""),
// };

// // pub const SCRIPT: Tag = Tag {
// //     open:   Cow::Borrowed("<script"),
// //     close: Cow::Borrowed("</script>"),
// // };

// pub const BODY: TagName = TagName {
//     open: Cow::Borrowed("<body"),
//     close: Cow::Borrowed("</body>"),
// };

// pub const DIV: TagName = TagName {
//     open: Cow::Borrowed("<div"),
//     close: Cow::Borrowed("</div>"),
// };

// pub const H1: TagName = TagName {
//     open: Cow::Borrowed("<h1"),
//     close: Cow::Borrowed("</h1>"),
// };
