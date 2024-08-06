use super::*;
use text::*;
use deserializer::NodeDeserializer;
use std::result;

fn make_doc() -> (Node, Node) {
    let path = STORAGE.leaf().node();
    let repo = Repo::new().path(path).node();
    let atts = html::attribute_set();
    let mut html = html::Doc::new(&atts, "Delimit index page").html();
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
    let (repo, doc) = make_doc();
    let doc_stems = doc.query().stems()?;
    let plain = doc.at(PLAIN)?;
    let plain_stems = plain.query().stems()?;
    let mut alter = repo.alter();
    alter.insert(doc).extend(doc_stems);
    alter.insert(plain).extend(plain_stems);
    alter.run()?;
    repo.query().export()?;
    Ok(())
}

#[test]
fn load_repo() -> result::Result<(), Error> {
    let path = STORAGE.leaf().node();
    let deserializer = NodeDeserializer::new();
    let repo = Repo::new().path(path).deserializer(deserializer).node();
    repo.alter().import()?;
    // repo.query().cmd(SAVE)?;
    // Err("crap".into())
    Ok(())
}

#[test]
fn find_node_in_loaded_repo() -> result::Result<(), Error> {
    let path = STORAGE.leaf().node();
    let deserializer = NodeDeserializer::new();
    let repo = Repo::new().path(path).deserializer(deserializer).node();
    repo.alter().import()?;
    if let Tray::Node(node) = repo.query().find("Delimit index page")? {
        eprintln!("node: {:?}", node);
    }
    // Err("crap".into())
    Ok(())
}


