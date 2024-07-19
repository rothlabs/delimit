use super::*;

fn make_doc() -> Role {
    let mut html = Doc::new().html();
    html.lang("en");
    let mut title = html.head().title();
    title.add_str("Delimit");
    let mut meta = title.root().meta();
    meta.charset("utf-8");
    meta = meta.root().meta();
    meta.name("viewport")
        .content("width=device-width, initial-scale=1");
    meta = meta.root().meta();
    meta.name("author").content("Roth Labs LLC");
    let mut script = meta.root().script();
    script.r#type("importmap");
    script.add_str(&r#"{"imports":{"init":"/client.js"}}"#);
    let mut body = script.up_to_html().body();
    body.add_str("Delimit");
    let mut canvas = body.canvas();
    canvas.id("canvas");
    let mut script = canvas.root().script();
    script.src("/app.js").r#type("module");
    script.up_to_doc().role()
}

#[test]
fn basic_doc() {
    let string = make_doc().grant().grant().load();
    assert_eq!(DOC, string);
}

#[test]
fn mutate_script_tag() {
    let role = make_doc();
    let plain = role.grant();
    let string = role.grant().grant().load();
    assert_eq!(DOC_MUTATED_TAG, string);
}

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

const DOC_MUTATED_TAG: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<title>
Delimit
</title>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="author" content="Roth Labs LLC">
<script_mutated type="importmap">
{"imports":{"init":"/client.js"}}
</script>
</head>
<body>
Delimit
<canvas id="canvas">
</canvas>
<script_mutated src="/app.js" type="module">
</script>
</body>
</html>"#;



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
//                             let string = tag.grant().grant().load();
//                             println!("html element: {string}");     
//                         }
//                     });
//                 }
//             }
//         });
//     }
//     let string = role.grant().grant().load();
//     assert_eq!(DOC_MUTATED_TAG, string);
// }