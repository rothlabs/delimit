use super::*;
use atlas::*;
use graph::{BaseDown, Lake};
use std::{fs, io::BufReader};
use text::*;

#[tokio::test]
async fn write_and_read_serial_page() -> graph::Result<()> {
    let serial = html::default_bay()?.lake()?.serial()?;
    let path = STORAGE.to_owned() + "/page.json";
    fs::write(&path, serial)?;
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut lake: Lake = serde_json::from_reader(reader)?;
    lake.atlas(Box::new(Atlas::default()));
    let bay = lake.tree()?;
    bay.hydrate()?;
    let html = bay.get("page")?.string()?;
    assert_eq!(html.base().await?, html::default::PAGE);
    let plain = html.down(PLAIN).await?;
    bay.get("title_element")?.set(0, "html mutated")?;
    assert_eq!(
        html.base().await?,
        html::default::HTML_PAGE_WITH_MUTATED_TITLE
    );
    plain.get(1)?.get(1)?.get(1)?.set(1, "plain mutated")?;
    assert_eq!(
        plain.base().await?,
        html::default::PLAIN_PAGE_WITH_MUTATED_TITLE
    );
    Ok(())
}

// #[test]
// fn save_graph() -> result::Result<(), Error> {
//     let lake = make_doc().lake()?;
//     let serial = lake.serial()?.string()?;
//     fs::write(STORAGE, serial)?;
//     Ok(())
// }

// #[test]
// fn load_graph() -> result::Result<(), Error> {
//     let file = File::open(STORAGE)?;
//     let reader = BufReader::new(file);
//     let mut lake: Lake = serde_json::from_reader(reader)?;
//     lake.atlas(Atlas::new());
//     let hub = lake.root("root")?.hub()?;
//     hub.trade(&lake);
//     Ok(())
// }

// #[test]
// fn find_hub_in_loaded_repo() -> result::Result<(), Error> {
//     let path = STORAGE.leaf().hub();
//     let deserializer = HubDeserializer::new();
//     let repo = Repo::new().path(path).deserializer(deserializer).hub();
//     repo.alter().import()?;
//     if let Gain::Hub(_) = repo.query().find("Delimit index page")? {
//         Ok(())
//     } else {
//         Err("did not find hub in loaded repo")?
//     }
// }

// #[test]
// fn save_repo() -> result::Result<(), Error> {
//     let (bay, doc) = make_doc();
//     let plain = doc.at(PLAIN)?;
//     let mut hubes = vec![plain.clone()]; // doc.clone(),
//                                          // hubes.extend(doc.query().deep_stems()?);
//     hubes.extend(plain.query().deep_stems()?);
//     bay.alter().extend(hubes)?;
//     bay.query().export()?;
//     Ok(())
// }

// #[test]
// fn load_graph() -> result::Result<(), Error> {
//     let path = STORAGE.leaf().hub();
//     let deserializer = HubDeserializer::new();
//     let repo = Bay::new().path(path).deserializer(deserializer).hub();
//     repo.alter().import()?;
//     Ok(())
// }

// fn make_doc() -> Hub {
//     let atts = html::attribute_set();
//     let mut html = html::Doc::new(&atts, "Delimit index page").html();
//     html.attribute("lang", "en");
//     let mut title = html.head().title();
//     title.add_str("Delimit");
//     let mut meta = title.root().meta();
//     meta.attribute("charset", "utf-8");
//     meta = meta.root().meta();
//     meta.attribute("name", "viewport")
//         .attribute("content", "width=device-width, initial-scale=1");
//     meta = meta.root().meta();
//     meta.attribute("name", "author")
//         .attribute("content", "Roth Labs LLC");
//     let mut script = meta.root().script();
//     script.attribute("type", "importmap");
//     script.add_str(&r#"{"imports":{"init":"/client.js"}}"#);
//     let mut body = script.up_to_html().body();
//     body.add_str("Delimit");
//     let mut canvas = body.canvas();
//     canvas.attribute("id", "canvas");
//     let mut script = canvas.root().script();
//     script
//         .attribute("src", "/app.js")
//         .attribute("type", "module");
//     let doc = script.up_to_doc().hub();
//     doc
// }
