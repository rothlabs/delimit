use super::*;

pub fn default_bay() -> Result<Hub<()>> {
    let mut bay = Bay::new();

    let title = "Delimit".leaf().hub();
    bay.insert("title", title.clone())?;
    let title = title.pathed("title");

    let lang = Attribute::new().name("lang").content("en").hub()?;
    bay.insert("html", Tag::new().name("html").attribute(lang).hub()?)?;

    bay.insert("head", Tag::new().name("head").hub()?)?;

    bay.insert("body", Tag::new().name("body").hub()?)?;

    let tag = Tag::new().name("title").hub()?;
    let title_element = Element::new()
        .open(tag)
        .item(&title)
        .close()?
        .import(WORLD_ALL)
        .hub();
    bay.insert("title_element", title_element)?;

    let charset = Attribute::new().name("charset").content("utf-8").hub()?;
    bay.insert("charset", Tag::new().name("meta").attribute(charset).hub()?)?;

    let name = Attribute::new().name("name").content("viewport").hub()?;
    let content = Attribute::new()
        .name("content")
        .content("width=device-width, initial-scale=1")
        .hub()?;
    let viewport = Tag::new()
        .name("meta")
        .attribute(name)
        .attribute(content)
        .hub()?;
    bay.insert("viewport", viewport)?;

    let name = Attribute::new().name("name").content("author").hub()?;
    let content = Attribute::new()
        .name("content")
        .content("Roth Labs LLC")
        .hub()?;
    let author = Tag::new()
        .name("meta")
        .attribute(name)
        .attribute(content)
        .hub()?;
    bay.insert("author", author)?;

    let att = Attribute::new().name("type").content("importmap").hub()?;
    let tag = Tag::new().name("script").attribute(att).hub()?;
    let raw = serde_json::to_string(&Importmap::default())?;
    let importmap = Element::new().open(tag).item(raw).close()?.hub()?;
    bay.insert("importmap", importmap)?;

    let att = Attribute::new().name("id").content("canvas").hub()?;
    let tag = Tag::new().name("canvas").attribute(att).hub()?;
    let canvas = Element::new().open(tag).close()?.hub()?;
    bay.insert("canvas", canvas)?;

    let src = Attribute::new().name("src").content("/boot.js").hub()?;
    let module = Attribute::new().name("type").content("module").hub()?;
    let tag = Tag::new()
        .name("script")
        .attribute(src)
        .attribute(module)
        .hub()?;
    let boot = Element::new().open(tag).close()?.hub()?;
    bay.insert("boot", boot)?;

    let root = bay.hub()?;

    let page = page(&root)?;
    root.insert("page", page)?;

    Ok(root)
}

pub fn page(bay: &Hub<()>) -> Result<Hub<String>> {
    let head = Element::new()
        .open(bay.get("head")?.pathed("head").string()?)
        .item(bay.get("title_element")?.pathed("title_element").string()?)
        .item(bay.get("charset")?.pathed("charset").string()?)
        .item(bay.get("viewport")?.pathed("viewport").string()?)
        .item(bay.get("author")?.pathed("author").string()?)
        .item(bay.get("importmap")?.pathed("importmap").string()?)
        .close()?
        .import(WORLD_ALL)
        .hub();
    let body = Element::new()
        .open(bay.get("body")?.pathed("body").string()?)
        .item(bay.get("title")?.pathed("title").string()?)
        .item(bay.get("canvas")?.pathed("canvas").string()?)
        .item(bay.get("boot")?.pathed("boot").string()?)
        .close()?
        .import(WORLD_ALL)
        .hub();
    let html = Element::new()
        .open(bay.get("html")?.pathed("html").string()?)
        .item(head)
        .item(body)
        .close()?
        .import(WORLD_ALL)
        .hub();
    let tag = Tag::new().name("!DOCTYPE html").hub()?;
    let page = Element::new().open(tag).item(html).hub()?;
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
<script src="/boot.js" type="module">
</script>
</body>
</html>"#;

pub const HTML_PAGE_WITH_MUTATED_TITLE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<title>
html mutated
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
<script src="/boot.js" type="module">
</script>
</body>
</html>"#;

pub const PLAIN_PAGE_WITH_MUTATED_TITLE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<title>
plain mutated
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
<script src="/boot.js" type="module">
</script>
</body>
</html>"#;
