use super::*;
use deserializer::NodeDeserializer;
use std::result;
use text::*;

fn make_doc() -> (Node, Node) {
    let path = STORAGE.leaf().node();
    let bay = Bay::new().path(path).node();
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
    (bay, doc)
}

#[test]
fn save_repo() -> result::Result<(), Error> {
    let (bay, doc) = make_doc();
    let plain = doc.at(PLAIN)?;
    let mut nodes = vec![plain.clone()]; // doc.clone(),
                                         // nodes.extend(doc.query().deep_stems()?);
    nodes.extend(plain.query().deep_stems()?);
    bay.alter().extend(nodes)?;
    bay.query().export()?;
    Ok(())
}

#[test]
fn load_repo() -> result::Result<(), Error> {
    let path = STORAGE.leaf().node();
    let deserializer = NodeDeserializer::new();
    let repo = Bay::new().path(path).deserializer(deserializer).node();
    repo.alter().import()?;
    Ok(())
}

// #[test]
// fn find_node_in_loaded_repo() -> result::Result<(), Error> {
//     let path = STORAGE.leaf().node();
//     let deserializer = NodeDeserializer::new();
//     let repo = Repo::new().path(path).deserializer(deserializer).node();
//     repo.alter().import()?;
//     if let Tray::Node(_) = repo.query().find("Delimit index page")? {
//         Ok(())
//     } else {
//         Err("did not find node in loaded repo".into())
//     }
// }
