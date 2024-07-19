use super::*;

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

fn make_doc() -> Stem {
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
    script.up_to_doc().element()
}

#[test]
fn basic_doc() {

}
