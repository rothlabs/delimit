use serde::Serialize;

use text::html::Doc;

use crate::{BOOT, INIT};

#[cfg(test)]
mod tests;

#[derive(Default, Serialize)]
pub struct Importmap {
    imports: Imports,
}

#[derive(Serialize)]
pub struct Imports {
    init: String,
}

impl Default for Imports {
    fn default() -> Self {
        Self {
            init: String::from(INIT),
        }
    }
}

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
    let mut script = meta.root().script();
    script.r#type("importmap");
    script.add_str(&serde_json::to_string(&Importmap::default()).unwrap());
    let mut body = script.up_to_html().body();
    body.add_str("Delimit");
    let mut script = body.script();
    script.src(BOOT).r#type("module");
    script.up_to_doc().string()
}
