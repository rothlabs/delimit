use super::*;

fn doc() -> Result<Apex, Error> {
    let tags = tags();
    let lang = Attribute::new().name("lang").content("en").apex();
    // let name = tags.get("!DOCTYPE html")?;
    let name = tags.get("html")?;
    let tag = Tag::new().name(&name).attribute(lang).apex();
    let html = Element::new().tag(tag).item("Roth Labs").close(name).apex();
    Ok(html)
}

#[test]
fn index_page_gain() -> Result<(), Error> {
    assert_eq!(DOC, doc()?.string()?);
    Ok(())
}

// /// The lower graph (plain) should rebut up to the doc (pipe)
// #[test]
// fn mutate_lower_graph_plain() -> Result<(), Error> {
//     let (_, doc, _, atts) = make_doc();
//     let plain = doc.at(PLAIN)?;
//     let _r = plain.tray()?;
//     let _r = doc.tray()?;
//     atts.get("type")
//         .unwrap()
//         .write(|tray| {
//             if let Tray::String(string) = tray {
//                 *string += "_mutated"
//             } else {
//                 panic!("not a string")
//             }
//         })
//         .ok();
//     //let string = doc.tray()?;
//     let string = plain.tray()?;
//     assert_eq!(Tray::String(MUTATED_ATTRIB.into()), string);
//     Ok(())
// }

// /// The upper graph (html) should rebut up to the doc (pipe)
// #[test]
// fn mutate_upper_graph_html() -> Result<(), Error> {
//     let (_, doc, head, _) = make_doc();
//     let plain = doc.at(PLAIN)?;
//     let _r = plain.tray()?;
//     let _r = doc.tray()?;
//     head.write(|pack| {
//         pack.unit.items.remove(0);
//     })
//     .ok();
//     let string = doc.tray()?;
//     //let string = plain.tray()?;
//     assert_eq!(Tray::String(REMOVED_TITLE.into()), string);
//     Ok(())
// }

const DOC: &str = r#"<!DOCTYPE html>
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

const MUTATED_ATTRIB: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<title>
Delimit
</title>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="author" content="Roth Labs LLC">
<script type_mutated="importmap">
{"imports":{"init":"/client.js"}}
</script>
</head>
<body>
Delimit
<canvas id="canvas">
</canvas>
<script src="/app.js" type_mutated="module">
</script>
</body>
</html>"#;

const REMOVED_TITLE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
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

// /// The goal is to get the sub plain graph to react
// #[test]
// fn mutate_plain() {
//     let (doc, _, atts) = make_doc();
//     let att = atts.get("type").unwrap();
//     let plain = doc.grant();
//     att.write(|unit| *unit += "_mutated");
//     let ace = plain.grant();
//     let string = ace.tray();
//     assert_eq!(MUTATED_ATTRIB, string);
// }

// let atts = attribute_set();
//     let mut html = Doc::new(&atts).html();
//     html.attribute("lang", "en");
//     let mut title = html.head().title();
//     title.add_str("Delimit");
//     let mut meta = title.root().meta();
//     meta.attribute("charset", "utf-8");
//     meta = meta.root().meta();
//     meta.attribute("name", "viewport")
//         .attribute("content", "width=device-width, initial-scale=1");
//     meta = meta.root().meta();
//     meta.attribute("name", "author").attribute("content", "Roth Labs LLC");
//     let mut script = meta.root().script();
//     script.attribute("type", "importmap");
//     script.add_str(&r#"{"imports":{"init":"/client.js"}}"#);
//     let mut body = script.up_to_html().body();
//     body.add_str("Delimit");
//     let mut canvas = body.canvas();
//     canvas.attribute("id", "canvas");
//     let mut script = canvas.root().script();
//     script.attribute("src", "/app.js").attribute("type", "module");
//     (script.up_to_doc().role(), atts)

// if let Part::Element(doc) = &doc.part {
//     doc.read(|unit|{
//         if let View::Role(html) = &unit.items[0] {
//             if let Part::Element(html) = &html.part {
//                 html.read(|unit|{
//                     if let View::Role(tag) = &unit.tag {
//                         let string = tag.grant().grant().tray();
//                         println!("html element: {string}");
//                     }
//                 });
//             }
//         }
//     });
// }

// let atts = attribute_set();
// let mut html = Doc::new(&atts).html();
// html.lang("en");
// let mut title = html.head().title();
// title.add_str("Delimit");
// let mut meta = title.root().meta();
// meta.charset("utf-8");
// meta = meta.root().meta();
// meta.name("viewport")
//     .content("width=device-width, initial-scale=1");
// meta = meta.root().meta();
// meta.name("author").content("Roth Labs LLC");
// let mut script = meta.root().script();
// script.r#type("importmap");
// script.add_str(&r#"{"imports":{"init":"/client.js"}}"#);
// let mut body = script.up_to_html().body();
// body.add_str("Delimit");
// let mut canvas = body.canvas();
// canvas.id("canvas");
// let mut script = canvas.root().script();
// script.src("/app.js").r#type("module");
// (script.up_to_doc().role(), atts)

// #[test]
// fn mutate_script_tag() {
//     let role = make_doc();
//     //println!("WOOOOOOOW!");
//     if let Part::Element(doc) = &role.part {
//         doc.read(|unit|{
//             //println!("reading element!");
//             if let View::Role(html) = &unit.items[0] {
//                 if let Part::Element(html) = &html.part {
//                     html.read(|unit|{
//                         if let View::Role(tag) = &unit.tag {
//                             let string = tag.grant().grant().tray();
//                             println!("html element: {string}");
//                         }
//                     });
//                 }
//             }
//         });
//     }
//     let string = role.grant().grant().tray();
//     assert_eq!(DOC_MUTATED_TAG, string);
// }
