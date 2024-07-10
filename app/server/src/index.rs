use text::html::Doc;

#[cfg(test)]
mod tests;

pub fn index() -> String {
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
    let mut body = meta.up_to_html().body();
    body.add_str("Delimit");
    body.up_to_doc().string()
}
