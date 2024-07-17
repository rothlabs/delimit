use super::*;

pub struct Tag {
    pub name: Stem,
    pub attributes: Vec<Stem>,
}

impl Tag {
    pub fn new(name: &Stem) -> Hold<Link<Self>, Stem> {
        let link = Link::new(Self {
            name: name.clone(),
            attributes: vec![],
        });
        let role = Stem::Role(Role {
            part: Part::Tag(link.clone()),
            form: link.ploy(),
        });
        Hold { link, role }
    }
}

impl Grant for Tag {
    type Load = Load;
    fn grant(&self) -> Self::Load {
        let Hold { link, role } = "".list();
        link.write(|pack| {
            let mut tag = pack.unit.items.back(pack.back);
            let Hold { link, role } = " ".list();
            link.write(|Pack { unit, back }| {
                let mut inner = unit.items.back(back);
                inner.push(&self.name.grant());
                // inner.use_ploy(&self.name);
                for att in &self.attributes {
                    inner.push(&att.grant());
                    // inner.use_ploy(att);
                }
            });
            tag.str("<").add_role(&role).str(">");
        });
        role
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
