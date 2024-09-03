use super::*;
use atlas::*;
use graph::Lake;
use std::{fs, io::BufReader};
use text::*;

#[test]
fn write_and_read_serial_page() -> graph::Result<()> {
    let serial = html::default_bay()?.lake()?.serial()?.string()?;
    let path = STORAGE.to_owned() + "/page.json";
    fs::write(&path, serial)?;
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut lake: Lake = serde_json::from_reader(reader)?;
    lake.atlas(Box::new(Atlas::default()));
    let bay = lake.tree()?;
    bay.hydrate()?;
    let mut page = bay.get("page")?;
    assert_eq!(page.string()?, html::default::PAGE);
    page.set_rank()?;
    // let mut wow = page.get(0)?;

    // eprintln!("page part rank: {:?}", page.get(0)?.rank());

    // // bay.get("title_element")?.set(0, "html mutated")?;
    // page.get(0)?.get(0)?.get(0)?.set(0, "html mutated")?;
    // assert_eq!(page.string()?, html::default::HTML_PAGE_WITH_MUTATED_TITLE);

    let mut plain = page.down(PLAIN)?;
    // eprintln!("plain: {:?}", plain.get(1)?);
    plain.set_rank()?;
    plain.get(1)?.get(1)?.get(1)?.set(0, "plain mutated")?;
    assert_eq!(plain.string()?, html::default::PLAIN_PAGE_WITH_MUTATED_TITLE);
    // eprintln!("plain rank: {:?}", plain.rank());
    // eprintln!("plain.get(1)?: {:?}", plain.get(1)?.rank());
    // eprintln!("plain.get(1)?.get(1)?: {:?}", plain.get(1)?.get(1)?.rank());
    // eprintln!("plain.get(1)?.get(1)?.get(1)?: {:?}", plain.get(1)?.get(1)?.get(1)?.string());
    
    
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
//     let apex = lake.root("root")?.apex()?;
//     apex.trade(&lake);
//     Ok(())
// }

// #[test]
// fn find_apex_in_loaded_repo() -> result::Result<(), Error> {
//     let path = STORAGE.leaf().apex();
//     let deserializer = ApexDeserializer::new();
//     let repo = Repo::new().path(path).deserializer(deserializer).apex();
//     repo.alter().import()?;
//     if let Gain::Apex(_) = repo.query().find("Delimit index page")? {
//         Ok(())
//     } else {
//         Err("did not find apex in loaded repo")?
//     }
// }

// #[test]
// fn save_repo() -> result::Result<(), Error> {
//     let (bay, doc) = make_doc();
//     let plain = doc.at(PLAIN)?;
//     let mut apexes = vec![plain.clone()]; // doc.clone(),
//                                          // apexes.extend(doc.query().deep_stems()?);
//     apexes.extend(plain.query().deep_stems()?);
//     bay.alter().extend(apexes)?;
//     bay.query().export()?;
//     Ok(())
// }

// #[test]
// fn load_graph() -> result::Result<(), Error> {
//     let path = STORAGE.leaf().apex();
//     let deserializer = ApexDeserializer::new();
//     let repo = Bay::new().path(path).deserializer(deserializer).apex();
//     repo.alter().import()?;
//     Ok(())
// }

// fn make_doc() -> Apex {
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
//     let doc = script.up_to_doc().apex();
//     doc
// }
