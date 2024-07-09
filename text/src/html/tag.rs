use std::borrow::Cow;

use serde::Serialize;

use crate::html::*;

#[derive(Default, Clone, Serialize)]
pub struct TagName {
    pub open: Cow<'static, str>,
    pub close: Cow<'static, str>,
}

pub const DOCTYPE: TagName = TagName {
    open: Cow::Borrowed("<!DOCTYPE html"),
    close: Cow::Borrowed(""),
};

pub const HTML: TagName = TagName {
    open: Cow::Borrowed("<html"),
    close: Cow::Borrowed("</html"),
};

pub const HEAD: TagName = TagName {
    open: Cow::Borrowed("<head"),
    close: Cow::Borrowed("</head>"),
};

pub const TITLE: TagName = TagName {
    open: Cow::Borrowed("<title"),
    close: Cow::Borrowed("</title>"),
};

pub const META: TagName = TagName {
    open: Cow::Borrowed("<meta"),
    close: Cow::Borrowed(""),
};

// pub const SCRIPT: Tag = Tag {
//     open:   Cow::Borrowed("<script"),
//     close: Cow::Borrowed("</script>"),
// };

pub const BODY: TagName = TagName {
    open: Cow::Borrowed("<body"),
    close: Cow::Borrowed("</body>"),
};

pub const DIV: TagName = TagName {
    open: Cow::Borrowed("<div"),
    close: Cow::Borrowed("</div>"),
};

pub const H1: TagName = TagName {
    open: Cow::Borrowed("<h1"),
    close: Cow::Borrowed("</h1>"),
};

pub struct Tag {
    name: Item,
    attributes: Vec<Item>,
}

impl Tag {
    pub fn new() -> (Item, Html<Tag>) {
        let exact = Html::new(Tag {
            name: plain::string("untitled"),
            attributes: vec![],
        });
        let role = Item::Role(Role {
            exact: Exact::Tag(exact.clone()),
            solver: exact.solver(),
        });
        (role, exact)
    }
}

impl Solve for Tag {
    type Load = Load;
    fn solve(&self) -> Self::Load {
        let (text, list) = " ".list();
        list.writer(|pack| {
            let mut tag = pack.unit.items.root(pack.root);
            tag.add_str("<").add_view(&self.name);
            for att in &self.attributes {
                tag.add_view(att);
            }
            tag.add_str(r#">"#);
        });
        text
    }
}
