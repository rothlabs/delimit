use super::*;

pub const PAGE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<title>
Delimit
</title>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="author" content="Roth Labs LLC">
<script type="importmap">
{"imports":{"init":"/client.js"}}
</script>
</head>
<body>
Delimit
<canvas id="canvas">
</canvas>
<script src="/app.js" type="module">
</script>
</body>
</html>"#;

pub fn default_bay() -> Result<Apex> {
    let mut bay = Bay::new();

    let title = "Delimit".leaf().apex();
    bay.insert("title", title.clone())?;
    let title = title.pathed("title");

    let lang = Attribute::new().name("lang").content("en").apex();
    bay.insert("html", Tag::new().name("html").attribute(lang).apex())?;

    bay.insert("head", Tag::new().name("head").apex())?;

    bay.insert("body", Tag::new().name("body").apex())?;

    let tag = Tag::new().name("title").apex();
    let title_element = Element::new()
        .open(tag)
        .item(&title)
        .close()?
        .import(WORLD_ALL)
        .apex();
    bay.insert("title_element", title_element)?;

    let charset = Attribute::new().name("charset").content("utf-8").apex();
    bay.insert("charset", Tag::new().name("meta").attribute(charset).apex())?;

    let name = Attribute::new().name("name").content("viewport").apex();
    let content = Attribute::new()
        .name("content")
        .content("width=device-width, initial-scale=1")
        .apex();
    let viewport = Tag::new()
        .name("meta")
        .attribute(name)
        .attribute(content)
        .apex();
    bay.insert("viewport", viewport)?;

    let name = Attribute::new().name("name").content("author").apex();
    let content = Attribute::new()
        .name("content")
        .content("Roth Labs LLC")
        .apex();
    let author = Tag::new()
        .name("meta")
        .attribute(name)
        .attribute(content)
        .apex();
    bay.insert("author", author)?;

    let att = Attribute::new().name("type").content("importmap").apex();
    let tag = Tag::new().name("script").attribute(att).apex();
    let raw = serde_json::to_string(&Importmap::default()).unwrap();
    let importmap = Element::new().open(tag).item(raw).close()?.apex();
    bay.insert("importmap", importmap)?;

    let att = Attribute::new().name("id").content("canvas").apex();
    let tag = Tag::new().name("canvas").attribute(att).apex();
    let canvas = Element::new().open(tag).close()?.apex();
    bay.insert("canvas", canvas)?;

    let src = Attribute::new().name("src").content("/app.js").apex();
    let module = Attribute::new().name("type").content("module").apex();
    let tag = Tag::new()
        .name("script")
        .attribute(src)
        .attribute(module)
        .apex();
    let app = Element::new().open(tag).close()?.apex();
    bay.insert("app", app)?;

    let root = bay.apex();

    let page = page(&root)?;
    root.insert("page", page)?;

    Ok(root)
}

pub fn page(bay: &Apex) -> Result<Apex> {
    let head = Element::new()
        .open(bay.get("head")?)
        .item(bay.get("title_element")?)
        .item(bay.get("charset")?)
        .item(bay.get("viewport")?)
        .item(bay.get("author")?)
        .item(bay.get("importmap")?)
        .close()?
        .import(WORLD_ALL)
        .apex();
    let body = Element::new()
        .open(bay.get("body")?)
        .item(bay.get("title")?)
        .item(bay.get("canvas")?)
        .item(bay.get("app")?)
        .close()?
        .import(WORLD_ALL)
        .apex();
    let html = Element::new()
        .open(bay.get("html")?)
        .item(head)
        .item(body)
        .close()?
        .import(WORLD_ALL)
        .apex();
    let tag = Tag::new().name("!DOCTYPE html").apex();
    let page = Element::new().open(tag).item(html).apex();
    Ok(page)
}

#[derive(Default, Serialize)]
pub struct Importmap {
    imports: Imports,
}

#[derive(Serialize)]
pub struct Imports {
    init: String,
}

impl Default for Imports {
    fn default() -> Self {
        Self {
            init: "/client.js".into(),
        }
    }
}
