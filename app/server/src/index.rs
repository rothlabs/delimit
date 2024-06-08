use chronicle::html::doc;

pub fn index() -> String {
    let mut html = doc().html();
    html.lang("en");
    let mut title = html.head().title();
    title.leaf("Delimit");
    let mut meta = title.root().meta();
    meta.charset("utf-8");
    meta = meta.root().meta();
    meta.name("viewport")
        .content("width=device-width, initial-scale=1");
    meta = meta.root().meta();
    meta.name("author").content("Roth Labs LLC");
    meta.up_to_doc().string()
}
