use std::borrow::Cow;

#[derive(Default)]
pub struct Tag {
    pub open: Cow<'static, str>,
    pub close: Cow<'static, str>,
}

pub const DOCTYPE: Tag = Tag {
    open: Cow::Borrowed("<!DOCTYPE html"),
    close: Cow::Borrowed(""),
};

pub const HTML: Tag = Tag {
    open: Cow::Borrowed("<html"),
    close: Cow::Borrowed("</html"),
};

pub const HEAD: Tag = Tag {
    open: Cow::Borrowed("<head"),
    close: Cow::Borrowed("</head>"),
};

pub const TITLE: Tag = Tag {
    open: Cow::Borrowed("<title"),
    close: Cow::Borrowed("</title>"),
};

pub const META: Tag = Tag {
    open: Cow::Borrowed("<meta"),
    close: Cow::Borrowed(""),
};

// pub const SCRIPT: Tag = Tag {
//     open:   Cow::Borrowed("<script"),
//     close: Cow::Borrowed("</script>"),
// };

pub const BODY: Tag = Tag {
    open: Cow::Borrowed("<body"),
    close: Cow::Borrowed("</body>"),
};

pub const DIV: Tag = Tag {
    open: Cow::Borrowed("<div"),
    close: Cow::Borrowed("</div>"),
};

pub const H1: Tag = Tag {
    open: Cow::Borrowed("<h1"),
    close: Cow::Borrowed("</h1>"),
};
