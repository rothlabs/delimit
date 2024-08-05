use std::result;
use super::*;
use text::*;

fn make_doc() -> (Node, Node) {
    let path = STORAGE.leaf().node();
    let repo = Repo::new().path(path).node();
    let atts = html::attribute_set();
    let mut html = html::Doc::new(&repo, &atts).html();
    html.attribute("lang", "en");
    let mut title = html.head().title();
    title.add_str("Delimit");
    let mut meta = title.root().meta();
    meta.attribute("charset", "utf-8");
    meta = meta.root().meta();
    meta.attribute("name", "viewport")
        .attribute("content", "width=device-width, initial-scale=1");
    meta = meta.root().meta();
    meta.attribute("name", "author")
        .attribute("content", "Roth Labs LLC");
    let mut script = meta.root().script();
    script.attribute("type", "importmap");
    script.add_str(&r#"{"imports":{"init":"/client.js"}}"#);
    let mut body = script.up_to_html().body();
    body.add_str("Delimit");
    let mut canvas = body.canvas();
    canvas.attribute("id", "canvas");
    let mut script = canvas.root().script();
    script
        .attribute("src", "/app.js")
        .attribute("type", "module");
    let doc = script.up_to_doc().node();
    (repo, doc)
}

#[test]
fn save_repo() -> result::Result<(), Error> {
    let (repo, _) = make_doc();
    repo.query().cmd(SAVE)?;
    Ok(())
}

fn load_repo() -> result::Result<(), Error> {
    let path = STORAGE.leaf().node();
    let repo = Repo::new().path(path).node();
    repo.edit().cmd(LOAD)?;
    Ok(())
} 