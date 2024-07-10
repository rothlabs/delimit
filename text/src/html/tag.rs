use crate::html::*;

pub struct Tag {
    pub name: Item,
    pub attributes: Vec<Item>,
}

impl Tag {
    pub fn new(name: &Item) -> Hold<Html<Self>, Item> {
        let link = Html::new(Self {
            name: name.clone(),
            attributes: vec![],
        });
        let view = Item::Role(Role {
            exact: Exact::Tag(link.clone()),
            solver: link.solver(),
        });
        Hold { link, view }
    }
}

impl Solve for Tag {
    type Load = Load;
    fn solve(&self) -> Self::Load {
        let Hold { link, view } = "".list();
        link.writer(|pack| {
            let mut tag = pack.unit.items.root(pack.root);
            let Hold { link, view } = " ".list();
            link.writer(|pack| {
                let mut inner = pack.unit.items.root(pack.root);
                inner.add_item(&self.name);
                for att in &self.attributes {
                    inner.add_item(att);
                }
            });
            tag.add_str("<").add_role(&view).add_str(">");
        });
        view
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
pub const H1: &str = "h1";

pub const TAGS: [&str; 9] = [DOCTYPE, HTML, HEAD, TITLE, META, SCRIPT, BODY, DIV, H1];

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
